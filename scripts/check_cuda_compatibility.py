#!/usr/bin/env python3
"""Verifica compatibilidade entre PyTorch e CUDA instalados"""

import sys

print("=" * 60)
print("🔍 Verificação de Compatibilidade PyTorch + CUDA")
print("=" * 60)
print()

try:
    import torch
    
    # Informações do PyTorch
    print("📦 PyTorch:")
    print(f"   Versão: {torch.__version__}")
    
    # Verificar se tem CUDA compilado
    has_cuda_build = hasattr(torch.version, 'cuda') and torch.version.cuda is not None
    if has_cuda_build:
        print(f"   CUDA compilado: {torch.version.cuda}")
    else:
        print("   CUDA compilado: N/A (CPU-only build)")
    
    # Verificar se CUDA está disponível em runtime
    cuda_available = torch.cuda.is_available()
    print(f"   CUDA disponível: {cuda_available}")
    
    if cuda_available:
        print(f"   Dispositivos CUDA: {torch.cuda.device_count()}")
        for i in range(torch.cuda.device_count()):
            print(f"      GPU {i}: {torch.cuda.get_device_name(i)}")
            print(f"         Capability: {torch.cuda.get_device_capability(i)}")
            print(f"         Memória: {torch.cuda.get_device_properties(i).total_memory / 1024**3:.2f} GB")
        
        # Teste rápido
        print()
        print("🧪 Teste rápido:")
        try:
            x = torch.randn(1000, 1000).cuda()
            y = torch.randn(1000, 1000).cuda()
            z = torch.matmul(x, y)
            print("   ✅ Operação CUDA funcionando!")
        except Exception as e:
            print(f"   ❌ Erro ao executar operação CUDA: {e}")
    else:
        print()
        print("⚠️  CUDA não está disponível")
        if not has_cuda_build:
            print("   Motivo: PyTorch foi compilado sem suporte CUDA (CPU-only)")
            print("   Solução: Reinstale PyTorch com CUDA")
        else:
            print("   Motivo: Drivers CUDA não encontrados ou incompatíveis")
            print("   Verifique: nvidia-smi")
    
    print()
    print("=" * 60)
    
    # Verificar versão do CUDA do sistema (se nvidia-smi disponível)
    import subprocess
    try:
        result = subprocess.run(['nvidia-smi', '--query-gpu=name,driver_version,cuda_version', '--format=csv,noheader'], 
                              capture_output=True, text=True, timeout=5)
        if result.returncode == 0:
            print("🖥️  Sistema (nvidia-smi):")
            lines = result.stdout.strip().split('\n')
            for line in lines:
                if line.strip():
                    parts = [p.strip() for p in line.split(',')]
                    if len(parts) >= 3:
                        print(f"   GPU: {parts[0]}")
                        print(f"   Driver: {parts[1]}")
                        print(f"   CUDA Runtime: {parts[2]}")
        else:
            print("⚠️  nvidia-smi não disponível ou erro")
    except Exception as e:
        print(f"⚠️  Não foi possível executar nvidia-smi: {e}")
    
    print("=" * 60)
    
    # Resumo
    print()
    if cuda_available:
        print("✅ STATUS: PyTorch está pronto para usar GPU!")
        sys.exit(0)
    else:
        print("❌ STATUS: PyTorch não pode usar GPU")
        sys.exit(1)
        
except ImportError:
    print("❌ PyTorch não está instalado")
    sys.exit(1)
except Exception as e:
    print(f"❌ Erro: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)

