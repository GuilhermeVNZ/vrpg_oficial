# Script de Teste do Modelo SoVITS
# Execute este script APOS iniciar o TTS service na porta 3002

$testText = "In the dim light of the ancient library, dust motes danced in the air like forgotten memories. The old tome lay open on the mahogany desk, its pages yellowed with age and secrets. As the reader's fingers traced the ancient runes, a whisper seemed to echo from the very stones themselves, telling tales of heroes long past and battles yet to come."

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE DO MODELO SOVITS" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nTexto de narracao:" -ForegroundColor Cyan
Write-Host $testText -ForegroundColor White
Write-Host "`nEnviando requisicao para TTS service..." -ForegroundColor Cyan

$jsonBody = @{
    text = "<VOICE actor=`"dungeon_master_en`" emotion=`"neutral`" style=`"narrative`">$testText</VOICE>"
    language = "en"
} | ConvertTo-Json

try {
    Write-Host "Aguardando resposta (pode demorar alguns segundos)..." -ForegroundColor Yellow
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $jsonBody -ContentType "application/json" -ErrorAction Stop -TimeoutSec 120
    
    Write-Host "`nSUCESSO! Audio gerado!" -ForegroundColor Green
    Write-Host "`nDetalhes do audio:" -ForegroundColor Cyan
    Write-Host "  Duracao: $($response.duration_ms) ms ($([math]::Round($response.duration_ms/1000, 2)) segundos)" -ForegroundColor White
    Write-Host "  Sample rate: $($response.sample_rate) Hz" -ForegroundColor White
    Write-Host "  Channels: $($response.channels)" -ForegroundColor White
    Write-Host "  Actor: $($response.actor)" -ForegroundColor White
    Write-Host "  Emotion: $($response.emotion)" -ForegroundColor White
    Write-Host "  Style: $($response.style)" -ForegroundColor White
    
    if ($response.audio -and $response.audio.Count -gt 0) {
        Write-Host "`nAudio recebido: $($response.audio.Count) amostras" -ForegroundColor Green
        Write-Host "`nModelo SoVITS 'dungeon_master_en' funcionando corretamente!" -ForegroundColor Green
        Write-Host "O audio foi gerado com sucesso usando a voz do mestre treinada." -ForegroundColor Green
    }
} catch {
    Write-Host "`nERRO na requisicao:" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Yellow
    if ($_.Exception.Response) {
        try {
            $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
            $responseBody = $reader.ReadToEnd()
            Write-Host "Resposta do servidor: $responseBody" -ForegroundColor Yellow
        } catch {}
    }
    Write-Host "`nCertifique-se de que o TTS service esta rodando na porta 3002" -ForegroundColor Yellow
}
