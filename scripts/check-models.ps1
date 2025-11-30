# Script para verificar e baixar modelos necessários
# Usage: .\scripts\check-models.ps1

$ErrorActionPreference = "Stop"

$modelsDir = "G:\vrpg\vrpg-client\assets-and-models\models"
$llmDir = Join-Path $modelsDir "llm"
$asrDir = Join-Path $modelsDir "asr"
$ttsDir = Join-Path $modelsDir "tts"

Write-Host "🔍 Verificando modelos necessários..." -ForegroundColor Cyan
Write-Host ""

# Verificar modelo LLM
Write-Host "📦 Modelo LLM:" -ForegroundColor Yellow
$llmModel = Join-Path $llmDir "qwen2.5-14b-instruct-q4_k_m.gguf"
if (Test-Path $llmModel) {
    $size = (Get-Item $llmModel).Length / 1GB
    Write-Host "  ✅ Qwen 2.5 14B encontrado: $([math]::Round($size, 2)) GB" -ForegroundColor Green
} else {
    Write-Host "  ❌ Qwen 2.5 14B não encontrado" -ForegroundColor Red
    Write-Host "     URL: https://huggingface.co/Qwen/Qwen2.5-14B-Instruct-GGUF" -ForegroundColor Gray
    Write-Host "     Arquivo: qwen2.5-14b-instruct-q4_k_m.gguf" -ForegroundColor Gray
}

Write-Host ""

# Verificar modelo ASR
Write-Host "🎤 Modelo ASR (Whisper):" -ForegroundColor Yellow
$asrModel = Join-Path $asrDir "whisper-large-v3.bin"
$asrModelAlt = Join-Path $asrDir "ggml-large-v3.bin"
if (Test-Path $asrModel) {
    $size = (Get-Item $asrModel).Length / 1GB
    Write-Host "  ✅ Whisper Large V3 encontrado: $([math]::Round($size, 2)) GB" -ForegroundColor Green
} elseif (Test-Path $asrModelAlt) {
    $size = (Get-Item $asrModelAlt).Length / 1GB
    Write-Host "  ✅ Whisper Large V3 (ggml) encontrado: $([math]::Round($size, 2)) GB" -ForegroundColor Green
} else {
    Write-Host "  ⚠️  Whisper Large V3 não encontrado (opcional)" -ForegroundColor Yellow
    Write-Host "     URL: https://huggingface.co/ggerganov/whisper.cpp" -ForegroundColor Gray
    Write-Host "     Arquivo: ggml-large-v3.bin" -ForegroundColor Gray
}

Write-Host ""

# Verificar modelo TTS
Write-Host "🔊 Modelo TTS:" -ForegroundColor Yellow
$ttsModel = Join-Path $ttsDir "xtts_v2.onnx"
$ttsModelAlt = Join-Path $ttsDir "model.pth"
if (Test-Path $ttsModel) {
    $size = (Get-Item $ttsModel).Length / 1GB
    Write-Host "  ✅ XTTS v2 encontrado: $([math]::Round($size, 2)) GB" -ForegroundColor Green
} elseif (Test-Path $ttsModelAlt) {
    $size = (Get-Item $ttsModelAlt).Length / 1GB
    $sizeRounded = [math]::Round($size, 2)
    Write-Host "  ✅ XTTS v2 (PyTorch) encontrado: $sizeRounded GB" -ForegroundColor Green
} else {
    Write-Host "  ⚠️  XTTS v2 não encontrado (opcional)" -ForegroundColor Yellow
    Write-Host "     URL: https://huggingface.co/coqui/XTTS-v2" -ForegroundColor Gray
    Write-Host "     Arquivo: model.pth" -ForegroundColor Gray
}

Write-Host ""
Write-Host "📋 Resumo:" -ForegroundColor Cyan
Write-Host "  - LLM: Necessário para funcionamento básico" -ForegroundColor White
Write-Host "  - ASR: Opcional (pode usar input de texto)" -ForegroundColor White
Write-Host "  - TTS: Opcional (pode usar texto na tela)" -ForegroundColor White
Write-Host ""

