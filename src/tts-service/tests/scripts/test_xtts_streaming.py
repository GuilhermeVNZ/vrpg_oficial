#!/usr/bin/env python3
"""
Teste de streaming XTTS - Gera e toca áudio conforme vai sendo gerado
"""

import sys
import os
from pathlib import Path
import numpy as np
import time
import re

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent
sovits_dir = script_dir.parent.parent.parent.parent / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

try:
    from TTS.api import TTS
    import torch
    import soundfile as sf
    import torchaudio
    try:
        import sounddevice as sd
        HAS_SOUNDDEVICE = True
    except ImportError:
        HAS_SOUNDDEVICE = False
        print("⚠️  sounddevice não instalado. Instale com: pip install sounddevice")
        print("   O áudio será salvo mas não tocado em tempo real.")
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    sys.exit(1)

# Fix para PyTorch 2.6+
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load

# Monkey patch torchaudio.load para usar soundfile
original_torchaudio_load = torchaudio.load
def patched_torchaudio_load(filepath, *args, **kwargs):
    try:
        audio, sr = sf.read(filepath)
        if len(audio.shape) == 1:
            audio = audio.reshape(1, -1)
        audio_tensor = torch.from_numpy(audio).float()
        return audio_tensor, sr
    except:
        return original_torchaudio_load(filepath, *args, **kwargs)

torchaudio.load = patched_torchaudio_load


