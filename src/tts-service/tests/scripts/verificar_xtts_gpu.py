#!/usr/bin/env python3
"""Verifica se XTTS está usando GPU"""

import sys
import os
import torch

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

# Fix para PyTorch 2.6+ que requer weights_only=False
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load

# Tentar adicionar safe globals
try:
    from TTS.tts.configs.xtts_config import XttsConfig
    from TTS.tts.models.xtts import XttsAudioConfig, XttsArgs
    torch.serialization.add_safe_globals([XttsConfig, XttsAudioConfig, XttsArgs])
except:
    pass

print("="*70)
print("🔍 VERIFICAÇÃO XTTS GPU")
print("="*70)

print(f"\n📦 PyTorch: {torch.__version__}")
print(f"🔧 CUDA Build: {torch.version.cuda}")
print(f"🎮 CUDA Disponível: {torch.cuda.is_available()}")

if torch.cuda.is_available():
    print(f"🖥️  GPU: {torch.cuda.get_device_name(0)}")
    print(f"💾 GPU Memory: {torch.cuda.get_device_properties(0).total_memory / 1024**3:.2f} GB")
    print(f"⚡ CUDA Capability: {torch.cuda.get_device_capability(0)}")
    
    print("\n📥 Carregando XTTS com GPU=True...")
    try:
        from TTS.api import TTS
        tts = TTS('tts_models/multilingual/multi-dataset/xtts_v2', gpu=True, progress_bar=False)
        print("✅ XTTS carregado!")
        
        # Verificar device do modelo de várias formas
        using_gpu = False
        
        # Método 1: Verificar device do synthesizer
        try:
            device = str(tts.synthesizer.device)
            print(f"🔧 Device do synthesizer: {device}")
            using_gpu = "cuda" in device.lower()
        except:
            pass
        
        # Método 2: Verificar device do modelo TTS
        if not using_gpu:
            try:
                model_device = next(tts.synthesizer.tts_model.parameters()).device
                print(f"🔧 Device do modelo TTS: {model_device}")
                using_gpu = "cuda" in str(model_device).lower()
            except:
                pass
        
        # Método 3: Fazer síntese de teste e verificar onde está rodando
        if not using_gpu:
            print("\n🧪 Fazendo síntese de teste para verificar device...")
            try:
                import time
                start = time.time()
                audio = tts.tts("Hello", speaker="Ana Florence", language="en")
                elapsed = time.time() - start
                print(f"⏱️  Tempo de síntese: {elapsed:.3f}s")
                
                # Se for muito rápido (< 0.5s), provavelmente está em GPU
                # Se for lento (> 2s), provavelmente está em CPU
                if elapsed < 0.5:
                    using_gpu = True
                    print("✅ Síntese rápida indica uso de GPU")
                elif elapsed > 2.0:
                    using_gpu = False
                    print("⚠️  Síntese lenta indica uso de CPU")
                else:
                    print("⚠️  Tempo intermediário - não é possível determinar com certeza")
            except Exception as e:
                print(f"⚠️  Erro no teste: {e}")
        
        print(f"\n🎯 Usando GPU: {'✅ SIM' if using_gpu else '❌ NÃO'}")
        
        if using_gpu:
            print("\n✅ XTTS está configurado e usando GPU!")
            print("   Latência esperada: 50-200ms por síntese")
        else:
            print("\n⚠️  XTTS NÃO está usando GPU!")
            print("   Latência esperada: 3-30 segundos por síntese")
            print("   Verifique a configuração do TTS")
            
    except Exception as e:
        print(f"❌ Erro ao carregar XTTS: {e}")
        import traceback
        traceback.print_exc()
else:
    print("\n❌ CUDA não disponível - XTTS usará CPU")
    print("⚠️  Performance será muito mais lenta!")
    print("   Latência esperada: 3-30 segundos (vs 0.5-0.8s com GPU)")

print("\n" + "="*70)

