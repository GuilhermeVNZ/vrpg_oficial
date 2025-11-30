#!/usr/bin/env python3
"""
Cria embedding XTTS com dataset limpo e normalizado
Processa, limpa e normaliza arquivos antes de criar o embedding
"""

import sys
import os
from pathlib import Path
import numpy as np

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent
sovits_dir = script_dir.parent.parent.parent.parent / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

try:
    import soundfile as sf
    import torchaudio
    from scipy import signal
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("   Instale: pip install soundfile scipy", file=sys.stderr)
    sys.exit(1)

# Monkey patch torchaudio.load para usar soundfile diretamente
original_torchaudio_load = torchaudio.load
def patched_torchaudio_load(filepath, *args, **kwargs):
    try:
        audio, sr = sf.read(filepath)
        if len(audio.shape) == 1:
            audio = audio.reshape(1, -1)
        import torch
        audio_tensor = torch.from_numpy(audio).float()
        return audio_tensor, sr
    except:
        return original_torchaudio_load(filepath, *args, **kwargs)

torchaudio.load = patched_torchaudio_load

# Dataset directory
dataset_dir = Path("G:/vrpg/vrpg-client/assets-and-models/models/tts/sovits/dataset/44k/dungeon_master_en")

# Lista de arquivos selecionados
selected_files = [
    "NewsP - Digits & Special Symbols - Large Numbers.wav",
    "NewsP - Digits & Special Symbols - Other Numbers.wav",
    "NewsP - Digits & Special Symbols - Phone Numbers.wav",
    "NewsP - Digits & Special Symbols - Symbols.wav",
    "NewsP - Exclamations.wav",
    "NewsP - Hesitations.wav",
    "NewsP - Rainbow Passage.wav",
    "Prompt-01.wav",
    "Prompt-02.wav",
    "Prompt-03.wav",
    "Prompt-04.wav",
    "Prompt-05.wav",
    "Prompt-06.wav",
    "Prompt-07.wav",
    "Prompt-08.wav",
    "Prompt-09.wav",
    "Prompt-10.wav",
    "Prompt-11.wav",
    "Prompt-12.wav",
    "Prompt-13.wav",
    "Prompt-14.wav",
    "Prompt-15.wav",
    "Prompt-16.wav",
    "Prompt-17.wav",
    "Prompt-18.wav",
    "Prompt-19.wav",
    "Prompt-20.wav",
]


def clean_audio(audio: np.ndarray, sr: int) -> np.ndarray:
    """
    Limpa e normaliza áudio:
    - Remove DC offset
    - Reduz ruído
    - Aplica filtros de limpeza
    - Normaliza volume
    """
    # Converter para mono se necessário
    if len(audio.shape) > 1:
        audio = np.mean(audio, axis=1)
    
    # 1. Remover DC offset
    audio = audio - np.mean(audio)
    
    # 2. Aplicar filtros de limpeza
    nyquist = sr / 2
    
    # High-pass filter (80Hz) - Remove ruídos de baixa frequência
    if nyquist > 80:
        high_cutoff = 80 / nyquist
        b_high, a_high = signal.butter(4, high_cutoff, btype='high')
        audio = signal.filtfilt(b_high, a_high, audio)
    
    # Low-pass filter (15kHz) - Remove ruídos de alta frequência
    if nyquist > 15000:
        low_cutoff = 15000 / nyquist
        b_low, a_low = signal.butter(4, low_cutoff, btype='low')
        audio = signal.filtfilt(b_low, a_low, audio)
    
    # 3. Redução de ruído usando filtro de média móvel suave
    # Remove ruído de fundo sem afetar a fala
    window_size = int(sr * 0.005)  # 5ms
    if window_size > 0 and len(audio) > window_size * 2:
        # Aplicar apenas levemente para não perder detalhes
        smoothed = np.convolve(audio, np.ones(window_size)/window_size, mode='same')
        audio = audio * 0.9 + smoothed * 0.1
    
    # 4. Normalizar volume (RMS normalization)
    # Calcular RMS (Root Mean Square)
    rms = np.sqrt(np.mean(audio**2))
    if rms > 0:
        # Normalizar para RMS de 0.1 (volume confortável)
        target_rms = 0.1
        audio = audio * (target_rms / rms)
    
    # 5. Normalizar pico (evitar clipping)
    max_val = np.max(np.abs(audio))
    if max_val > 0.95:
        audio = audio * (0.95 / max_val)
    
    return audio


