#!/usr/bin/env python3
"""
Gera APENAS os áudios problemáticos corrigidos
Versão com prompts otimizados para evitar ruídos, repetições e cortes estranhos
"""

import sys
import os
import time
from pathlib import Path

# Configurar encoding para UTF-8
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

script_dir = Path(__file__).parent
base_dir = script_dir.parent.parent.parent.parent

try:
    import soundfile as sf
    import torch
    import torchaudio
    from TTS.api import TTS
    import numpy as np
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("   Instale: pip install TTS soundfile torch torchaudio", file=sys.stderr)
    sys.exit(1)

# --- Fix para PyTorch 2.6+ ---
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load

# --- Monkey Patch para torchaudio.load ---
_original_torchaudio_load = torchaudio.load

def patched_torchaudio_load(filepath, *args, **kwargs):
    try:
        return _original_torchaudio_load(filepath, *args, **kwargs)
    except (RuntimeError, ImportError, OSError) as e:
        error_str = str(e).lower()
        if any(keyword in error_str for keyword in ["torchcodec", "ffmpeg", "dll", "libtorchcodec"]):
            try:
                audio, sr = sf.read(filepath)
                if len(audio.shape) == 1:
                    audio = audio.reshape(1, -1)
                elif len(audio.shape) == 2 and audio.shape[0] > audio.shape[1]:
                    audio = audio.T
                audio_tensor = torch.from_numpy(audio.copy()).float()
                return audio_tensor, int(sr)
            except Exception as sf_error:
                raise RuntimeError(
                    f"Failed to load audio with both torchcodec and soundfile. "
                    f"torchcodec error: {e}, soundfile error: {sf_error}"
                ) from e
        else:
            raise

torchaudio.load = patched_torchaudio_load
# --- Fim Monkey Patch ---

# APENAS os 4 arquivos problemáticos restantes com textos otimizados
# Estratégia: textos curtos, pontuação final clara, sem reticências que causam ruído
FIX_TEXTS = {
    "dm_hmm_03": "Hmm",  # Sem pontuação (versão alternativa) - melhorar limpeza final
    "dm_so_01": "So,",  # Adicionar vírgula para evitar pronúncia com Z
    "dm_so_03": "So,",  # Segunda versão com vírgula
    "dm_uh_01": "Uh",  # Sem pontuação - melhorar corte final
}

