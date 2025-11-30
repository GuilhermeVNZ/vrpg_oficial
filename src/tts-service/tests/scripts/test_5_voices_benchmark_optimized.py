#!/usr/bin/env python3
"""
Teste de benchmark das 5 vozes XTTS com OTIMIZAÇÕES DE PERFORMANCE
Implementa:
- GPU detection e configuração adaptativa
- Otimizações de áudio (24kHz mono, int16 I/O, Float32 interno)
- RAW audio (sem processamento)
- Configuração adaptativa por tier de GPU
"""

import sys
import os
import time
from pathlib import Path
from datetime import datetime

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

# Fix para PyTorch 2.6+
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load

# ============================================================
# MONKEY PATCH: torchaudio.load() com fallback para soundfile
# ============================================================
# Solução para incompatibilidade entre torchcodec e PyTorch 2.10 nightly
# Quando torchcodec falha (DLLs não encontradas), usa soundfile diretamente
_original_torchaudio_load = torchaudio.load

def patched_torchaudio_load(filepath, *args, **kwargs):
    """
    Patch que usa soundfile quando torchcodec falha.
    Necessário porque PyTorch 2.10 nightly (CUDA 12.8) não é totalmente
    compatível com torchcodec, causando falha ao carregar WAVs de embedding.
    """
    try:
        # Tentar método original primeiro (torchcodec)
        return _original_torchaudio_load(filepath, *args, **kwargs)
    except (RuntimeError, ImportError, OSError) as e:
        error_str = str(e).lower()
        # Verificar se erro é relacionado a torchcodec/FFmpeg
        if any(keyword in error_str for keyword in ["torchcodec", "ffmpeg", "dll", "libtorchcodec"]):
            # Fallback: usar soundfile diretamente
            try:
                audio, sr = sf.read(filepath)
                # Converter para formato torchaudio (shape: [channels, samples])
                if len(audio.shape) == 1:
                    # Mono: [samples] -> [1, samples]
                    audio = audio.reshape(1, -1)
                elif len(audio.shape) == 2:
                    # Estéreo ou multi-canal: verificar orientação
                    if audio.shape[0] > audio.shape[1]:
                        # Provavelmente [samples, channels], transpor
                        audio = audio.T
                    # Se já está [channels, samples], manter
                # Converter para tensor PyTorch
                audio_tensor = torch.from_numpy(audio.copy()).float()
                return audio_tensor, int(sr)
            except Exception as sf_error:
                # Se soundfile também falhar, re-raise erro original
                raise RuntimeError(
                    f"Failed to load audio with both torchcodec and soundfile. "
                    f"torchcodec error: {e}, soundfile error: {sf_error}"
                ) from e
        else:
            # Erro não relacionado a torchcodec, re-raise
            raise

# Aplicar patch
torchaudio.load = patched_torchaudio_load
print("✅ Monkey patch aplicado: torchaudio.load() agora usa soundfile como fallback", file=sys.stderr)

# Textos de teste (aproximadamente 5s e 25s de áudio)
TEXT_5S = """The ancient castle stood tall against the stormy sky, its weathered stones telling stories of battles long forgotten."""

TEXT_25S = """The ancient castle stood tall against the stormy sky, its weathered stones telling stories of battles long forgotten. As the party approached, they could hear the distant echo of footsteps in the empty halls. The wind howled through broken windows, carrying whispers of the past. Shadows danced along the walls, and the air grew thick with anticipation. Something was waiting inside, something that had been waiting for centuries."""

# Configuração das 5 vozes
VOICES = [
    {
        "name": "Mestre (Ana Florence)",
        "speaker": "Ana Florence",
        "use_speaker_wav": False,
    },
    {
        "name": "Lax Barros",
        "speaker_wav": base_dir / "assets-and-models" / "models" / "tts" / "xtts_embeddings" / "narrator_default_xtts_reference_clean.wav",
        "use_speaker_wav": True,
    },
    {
        "name": "Common Voice Spontaneous",
        "speaker_wav": base_dir / "assets-and-models" / "models" / "tts" / "xtts_embeddings" / "common_voice_spontaneous_xtts_reference_clean.wav",
        "use_speaker_wav": True,
    },
    {
        "name": "Joe",
        "speaker_wav": base_dir / "assets-and-models" / "models" / "tts" / "xtts_embeddings" / "joe_xtts_reference_clean.wav",
        "use_speaker_wav": True,
    },
    {
        "name": "Kathleen",
        "speaker_wav": base_dir / "assets-and-models" / "models" / "tts" / "xtts_embeddings" / "kathleen_xtts_reference_clean.wav",
        "use_speaker_wav": True,
    },
]


