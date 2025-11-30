# Script para diagnosticar problema de velocidade do áudio
# Testa com texto curto e longo para verificar duração e inteligibilidade

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  DIAGNÓSTICO: VELOCIDADE DO ÁUDIO" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

# Teste 1: Texto curto
Write-Host "`n1. TESTE COM TEXTO CURTO (Hello World)" -ForegroundColor Cyan
$testText1 = "Hello world"
$body1 = @{
    text = '<VOICE actor="piper_only_test" emotion="neutral" style="narrative">' + $testText1 + '</VOICE>'
    language = "en"
} | ConvertTo-Json

try {
    $startTime = Get-Date
    $response1 = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $body1 -ContentType "application/json" -TimeoutSec 30 -ErrorAction Stop
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalMilliseconds
    
    Write-Host "   ✅ Resposta recebida em $([math]::Round($duration, 0)) ms" -ForegroundColor Green
    Write-Host "   📊 Duração do áudio: $($response1.duration_ms) ms ($([math]::Round($response1.duration_ms/1000, 2)) segundos)" -ForegroundColor White
    Write-Host "   📊 Amostras: $($response1.audio.Count)" -ForegroundColor White
    Write-Host "   📊 Sample rate: $($response1.sample_rate) Hz" -ForegroundColor White
    
    # Calcular duração esperada (estimativa: ~150ms por palavra)
    $expectedDuration = $testText1.Split(' ').Count * 150
    Write-Host "   📊 Duração esperada: ~$expectedDuration ms" -ForegroundColor Yellow
    
    if ($response1.duration_ms -lt $expectedDuration * 0.5) {
        Write-Host "   ⚠️ ÁUDIO MUITO CURTO! (menos de 50% do esperado)" -ForegroundColor Red
    } elseif ($response1.duration_ms -lt $expectedDuration * 0.8) {
        Write-Host "   ⚠️ Áudio um pouco curto (menos de 80% do esperado)" -ForegroundColor Yellow
    } else {
        Write-Host "   ✅ Duração parece adequada" -ForegroundColor Green
    }
    
    # Salvar áudio
    $outputFile1 = Join-Path $PWD "test_hello_world_speed.wav"
    Save-AudioToWav -Samples $response1.audio -SampleRate $response1.sample_rate -Channels $response1.channels -OutputFile $outputFile1
    Write-Host "   💾 Áudio salvo em: $outputFile1" -ForegroundColor Cyan
} catch {
    Write-Host "   ❌ Erro: $($_.Exception.Message)" -ForegroundColor Red
}

# Teste 2: Texto médio
Write-Host "`n2. TESTE COM TEXTO MÉDIO (50 caracteres)" -ForegroundColor Cyan
$testText2 = "The quick brown fox jumps over the lazy dog."
$body2 = @{
    text = '<VOICE actor="piper_only_test" emotion="neutral" style="narrative">' + $testText2 + '</VOICE>'
    language = "en"
} | ConvertTo-Json

try {
    $startTime = Get-Date
    $response2 = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $body2 -ContentType "application/json" -TimeoutSec 30 -ErrorAction Stop
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalMilliseconds
    
    Write-Host "   ✅ Resposta recebida em $([math]::Round($duration, 0)) ms" -ForegroundColor Green
    Write-Host "   📊 Duração do áudio: $($response2.duration_ms) ms ($([math]::Round($response2.duration_ms/1000, 2)) segundos)" -ForegroundColor White
    Write-Host "   📊 Amostras: $($response2.audio.Count)" -ForegroundColor White
    Write-Host "   📊 Sample rate: $($response2.sample_rate) Hz" -ForegroundColor White
    
    # Calcular duração esperada
    $wordCount = $testText2.Split(' ').Count
    $expectedDuration = $wordCount * 150
    Write-Host "   📊 Duração esperada: ~$expectedDuration ms (~$([math]::Round($expectedDuration/1000, 2)) segundos)" -ForegroundColor Yellow
    
    if ($response2.duration_ms -lt $expectedDuration * 0.5) {
        Write-Host "   ⚠️ ÁUDIO MUITO CURTO! (menos de 50% do esperado)" -ForegroundColor Red
    } elseif ($response2.duration_ms -lt $expectedDuration * 0.8) {
        Write-Host "   ⚠️ Áudio um pouco curto (menos de 80% do esperado)" -ForegroundColor Yellow
    } else {
        Write-Host "   ✅ Duração parece adequada" -ForegroundColor Green
    }
    
    # Salvar áudio
    $outputFile2 = Join-Path $PWD "test_medium_text_speed.wav"
    Save-AudioToWav -Samples $response2.audio -SampleRate $response2.sample_rate -Channels $response2.channels -OutputFile $outputFile2
    Write-Host "   💾 Áudio salvo em: $outputFile2" -ForegroundColor Cyan
} catch {
    Write-Host "   ❌ Erro: $($_.Exception.Message)" -ForegroundColor Red
}

# Função auxiliar para salvar áudio em WAV
function Save-AudioToWav {
    param(
        [float[]]$Samples,
        [int]$SampleRate,
        [int]$Channels,
        [string]$OutputFile
    )
    
    $bytesPerSample = 2
    $dataSize = $Samples.Count * $bytesPerSample
    $fileSize = 36 + $dataSize
    $byteRate = $SampleRate * $Channels * $bytesPerSample
    $blockAlign = $Channels * $bytesPerSample
    
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
    $writer.Write([uint16]$Channels)
    $writer.Write([uint32]$SampleRate)
    $writer.Write([uint32]$byteRate)
    $writer.Write([uint16]$blockAlign)
    $writer.Write([uint16]16)
    
    # data chunk
    $writer.Write([System.Text.Encoding]::ASCII.GetBytes("data"))
    $writer.Write([uint32]$dataSize)
    
    # Escrever samples
    foreach ($sample in $Samples) {
        $clamped = [Math]::Max(-1.0, [Math]::Min(1.0, [double]$sample))
        $int16Sample = [int16]([Math]::Round($clamped * 32767.0))
        $writer.Write($int16Sample)
    }
    
    $writer.Close()
    [System.IO.File]::WriteAllBytes($OutputFile, $ms.ToArray())
    $ms.Close()
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  DIAGNÓSTICO CONCLUÍDO" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`n📋 PRÓXIMOS PASSOS:" -ForegroundColor Yellow
Write-Host "1. Ouça os arquivos de áudio gerados" -ForegroundColor White
Write-Host "2. Verifique os logs do servidor para ver:" -ForegroundColor White
Write-Host "   - Quantos fonemas IPA foram gerados" -ForegroundColor White
Write-Host "   - Quantos foram mapeados vs pulados" -ForegroundColor White
Write-Host "   - A duração real do áudio gerado" -ForegroundColor White
Write-Host "3. Compare a duração esperada vs obtida" -ForegroundColor White



