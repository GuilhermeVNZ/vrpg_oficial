#!/usr/bin/env python3
"""Verifica se Whisper e Qwen estão usando GPU"""

import sys
import subprocess
import os

print("=" * 70)
print("🔍 Verificação GPU: Whisper e Qwen")
print("=" * 70)
print()

# 1. Verificar Whisper
print("1️⃣ Whisper (ASR):")
print("   Verificando implementação...")

# Verificar se é whisper.cpp ou faster-whisper
try:
    # Tentar faster-whisper (suporta GPU)
    from faster_whisper import WhisperModel
    print("   ✅ faster-whisper instalado")
    
    # Verificar se pode usar GPU
    import torch
    if torch.cuda.is_available():
        print(f"   🎮 GPU disponível: {torch.cuda.get_device_name(0)}")
        print("   💡 faster-whisper pode usar GPU com device='cuda'")
        print("   ⚠️  Verifique se está configurado para usar GPU no código")
    else:
        print("   ⚠️  GPU não disponível no PyTorch")
except ImportError:
    print("   ⚠️  faster-whisper não instalado")
    print("   💡 Para usar GPU: pip install faster-whisper")
    
    # Verificar whisper.cpp
    try:
        result = subprocess.run(['whisper-cpp', '--help'], 
                              capture_output=True, text=True, timeout=5)
        if result.returncode == 0:
            print("   ✅ whisper.cpp encontrado")
            print("   💡 whisper.cpp pode usar GPU com CUDA (se compilado com CUDA)")
        else:
            print("   ⚠️  whisper.cpp não encontrado")
    except:
        print("   ⚠️  whisper.cpp não encontrado")

print()

# 2. Verificar Qwen (llama.cpp)
print("2️⃣ Qwen (LLM):")
print("   Verificando implementação...")

# Qwen provavelmente usa llama.cpp (GGUF)
try:
    # Verificar se llama.cpp está disponível
    result = subprocess.run(['llama-cli', '--help'], 
                          capture_output=True, text=True, timeout=5)
    if result.returncode == 0:
        print("   ✅ llama-cli encontrado")
        print("   💡 Para usar GPU: --n-gpu-layers 35 (ou máximo)")
    else:
        print("   ⚠️  llama-cli não encontrado")
except:
    print("   ⚠️  llama-cli não encontrado")
    print("   💡 Qwen provavelmente usa llama.cpp via FFI ou biblioteca Rust")

print()

# 3. Verificar configurações
print("3️⃣ Configurações:")
env_vars = {
    "VRPG_ASR_USE_GPU": os.getenv("VRPG_ASR_USE_GPU", "não definida"),
    "VRPG_LLM_USE_GPU": os.getenv("VRPG_LLM_USE_GPU", "não definida"),
    "VRPG_GPU_LAYERS": os.getenv("VRPG_GPU_LAYERS", "não definida"),
}

for var, value in env_vars.items():
    status = "✅" if value.lower() in ["true", "35"] else "⚠️"
    print(f"   {status} {var} = {value}")

print()

# 4. Resumo
print("=" * 70)
print("📊 Resumo:")
print("=" * 70)
print()
print("Whisper:")
print("   - faster-whisper: Suporta GPU (device='cuda')")
print("   - whisper.cpp: Suporta GPU se compilado com CUDA")
print("   - Status: Verificar implementação no código Rust")
print()
print("Qwen:")
print("   - llama.cpp: Suporta GPU via --n-gpu-layers")
print("   - Status: Verificar se está usando GPU layers no código")
print()
print("💡 Para garantir uso de GPU:")
print("   1. Configure variáveis de ambiente (.env)")
print("   2. Verifique implementação nos arquivos Rust:")
print("      - src/asr-service/src/whisper.rs")
print("      - src/llm-core/src/inference.rs")
print("=" * 70)

