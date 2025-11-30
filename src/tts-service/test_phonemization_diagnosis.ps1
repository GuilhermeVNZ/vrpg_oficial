# Script para diagnosticar problemas de fonemização
# Execute este script APOS iniciar o TTS service na porta 3002

$testText = "Hello world"

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  DIAGNÓSTICO DE FONEMIZAÇÃO" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nTexto de teste:" -ForegroundColor Cyan
Write-Host "  '$testText'" -ForegroundColor White

# Testar espeak-ng diretamente
Write-Host "`n=== TESTE 1: ESPEAK-NG DIRETO ===" -ForegroundColor Yellow
$espeakPath = "C:\Program Files\eSpeak NG\espeak-ng.exe"
if (Test-Path $espeakPath) {
    $output = & $espeakPath -q --ipa -v en "$testText" 2>&1
    Write-Host "Saída IPA do espeak-ng:" -ForegroundColor Cyan
    Write-Host "  '$output'" -ForegroundColor White
    Write-Host "`nCaracteres:" -ForegroundColor Gray
    $output.ToCharArray() | ForEach-Object { Write-Host "  '$_' (U+$([int][char]$_).ToString('X4'))" -ForegroundColor Gray }
} else {
    Write-Host "espeak-ng não encontrado em $espeakPath" -ForegroundColor Red
}

# Testar Python phonemizer
Write-Host "`n=== TESTE 2: PYTHON PHONEMIZER ===" -ForegroundColor Yellow
$pythonCmd = "G:\vrpg\vrpg-client\assets-and-models\models\tts\sovits\venv310\Scripts\python.exe"
$scriptPath = "G:\vrpg\vrpg-client\src\tts-service\scripts\phonemize_for_piper.py"
if (Test-Path $pythonCmd -and Test-Path $scriptPath) {
    $output = & $pythonCmd $scriptPath $testText -l en-us 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Saída do Python phonemizer:" -ForegroundColor Cyan
        $output | ForEach-Object { Write-Host "  '$_'" -ForegroundColor White }
    } else {
        Write-Host "Erro ao executar Python phonemizer" -ForegroundColor Red
        $output | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    }
} else {
    Write-Host "Python ou script não encontrado" -ForegroundColor Red
}

# Testar TTS service
Write-Host "`n=== TESTE 3: TTS SERVICE ===" -ForegroundColor Yellow
$jsonBody = @{
    text = "<VOICE actor=`"piper_only_test`" emotion=`"neutral`" style=`"narrative`">$testText</VOICE>"
    language = "en"
} | ConvertTo-Json

$response = $null
try {
    Write-Host "Enviando requisição para TTS service..." -ForegroundColor Cyan
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $jsonBody -ContentType "application/json" -TimeoutSec 30
} catch {
    Write-Host "`n❌ Erro na requisição: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Certifique-se de que o TTS service está rodando na porta 3002" -ForegroundColor Yellow
}

if ($response) {
    Write-Host "`n✅ Resposta recebida" -ForegroundColor Green
    Write-Host "   Duração: $($response.duration_ms) ms" -ForegroundColor White
    Write-Host "   Amostras: $($response.audio.Count)" -ForegroundColor White
    Write-Host "`n📋 VERIFIQUE OS LOGS DO SERVIDOR para ver:" -ForegroundColor Yellow
    Write-Host "   - Fonemas IPA gerados" -ForegroundColor Gray
    Write-Host "   - Mapeamento para IDs" -ForegroundColor Gray
    Write-Host "   - Quantidade de pausas" -ForegroundColor Gray
}

Write-Host "`n========================================" -ForegroundColor Green