def resample_audio(audio: np.ndarray, original_sr: int, target_sr: int) -> np.ndarray:
    """Re-amostra áudio para sample rate alvo"""
    if original_sr == target_sr:
        return audio
    
    # Calcular número de amostras no áudio de destino
    num_samples = int(len(audio) * target_sr / original_sr)
    
    # Re-amostrar usando scipy
    resampled = signal.resample(audio, num_samples)
    
    return resampled


def process_and_clean_files():
    """Processa, limpa e normaliza todos os arquivos selecionados"""
    print("\n" + "="*70)
    print("  PROCESSAMENTO E LIMPEZA DE ÁUDIOS PARA EMBEDDING XTTS")
    print("="*70 + "\n")
    
    if not dataset_dir.exists():
        print(f"❌ ERRO: Dataset não encontrado: {dataset_dir}")
        sys.exit(1)
    
    target_sr = 24000  # XTTS espera 24kHz
    processed_audios = []
    total_duration = 0.0
    max_total_duration = 600.0  # 10 minutos máximo
    max_segment_duration = 30.0  # Limitar segmentos a 30s
    
    print(f"📁 Processando {len(selected_files)} arquivos selecionados...")
    print(f"   Diretório: {dataset_dir}\n")
    
    for i, filename in enumerate(selected_files, 1):
        file_path = dataset_dir / filename
        
        if not file_path.exists():
            print(f"[{i}/{len(selected_files)}] ⚠️  Arquivo não encontrado: {filename}")
            continue
        
        if total_duration >= max_total_duration:
            print(f"\n⚠️  Limite de duração atingido ({max_total_duration:.0f}s). Parando...")
            break
        
        print(f"[{i}/{len(selected_files)}] Processando: {filename}...", end=" ", flush=True)
        
        try:
            # Carregar áudio
            audio, sr = sf.read(str(file_path))
            
            # Converter para float32
            audio = audio.astype(np.float32)
            
            # Limpar e normalizar
            audio_clean = clean_audio(audio, sr)
            
            # Re-amostrar para 24kHz se necessário
            if sr != target_sr:
                audio_clean = resample_audio(audio_clean, sr, target_sr)
                sr = target_sr
            
            # Limitar duração do segmento
            duration = len(audio_clean) / sr
            if duration > max_segment_duration:
                max_samples = int(max_segment_duration * sr)
                audio_clean = audio_clean[:max_samples]
                duration = len(audio_clean) / sr
                print(f"✂️  Cortado para {duration:.1f}s", end=" ")
            
            # Verificar se ainda cabe no limite total
            if total_duration + duration > max_total_duration:
                remaining = max_total_duration - total_duration
                if remaining > 5.0:  # Só adicionar se sobrar pelo menos 5s
                    max_samples = int(remaining * sr)
                    audio_clean = audio_clean[:max_samples]
                    duration = len(audio_clean) / sr
                    print(f"✂️  Ajustado para {duration:.1f}s (limite total)", end=" ")
                else:
                    print(f"⚠️  Pulado (limite atingido)")
                    break
            
            processed_audios.append(audio_clean)
            total_duration += duration
            print(f"✅ ({duration:.1f}s, total: {total_duration/60:.1f}min)")
            
        except Exception as e:
            print(f"❌ Erro: {str(e)[:50]}")
            continue
    
    if not processed_audios:
        print("\n❌ ERRO: Nenhum áudio foi processado com sucesso!")
        sys.exit(1)
    
    print(f"\n📊 Resultado do processamento:")
    print(f"   ✅ Arquivos processados: {len(processed_audios)}/{len(selected_files)}")
    print(f"   ✅ Duração total: {total_duration:.2f}s ({total_duration/60:.1f} minutos)")
    
    return processed_audios, target_sr


