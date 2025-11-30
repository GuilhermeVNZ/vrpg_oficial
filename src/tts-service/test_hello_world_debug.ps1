# Script para testar Hello World e capturar logs detalhados

$testText = "Hello World"

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE HELLO WORLD - DEBUG" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nTexto: '$testText'" -ForegroundColor Cyan
Write-Host "`nEnviando requisição e aguardando logs do servidor..." -ForegroundColor Yellow
Write-Host "Verifique a janela do servidor TTS para ver os logs detalhados de:" -ForegroundColor White
Write-Host "  - Phonemization (quantos fonemas IPA foram gerados)" -ForegroundColor Gray
Write-Host "  - Phoneme mapping (quantos foram mapeados vs pulados)" -ForegroundColor Gray
Write-Host "  - Piper inference (quantos samples foram gerados)" -ForegroundColor Gray

$jsonBody = @{
    text = "<VOICE actor=`"piper_only_test`" emotion=`"neutral`" style=`"narrative`">$testText</VOICE>"
    language = "en"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $jsonBody -ContentType "application/json" -ErrorAction Stop -TimeoutSec 30
    
    Write-Host "`n✅ Resposta recebida" -ForegroundColor Green
    Write-Host "  Duração: $($response.duration_ms) ms" -ForegroundColor White
    Write-Host "  Amostras: $($response.audio.Count)" -ForegroundColor White
    Write-Host "`n⚠️ PROBLEMA: Áudio muito curto para 'Hello World' (esperado ~1-2s)" -ForegroundColor Yellow
    Write-Host "`nIsso indica que:" -ForegroundColor Cyan
    Write-Host "  1. Poucos fonemas foram gerados/mapeados" -ForegroundColor White
    Write-Host "  2. Muitos fonemas foram pulados (unknown phonemes)" -ForegroundColor White
    Write-Host "  3. O modelo ONNX não está gerando áudio suficiente" -ForegroundColor White
    Write-Host "`nVerifique os logs do servidor para diagnóstico detalhado" -ForegroundColor Yellow
    
} catch {
    Write-Host "`n❌ ERRO: $($_.Exception.Message)" -ForegroundColor Red
}



