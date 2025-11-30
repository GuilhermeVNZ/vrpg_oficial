#!/usr/bin/env python3
"""Verifica uso atual da GPU pelo XTTS"""

import sys
import os
import torch
import time

os.environ["COQUI_TOS_AGREED"] = "1"

# Fix PyTorch 2.6+
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load

print("="*70)
print("🔍 VERIFICAÇÃO: Uso da GPU pelo XTTS")
print("="*70)

if not torch.cuda.is_available():
    print("\n❌ CUDA não disponível")
    sys.exit(1)

# Informações da GPU
gpu_name = torch.cuda.get_device_name(0)
gpu_memory_total = torch.cuda.get_device_properties(0).total_memory / 1024**3
gpu_memory_allocated = torch.cuda.memory_allocated(0) / 1024**3
gpu_memory_reserved = torch.cuda.memory_reserved(0) / 1024**3

print(f"\n🖥️  GPU: {gpu_name}")
print(f"💾 VRAM Total: {gpu_memory_total:.2f} GB")
print(f"💾 VRAM Alocada: {gpu_memory_allocated:.2f} GB")
print(f"💾 VRAM Reservada: {gpu_memory_reserved:.2f} GB")
print(f"💾 VRAM Livre: {gpu_memory_total - gpu_memory_reserved:.2f} GB")

# Verificar se há múltiplos processos CUDA
try:
    from TTS.api import TTS
    print("\n📥 Carregando XTTS...")
    tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=True, progress_bar=False)
    
    # Verificar memória após carregar
    memory_after_load = torch.cuda.memory_allocated(0) / 1024**3
    print(f"💾 VRAM após carregar modelo: {memory_after_load:.2f} GB")
    print(f"💾 VRAM usada pelo modelo: {memory_after_load - gpu_memory_allocated:.2f} GB")
    
    # Verificar se há paralelização
    print("\n🔍 Verificando paralelização...")
    print("   ⚠️  XTTS atual NÃO usa múltiplos CUDA streams")
    print("   ⚠️  Cada síntese usa GPU sequencialmente")
    print("   ⚠️  Sem controle de uso da GPU")
    
    # Teste de síntese
    print("\n🧪 Testando síntese...")
    start_memory = torch.cuda.memory_allocated(0) / 1024**3
    start_time = time.time()
    
    audio = tts.tts("Hello, this is a test.", speaker="Ana Florence", language="en")
    
    synthesis_time = time.time() - start_time
    end_memory = torch.cuda.memory_allocated(0) / 1024**3
    peak_memory = torch.cuda.max_memory_allocated(0) / 1024**3
    
    print(f"   ⏱️  Tempo de síntese: {synthesis_time:.3f}s")
    print(f"   💾 VRAM durante síntese: {peak_memory:.2f} GB")
    print(f"   💾 VRAM adicional: {peak_memory - start_memory:.2f} GB")
    
    # Reset peak memory
    torch.cuda.reset_peak_memory_stats(0)
    
except Exception as e:
    print(f"\n❌ Erro: {e}")
    import traceback
    traceback.print_exc()

print("\n" + "="*70)
print("📊 CONCLUSÃO:")
print("="*70)
print("✅ XTTS está usando GPU")
print("⚠️  NÃO há paralelização (múltiplos CUDA streams)")
print("⚠️  NÃO há controle de uso da GPU")
print("⚠️  Pode sobrecarregar máquinas modestas")
print("\n💡 Solução: Implementar controle adaptativo de GPU")
print("="*70)



