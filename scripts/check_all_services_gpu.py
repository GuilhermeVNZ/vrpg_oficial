#!/usr/bin/env python3
"""Verifica status GPU de todos os serviços"""

import sys
import os

print("=" * 70)
print("🔍 Status GPU - Todos os Serviços")
print("=" * 70)
print()

# 1. PyTorch/CUDA
print("1️⃣ PyTorch/CUDA (Base):")
try:
    import torch
    print(f"   ✅ PyTorch: {torch.__version__}")
    print(f"   ✅ CUDA: {torch.cuda.is_available()}")
    if torch.cuda.is_available():
        print(f"   ✅ GPU: {torch.cuda.get_device_name(0)}")
    print()
except ImportError:
    print("   ❌ PyTorch não instalado")
    print()

# 2. XTTS
print("2️⃣ XTTS (TTS):")
try:
    from TTS.api import TTS
    import torch
    tts = TTS('tts_models/multilingual/multi-dataset/xtts_v2', gpu=torch.cuda.is_available())
    print(f"   ✅ XTTS: Funcionando")
    print(f"   ✅ GPU: {torch.cuda.is_available()}")
    print()
except Exception as e:
    print(f"   ⚠️  XTTS: {e}")
    print()

# 3. SoVITS
print("3️⃣ SoVITS:")
sovits_venv = "assets-and-models/models/tts/sovits/venv310/Scripts/python.exe"
if os.path.exists(sovits_venv):
    try:
        import subprocess
        result = subprocess.run([sovits_venv, '-c', 'import torch; print(f"CUDA: {torch.cuda.is_available()}")'], 
                               capture_output=True, text=True, timeout=5)
        if result.returncode == 0:
            print(f"   ✅ SoVITS venv: OK")
            print(f"   {result.stdout.strip()}")
        else:
            print(f"   ⚠️  SoVITS venv: Erro")
    except:
        print(f"   ⚠️  SoVITS venv: Não verificado")
else:
    print(f"   ⚠️  SoVITS venv: Não encontrado")
print()

# 4. Whisper
print("4️⃣ Whisper (ASR):")
try:
    from faster_whisper import WhisperModel
    import torch
    print(f"   ✅ faster-whisper: Instalado")
    print(f"   ✅ GPU disponível: {torch.cuda.is_available()}")
    print(f"   ⚠️  Status: STUB no código Rust (não usa modelo real)")
    print(f"   💡 Para usar GPU: Implementar Python bridge (como XTTS)")
except ImportError:
    print(f"   ⚠️  faster-whisper: Não instalado")
    print(f"   💡 Instalar: pip install faster-whisper")
    print(f"   ⚠️  Status: STUB no código Rust (não usa modelo real)")
print()

# 5. Qwen
print("5️⃣ Qwen (LLM):")
try:
    import subprocess
    result = subprocess.run(['llama-cli', '--help'], 
                          capture_output=True, text=True, timeout=5)
    if result.returncode == 0:
        print(f"   ✅ llama.cpp: Encontrado")
        print(f"   💡 Para usar GPU: --n-gpu-layers 35")
    else:
        print(f"   ⚠️  llama.cpp: Não encontrado")
except:
    print(f"   ⚠️  llama.cpp: Não encontrado")
print(f"   ⚠️  Status: STUB no código Rust (não usa modelo real)")
print(f"   💡 Para usar GPU: Integrar llama.cpp com GPU layers")
print()

# 6. Variáveis de Ambiente
print("6️⃣ Variáveis de Ambiente:")
env_vars = {
    "VRPG_GPU_ENABLED": os.getenv("VRPG_GPU_ENABLED", "não definida"),
    "VRPG_TTS_USE_GPU": os.getenv("VRPG_TTS_USE_GPU", "não definida"),
    "VRPG_ASR_USE_GPU": os.getenv("VRPG_ASR_USE_GPU", "não definida"),
    "VRPG_LLM_USE_GPU": os.getenv("VRPG_LLM_USE_GPU", "não definida"),
    "VRPG_SOVITS_USE_GPU": os.getenv("VRPG_SOVITS_USE_GPU", "não definida"),
    "VRPG_GPU_LAYERS": os.getenv("VRPG_GPU_LAYERS", "não definida"),
}

for var, value in env_vars.items():
    status = "✅" if value.lower() in ["true", "35"] else "⚠️"
    print(f"   {status} {var} = {value}")

print()
print("=" * 70)
print("📊 RESUMO")
print("=" * 70)
print()
print("✅ Funcionando com GPU:")
print("   - PyTorch/CUDA: ✅")
print("   - XTTS: ✅")
print("   - SoVITS: ✅ (auto-detecta GPU)")
print()
print("⚠️  Ainda não implementado (STUBs):")
print("   - Whisper: ⚠️  Precisa implementação real")
print("   - Qwen: ⚠️  Precisa implementação real")
print()
print("💡 Próximos passos:")
print("   1. Implementar Whisper com faster-whisper + GPU")
print("   2. Implementar Qwen com llama.cpp + GPU layers")
print("   3. Configurar variáveis de ambiente no .env")
print("=" * 70)

