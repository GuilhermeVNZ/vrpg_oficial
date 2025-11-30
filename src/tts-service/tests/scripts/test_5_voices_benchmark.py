#!/usr/bin/env python3
"""
Teste de benchmark das 5 vozes XTTS disponíveis
Testa tempo de geração para textos de 5s e 25s
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
    from TTS.api import TTS
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("   Instale: pip install TTS soundfile", file=sys.stderr)
    sys.exit(1)

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


def estimate_text_duration(text: str, words_per_minute: int = 150) -> float:
    """Estima duração do texto em segundos"""
    word_count = len(text.split())
    duration_minutes = word_count / words_per_minute
    return duration_minutes * 60


def test_voice(tts, voice_config, text, text_name):
    """Testa uma voz específica e retorna métricas"""
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
        
        print(f"✅ Geração concluída!")
        print(f"   ⏱️  Tempo de geração: {generation_time:.2f}s")
        print(f"   🎵 Duração do áudio: {audio_duration:.2f}s")
        print(f"   ⚡ Real-Time Factor: {rtf:.2f}x")
        print(f"   📊 Sample rate: {sample_rate} Hz")
        print(f"   📏 Amostras: {len(audio)}")
        
        return {
            "voice_name": voice_name,
            "text_name": text_name,
            "generation_time": generation_time,
            "audio_duration": audio_duration,
            "rtf": rtf,
            "sample_rate": sample_rate,
            "audio": audio,
        }
        
    except Exception as e:
        print(f"❌ Erro ao gerar áudio: {e}")
        import traceback
        traceback.print_exc()
        return None


def main():
    print("\n" + "="*70)
    print("  BENCHMARK: 5 VOZES XTTS - TEMPO DE GERAÇÃO")
    print("="*70)
    print("\n📋 Testando:")
    print("   - Texto curto (~5s de áudio)")
    print("   - Texto longo (~25s de áudio)")
    print("   - 5 vozes diferentes")
    
    # Verificar GPU
    try:
        import torch
        use_gpu = torch.cuda.is_available()
        if use_gpu:
            try:
                # Tentar detectar se a GPU é compatível
                device_name = torch.cuda.get_device_name(0)
                compute_capability = torch.cuda.get_device_capability(0)
                print(f"\n🎮 GPU disponível: {device_name}")
                print(f"   Compute Capability: {compute_capability[0]}.{compute_capability[1]}")
                
                # Verificar se PyTorch suporta esta GPU
                # Se compute capability > 9.0, pode não ser suportado
                if compute_capability[0] > 9:
                    print("   ⚠️  GPU pode não ser suportada pelo PyTorch atual")
                    print("   💻 Forçando uso de CPU para evitar erros")
                    use_gpu = False
            except Exception as e:
                print(f"   ⚠️  Erro ao verificar GPU: {e}")
                print("   💻 Usando CPU")
                use_gpu = False
        else:
            print("\n💻 Usando CPU")
    except:
        use_gpu = False
        print("\n💻 Usando CPU")
    
    # Carregar modelo XTTS
    print("\n📥 Carregando modelo XTTS v2...")
    print("   (Isso pode demorar alguns minutos na primeira vez)")
    
    try:
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=True)
        print("✅ Modelo XTTS carregado!\n")
    except Exception as e:
        print(f"❌ Erro ao carregar XTTS: {e}")
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
        result = test_voice(tts, voice, TEXT_5S, "Texto Curto (~5s)")
        if result:
            results.append(result)
    
    # Teste com texto longo (25s)
    print("\n" + "="*70)
    print("  TESTE 2: TEXTO LONGO (~25s)")
    print("="*70)
    
    for voice in VOICES:
        result = test_voice(tts, voice, TEXT_25S, "Texto Longo (~25s)")
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
    
    # Salvar áudios gerados
    print("\n💾 Salvando áudios gerados...")
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = script_dir / "benchmark_5_voices"
    output_dir.mkdir(exist_ok=True)
    
    for result in results:
        voice_safe = result["voice_name"].replace(" ", "_").replace("(", "").replace(")", "").lower()
        text_safe = result["text_name"].replace(" ", "_").replace("(", "").replace(")", "").replace("~", "").lower()
        filename = f"benchmark_{voice_safe}_{text_safe}_{timestamp}.wav"
        output_path = output_dir / filename
        
        sf.write(str(output_path), result["audio"], result["sample_rate"])
        print(f"   ✅ {filename}")
    
    print(f"\n✅ Todos os áudios salvos em: {output_dir}")
    print("\n" + "="*70)
    print("  BENCHMARK CONCLUÍDO!")
    print("="*70 + "\n")


if __name__ == "__main__":
    main()

