#!/usr/bin/env python3
"""
Teste de geração de áudio de 20s da voz do Mestre (Ana Florence)
e medição de latência inicial para streaming
"""

import sys
import os
import time
from pathlib import Path
from datetime import datetime

# Configurar encoding para UTF-8
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

script_dir = Path(__file__).parent
base_dir = script_dir.parent.parent.parent.parent

try:
    import soundfile as sf
    import numpy as np
    import torch
    import torchaudio
    from TTS.api import TTS
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

# Texto de ~20 segundos de áudio (aproximadamente 400-500 caracteres)
TEXT_20S = """In the depths of the forgotten library, ancient tomes whispered secrets to those who dared to listen. The air itself seemed to carry the weight of centuries, each breath tasting of dust and forgotten knowledge. As the scholar approached the central chamber, the very stones beneath their feet began to glow with an otherworldly light, revealing runes that had been hidden for millennia. The walls themselves seemed to pulse with a rhythm that matched the beating of a heart long since stilled, and shadows danced along the edges of vision, promising both revelation and ruin."""

def get_gpu_info():
    """Detecta informações da GPU"""
    if not torch.cuda.is_available():
        return None, None, None
    
    try:
        device_id = torch.cuda.current_device()
        gpu_name = torch.cuda.get_device_name(device_id)
        vram_total_gb = torch.cuda.get_device_properties(device_id).total_memory / (1024**3)
        compute_capability = torch.cuda.get_device_capability(device_id)
        return gpu_name, vram_total_gb, compute_capability
    except Exception as e:
        print(f"⚠️  Não foi possível obter informações detalhadas da GPU: {e}", file=sys.stderr)
        return None, None, None

def simulate_streaming_latency(tts, text, use_gpu):
    """Simula streaming e mede latência até primeiro chunk"""
    print("\n" + "="*70)
    print("  SIMULANDO STREAMING - MEDIÇÃO DE LATÊNCIA INICIAL")
    print("="*70)
    
    # Limpar cache CUDA
    if use_gpu and torch.cuda.is_available():
        torch.cuda.empty_cache()
    
    # Medir tempo até primeiro chunk estar pronto
    # (simulando chunking semântico de ~3-7s)
    chunk_size_chars = 200  # ~3-5s de áudio
    first_chunk_text = text[:chunk_size_chars]
    
    print(f"\n📝 Primeiro chunk (~{len(first_chunk_text)} caracteres):")
    print(f"   {first_chunk_text[:100]}...")
    
    # Medir latência inicial (tempo até primeiro chunk)
    start_time = time.time()
    
    try:
        # Gerar primeiro chunk
        audio = tts.tts(
            text=first_chunk_text,
            speaker="Ana Florence",
            language="en"
        )
        
        first_chunk_time = time.time() - start_time
        
        # Calcular duração do primeiro chunk
        sample_rate = tts.synthesizer.output_sample_rate
        first_chunk_duration = len(audio) / sample_rate
        
        print(f"\n✅ Primeiro chunk gerado!")
        print(f"   ⏱️  Tempo de geração: {first_chunk_time:.2f}s")
        print(f"   🎵 Duração do chunk: {first_chunk_duration:.2f}s")
        print(f"   ⚡ Latência inicial (texto → áudio): {first_chunk_time:.2f}s")
        
        return first_chunk_time, first_chunk_duration, audio, sample_rate
        
    except Exception as e:
        print(f"❌ Erro ao gerar primeiro chunk: {e}")
        import traceback
        traceback.print_exc()
        return None, None, None, None

def generate_full_audio(tts, text, use_gpu):
    """Gera áudio completo de 20s"""
    print("\n" + "="*70)
    print("  GERAÇÃO DE ÁUDIO COMPLETO (20s)")
    print("="*70)
    
    # Limpar cache CUDA
    if use_gpu and torch.cuda.is_available():
        torch.cuda.empty_cache()
    
    print(f"\n📝 Texto completo ({len(text)} caracteres):")
    print(f"   {text[:150]}...")
    
    # Medir tempo de geração
    start_time = time.time()
    
    try:
        audio = tts.tts(
            text=text,
            speaker="Ana Florence",
            language="en"
        )
        
        generation_time = time.time() - start_time
        
        # Calcular duração do áudio
        sample_rate = tts.synthesizer.output_sample_rate
        audio_duration = len(audio) / sample_rate
        
        # Calcular RTF
        rtf = generation_time / audio_duration if audio_duration > 0 else 0
        
        print(f"\n✅ Áudio gerado!")
        print(f"   ⏱️  Tempo de geração: {generation_time:.2f}s")
        print(f"   🎵 Duração do áudio: {audio_duration:.2f}s")
        print(f"   ⚡ Real-Time Factor: {rtf:.2f}x")
        print(f"   📊 Sample rate: {sample_rate} Hz")
        print(f"   📏 Amostras: {len(audio)}")
        
        # Verificar se é mono ou stereo
        if isinstance(audio, np.ndarray) and len(audio.shape) > 1:
            channels = audio.shape[0] if audio.shape[0] < audio.shape[1] else audio.shape[1]
            print(f"   🔊 Canais: {channels}")
            if channels > 1:
                print(f"   ⚠️  Áudio é stereo, convertendo para mono...")
                # Converter para mono (média dos canais)
                if audio.shape[0] > audio.shape[1]:
                    audio = np.mean(audio, axis=0)
                else:
                    audio = np.mean(audio, axis=1)
        else:
            print(f"   🔊 Canais: 1 (mono)")
        
        return audio, sample_rate, generation_time, audio_duration
        
    except Exception as e:
        print(f"❌ Erro ao gerar áudio: {e}")
        import traceback
        traceback.print_exc()
        return None, None, None, None

