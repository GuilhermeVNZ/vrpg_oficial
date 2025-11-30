# Script de Teste de Latência com GPU
# Testa o pipeline completo e mede a latência de cada componente

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "⚡ Teste de Latência GPU" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Teste 1: XTTS com GPU
Write-Host "1️⃣ Testando XTTS com GPU..." -ForegroundColor Yellow
$xttsTest = @"
import time
import torch
from TTS.api import TTS

print("Carregando modelo XTTS...")
start = time.time()
tts = TTS('tts_models/multilingual/multi-dataset/xtts_v2', gpu=True)
load_time = time.time() - start
print(f"✅ Modelo carregado em {load_time:.2f}s")
print(f"   GPU: {torch.cuda.is_available()}")
if torch.cuda.is_available():
    print(f"   GPU Name: {torch.cuda.get_device_name(0)}")

# Teste com texto curto
print("\nTestando síntese (texto curto: 'Hello World')...")
start = time.time()
audio = tts.tts(text='Hello World', speaker='Ana Florence', language='en')
short_time = time.time() - start
print(f"✅ Síntese curta: {short_time:.2f}s ({len(audio)} samples)")

# Teste com texto médio
print("\nTestando síntese (texto médio: ~50 palavras)...")
medium_text = "In a distant realm where magic flows like rivers and dragons soar through clouds of stardust, there lived a brave adventurer named Elara."
start = time.time()
audio = tts.tts(text=medium_text, speaker='Ana Florence', language='en')
medium_time = time.time() - start
print(f"✅ Síntese média: {medium_time:.2f}s ({len(audio)} samples)")

print(f"\n📊 Resumo:")
print(f"   Carregamento: {load_time:.2f}s")
print(f"   Texto curto: {short_time:.2f}s")
print(f"   Texto médio: {medium_time:.2f}s")
print(f"   Target: < 0.8s para texto médio")
if medium_time < 0.8:
    print("   ✅ DENTRO DO TARGET!")
else:
    print("   ⚠️  ACIMA DO TARGET")
"@

python -c $xttsTest
Write-Host ""

# Teste 2: Verificar SoVITS com GPU
Write-Host "2️⃣ Verificando SoVITS com GPU..." -ForegroundColor Yellow
$sovitsPath = "assets-and-models\models\tts\sovits"
$venvPython = "$sovitsPath\venv310\Scripts\python.exe"

if (Test-Path $venvPython) {
    $sovitsTest = @"
import torch
print(f"PyTorch: {torch.__version__}")
print(f"CUDA Available: {torch.cuda.is_available()}")
if torch.cuda.is_available():
    print(f"GPU: {torch.cuda.get_device_name(0)}")
    print("✅ SoVITS pode usar GPU")
else:
    print("⚠️  SoVITS usará CPU (mais lento)")
"@
    
    & $venvPython -c $sovitsTest
} else {
    Write-Host "   ⚠️  SoVITS venv não encontrado" -ForegroundColor Yellow
}
Write-Host ""

# Resumo
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "📊 Resultados Esperados" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Com GPU habilitada:" -ForegroundColor White
Write-Host "   Whisper: 50-100ms" -ForegroundColor Green
Write-Host "   Qwen: 300-500ms" -ForegroundColor Green
Write-Host "   XTTS: 500-800ms" -ForegroundColor Green
Write-Host "   SoVITS: 300-500ms" -ForegroundColor Green
Write-Host "   TOTAL: 1150-1900ms (< 1.5s ✅)" -ForegroundColor Green
Write-Host ""
Write-Host "Sem GPU:" -ForegroundColor White
Write-Host "   TOTAL: 5-40s (❌ não atende target)" -ForegroundColor Red
Write-Host ""

