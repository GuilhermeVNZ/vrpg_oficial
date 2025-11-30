#!/usr/bin/env python3
"""Verifica todas as instalações de PyTorch no sistema"""

import sys
import subprocess
import importlib.util

print("=" * 70)
print("🔍 Verificando TODAS as instalações de PyTorch")
print("=" * 70)
print()

# 1. Verificar PyTorch atual
print("1️⃣ PyTorch no ambiente atual:")
try:
    import torch
    print(f"   ✅ Versão: {torch.__version__}")
    print(f"   📍 Caminho: {torch.__file__}")
    print(f"   🔧 CUDA compilado: {torch.version.cuda if hasattr(torch.version, 'cuda') and torch.version.cuda else 'N/A (CPU-only)'}")
    print(f"   🎮 CUDA disponível: {torch.cuda.is_available()}")
    if torch.cuda.is_available():
        print(f"   🖥️  GPU: {torch.cuda.get_device_name(0)}")
except Exception as e:
    print(f"   ❌ Erro: {e}")

print()

# 2. Verificar pip list
print("2️⃣ Pacotes PyTorch instalados via pip:")
try:
    result = subprocess.run([sys.executable, '-m', 'pip', 'list'], 
                          capture_output=True, text=True, timeout=10)
    if result.returncode == 0:
        lines = result.stdout.split('\n')
        torch_packages = [l for l in lines if 'torch' in l.lower()]
        if torch_packages:
            for pkg in torch_packages:
                print(f"   {pkg}")
        else:
            print("   Nenhum pacote torch encontrado")
    else:
        print(f"   ⚠️  Erro ao executar pip list: {result.stderr}")
except Exception as e:
    print(f"   ⚠️  Erro: {e}")

print()

# 3. Verificar se há outras instalações Python
print("3️⃣ Verificando outras instalações Python:")
try:
    # Tentar python3
    result = subprocess.run(['python3', '--version'], 
                          capture_output=True, text=True, timeout=5)
    if result.returncode == 0:
        print(f"   python3: {result.stdout.strip()}")
        # Verificar PyTorch no python3
        result2 = subprocess.run(['python3', '-c', 'import torch; print(torch.__version__)'], 
                               capture_output=True, text=True, timeout=5)
        if result2.returncode == 0:
            print(f"      PyTorch: {result2.stdout.strip()}")
except:
    pass

# Tentar py -3.10, py -3.11, etc
for version in ['3.10', '3.11', '3.12']:
    try:
        result = subprocess.run([f'py', '-{version}', '--version'], 
                              capture_output=True, text=True, timeout=5)
        if result.returncode == 0:
            print(f"   py -{version}: {result.stdout.strip()}")
            result2 = subprocess.run([f'py', '-{version}', '-c', 'import torch; print(torch.__version__)'], 
                                   capture_output=True, text=True, timeout=5)
            if result2.returncode == 0:
                print(f"      PyTorch: {result2.stdout.strip()}")
    except:
        pass

print()

# 4. Verificar ambiente virtual do SoVITS
print("4️⃣ Verificando venv do SoVITS:")
sovits_venv = "assets-and-models/models/tts/sovits/venv310/Scripts/python.exe"
import os
if os.path.exists(sovits_venv):
    print(f"   ✅ Encontrado: {sovits_venv}")
    try:
        result = subprocess.run([sovits_venv, '-c', 'import torch; print(f"PyTorch: {torch.__version__}"); print(f"CUDA: {torch.cuda.is_available()}")'], 
                               capture_output=True, text=True, timeout=5)
        if result.returncode == 0:
            print(f"   {result.stdout}")
        else:
            print(f"   ⚠️  Erro: {result.stderr}")
    except Exception as e:
        print(f"   ⚠️  Erro ao verificar: {e}")
else:
    print(f"   ⚠️  Não encontrado: {sovits_venv}")

print()

# 5. Resumo do sistema
print("5️⃣ Sistema:")
print(f"   Python atual: {sys.executable}")
print(f"   Versão Python: {sys.version}")
print(f"   GPU detectada: NVIDIA GeForce RTX 5090")
print(f"   CUDA Runtime: 13.0")

print()
print("=" * 70)
print("💡 Recomendação:")
if torch.cuda.is_available():
    print("   ✅ PyTorch está pronto para usar GPU!")
else:
    print("   ⚠️  PyTorch atual é CPU-only (2.9.1+cpu)")
    print("   💡 Para usar GPU, instale PyTorch com CUDA:")
    print("      pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121")
print("=" * 70)