def main():
    print("\n" + "="*70)
    print("  TESTE: ÁUDIO DE 20s - VOZ DO MESTRE (ANA FLORENCE)")
    print("="*70)
    
    # Verificar GPU
    gpu_name, vram_gb, compute_capability = get_gpu_info()
    use_gpu = False
    
    if gpu_name:
        print(f"\n🎮 GPU disponível: {gpu_name}")
        if vram_gb:
            print(f"   💾 VRAM: {vram_gb:.1f} GB")
        if compute_capability:
            print(f"   🔧 Compute Capability: {compute_capability[0]}.{compute_capability[1]}")
            if compute_capability[0] >= 5:
                use_gpu = True
                print("   ✅ GPU será usada")
            else:
                print("   ⚠️  GPU com baixa capacidade, usando CPU")
        else:
            use_gpu = True
            print("   ✅ GPU será usada")
    else:
        print("\n❌ Nenhuma GPU CUDA disponível, usando CPU")
    
    # Carregar modelo XTTS
    print("\n📥 Carregando modelo XTTS v2...")
    print("   (Isso pode demorar ~1-2min na primeira vez)")
    try:
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=False)
        print("✅ Modelo XTTS carregado!")
    except Exception as e:
        print(f"❌ ERRO ao carregar modelo: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)
    
    # Teste 1: Medir latência inicial (streaming)
    first_chunk_time, first_chunk_duration, first_chunk_audio, sample_rate = simulate_streaming_latency(
        tts, TEXT_20S, use_gpu
    )
    
    if first_chunk_time is None:
        print("\n❌ Falha ao gerar primeiro chunk")
        sys.exit(1)
    
    # Teste 2: Gerar áudio completo
    full_audio, full_sample_rate, generation_time, audio_duration = generate_full_audio(
        tts, TEXT_20S, use_gpu
    )
    
    if full_audio is None:
        print("\n❌ Falha ao gerar áudio completo")
        sys.exit(1)
    
    # Garantir que é mono e 24kHz
    if isinstance(full_audio, np.ndarray):
        # Converter para mono se necessário
        if len(full_audio.shape) > 1:
            if full_audio.shape[0] > full_audio.shape[1]:
                full_audio = np.mean(full_audio, axis=0)
            else:
                full_audio = np.mean(full_audio, axis=1)
        
        # Resample para 24kHz se necessário
        if full_sample_rate != 24000:
            print(f"\n🔄 Re-amostrando de {full_sample_rate} Hz para 24000 Hz...")
            from scipy import signal
            num_samples = int(len(full_audio) * 24000 / full_sample_rate)
            full_audio = signal.resample(full_audio, num_samples)
            full_sample_rate = 24000
    
    # Salvar áudio
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = script_dir / "test_mestre_20s"
    output_dir.mkdir(exist_ok=True)
    
    # Salvar áudio completo (24kHz mono, Float32)
    output_path = output_dir / f"mestre_ana_florence_20s_{timestamp}.wav"
    sf.write(str(output_path), full_audio, 24000, subtype='FLOAT')
    
    print(f"\n💾 Áudio salvo em: {output_path}")
    print(f"   Formato: 24kHz, Mono, Float32")
    print(f"   Duração: {audio_duration:.2f}s")
    
    # Resumo final
    print("\n" + "="*70)
    print("  RESUMO")
    print("="*70)
    print(f"\n📊 Latência Inicial (Streaming):")
    print(f"   Tempo mínimo entre envio do texto e início da reprodução: {first_chunk_time:.2f}s")
    print(f"   Duração do primeiro chunk: {first_chunk_duration:.2f}s")
    print(f"   ✅ Primeiro chunk pronto em {first_chunk_time:.2f}s")
    
    print(f"\n📊 Áudio Completo:")
    print(f"   Duração: {audio_duration:.2f}s")
    print(f"   Tempo de geração: {generation_time:.2f}s")
    print(f"   RTF: {generation_time / audio_duration:.2f}x")
    print(f"   Sample rate: 24000 Hz (mono)")
    
    print(f"\n✅ Teste concluído!")
    print(f"   Arquivo: {output_path.name}")
    print("="*70 + "\n")

if __name__ == "__main__":
    main()

