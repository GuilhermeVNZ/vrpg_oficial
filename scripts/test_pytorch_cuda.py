#!/usr/bin/env python3
"""Teste simples para verificar se PyTorch pode usar CUDA"""

import torch

print("=" * 60)
print("🧪 Teste PyTorch + CUDA")
print("=" * 60)
print()

print(f"PyTorch versão: {torch.__version__}")
print(f"PyTorch caminho: {torch.__file__}")

# Verificar build
has_cuda_build = hasattr(torch.version, 'cuda') and torch.version.cuda is not None
print(f"Build com CUDA: {'Sim' if has_cuda_build else 'Não (CPU-only)'}")

# Verificar disponibilidade
cuda_available = torch.cuda.is_available()
print(f"CUDA disponível: {cuda_available}")

if cuda_available:
    print(f"GPU: {torch.cuda.get_device_name(0)}")
    print()
    print("✅ CUDA está funcionando!")
    
    # Teste prático
    try:
        x = torch.randn(100, 100).cuda()
        y = torch.randn(100, 100).cuda()
        z = torch.matmul(x, y)
        print("✅ Operação CUDA executada com sucesso!")
    except Exception as e:
        print(f"❌ Erro ao executar operação CUDA: {e}")
else:
    print()
    print("❌ CUDA não está disponível")
    if not has_cuda_build:
        print("   Motivo: PyTorch foi compilado sem suporte CUDA")
        print("   Solução: Reinstale PyTorch com CUDA")
    else:
        print("   Motivo: Drivers CUDA não encontrados ou incompatíveis")

print("=" * 60)

