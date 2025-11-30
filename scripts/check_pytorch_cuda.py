#!/usr/bin/env python3
"""Script rápido para verificar PyTorch e CUDA"""

import sys

try:
    import torch
    
    print(f"PyTorch: {torch.__version__}")
    print(f"CUDA Available: {torch.cuda.is_available()}")
    
    if torch.cuda.is_available():
        print(f"CUDA Version: {torch.version.cuda}")
        print(f"GPU Count: {torch.cuda.device_count()}")
        for i in range(torch.cuda.device_count()):
            print(f"GPU {i}: {torch.cuda.get_device_name(i)}")
        print("✅ CUDA está funcionando!")
        sys.exit(0)
    else:
        print("⚠️  CUDA não está disponível")
        print("   Verifique se os drivers NVIDIA estão instalados")
        sys.exit(1)
        
except ImportError:
    print("❌ PyTorch não está instalado")
    print("   Instale com: pip install torch")
    sys.exit(1)
except Exception as e:
    print(f"❌ Erro: {e}")
    sys.exit(1)