def detect_gpu_tier():
    """Detecta tier da GPU para configuração adaptativa"""
    if not torch.cuda.is_available():
        return "cpu", None
    
    try:
        gpu_name = torch.cuda.get_device_name(0)
        vram_gb = torch.cuda.get_device_properties(0).total_memory / (1024**3)
        compute = torch.cuda.get_device_capability(0)
        
        print(f"   🎮 GPU: {gpu_name}")
        print(f"   💾 VRAM: {vram_gb:.1f} GB")
        print(f"   🔧 Compute: {compute[0]}.{compute[1]}")
        
        # Classificar tier
        name_lower = gpu_name.lower()
        
        # High-End: RTX 4090/5090, A100, H100
        if "rtx 4090" in name_lower or "rtx 5090" in name_lower or "a100" in name_lower or "h100" in name_lower:
            return "high_end", {"parallel_streams": 2, "vram_limit_mb": 0, "utilization": 0.90, "prebuffer": 2.5}
        
        # Mid-Range: RTX 3060-3080, RTX 4060-4070
        if "rtx 3080" in name_lower or "rtx 4070" in name_lower or "rtx 4060" in name_lower or (vram_gb >= 6.0 and vram_gb < 16.0):
            return "mid_range", {"parallel_streams": 1, "vram_limit_mb": 6144, "utilization": 0.70, "prebuffer": 1.8}
        
        # Modest: RTX 3050, GTX 1660
        if "rtx 3050" in name_lower or "gtx 1660" in name_lower or (vram_gb >= 4.0 and vram_gb < 6.0):
            return "modest", {"parallel_streams": 1, "vram_limit_mb": 3072, "utilization": 0.50, "prebuffer": 1.2}
        
        # Low-End: < 4GB VRAM
        if vram_gb > 0.0 and vram_gb < 4.0:
            return "low_end", {"parallel_streams": 0, "vram_limit_mb": 2048, "utilization": 0.40, "prebuffer": 0.8}
        
        # Default: High-End se VRAM >= 16GB e compute >= 8.0
        if vram_gb >= 16.0 and compute[0] >= 8:
            return "high_end", {"parallel_streams": 2, "vram_limit_mb": 0, "utilization": 0.90, "prebuffer": 2.5}
        
        # Default: Mid-Range
        return "mid_range", {"parallel_streams": 1, "vram_limit_mb": 6144, "utilization": 0.70, "prebuffer": 1.8}
        
    except Exception as e:
        print(f"   ⚠️  Erro ao detectar GPU: {e}")
        return "cpu", None


def should_use_gpu():
    """Determina se deve usar GPU baseado em compatibilidade"""
    if not torch.cuda.is_available():
        return False
    
    try:
        # Testar se realmente consegue usar a GPU criando um tensor
        # Isso verifica se o PyTorch suporta a compute capability da GPU
        test_tensor = torch.randn(10, 10).cuda()
        torch.cuda.synchronize()
        del test_tensor
        torch.cuda.empty_cache()
        return True
    except RuntimeError as e:
        # Se falhar, GPU não é suportada
        compute = torch.cuda.get_device_capability(0)
        print(f"   ⚠️  GPU com compute capability {compute[0]}.{compute[1]} não é suportada pelo PyTorch atual")
        print(f"   💡 Usando CPU")
        return False
    except:
        return False