def test_xtts_streaming():
    """Testa XTTS com streaming - gera e toca conforme vai gerando"""
    print("\n" + "="*70)
    print("  TESTE: XTTS Streaming - Gera e toca conforme vai gerando")
    print("="*70 + "\n")
    
    # Caminho do arquivo de referência
    reference_wav = script_dir / "dungeon_master_en_xtts_reference_clean.wav"
    
    if not reference_wav.exists():
        reference_wav = script_dir / "dungeon_master_en_xtts_reference.wav"
    
    if not reference_wav.exists():
        print(f"❌ ERRO: Arquivo de referência não encontrado: {reference_wav}")
        sys.exit(1)
    
    print(f"✅ Arquivo de referência encontrado: {reference_wav.name}\n")
    
    # Texto longo para testar streaming
    long_text = """In the depths of the ancient dungeon, shadows danced along the stone walls as torchlight flickered. The air was thick with the scent of damp earth and something else—something that made the hairs on the back of your neck stand on end. You could hear the distant echo of water dripping, each drop a reminder that you were far from the safety of the surface. The corridor stretched before you, disappearing into darkness, and you knew that whatever lay ahead would test not just your strength, but your very resolve. As you take another step forward, the ground beneath your feet shifts slightly, and you realize you're standing on something that isn't stone. Your torch reveals the glint of metal—an ancient trap, still active after all these years. You freeze, holding your breath, knowing that one wrong move could mean your end."""
    
    print(f"📖 Texto completo ({len(long_text)} caracteres):")
    print("-" * 70)
    print(long_text)
    print("-" * 70)
    
    # Carregar modelo XTTS
    print("\n📥 Carregando modelo XTTS v2...")
    try:
        use_gpu = torch.cuda.is_available()
        device = "cuda" if use_gpu else "cpu"
        print(f"   Dispositivo: {device}")
        
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=False)
        print("✅ Modelo XTTS carregado!\n")
    except Exception as e:
        print(f"❌ ERRO ao carregar modelo: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)
    
    # Dividir texto em sentenças
    sentences = re.split(r'([.!?]+\s+)', long_text)
    # Rejuntar sentenças com pontuação
    sentences_clean = []
    for i in range(0, len(sentences) - 1, 2):
        if i + 1 < len(sentences):
            sentences_clean.append(sentences[i] + sentences[i + 1])
        else:
            sentences_clean.append(sentences[i])
    
    sentences_clean = [s.strip() for s in sentences_clean if s.strip()]
    
    print(f"📝 Texto dividido em {len(sentences_clean)} sentenças:")
    for i, sent in enumerate(sentences_clean, 1):
        print(f"   {i}. {sent[:60]}...")
    print()
    
    # Configurar áudio para tocar
    sr = tts.synthesizer.output_sample_rate
    
    if HAS_SOUNDDEVICE:
        print("🔊 Reprodução em tempo real habilitada!\n")
    else:
        print("💾 Áudio será salvo (sem reprodução em tempo real)\n")
    
    # Lista para armazenar todos os chunks
    all_chunks = []
    total_generation_time = 0
    total_audio_duration = 0
    
    print("🎙️  Iniciando síntese em streaming...\n")
    print("="*70)
    
    # Gerar e tocar cada sentença
    for i, sentence in enumerate(sentences_clean, 1):
        print(f"\n📝 Sentença {i}/{len(sentences_clean)}: {sentence[:50]}...")
        
        # Medir tempo de geração
        start_time = time.time()
        
        try:
            # Gerar áudio para esta sentença
            audio = tts.tts(
                text=sentence,
                speaker_wav=str(reference_wav),
                language="en",
            )
            
            generation_time = time.time() - start_time
            
            # Converter para numpy
            if isinstance(audio, torch.Tensor):
                audio_np = audio.cpu().numpy().astype(np.float32)
            else:
                audio_np = np.array(audio, dtype=np.float32)
            
            if len(audio_np.shape) > 1:
                audio_np = audio_np.flatten()
            
            audio_duration = len(audio_np) / sr
            
            print(f"   ⏱️  Geração: {generation_time:.2f}s | Duração: {audio_duration:.2f}s | RTF: {generation_time/audio_duration:.2f}x")
            
            total_generation_time += generation_time
            total_audio_duration += audio_duration
            all_chunks.append(audio_np)
            
            # Tocar imediatamente (se sounddevice disponível)
            if HAS_SOUNDDEVICE:
                print(f"   🔊 Tocando agora...")
                try:
                    sd.play(audio_np, sr)
                    sd.wait()  # Esperar terminar de tocar
                except Exception as e:
                    print(f"   ⚠️  Erro ao tocar: {e}")
            
        except Exception as e:
            print(f"   ❌ Erro ao gerar sentença {i}: {e}")
            import traceback
            traceback.print_exc()
            continue
    
    print("\n" + "="*70)
    print("✅ STREAMING CONCLUÍDO!\n")
    
    # Estatísticas finais
    print("📊 Estatísticas:")
    print(f"   - Total de sentenças: {len(sentences_clean)}")
    print(f"   - Tempo total de geração: {total_generation_time:.2f}s")
    print(f"   - Duração total do áudio: {total_audio_duration:.2f}s")
    print(f"   - Real-time factor médio: {total_generation_time/total_audio_duration:.2f}x")
    print(f"   - Latência percebida: {total_generation_time:.2f}s (vs {total_audio_duration:.2f}s de áudio)")
    
    if total_generation_time < total_audio_duration:
        print(f"   ✅ Geração mais rápida que o áudio! (economia de {total_audio_duration - total_generation_time:.2f}s)")
    else:
        print(f"   ⚠️  Geração mais lenta que o áudio (atraso de {total_generation_time - total_audio_duration:.2f}s)")
    
    # Salvar áudio completo
    if all_chunks:
        print("\n💾 Salvando áudio completo...")
        audio_complete = np.concatenate(all_chunks)
        
        from datetime import datetime
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_path = script_dir / f"test_xtts_streaming_{timestamp}.wav"
        sf.write(str(output_path), audio_complete, sr, subtype='FLOAT')
        
        print(f"   ✅ Áudio salvo: {output_path.name}")
        print(f"   - Duração: {len(audio_complete) / sr:.2f}s")
        print(f"   - Sample rate: {sr} Hz")
    
    print("\n" + "="*70 + "\n")


if __name__ == "__main__":
    test_xtts_streaming()



