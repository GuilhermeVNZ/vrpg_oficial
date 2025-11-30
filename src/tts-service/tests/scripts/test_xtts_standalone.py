#!/usr/bin/env python3
"""
Script standalone para testar XTTS e mostrar progresso em tempo real
Execute: python test_xtts_standalone.py
"""

import sys
import os
import json
import numpy as np
import torch

# Aceitar termos de serviço
os.environ["COQUI_TOS_AGREED"] = "1"

# Fix para PyTorch 2.6+ - adicionar todas as classes necessárias
safe_classes = []
try:
    from TTS.tts.configs.xtts_config import XttsConfig
    safe_classes.append(XttsConfig)
except:
    pass

try:
    from TTS.tts.models.xtts import XttsAudioConfig
    safe_classes.append(XttsAudioConfig)
except:
    pass

try:
    from TTS.config.shared_configs import BaseDatasetConfig
    safe_classes.append(BaseDatasetConfig)
except:
    pass

try:
    from TTS.config.shared_configs import BaseAudioConfig
    safe_classes.append(BaseAudioConfig)
except:
    pass

try:
    from TTS.config.shared_configs import BaseTrainingConfig
    safe_classes.append(BaseTrainingConfig)
except:
    pass

try:
    from TTS.tts.models.xtts import XttsArgs
    safe_classes.append(XttsArgs)
except:
    pass

# Tentar adicionar todas as classes de uma vez
if safe_classes:
    try:
        torch.serialization.add_safe_globals(safe_classes)
        print(f"✅ PyTorch 2.6+ fix aplicado ({len(safe_classes)} classes)")
    except Exception as e:
        print(f"⚠️  Erro ao aplicar fix: {e}")
        # Tentar alternativa: monkey patch torch.load
        print("   Tentando alternativa (monkey patch torch.load)...")
        original_load = torch.load
        def patched_load(*args, **kwargs):
            kwargs['weights_only'] = False
            return original_load(*args, **kwargs)
        torch.load = patched_load
        print("   ✅ Monkey patch aplicado (weights_only=False)")
else:
    print("⚠️  Não foi possível aplicar PyTorch fix")

print("\n" + "="*60)
print("🎤 Teste XTTS - Hello World")
print("="*60 + "\n")

try:
    from TTS.api import TTS
    print("✅ Coqui TTS importado com sucesso")
except ImportError as e:
    print(f"❌ Erro ao importar Coqui TTS: {e}")
    print("   Instale com: pip install TTS")
    sys.exit(1)

print("\n📥 Carregando modelo XTTS v2...")
print("   (Isso pode levar vários minutos na primeira vez - download ~1.5GB)")
print("   Você verá uma barra de progresso abaixo:\n")

try:
    tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=False, progress_bar=True)
    print("\n✅ Modelo XTTS carregado com sucesso!")
except Exception as e:
    print(f"\n❌ Erro ao carregar modelo: {e}")
    sys.exit(1)

print("\n🎙️  Sintetizando 'Hello World'...")
print("   (Isso pode levar alguns segundos)\n")

try:
    # XTTS v2 requer um speaker. Vamos usar um speaker padrão
    # Lista de speakers comuns do XTTS v2: "Ana Florence", "Claribel Dervla", etc.
    speaker = "Ana Florence"  # Speaker padrão do XTTS v2
    print(f"   Usando speaker: {speaker}\n")
    
    # Sintetizar áudio
    audio = tts.tts(
        text="Hello World",
        speaker=speaker,
        language="en",
    )
    
    print(f"\n✅ Áudio gerado com sucesso!")
    print(f"   - Amostras: {len(audio)}")
    print(f"   - Sample rate: {tts.synthesizer.output_sample_rate} Hz")
    print(f"   - Duração: {len(audio) / tts.synthesizer.output_sample_rate:.2f} segundos")
    
    # Verificar amplitude
    if isinstance(audio, np.ndarray):
        max_amp = np.abs(audio).max()
        print(f"   - Amplitude máxima: {max_amp:.4f}")
    
    # Salvar WAV
    try:
        import soundfile as sf
        output_path = "test_hello_world_xtts_real.wav"
        sf.write(output_path, audio, tts.synthesizer.output_sample_rate)
        print(f"\n💾 Áudio salvo em: {output_path}")
        print(f"   Tamanho do arquivo: {os.path.getsize(output_path) / 1024:.1f} KB")
    except Exception as e:
        print(f"\n⚠️  Não foi possível salvar WAV (soundfile não disponível): {e}")
        print("   Mas o áudio foi gerado com sucesso!")
    
    print("\n" + "="*60)
    print("✅ TESTE CONCLUÍDO COM SUCESSO!")
    print("="*60)
    print("\nO áudio 'Hello World' foi gerado e deve ser inteligível.")
    print("Compare com o áudio do Piper para verificar a diferença.\n")
    
except Exception as e:
    print(f"\n❌ Erro ao sintetizar áudio: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)