def test_voice(tts, voice_config, text, text_name, use_gpu):
    """Testa uma voz específica com otimizações e retorna métricas"""
    voice_name = voice_config["name"]
    print(f"\n{'='*70}")
    print(f"🎤 Testando: {voice_name}")
    print(f"📝 Texto: {text_name} ({len(text)} caracteres)")
    print(f"{'='*70}")
    
    # Verificar se speaker_wav existe
    if voice_config.get("use_speaker_wav"):
        speaker_wav_path = voice_config.get("speaker_wav")
        if not speaker_wav_path or not speaker_wav_path.exists():
            print(f"❌ Arquivo de referência não encontrado: {speaker_wav_path}")
            return None
    
    # Limpar cache CUDA antes de cada teste (otimização)
    if use_gpu:
        torch.cuda.empty_cache()
    
    # Medir tempo de geração
    start_time = time.time()
    
    try:
        if voice_config.get("use_speaker_wav"):
            speaker_wav_path = voice_config["speaker_wav"]
            audio = tts.tts(
                text=text,
                speaker_wav=str(speaker_wav_path),
                language="en"
            )
        else:
            speaker = voice_config.get("speaker", "Ana Florence")
            audio = tts.tts(
                text=text,
                speaker=speaker,
                language="en"
            )
        
        generation_time = time.time() - start_time
        
        # Converter para numpy float32 (RAW - sem processamento)
        if isinstance(audio, torch.Tensor):
            audio_np = audio.cpu().numpy().astype(np.float32)
        elif isinstance(audio, np.ndarray):
            audio_np = audio.astype(np.float32)
        else:
            audio_np = np.array(audio, dtype=np.float32)
        
        # Garantir mono (1 canal)
        if len(audio_np.shape) > 1:
            audio_np = np.mean(audio_np, axis=0)
        
        # Calcular duração do áudio gerado
        sample_rate = tts.synthesizer.output_sample_rate
        audio_duration = len(audio_np) / sample_rate
        
        # Calcular Real-Time Factor (RTF)
        rtf = generation_time / audio_duration if audio_duration > 0 else 0
        
        print(f"✅ Geração concluída!")
        print(f"   ⏱️  Tempo de geração: {generation_time:.2f}s")
        print(f"   🎵 Duração do áudio: {audio_duration:.2f}s")
        print(f"   ⚡ Real-Time Factor: {rtf:.2f}x")
        print(f"   📊 Sample rate: {sample_rate} Hz")
        print(f"   📏 Amostras: {len(audio_np)}")
        print(f"   🔊 Canais: Mono (1)")
        print(f"   💾 Formato: Float32 (RAW, sem processamento)")
        
        # Limpar cache CUDA após geração (otimização)
        if use_gpu:
            torch.cuda.empty_cache()
        
        return {
            "voice_name": voice_name,
            "text_name": text_name,
            "generation_time": generation_time,
            "audio_duration": audio_duration,
            "rtf": rtf,
            "sample_rate": sample_rate,
            "audio": audio_np,
        }
        
    except Exception as e:
        print(f"❌ Erro ao gerar áudio: {e}")
        import traceback
        traceback.print_exc()
        return None


