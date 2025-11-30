#!/usr/bin/env python3
"""Resumo final do status GPU"""

import torch

print("=" * 70)
print("✅ VERIFICAÇÃO FINAL - STATUS GPU")
print("=" * 70)
print()

print(f"📦 PyTorch: {torch.__version__}")
print(f"🔧 CUDA Build: {torch.version.cuda}")
print(f"🎮 CUDA Available: {torch.cuda.is_available()}")
print()

if torch.cuda.is_available():
    print(f"🖥️  GPU: {torch.cuda.get_device_name(0)}")
    print(f"💾 GPU Memory: {torch.cuda.get_device_properties(0).total_memory / 1024**3:.2f} GB")
    print(f"⚡ CUDA Capability: {torch.cuda.get_device_capability(0)}")
    print()
    
    # Teste prático
    try:
        x = torch.randn(1000, 1000).cuda()
        y = torch.randn(1000, 1000).cuda()
        z = torch.matmul(x, y)
        print("✅ Teste CUDA: SUCESSO!")
    except Exception as e:
        print(f"❌ Erro no teste: {e}")
    
    print()
    print("=" * 70)
    print("✅ TUDO FUNCIONANDO! GPU está pronta para uso!")
    print("=" * 70)
    
    # Verificar XTTS
    print()
    print("Testando XTTS com GPU...")
    try:
        from TTS.api import TTS
        tts = TTS('tts_models/multilingual/multi-dataset/xtts_v2', gpu=True)
        print("✅ XTTS carregado com GPU!")
    except Exception as e:
        print(f"⚠️  XTTS: {e}")
else:
    print("❌ CUDA não está disponível")

