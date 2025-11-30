# Script para testar TTS com texto narrativo
# Execute este script APOS iniciar o TTS service na porta 3002

$narrativeText = @"
In the dim light of the ancient library, dust motes danced in the air like forgotten memories. The old tome lay open on the mahogany desk, its pages yellowed with age and secrets. As the reader's fingers traced the ancient runes, a whisper seemed to echo from the very stones themselves, telling tales of heroes long past and battles yet to come. The shadows cast by flickering candlelight seemed to move with a life of their own, and in the silence between heartbeats, one could almost hear the voices of those who had come before, their stories etched into the very fabric of this sacred place.
"@

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE DE NARRAÇÃO - TTS COMPLETO" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nTexto narrativo:" -ForegroundColor Cyan
Write-Host $narrativeText -ForegroundColor White
Write-Host "`nEnviando requisição para TTS service..." -ForegroundColor Cyan

$jsonBody = @{
    text = "<VOICE actor=`"dungeon_master_en`" emotion=`"neutral`" style=`"narrative`">$narrativeText</VOICE>"
    language = "en"
} | ConvertTo-Json

try {
    Write-Host "Aguardando resposta (pode demorar alguns segundos)..." -ForegroundColor Yellow
    $startTime = Get-Date
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $jsonBody -ContentType "application/json" -ErrorAction Stop -TimeoutSec 180
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalMilliseconds
    
    Write-Host "`n✅ SUCESSO! Tempo de resposta: $([math]::Round($duration, 0)) ms" -ForegroundColor Green
    Write-Host "`nDetalhes do áudio:" -ForegroundColor Cyan
    Write-Host "  Duração: $($response.duration_ms) ms ($([math]::Round($response.duration_ms/1000, 2)) segundos)" -ForegroundColor White
    Write-Host "  Sample rate: $($response.sample_rate) Hz" -ForegroundColor White
    Write-Host "  Channels: $($response.channels)" -ForegroundColor White
    Write-Host "  Actor: $($response.actor)" -ForegroundColor White
    Write-Host "  Emotion: $($response.emotion)" -ForegroundColor White
    Write-Host "  Style: $($response.style)" -ForegroundColor White
    
    if ($response.audio -and $response.audio.Count -gt 0) {
        Write-Host "`nAudio recebido: $($response.audio.Count) amostras" -ForegroundColor Green
        
        # Converter floats para bytes de áudio WAV
        Write-Host "`nConvertendo áudio para WAV..." -ForegroundColor Cyan
        
        $sampleRate = $response.sample_rate
        $channels = $response.channels
        $samples = $response.audio
        
        # Criar arquivo WAV
        $outputFile = Join-Path $PWD "test_narrative_dungeon_master_en.wav"
        
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
        $writer.Write([uint32]$fileSize)
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
        foreach ($sample in $samples) {
            $clamped = [Math]::Max(-1.0, [Math]::Min(1.0, [double]$sample))
            $int16Sample = [int16]([Math]::Round($clamped * 32767.0))
            $writer.Write($int16Sample)
        }
        
        $writer.Close()
        
        # Salvar arquivo
        [System.IO.File]::WriteAllBytes($outputFile, $ms.ToArray())
        $ms.Close()
        
        Write-Host "`n✅ Áudio salvo em: $outputFile" -ForegroundColor Green
        $fileInfo = Get-Item $outputFile
        Write-Host "  Tamanho: $([math]::Round($fileInfo.Length/1KB, 2)) KB" -ForegroundColor White
        Write-Host "  Duração calculada: $([math]::Round($dataSize / ($sampleRate * $channels * 2), 2)) segundos" -ForegroundColor White
        
        Write-Host "`n🎧 Abrindo arquivo automaticamente..." -ForegroundColor Cyan
        Start-Process $outputFile
        
        Write-Host "`n✅ Teste concluído com sucesso!" -ForegroundColor Green
        Write-Host "`nVerificações:" -ForegroundColor Yellow
        Write-Host "  1. Pronúncia deve estar clara e compreensível" -ForegroundColor White
        Write-Host "  2. Voz deve se parecer com a do mestre treinado" -ForegroundColor White
        Write-Host "  3. Duração deve ser adequada ao texto" -ForegroundColor White
    } else {
        Write-Host "`n⚠️  AVISO: Nenhum áudio recebido na resposta" -ForegroundColor Yellow
    }
} catch {
    Write-Host "`n❌ ERRO na requisição:" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Yellow
    if ($_.Exception.Response) {
        try {
            $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
            $responseBody = $reader.ReadToEnd()
            Write-Host "Resposta do servidor: $responseBody" -ForegroundColor Yellow
        } catch {}
    }
    Write-Host "`nCertifique-se de que o TTS service está rodando na porta 3002" -ForegroundColor Yellow
    Write-Host "Inicie com: cargo run --bin tts-server" -ForegroundColor Cyan
}

