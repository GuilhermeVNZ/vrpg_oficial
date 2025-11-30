# Teste de narração em inglês com modelo SoVITS treinado

$baseUrl = "http://localhost:3002"
$outputFile = "narracao_test.wav"

# Texto de narração em inglês (como se fosse um trecho de livro)
$narrationText = @"
The ancient castle stood atop the misty mountain, its stone walls weathered by centuries of storms. 
As the sun set behind the distant peaks, shadows crept across the courtyard, and the old wooden doors 
creaked in the evening breeze. Inside, the grand hall echoed with the whispers of forgotten tales, 
where knights once gathered to plan their quests and wizards shared secrets of the arcane arts.
"@

Write-Host "========================================" -ForegroundColor Green
Write-Host "  TESTE DE NARRACAO EM INGLES" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nTexto de narracao:" -ForegroundColor Cyan
Write-Host $narrationText -ForegroundColor White
Write-Host "`nEnviando para o TTS Service..." -ForegroundColor Cyan

# Preparar requisição - O texto deve conter a tag VOICE
$textWithVoice = "<VOICE actor=`"dungeon_master`" emotion=`"neutral`" style=`"narration`">$narrationText</VOICE>"

$requestBody = @{
    text = $textWithVoice
    language = "en"
} | ConvertTo-Json

try {
    Write-Host "`nSintetizando audio..." -ForegroundColor Yellow
    
    $response = Invoke-RestMethod -Uri "$baseUrl/speak" `
        -Method Post `
        -Body $requestBody `
        -ContentType "application/json" `
        -TimeoutSec 30
    
    if ($response.audio_base64) {
        Write-Host "`n[SUCESSO] Audio gerado!" -ForegroundColor Green
        Write-Host "  Duracao: $($response.duration_ms) ms" -ForegroundColor White
        Write-Host "  Sample rate: $($response.sample_rate) Hz" -ForegroundColor White
        Write-Host "  Canais: $($response.channels)" -ForegroundColor White
        
        # Decodificar base64 e salvar WAV
        Write-Host "`nSalvando audio em: $outputFile" -ForegroundColor Cyan
        $audioBytes = [Convert]::FromBase64String($response.audio_base64)
        
        # Ler o WAV que já vem do servidor (já tem header)
        [System.IO.File]::WriteAllBytes($outputFile, $audioBytes)
        
        Write-Host "`n[OK] Audio salvo em: $outputFile" -ForegroundColor Green
        Write-Host "`nPara ouvir, execute:" -ForegroundColor Yellow
        Write-Host "  Start-Process `"$outputFile`"" -ForegroundColor White
        
        # Abrir automaticamente
        Start-Process $outputFile
        
    } else {
        Write-Host "`n[ERRO] Resposta invalida do servidor" -ForegroundColor Red
        $response | ConvertTo-Json -Depth 3 | Write-Host
    }
    
} catch {
    Write-Host "`n[ERRO] Falha ao sintetizar audio:" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "`nResposta do servidor:" -ForegroundColor Yellow
        Write-Host $responseBody -ForegroundColor White
    }
    
    Write-Host "`nVerifique se o servidor TTS esta rodando:" -ForegroundColor Yellow
    Write-Host "  cargo run --release --bin tts-server" -ForegroundColor White
}

Write-Host "`n========================================" -ForegroundColor Green