def create_consolidated_embedding(audio_segments: list, target_sr: int):
    """Cria arquivo de referência consolidado com crossfade"""
    print(f"\n🔄 Consolidando {len(audio_segments)} segmentos de áudio...")
    
    def apply_crossfade(audio1, audio2, fade_duration=0.1, sr=target_sr):
        """Aplica crossfade suave entre dois segmentos"""
        fade_samples = int(sr * fade_duration)
        
        if len(audio1) < fade_samples or len(audio2) < fade_samples:
            return np.concatenate([audio1, audio2])
        
        fade_out = np.linspace(1.0, 0.0, fade_samples)
        fade_in = np.linspace(0.0, 1.0, fade_samples)
        
        audio1_faded = audio1.copy()
        audio1_faded[-fade_samples:] *= fade_out
        
        audio2_faded = audio2.copy()
        audio2_faded[:fade_samples] *= fade_in
        
        result = np.concatenate([
            audio1_faded[:-fade_samples],
            audio1_faded[-fade_samples:] + audio2_faded[:fade_samples],
            audio2_faded[fade_samples:]
        ])
        
        return result
    
    # Normalizar cada segmento antes de concatenar
    print("   🔧 Normalizando segmentos individuais...")
    normalized_segments = []
    for segment in audio_segments:
        max_val = np.max(np.abs(segment))
        if max_val > 0:
            segment = segment * (0.95 / max_val)
        normalized_segments.append(segment)
    
    # Concatenar com crossfade
    consolidated_audio = normalized_segments[0]
    for i, segment in enumerate(normalized_segments[1:], 1):
        consolidated_audio = apply_crossfade(consolidated_audio, segment, fade_duration=0.1, sr=target_sr)
        if (i + 1) % 5 == 0:
            print(f"   ✅ Processados {i + 1}/{len(normalized_segments)} segmentos...")
    
    # Processamento final
    print("\n🔧 Aplicando processamento final...")
    
    # Remover DC offset
    consolidated_audio = consolidated_audio - np.mean(consolidated_audio)
    
    # Normalização final
    max_val = np.max(np.abs(consolidated_audio))
    if max_val > 0:
        consolidated_audio = consolidated_audio * (0.95 / max_val)
    
    # Fade in/out nas extremidades
    fade_samples = int(target_sr * 0.05)  # 50ms
    if len(consolidated_audio) > fade_samples * 2:
        fade_curve = np.linspace(0.0, 1.0, fade_samples)
        consolidated_audio[:fade_samples] *= fade_curve
        consolidated_audio[-fade_samples:] *= np.flip(fade_curve)
    
    final_duration = len(consolidated_audio) / target_sr
    print(f"✅ Áudio consolidado criado!")
    print(f"   Duração total: {final_duration:.2f}s ({final_duration/60:.1f} minutos)")
    print(f"   Sample rate: {target_sr} Hz")
    print(f"   Amostras: {len(consolidated_audio)}")
    print(f"   Normalizado: Sim (0.95 peak)")
    print(f"   Crossfade: Sim (100ms entre segmentos)")
    print(f"   Limpeza aplicada: Sim (filtros, redução de ruído, normalização)")
    
    return consolidated_audio


def main():
    """Função principal"""
    # Processar e limpar arquivos
    processed_audios, target_sr = process_and_clean_files()
    
    # Criar embedding consolidado
    consolidated_audio = create_consolidated_embedding(processed_audios, target_sr)
    
    # Salvar arquivo de referência
    output_path = script_dir / "dungeon_master_en_xtts_reference_clean.wav"
    
    print(f"\n💾 Salvando arquivo de referência limpo em: {output_path}")
    sf.write(str(output_path), consolidated_audio, target_sr)
    
    # Estatísticas finais
    print("\n" + "="*70)
    print("  RESULTADO FINAL")
    print("="*70)
    print(f"✅ Arquivo de referência limpo criado com sucesso!")
    print(f"✅ Arquivo salvo: {output_path.name}")
    print(f"✅ Duração: {len(consolidated_audio) / target_sr:.2f}s")
    print(f"✅ Sample rate: {target_sr} Hz")
    print(f"✅ Baseado em {len(processed_audios)} arquivos processados e limpos")
    print(f"\n📝 Para usar este arquivo com XTTS:")
    print(f"   tts = TTS('tts_models/multilingual/multi-dataset/xtts_v2')")
    print(f"   audio = tts.tts(")
    print(f"       text='Seu texto aqui',")
    print(f"       speaker_wav='{output_path.name}',  # Use este arquivo limpo")
    print(f"       language='en'")
    print(f"   )")
    print(f"\n📋 Processamento aplicado:")
    print(f"   - Limpeza de ruído (filtros high-pass 80Hz, low-pass 15kHz)")
    print(f"   - Normalização de volume (RMS + peak)")
    print(f"   - Remoção de DC offset")
    print(f"   - Crossfade entre segmentos")
    print(f"   - Padronização de qualidade")
    print("="*70 + "\n")


if __name__ == "__main__":
    main()



