# Script para testar TTS e salvar audio em arquivo WAV (CORRIGIDO)
# Execute este script APOS iniciar o TTS service na porta 3002

$testText = "In the ancient library, dust motes danced in the air like forgotten memories. The old tome lay open on the mahogany desk, its pages yellowed with age and secrets. As the reader's fingers traced the ancient runes, a whisper seemed to echo from the very stones themselves, telling tales of heroes long past and battles yet to come."

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE E SALVAMENTO DE AUDIO (CORRIGIDO)" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nTexto de narracao:" -ForegroundColor Cyan
Write-Host $testText -ForegroundColor White
Write-Host "`nEnviando requisicao para TTS service..." -ForegroundColor Cyan

$jsonBody = @{
    text = "<VOICE actor=`"piper_only_test`" emotion=`"neutral`" style=`"narrative`">$testText</VOICE>"
    language = "en"
} | ConvertTo-Json

try {
    Write-Host "Aguardando resposta (pode demorar alguns segundos)..." -ForegroundColor Yellow
    $startTime = Get-Date
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $jsonBody -ContentType "application/json" -ErrorAction Stop -TimeoutSec 120
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalMilliseconds
    
    Write-Host "`nSUCESSO! Tempo de resposta: $([math]::Round($duration, 0)) ms" -ForegroundColor Green
    Write-Host "`nDetalhes do audio:" -ForegroundColor Cyan
    Write-Host "  Duracao: $($response.duration_ms) ms ($([math]::Round($response.duration_ms/1000, 2)) segundos)" -ForegroundColor White
    Write-Host "  Sample rate: $($response.sample_rate) Hz" -ForegroundColor White
    Write-Host "  Channels: $($response.channels)" -ForegroundColor White
    Write-Host "  Amostras: $($response.audio.Count)" -ForegroundColor White
    
    if ($response.audio -and $response.audio.Count -gt 0) {
        Write-Host "`nAudio recebido: $($response.audio.Count) amostras" -ForegroundColor Green
        
        # Converter floats para bytes de audio WAV
        Write-Host "`nConvertendo audio para WAV..." -ForegroundColor Cyan
        
        $sampleRate = $response.sample_rate
        $channels = $response.channels
        $samples = $response.audio
        
        # Criar arquivo WAV
        $outputFile = Join-Path $PWD "test_audio_fixed.wav"
        
        # Calcular tamanhos
        $bitsPerSample = 16
        $bytesPerSample = $bitsPerSample / 8
        $dataSize = $samples.Count * $channels * $bytesPerSample
        $fileSize = 36 + $dataSize  # 36 = 12 (RIFF header) + 24 (fmt chunk) + dataSize
        
        Write-Host "  Data size: $dataSize bytes" -ForegroundColor Gray
        Write-Host "  File size: $fileSize bytes" -ForegroundColor Gray
        
        # Criar MemoryStream para escrever o WAV
        $ms = New-Object System.IO.MemoryStream
        $writer = New-Object System.IO.BinaryWriter $ms
        
        # RIFF header
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("RIFF"))
        $writer.Write([uint32]$fileSize)  # File size - 8 (sem contar "RIFF" e este campo)
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("WAVE"))
        
        # fmt chunk
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("fmt "))
        $writer.Write([uint32]16)  # fmt chunk size (16 para PCM)
        $writer.Write([uint16]1)    # audio format (1 = PCM)
        $writer.Write([uint16]$channels)
        $writer.Write([uint32]$sampleRate)
        $byteRate = $sampleRate * $channels * $bytesPerSample
        $writer.Write([uint32]$byteRate)
        $blockAlign = $channels * $bytesPerSample
        $writer.Write([uint16]$blockAlign)
        $writer.Write([uint16]$bitsPerSample)  # bits per sample
        
        # data chunk
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("data"))
        $writer.Write([uint32]$dataSize)
        
        # Escrever samples (converter float [-1.0, 1.0] para int16 [-32768, 32767])
        foreach ($sample in $samples) {
            # Clamp sample to [-1.0, 1.0]
            $clamped = [Math]::Max(-1.0, [Math]::Min(1.0, [double]$sample))
            # Convert to int16 (scale to full range)
            $int16Sample = [int16]([Math]::Round($clamped * 32767.0))
            $writer.Write($int16Sample)
        }
        
        $writer.Close()
        
        # Salvar arquivo
        [System.IO.File]::WriteAllBytes($outputFile, $ms.ToArray())
        $ms.Close()
        
        Write-Host "`nAudio salvo em: $outputFile" -ForegroundColor Green
        Write-Host "Tamanho: $([math]::Round((Get-Item $outputFile).Length/1KB, 2)) KB" -ForegroundColor White
        
        # Verificar o arquivo
        $verifyBytes = [System.IO.File]::ReadAllBytes($outputFile)
        $verifyRiff = [System.Text.Encoding]::ASCII.GetString($verifyBytes[0..3])
        $verifyWave = [System.Text.Encoding]::ASCII.GetString($verifyBytes[8..11])
        $verifyBits = [BitConverter]::ToUInt16($verifyBytes, 34)
        Write-Host "`nVerificacao:" -ForegroundColor Cyan
        Write-Host "  RIFF: $verifyRiff" -ForegroundColor $(if ($verifyRiff -eq "RIFF") { "Green" } else { "Red" })
        Write-Host "  WAVE: $verifyWave" -ForegroundColor $(if ($verifyWave -eq "WAVE") { "Green" } else { "Red" })
        Write-Host "  Bits per sample: $verifyBits" -ForegroundColor $(if ($verifyBits -eq 16) { "Green" } else { "Red" })
        
        # Abrir automaticamente
        Write-Host "`nAbrindo arquivo automaticamente..." -ForegroundColor Cyan
        Start-Process $outputFile
        Write-Host "✅ Arquivo aberto no player padrao" -ForegroundColor Green
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



