#!/usr/bin/env python3
"""
Benchmark comparativo CPU vs GPU para todas as 5 vozes XTTS
Usa um novo parágrafo e compara performance lado a lado
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
_original_torchaudio_load = torchaudio.load

def patched_torchaudio_load(filepath, *args, **kwargs):
    """Patch que usa soundfile quando torchcodec falha"""
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

# Novo parágrafo para teste
TEXT_NOVO = """In the depths of the forgotten library, ancient tomes whispered secrets to those who dared to listen. The air itself seemed to carry the weight of centuries, each breath tasting of dust and forgotten knowledge. As the scholar approached the central chamber, the very stones beneath their feet began to glow with an otherworldly light, revealing runes that had been hidden for millennia."""

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

def test_voice(tts, voice_config, text, use_gpu):
    """Testa uma voz específica e retorna métricas"""
    voice_name = voice_config["name"]
    
    # Verificar se speaker_wav existe
    if voice_config.get("use_speaker_wav"):
        speaker_wav_path = voice_config.get("speaker_wav")
        if not speaker_wav_path or not speaker_wav_path.exists():
            return None
    
    # Limpar cache CUDA antes de cada teste
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
        
        # Calcular duração do áudio gerado
        sample_rate = tts.synthesizer.output_sample_rate
        audio_duration = len(audio) / sample_rate
        
        # Calcular Real-Time Factor (RTF)
        rtf = generation_time / audio_duration if audio_duration > 0 else 0
        
        return {
            "voice_name": voice_name,
            "generation_time": generation_time,
            "audio_duration": audio_duration,
            "rtf": rtf,
            "sample_rate": sample_rate,
            "audio": audio,
            "success": True,
        }
        
    except Exception as e:
        return {
            "voice_name": voice_name,
            "error": str(e),
            "success": False,
        }

def run_benchmark(tts, use_gpu, device_name):
    """Executa benchmark completo para CPU ou GPU usando modelo já carregado"""
    print(f"\n{'='*80}")
    print(f"  BENCHMARK: {device_name.upper()}")
    print(f"{'='*80}")
    
    # Nota: TTS.api não suporta mover modelo entre devices facilmente
    # Vamos usar o modelo como está (já carregado com GPU se disponível)
    # Para CPU, precisamos recarregar ou aceitar que vai usar GPU mesmo
    
    results = []
    
    print(f"\nTestando {len(VOICES)} vozes com novo paragrafo...")
    print(f"Texto: {len(TEXT_NOVO)} caracteres\n")
    
    for i, voice_config in enumerate(VOICES, 1):
        print(f"[{i}/{len(VOICES)}] {voice_config['name']}...", end=" ", flush=True)
        result = test_voice(tts, voice_config, TEXT_NOVO, use_gpu)
        
        if result and result.get("success"):
            print(f"OK {result['generation_time']:.2f}s (RTF: {result['rtf']:.2f}x)")
            results.append(result)
        else:
            error = result.get("error", "Erro desconhecido") if result else "Falha"
            print(f"ERRO {error[:50]}")
            results.append({
                "voice_name": voice_config["name"],
                "success": False,
                "error": error
            })
    
    return results

def compare_results(cpu_results, gpu_results):
    """Compara resultados CPU vs GPU"""
    print("\n" + "="*100)
    print("  COMPARAÇÃO: CPU vs GPU")
    print("="*100)
    print()
    print(f"{'Voz':<45} {'CPU':<25} {'GPU':<25} {'Melhoria':<15}")
    print("-" * 100)
    
    for voice_config in VOICES:
        voice_name = voice_config["name"]
        
        # Encontrar resultados
        cpu_result = next((r for r in cpu_results if r.get("voice_name") == voice_name), None)
        gpu_result = next((r for r in gpu_results if r.get("voice_name") == voice_name), None)
        
        # Formatar CPU
        if cpu_result and cpu_result.get("success"):
            cpu_str = f"{cpu_result['generation_time']:.2f}s (RTF: {cpu_result['rtf']:.2f}x)"
        else:
            cpu_str = "N/A (ERRO)"
        
        # Formatar GPU
        if gpu_result and gpu_result.get("success"):
            gpu_str = f"{gpu_result['generation_time']:.2f}s (RTF: {gpu_result['rtf']:.2f}x)"
        else:
            gpu_str = "N/A (ERRO)"
        
        # Calcular melhoria
        if cpu_result and cpu_result.get("success") and gpu_result and gpu_result.get("success"):
            cpu_time = cpu_result['generation_time']
            gpu_time = gpu_result['generation_time']
            improvement = ((cpu_time - gpu_time) / cpu_time) * 100
            if improvement > 0:
                improvement_str = f"⬇️ {improvement:.1f}%"
            elif improvement < 0:
                improvement_str = f"⬆️ {abs(improvement):.1f}%"
            else:
                improvement_str = "➡️ 0%"
        else:
            improvement_str = "N/A"
        
        print(f"{voice_name:<45} {cpu_str:<25} {gpu_str:<25} {improvement_str:<15}")
    
    # Estatísticas
    print("\n" + "="*100)
    print("  ESTATÍSTICAS")
    print("="*100)
    
    cpu_success = sum(1 for r in cpu_results if r.get("success"))
    gpu_success = sum(1 for r in gpu_results if r.get("success"))
    
    print(f"\nTaxa de sucesso:")
    print(f"   CPU: {cpu_success}/{len(VOICES)} vozes ({cpu_success/len(VOICES)*100:.0f}%)")
    print(f"   GPU: {gpu_success}/{len(VOICES)} vozes ({gpu_success/len(VOICES)*100:.0f}%)")
    
    # RTF médio
    cpu_rtf = [r['rtf'] for r in cpu_results if r.get("success")]
    gpu_rtf = [r['rtf'] for r in gpu_results if r.get("success")]
    
    if cpu_rtf and gpu_rtf:
        cpu_avg_rtf = sum(cpu_rtf) / len(cpu_rtf)
        gpu_avg_rtf = sum(gpu_rtf) / len(gpu_rtf)
        print(f"\nRTF Medio:")
        print(f"   CPU: {cpu_avg_rtf:.2f}x")
        print(f"   GPU: {gpu_avg_rtf:.2f}x")
        improvement_rtf = ((cpu_avg_rtf - gpu_avg_rtf) / cpu_avg_rtf) * 100
        print(f"   Melhoria: {improvement_rtf:+.1f}%")
    
    # Tempo médio
    cpu_times = [r['generation_time'] for r in cpu_results if r.get("success")]
    gpu_times = [r['generation_time'] for r in gpu_results if r.get("success")]
    
    if cpu_times and gpu_times:
        cpu_avg_time = sum(cpu_times) / len(cpu_times)
        gpu_avg_time = sum(gpu_times) / len(gpu_times)
        print(f"\nTempo Medio de Geracao:")
        print(f"   CPU: {cpu_avg_time:.2f}s")
        print(f"   GPU: {gpu_avg_time:.2f}s")
        improvement_time = ((cpu_avg_time - gpu_avg_time) / cpu_avg_time) * 100
        print(f"   Melhoria: {improvement_time:+.1f}%")

def save_audios(results, device_name, timestamp):
    """Salva áudios gerados"""
    output_dir = script_dir / f"benchmark_cpu_vs_gpu_{timestamp}"
    output_dir.mkdir(exist_ok=True)
    
    for result in results:
        if result.get("success"):
            voice_safe = result["voice_name"].replace(" ", "_").replace("(", "").replace(")", "").lower()
            filename = f"{device_name}_{voice_safe}_{timestamp}.wav"
            output_path = output_dir / filename
            sf.write(str(output_path), result["audio"], result["sample_rate"], subtype='FLOAT')
    
    return output_dir

def main():
    # Fix encoding para Windows
    import sys
    if sys.platform == 'win32':
        import io
        sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
        sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')
    
    print("="*100)
    print("  BENCHMARK COMPARATIVO: CPU vs GPU - TODAS AS 5 VOZES")
    print("="*100)
    print(f"\nNovo paragrafo de teste ({len(TEXT_NOVO)} caracteres):")
    print(f"   {TEXT_NOVO[:100]}...")
    print()
    
    # Verificar GPU disponível
    gpu_available = torch.cuda.is_available()
    if gpu_available:
        try:
            # Testar se GPU realmente funciona
            test_tensor = torch.randn(10, 10).cuda()
            torch.cuda.synchronize()
            del test_tensor
            torch.cuda.empty_cache()
            gpu_works = True
        except:
            gpu_works = False
    else:
        gpu_works = False
    
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    
    # Executar benchmark CPU primeiro (mais rápido de carregar)
    print("\n" + "="*100)
    print("  FASE 1: BENCHMARK CPU")
    print("="*100)
    print("\nCarregando modelo XTTS v2 para CPU...")
    try:
        tts_cpu = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=False, progress_bar=False)
        print("Modelo CPU carregado!")
    except Exception as e:
        print(f"Erro ao carregar XTTS CPU: {e}")
        tts_cpu = None
    
    cpu_results = None
    if tts_cpu:
        cpu_results = run_benchmark(tts_cpu, use_gpu=False, device_name="CPU")
        if cpu_results:
            cpu_dir = save_audios(cpu_results, "cpu", timestamp)
            print(f"\nAudios CPU salvos em: {cpu_dir}")
        # Liberar memória
        del tts_cpu
        import gc
        gc.collect()
    
    # Executar benchmark GPU (se disponível)
    gpu_results = None
    if gpu_works:
        print("\n" + "="*100)
        print("  FASE 2: BENCHMARK GPU")
        print("="*100)
        print("\nCarregando modelo XTTS v2 para GPU...")
        try:
            tts_gpu = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=True, progress_bar=False)
            print("Modelo GPU carregado!")
        except Exception as e:
            print(f"Erro ao carregar XTTS GPU: {e}")
            tts_gpu = None
        
        if tts_gpu:
            gpu_results = run_benchmark(tts_gpu, use_gpu=True, device_name="GPU")
            if gpu_results:
                gpu_dir = save_audios(gpu_results, "gpu", timestamp)
                print(f"\nAudios GPU salvos em: {gpu_dir}")
            del tts_gpu
            torch.cuda.empty_cache()
        
        # Comparar resultados
        if cpu_results and gpu_results:
            compare_results(cpu_results, gpu_results)
    else:
        print("\nGPU nao disponivel ou nao compativel. Pulando benchmark GPU.")
        print("   Apenas benchmark CPU foi executado.")
    
    print("\n" + "="*100)
    print("  BENCHMARK CONCLUÍDO!")
    print("="*100)
    print()

if __name__ == "__main__":
    main()

