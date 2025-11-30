# Script para testar a velocidade corrigida do Piper
# Execute este script APOS iniciar o TTS service na porta 3002

$testText = "In the ancient library, dust motes danced in the air like forgotten memories. The old tome lay open on the mahogany desk, its pages yellowed with age and secrets. As the reader's fingers traced the ancient runes, a whisper seemed to echo from the very stones themselves, telling tales of heroes long past and battles yet to come."

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE VELOCIDADE CORRIGIDA (length_scale=1.5)" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nTexto de teste (329 caracteres):" -ForegroundColor Cyan
Write-Host $testText -ForegroundColor White
Write-Host "`nEnviando requisição para TTS service..." -ForegroundColor Cyan

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
    Write-Host "`nDetalhes do áudio:" -ForegroundColor Cyan
    Write-Host "  Duração: $($response.duration_ms) ms ($([math]::Round($response.duration_ms/1000, 2)) segundos)" -ForegroundColor White
    Write-Host "  Sample rate: $($response.sample_rate) Hz" -ForegroundColor White
    Write-Host "  Channels: $($response.channels)" -ForegroundColor White
    Write-Host "  Amostras: $($response.audio.Count)" -ForegroundColor White
    
    # Calcular duração esperada (estimativa: ~150ms por caractere para fala normal)
    $expectedDuration = ($testText.Length * 0.15) * 1000  # em ms
    Write-Host "`nAnalise de velocidade:" -ForegroundColor Cyan
    Write-Host "  Duração esperada (fala normal): ~$([math]::Round($expectedDuration, 0)) ms" -ForegroundColor White
    Write-Host "  Duração obtida: $($response.duration_ms) ms" -ForegroundColor White
    
    $ratio = $response.duration_ms / $expectedDuration
    if ($ratio -lt 0.7) {
        Write-Host "  ATENCAO: Audio ainda muito rapido (ratio: $([math]::Round($ratio, 2)))" -ForegroundColor Yellow
        Write-Host "     Pode ser necessário aumentar ainda mais o length_scale" -ForegroundColor Yellow
    } elseif ($ratio -gt 1.3) {
        Write-Host "  ATENCAO: Audio muito lento (ratio: $([math]::Round($ratio, 2)))" -ForegroundColor Yellow
        Write-Host "     Pode ser necessário diminuir o length_scale" -ForegroundColor Yellow
    } else {
        Write-Host "  OK Duracao parece adequada (ratio: $([math]::Round($ratio, 2)))" -ForegroundColor Green
    }
    
    if ($response.audio -and $response.audio.Count -gt 0) {
        Write-Host "`nConvertendo áudio para WAV..." -ForegroundColor Cyan
        
        $sampleRate = $response.sample_rate
        $channels = $response.channels
        $samples = $response.audio
        
        # Criar arquivo WAV
        $outputFile = Join-Path $PWD "test_velocidade_corrigida.wav"
        
        # WAV Header
        $bytesPerSample = 2 # 16-bit = 2 bytes por sample
        $dataSize = $samples.Count * $bytesPerSample
        $fileSize = 36 + $dataSize
        $byteRate = $sampleRate * $channels * $bytesPerSample
        $blockAlign = $channels * $bytesPerSample
        
        # Criar MemoryStream para escrever o WAV
        $ms = New-Object System.IO.MemoryStream
        $writer = New-Object System.IO.BinaryWriter $ms
        
        # RIFF header
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("RIFF"))
        $writer.Write([uint32]$fileSize)
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("WAVE"))
        
        # fmt chunk
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("fmt "))
        $writer.Write([uint32]16)
        $writer.Write([uint16]1)
        $writer.Write([uint16]$channels)
        $writer.Write([uint32]$sampleRate)
        $writer.Write([uint32]$byteRate)
        $writer.Write([uint16]$blockAlign)
        $writer.Write([uint16]16)
        
        # data chunk
        $writer.Write([System.Text.Encoding]::ASCII.GetBytes("data"))
        $writer.Write([uint32]$dataSize)
        
        # Escrever samples
        foreach ($sample in $samples) {
            $clamped = [Math]::Max(-1.0, [Math]::Min(1.0, [double]$sample))
            $int16Sample = [int16]([Math]::Round($clamped * 32767.0))
            $writer.Write($int16Sample)
        }
        
        $writer.Close()
        [System.IO.File]::WriteAllBytes($outputFile, $ms.ToArray())
        $ms.Close()
        
        Write-Host "`nOK Audio salvo em: $outputFile" -ForegroundColor Green
        Write-Host "   Tamanho: $([math]::Round((Get-Item $outputFile).Length/1KB, 2)) KB" -ForegroundColor White
        
        # Abrir automaticamente
        Write-Host "`nAbrindo arquivo automaticamente..." -ForegroundColor Cyan
        Start-Process $outputFile
        Write-Host "OK Arquivo aberto no player padrao" -ForegroundColor Green
        
        Write-Host "`nVERIFIQUE:" -ForegroundColor Yellow
        Write-Host "   - A velocidade da fala deve estar mais lenta e natural" -ForegroundColor White
        Write-Host "   - O áudio deve ser inteligível (palavras claras)" -ForegroundColor White
        Write-Host "   - A duração deve ser adequada para o texto" -ForegroundColor White
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
    Write-Host "`nCertifique-se de que o TTS service está rodando na porta 3002" -ForegroundColor Yellow
}