def generate_fixes():
    """Gera APENAS os arquivos problemáticos corrigidos"""
    print("\n" + "="*70)
    print("  CORREÇÃO DE INTERJEIÇÕES PROBLEMÁTICAS")
    print("="*70)
    
    # Verificar GPU
    use_gpu = torch.cuda.is_available()
    if use_gpu:
        gpu_name = torch.cuda.get_device_name(0)
        print(f"\n🎮 GPU: {gpu_name}")
    else:
        print("\n💻 Usando CPU")
    
    # Carregar modelo XTTS
    print("\n📥 Carregando modelo XTTS v2...")
    try:
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=False)
        print("✅ Modelo XTTS carregado!")
        
        # Configurar FP16
        if use_gpu and torch.cuda.is_available():
            try:
                if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
                    tts.synthesizer.model = tts.synthesizer.model.half().cuda()
                    print("✅ Modelo configurado para FP16")
            except Exception as e:
                print(f"⚠️  Não foi possível configurar FP16: {e}")
        
        # Warm-up
        print("🔥 Executando warm-up...")
        with torch.inference_mode():
            _ = tts.tts("Warmup", speaker="Ana Florence", language="en")
        print("✅ Warm-up concluído")
        
    except Exception as e:
        print(f"❌ ERRO ao carregar modelo: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Criar diretório de saída (subpasta dentro de interjections)
    output_dir = Path(r"G:\vrpg\vrpg-client\assets-and-models\voices\interjections\fix")
    output_dir.mkdir(parents=True, exist_ok=True)
    print(f"\n📁 Diretório de saída: {output_dir}")
    
    # Gerar apenas os arquivos problemáticos
    print(f"\n🎵 Gerando {len(FIX_TEXTS)} arquivos corrigidos...\n")
    
    results = []
    
    for clip_id, text in FIX_TEXTS.items():
        print(f"🎤 {clip_id}: '{text}'")
        
        try:
            # Gerar áudio
            start_time = time.time()
            with torch.inference_mode():
                audio = tts.tts(text, speaker="Ana Florence", language="en")
            gen_time = time.time() - start_time
            
            # Converter para numpy se necessário
            if isinstance(audio, list):
                audio = np.array(audio)
            
            # Converter para mono se necessário
            if len(audio.shape) > 1:
                audio = np.mean(audio, axis=0)
            
            # Obter sample rate
            sample_rate = tts.synthesizer.output_sample_rate
            
            # Calcular duração
            duration = len(audio) / sample_rate
            
            # Remover silêncio final excessivo e ruídos (mais agressivo)
            # Isso ajuda a evitar ruídos, cortes estranhos e sons aleatórios
            threshold = 0.005  # Threshold mais baixo para detectar ruídos sutis
            silence_samples = int(sample_rate * 0.2)  # 0.2s de silêncio máximo (mais agressivo)
            
            # Encontrar último ponto com áudio significativo
            abs_audio = np.abs(audio)
            last_non_silent = len(audio) - 1
            
            # Procurar do final para o início, encontrando o último som significativo
            for i in range(len(audio) - 1, max(0, len(audio) - silence_samples * 2), -1):
                # Verificar janela de 0.05s para evitar ruídos pontuais
                window_size = int(sample_rate * 0.05)
                window_start = max(0, i - window_size)
                window_avg = np.mean(abs_audio[window_start:i+1])
                
                if window_avg > threshold:
                    last_non_silent = i
                    break
            
            # Adicionar fade-out mais longo (0.1s) para evitar corte abrupto e ruídos
            fade_samples = int(sample_rate * 0.1)
            fade_start = max(0, last_non_silent - fade_samples)
            fade_end = min(len(audio), last_non_silent + int(sample_rate * 0.05))
            
            # Aplicar fade-out suave
            if fade_end > fade_start:
                fade_curve = np.linspace(1.0, 0.0, fade_end - fade_start)
                audio[fade_start:fade_end] *= fade_curve
            
            # Cortar silêncio final excessivo (deixar apenas 0.05s para evitar ruídos)
            final_silence = int(sample_rate * 0.05)
            audio = audio[:min(len(audio), last_non_silent + final_silence)]
            
            duration = len(audio) / sample_rate
            print(f"   ✅ Duração: {duration:.2f}s (após limpeza)")
            
            # Salvar arquivo
            output_file = output_dir / f"{clip_id}.wav"
            
            # Salvar como Float32 mono (qualidade máxima)
            sf.write(
                str(output_file),
                audio.astype(np.float32),
                sample_rate,
                subtype='FLOAT'
            )
            
            print(f"   💾 Salvo: {output_file}")
            print(f"   ⏱️  Tempo de geração: {gen_time:.3f}s\n")
            
            results.append({
                'id': clip_id,
                'text': text,
                'duration': duration,
                'file': str(output_file),
                'gen_time': gen_time,
            })
            
        except Exception as e:
            print(f"   ❌ Erro ao gerar {clip_id}: {e}\n")
            import traceback
            traceback.print_exc()
    
    # Resumo
    print("="*70)
    print("  RESUMO")
    print("="*70)
    
    total_duration = sum(r['duration'] for r in results)
    total_gen_time = sum(r['gen_time'] for r in results)
    avg_duration = total_duration / len(results) if results else 0
    max_duration_found = max((r['duration'] for r in results), default=0)
    min_duration_found = min((r['duration'] for r in results), default=0)
    
    print(f"\n📊 Estatísticas:")
    print(f"   Total de arquivos corrigidos: {len(results)}")
    print(f"   Duração média: {avg_duration:.2f}s")
    print(f"   Duração mínima: {min_duration_found:.2f}s")
    print(f"   Duração máxima: {max_duration_found:.2f}s")
    print(f"   Total de áudio: {total_duration:.2f}s")
    print(f"   Tempo total de geração: {total_gen_time:.2f}s")
    
    print(f"\n📁 Arquivos salvos em: {output_dir}")
    print("\n" + "="*70 + "\n")
    
    return results

if __name__ == "__main__":
    results = generate_fixes()
    
    # Criar arquivo de resumo
    summary_file = Path(__file__).parent / "interjections_fix_summary.md"
    with open(summary_file, 'w', encoding='utf-8') as f:
        f.write("# Interjeições Corrigidas\n\n")
        f.write(f"**Data**: {time.strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        f.write(f"**Total**: {len(results)} arquivos corrigidos\n\n")
        f.write("| ID | Texto | Duração (s) | Arquivo |\n")
        f.write("|----|-------|-------------|----------|\n")
        for r in results:
            f.write(f"| {r['id']} | {r['text']} | {r['duration']:.2f} | {Path(r['file']).name} |\n")
    
    print(f"📄 Resumo salvo em: {summary_file}")

