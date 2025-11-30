# Script para testar com "Hello World"
# Execute este script APOS iniciar o TTS service na porta 3002

$testText = "Hello World"

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE HELLO WORLD" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nTexto de teste:" -ForegroundColor Cyan
Write-Host "  '$testText'" -ForegroundColor White

$jsonBody = @{
    text = "<VOICE actor=`"piper_only_test`" emotion=`"neutral`" style=`"narrative`">$testText</VOICE>"
    language = "en"
} | ConvertTo-Json

try {
    Write-Host "`nEnviando requisição..." -ForegroundColor Cyan
    $startTime = Get-Date
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $jsonBody -ContentType "application/json" -ErrorAction Stop -TimeoutSec 30
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalMilliseconds
    
    Write-Host "`n✅ SUCESSO! Tempo de resposta: $([math]::Round($duration, 0)) ms" -ForegroundColor Green
    Write-Host "`nDetalhes do áudio:" -ForegroundColor Cyan
    Write-Host "  Duração: $($response.duration_ms) ms ($([math]::Round($response.duration_ms/1000, 2)) segundos)" -ForegroundColor White
    Write-Host "  Sample rate: $($response.sample_rate) Hz" -ForegroundColor White
    Write-Host "  Channels: $($response.channels)" -ForegroundColor White
    Write-Host "  Amostras: $($response.audio.Count)" -ForegroundColor White
    
    if ($response.audio -and $response.audio.Count -gt 0) {
        Write-Host "`nConvertendo áudio para WAV..." -ForegroundColor Cyan
        
        $sampleRate = $response.sample_rate
        $channels = $response.channels
        $samples = $response.audio
        
        $outputFile = Join-Path $PWD "test_hello_world.wav"
        
        $bytesPerSample = 2
        $dataSize = $samples.Count * $bytesPerSample
        $fileSize = 36 + $dataSize
        $byteRate = $sampleRate * $channels * $bytesPerSample
        $blockAlign = $channels * $bytesPerSample
        
        $ms = New-Object System.IO.MemoryStream
        $writer = New-Object System.IO.BinaryWriter $ms
        
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("RIFF"))
        $writer.Write([uint32]$fileSize)
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("WAVE"))
        
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("fmt "))
        $writer.Write([uint32]16)
        $writer.Write([uint16]1)
        $writer.Write([uint16]$channels)
        $writer.Write([uint32]$sampleRate)
        $writer.Write([uint32]$byteRate)
        $writer.Write([uint16]$blockAlign)
        $writer.Write([uint16]16)
        
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("data"))
        $writer.Write([uint32]$dataSize)
        
        foreach ($sample in $samples) {
            $clamped = [Math]::Max(-1.0, [Math]::Min(1.0, [double]$sample))
            $int16Sample = [int16]([Math]::Round($clamped * 32767.0))
            $writer.Write($int16Sample)
        }
        
        $writer.Close()
        [System.IO.File]::WriteAllBytes($outputFile, $ms.ToArray())
        $ms.Close()
        
        Write-Host "`n✅ Áudio salvo em: $outputFile" -ForegroundColor Green
        Write-Host "   Tamanho: $([math]::Round((Get-Item $outputFile).Length/1KB, 2)) KB" -ForegroundColor White
        
        Write-Host "`nAbrindo arquivo..." -ForegroundColor Cyan
        Start-Process $outputFile
    }
} catch {
    Write-Host "`n❌ ERRO:" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Yellow
}



