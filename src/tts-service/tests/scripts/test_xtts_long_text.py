#!/usr/bin/env python3
"""
Script standalone para testar XTTS com texto longo
Execute: python test_xtts_long_text.py
"""

import sys
import os
import json
import numpy as np
import torch

# Aceitar termos de serviço
os.environ["COQUI_TOS_AGREED"] = "1"

# Fix para PyTorch 2.6+
safe_classes = []
try:
    from TTS.tts.configs.xtts_config import XttsConfig
    safe_classes.append(XttsConfig)
except:
    pass

try:
    from TTS.tts.models.xtts import XttsAudioConfig, XttsArgs
    safe_classes.append(XttsAudioConfig)
    safe_classes.append(XttsArgs)
except:
    pass

try:
    from TTS.config.shared_configs import BaseDatasetConfig, BaseAudioConfig, BaseTrainingConfig
    safe_classes.extend([BaseDatasetConfig, BaseAudioConfig, BaseTrainingConfig])
except:
    pass

if safe_classes:
    try:
        torch.serialization.add_safe_globals(safe_classes)
    except:
        # Fallback: monkey patch torch.load
        original_load = torch.load
        def patched_load(*args, **kwargs):
            kwargs['weights_only'] = False
            return original_load(*args, **kwargs)
        torch.load = patched_load
else:
    # Fallback: monkey patch torch.load
    original_load = torch.load
    def patched_load(*args, **kwargs):
        kwargs['weights_only'] = False
        return original_load(*args, **kwargs)
    torch.load = patched_load

print("\n" + "="*60)
print("🎤 Teste XTTS - Texto Longo")
print("="*60 + "\n")

try:
    from TTS.api import TTS
    print("✅ Coqui TTS importado com sucesso")
except ImportError as e:
    print(f"❌ Erro ao importar Coqui TTS: {e}")
    print("   Instale com: pip install TTS")
    sys.exit(1)

print("\n📥 Carregando modelo XTTS v2...")
print("   (Modelo já deve estar em cache)\n")

try:
    tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=False, progress_bar=True)
    print("\n✅ Modelo XTTS carregado com sucesso!")
except Exception as e:
    print(f"\n❌ Erro ao carregar modelo: {e}")
    sys.exit(1)

# Texto longo para teste
long_text = """In a distant realm where magic flows like rivers and dragons soar through 
clouds of stardust, there lived a brave adventurer named Elara. She had spent years 
training in the ancient arts of combat and spellcasting, preparing for the day when 
she would face the Dark Lord Malachar. The prophecy had foretold that only one with 
pure heart and unwavering courage could defeat the darkness that threatened to consume 
the world. With her trusted companions - a wise wizard named Theron and a fierce 
warrior named Kael - Elara embarked on a perilous journey through enchanted forests, 
across treacherous mountains, and into the depths of forgotten dungeons. Along the way, 
they discovered ancient artifacts of immense power and forged alliances with mystical 
creatures who had long been hidden from mortal eyes. The final battle would test not 
only their strength and skill, but also their bonds of friendship and their faith in 
the light that still remained in the world."""

print(f"\n📝 Texto para síntese:")
print(f"   - Caracteres: {len(long_text)}")
print(f"   - Palavras: {len(long_text.split())}")
print(f"   - Linhas: {len(long_text.splitlines())}")
print(f"   - Duração estimada: ~{len(long_text) / 10:.1f} segundos\n")

print("🎙️  Sintetizando texto longo...")
print("   (Isso pode levar alguns minutos para texto longo)\n")

speaker = "Ana Florence"
print(f"   Usando speaker: {speaker}\n")

try:
    # Sintetizar áudio
    audio = tts.tts(
        text=long_text,
        speaker=speaker,
        language="en",
    )
    
    print(f"\n✅ Áudio gerado com sucesso!")
    print(f"   - Amostras: {len(audio):,}")
    print(f"   - Sample rate: {tts.synthesizer.output_sample_rate} Hz")
    duration = len(audio) / tts.synthesizer.output_sample_rate
    print(f"   - Duração: {duration:.2f} segundos ({duration/60:.1f} minutos)")
    
    # Verificar amplitude
    if isinstance(audio, np.ndarray):
        max_amp = np.abs(audio).max()
        rms = np.sqrt(np.mean(audio**2))
        print(f"   - Amplitude máxima: {max_amp:.4f}")
        print(f"   - RMS: {rms:.4f}")
    
    # Salvar WAV
    try:
        import soundfile as sf
        output_path = "test_xtts_long_text.wav"
        sf.write(output_path, audio, tts.synthesizer.output_sample_rate)
        file_size_kb = os.path.getsize(output_path) / 1024
        file_size_mb = file_size_kb / 1024
        print(f"\n💾 Áudio salvo em: {output_path}")
        if file_size_mb >= 1.0:
            print(f"   Tamanho do arquivo: {file_size_mb:.2f} MB")
        else:
            print(f"   Tamanho do arquivo: {file_size_kb:.1f} KB")
    except Exception as e:
        print(f"\n⚠️  Não foi possível salvar WAV (soundfile não disponível): {e}")
        print("   Mas o áudio foi gerado com sucesso!")
    
    print("\n" + "="*60)
    print("✅ TESTE CONCLUÍDO COM SUCESSO!")
    print("="*60)
    print("\nO áudio do texto longo foi gerado e deve ser inteligível.")
    print("Ouça o arquivo para verificar a qualidade da síntese.\n")
    
except Exception as e:
    print(f"\n❌ Erro ao sintetizar áudio: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)

