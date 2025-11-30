# Script para testar TTS e salvar audio em arquivo WAV
# Execute este script APOS iniciar o TTS service na porta 3002

$testText = "In the dim light of the ancient library, dust motes danced in the air like forgotten memories. The old tome lay open on the mahogany desk, its pages yellowed with age and secrets. As the reader's fingers traced the ancient runes, a whisper seemed to echo from the very stones themselves, telling tales of heroes long past and battles yet to come."

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE E SALVAMENTO DE AUDIO" -ForegroundColor Green
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
    $startTime = Get-Date
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $jsonBody -ContentType "application/json" -ErrorAction Stop -TimeoutSec 120
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalMilliseconds
    
    Write-Host "`nSUCESSO! Tempo de resposta: $([math]::Round($duration, 0)) ms" -ForegroundColor Green
    Write-Host "`nDetalhes do audio:" -ForegroundColor Cyan
    Write-Host "  Duracao: $($response.duration_ms) ms ($([math]::Round($response.duration_ms/1000, 2)) segundos)" -ForegroundColor White
    Write-Host "  Sample rate: $($response.sample_rate) Hz" -ForegroundColor White
    Write-Host "  Channels: $($response.channels)" -ForegroundColor White
    Write-Host "  Actor: $($response.actor)" -ForegroundColor White
    Write-Host "  Emotion: $($response.emotion)" -ForegroundColor White
    Write-Host "  Style: $($response.style)" -ForegroundColor White
    
    if ($response.audio -and $response.audio.Count -gt 0) {
        Write-Host "`nAudio recebido: $($response.audio.Count) amostras" -ForegroundColor Green
        
        # Converter floats para bytes de audio WAV
        Write-Host "`nConvertendo audio para WAV..." -ForegroundColor Cyan
        
        $sampleRate = $response.sample_rate
        $channels = $response.channels
        $samples = $response.audio
        
        # Criar arquivo WAV
        $outputFile = Join-Path $PWD "test_audio_dungeon_master_en.wav"
        
        # WAV Header
        $dataSize = $samples.Count * 2  # 16-bit = 2 bytes por sample
        $fileSize = 36 + $dataSize
        $byteRate = $sampleRate * $channels * 2
        $blockAlign = $channels * 2
        
        # Criar MemoryStream para escrever o WAV
        $ms = New-Object System.IO.MemoryStream
        $writer = New-Object System.IO.BinaryWriter $ms
        
        # RIFF header (little-endian)
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("RIFF"))
        $writer.Write([uint32]$fileSize)  # Little-endian por padrão no BinaryWriter
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("WAVE"))
        
        # fmt chunk
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("fmt "))
        $writer.Write([uint32]16)  # fmt chunk size
        $writer.Write([uint16]1)    # audio format (PCM)
        $writer.Write([uint16]$channels)
        $writer.Write([uint32]$sampleRate)
        $writer.Write([uint32]$byteRate)
        $writer.Write([uint16]$blockAlign)
        $writer.Write([uint16]16)   # bits per sample
        
        # data chunk
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("data"))
        $writer.Write([uint32]$dataSize)
        
        # Escrever samples (converter float [-1.0, 1.0] para int16 [-32768, 32767])
        # WAV usa little-endian, que é o padrão do BinaryWriter
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
        Write-Host "`nPara reproduzir:" -ForegroundColor Yellow
        Write-Host "  Start-Process `"$outputFile`"" -ForegroundColor Cyan
        Write-Host "`nOu abra o arquivo manualmente com seu player de audio favorito." -ForegroundColor White
        
        # Abrir automaticamente
        Write-Host "`nAbrindo arquivo automaticamente..." -ForegroundColor Cyan
        Start-Process $outputFile
        Write-Host "✅ Arquivo aberto no player padrão" -ForegroundColor Green
        
        Write-Host "`nModelo SoVITS 'dungeon_master_en' funcionando!" -ForegroundColor Green
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

