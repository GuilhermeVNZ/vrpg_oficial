# Script para testar o mapeamento de fonemas
# Compara o que estamos gerando com o que deveria ser gerado

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  TESTE DE MAPEAMENTO DE FONEMAS" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

# Teste 1: Verificar o que espeak-ng gera
Write-Host "1. Testando espeak-ng diretamente..." -ForegroundColor Cyan
$testText = "Hello world"
$espeakCmd = "C:\Program Files\eSpeak NG\espeak-ng.exe"
if (-not (Test-Path $espeakCmd)) {
    $espeakCmd = "espeak-ng"
}

try {
    $output = & $espeakCmd -q --ipa -v en-us $testText 2>&1
    Write-Host "   Texto: '$testText'" -ForegroundColor White
    Write-Host "   IPA do espeak-ng: '$output'" -ForegroundColor Yellow
} catch {
    Write-Host "   ERRO ao executar espeak-ng: $_" -ForegroundColor Red
}

# Teste 2: Verificar o que o servidor está gerando
Write-Host ""
Write-Host "2. Testando servidor TTS..." -ForegroundColor Cyan
$body = @{
    text = '<VOICE actor="piper_only_test" emotion="neutral" style="narrative">' + $testText + '</VOICE>'
    language = "en"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 30 -ErrorAction Stop
    Write-Host "   SUCESSO - Audio gerado" -ForegroundColor Green
    Write-Host "   Duracao: $($response.duration_ms) ms" -ForegroundColor White
    Write-Host "   Amostras: $($response.audio.Count)" -ForegroundColor White
    
    # Salvar audio para analise
    $outputFile = Join-Path $PWD "test_phoneme_mapping.wav"
    $sampleRate = $response.sample_rate
    $channels = $response.channels
    $samples = $response.audio
    
    $dataSize = $samples.Count * 2
    $fileSize = 36 + $dataSize
    $byteRate = $sampleRate * $channels * 2
    $blockAlign = $channels * 2
    
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
    
    Write-Host "   Audio salvo em: $outputFile" -ForegroundColor Green
    Start-Process $outputFile
} catch {
    Write-Host "   ERRO: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  VERIFIQUE OS LOGS DO SERVIDOR" -ForegroundColor Yellow
Write-Host "  Procure por:" -ForegroundColor Yellow
Write-Host "    - IPA phonemes gerados" -ForegroundColor White
Write-Host "    - Phoneme IDs mapeados" -ForegroundColor White
Write-Host "    - Primeiros 50 IDs" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Green