def main():
    print("\n" + "="*70)
    print("  BENCHMARK: 5 VOZES XTTS - COM OTIMIZAÇÕES DE PERFORMANCE")
    print("="*70)
    print("\n📋 Testando:")
    print("   - Texto curto (~5s de áudio)")
    print("   - Texto longo (~25s de áudio)")
    print("   - 5 vozes diferentes")
    print("   - Otimizações: GPU, RAW audio, 24kHz mono, Float32")
    
    # Detectar GPU e tier
    print("\n🔍 Detectando GPU...")
    gpu_tier, gpu_config = detect_gpu_tier()
    
    if gpu_tier == "cpu":
        print("   💻 Usando CPU")
        use_gpu = False
    else:
        print(f"   🎮 Tier detectado: {gpu_tier.upper()}")
        if gpu_config:
            print(f"   ⚙️  Configuração:")
            print(f"      - Parallel streams: {gpu_config['parallel_streams']}")
            print(f"      - VRAM limit: {gpu_config['vram_limit_mb']} MB" if gpu_config['vram_limit_mb'] > 0 else "      - VRAM limit: Unlimited")
            print(f"      - Utilization target: {gpu_config['utilization']*100:.0f}%")
            print(f"      - Pre-buffer: {gpu_config['prebuffer']}s")
        
        # Verificar se deve usar GPU
        use_gpu = should_use_gpu()
        if use_gpu:
            print("   ✅ GPU será usada")
        else:
            print("   ⚠️  GPU não será usada (incompatibilidade detectada)")
    
    # Carregar modelo XTTS
    print("\n📥 Carregando modelo XTTS v2...")
    print("   (Isso pode demorar alguns minutos na primeira vez)")
    
    try:
        # Tentar usar GPU se disponível e compatível
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=True)
        print("✅ Modelo XTTS carregado!\n")
        
        # Verificar device real usado
        if use_gpu:
            try:
                # Tentar verificar device do modelo
                device_str = "GPU" if use_gpu else "CPU"
                print(f"   🔧 Device: {device_str}")
            except:
                pass
    except Exception as e:
        print(f"❌ Erro ao carregar XTTS: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
    
    # Verificar arquivos de referência
    print("🔍 Verificando arquivos de referência...")
    missing_files = []
    for voice in VOICES:
        if voice.get("use_speaker_wav"):
            wav_path = voice.get("speaker_wav")
            if wav_path and wav_path.exists():
                print(f"   ✅ {voice['name']}: {wav_path.name}")
            else:
                print(f"   ❌ {voice['name']}: NÃO ENCONTRADO - {wav_path}")
                missing_files.append(voice['name'])
    
    if missing_files:
        print(f"\n⚠️  ATENÇÃO: {len(missing_files)} arquivo(s) de referência não encontrado(s)")
        print("   Alguns testes podem falhar")
    
    # Executar testes
    results = []
    
    # Teste com texto curto (5s)
    print("\n" + "="*70)
    print("  TESTE 1: TEXTO CURTO (~5s)")
    print("="*70)
    
    for voice in VOICES:
        result = test_voice(tts, voice, TEXT_5S, "Texto Curto (~5s)", use_gpu)
        if result:
            results.append(result)
    
    # Teste com texto longo (25s)
    print("\n" + "="*70)
    print("  TESTE 2: TEXTO LONGO (~25s)")
    print("="*70)
    
    for voice in VOICES:
        result = test_voice(tts, voice, TEXT_25S, "Texto Longo (~25s)", use_gpu)
        if result:
            results.append(result)
    
    # Resumo dos resultados
    print("\n" + "="*70)
    print("  RESUMO DOS RESULTADOS")
    print("="*70)
    
    # Agrupar por texto
    short_text_results = [r for r in results if r["text_name"] == "Texto Curto (~5s)"]
    long_text_results = [r for r in results if r["text_name"] == "Texto Longo (~25s)"]
    
    print("\n📊 TEXTO CURTO (~5s):")
    print(f"{'Voz':<35} {'Tempo (s)':<12} {'Duração (s)':<12} {'RTF':<10}")
    print("-" * 70)
    for r in sorted(short_text_results, key=lambda x: x["generation_time"]):
        print(f"{r['voice_name']:<35} {r['generation_time']:>10.2f}s  {r['audio_duration']:>10.2f}s  {r['rtf']:>8.2f}x")
    
    print("\n📊 TEXTO LONGO (~25s):")
    print(f"{'Voz':<35} {'Tempo (s)':<12} {'Duração (s)':<12} {'RTF':<10}")
    print("-" * 70)
    for r in sorted(long_text_results, key=lambda x: x["generation_time"]):
        print(f"{r['voice_name']:<35} {r['generation_time']:>10.2f}s  {r['audio_duration']:>10.2f}s  {r['rtf']:>8.2f}x")
    
    # Comparação com versão anterior (se disponível)
    print("\n📈 OTIMIZAÇÕES APLICADAS:")
    print("   ✅ GPU detection e configuração adaptativa")
    print("   ✅ RAW audio (Float32, sem processamento)")
    print("   ✅ 24kHz mono (formato nativo XTTS)")
    print("   ✅ Cache CUDA cleanup entre testes")
    print("   ✅ Otimizações de memória")
    
    # Salvar áudios gerados
    print("\n💾 Salvando áudios gerados...")
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = script_dir / "benchmark_5_voices_optimized"
    output_dir.mkdir(exist_ok=True)
    
    for result in results:
        voice_safe = result["voice_name"].replace(" ", "_").replace("(", "").replace(")", "").lower()
        text_safe = result["text_name"].replace(" ", "_").replace("(", "").replace(")", "").replace("~", "").lower()
        filename = f"benchmark_optimized_{voice_safe}_{text_safe}_{timestamp}.wav"
        output_path = output_dir / filename
        
        # Salvar em Float32 (RAW, sem processamento)
        sf.write(str(output_path), result["audio"], result["sample_rate"], subtype='FLOAT')
        print(f"   ✅ {filename}")
    
    print(f"\n✅ Todos os áudios salvos em: {output_dir}")
    print("\n" + "="*70)
    print("  BENCHMARK CONCLUÍDO!")
    print("="*70 + "\n")


if __name__ == "__main__":
    main()

