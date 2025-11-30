# Diagnóstico completo do pipeline TTS
# Testa cada etapa: Piper recebendo texto -> Piper gerando áudio -> SoVITS recebendo -> SoVITS convertendo

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  DIAGNÓSTICO COMPLETO DO PIPELINE TTS" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

$testText = "Hello world. This is a test."

# Função para salvar áudio
function Save-AudioToWav {
    param([array]$samples, [int]$sampleRate, [int]$channels, [string]$outputFile)
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
}

# Teste 1: Verificar servidor
Write-Host "`n[TESTE 1] Verificando servidor TTS..." -ForegroundColor Cyan
try {
    $health = Invoke-RestMethod -Uri "http://localhost:3002/health" -Method Get -TimeoutSec 2
    Write-Host "✅ Servidor respondendo" -ForegroundColor Green
    Write-Host "   Status: $($health.status)" -ForegroundColor White
    Write-Host "   Modelos carregados: $($health.model_loaded)" -ForegroundColor White
    Write-Host "   Vozes disponíveis: $($health.voices -join ', ')" -ForegroundColor White
} catch {
    Write-Host "❌ Servidor não está respondendo" -ForegroundColor Red
    Write-Host "   Inicie com: cargo run --bin tts-server" -ForegroundColor Yellow
    exit 1
}

# Teste 2: Testar apenas Piper (actor que não existe = sem SoVITS)
Write-Host "`n[TESTE 2] Testando Piper isoladamente (sem SoVITS)..." -ForegroundColor Cyan
Write-Host "   Texto: '$testText'" -ForegroundColor Gray
$piperOnlyText = '<VOICE actor="test_piper_only" emotion="neutral" style="neutral">' + $testText + '</VOICE>'
$piperOnlyBody = @{
    text = $piperOnlyText
    language = "en"
} | ConvertTo-Json

try {
    $startTime = Get-Date
    $piperResponse = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $piperOnlyBody -ContentType "application/json" -TimeoutSec 30
    $duration = ((Get-Date) - $startTime).TotalMilliseconds
    
    Write-Host "✅ Piper respondeu em $([math]::Round($duration, 0)) ms" -ForegroundColor Green
    $durationSec = [math]::Round($piperResponse.duration_ms/1000, 2)
    Write-Host "   Duração do áudio: $($piperResponse.duration_ms) ms" -ForegroundColor White
    Write-Host "   Duração em segundos: $durationSec" -ForegroundColor White
    Write-Host "   Amostras: $($piperResponse.audio.Count)" -ForegroundColor White
    Write-Host "   Sample rate: $($piperResponse.sample_rate) Hz" -ForegroundColor White
    
    $piperFile = "diagnose_piper_only.wav"
    Save-AudioToWav -samples $piperResponse.audio -sampleRate $piperResponse.sample_rate -channels $piperResponse.channels -outputFile $piperFile
    Write-Host "   ✅ Áudio salvo: $piperFile" -ForegroundColor Green
    Write-Host "   📝 Ouça este arquivo para verificar se Piper está gerando áudio correto" -ForegroundColor Yellow
} catch {
    Write-Host "❌ Erro ao testar Piper: $($_.Exception.Message)" -ForegroundColor Red
    if ($_.Exception.Response) {
        try {
            $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
            $responseBody = $reader.ReadToEnd()
            Write-Host "   Resposta: $responseBody" -ForegroundColor Yellow
        } catch {}
    }
}

# Teste 3: Testar com dungeon_master_en (deve usar SoVITS)
Write-Host "`n[TESTE 3] Testando pipeline completo (Piper + SoVITS)..." -ForegroundColor Cyan
Write-Host "   Texto: '$testText'" -ForegroundColor Gray
Write-Host "   Actor: dungeon_master_en (deve usar SoVITS)" -ForegroundColor Gray
$fullPipelineText = '<VOICE actor="dungeon_master_en" emotion="neutral" style="narrative">' + $testText + '</VOICE>'
$fullPipelineBody = @{
    text = $fullPipelineText
    language = "en"
} | ConvertTo-Json

try {
    $startTime = Get-Date
    $fullResponse = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $fullPipelineBody -ContentType "application/json" -TimeoutSec 60
    $duration = ((Get-Date) - $startTime).TotalMilliseconds
    
    Write-Host "✅ Pipeline completo respondeu em $([math]::Round($duration, 0)) ms" -ForegroundColor Green
    $durationSec2 = [math]::Round($fullResponse.duration_ms/1000, 2)
    Write-Host "   Duração do áudio: $($fullResponse.duration_ms) ms" -ForegroundColor White
    Write-Host "   Duração em segundos: $durationSec2" -ForegroundColor White
    Write-Host "   Amostras: $($fullResponse.audio.Count)" -ForegroundColor White
    Write-Host "   Sample rate: $($fullResponse.sample_rate) Hz" -ForegroundColor White
    Write-Host "   Actor: $($fullResponse.actor)" -ForegroundColor White
    
    $fullFile = "diagnose_full_pipeline.wav"
    Save-AudioToWav -samples $fullResponse.audio -sampleRate $fullResponse.sample_rate -channels $fullResponse.channels -outputFile $fullFile
    Write-Host "   ✅ Áudio salvo: $fullFile" -ForegroundColor Green
    Write-Host "   📝 Compare com diagnose_piper_only.wav para verificar se SoVITS está convertendo" -ForegroundColor Yellow
} catch {
    Write-Host "❌ Erro ao testar pipeline completo: $($_.Exception.Message)" -ForegroundColor Red
    if ($_.Exception.Response) {
        try {
            $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
            $responseBody = $reader.ReadToEnd()
            Write-Host "   Resposta: $responseBody" -ForegroundColor Yellow
        } catch {}
    }
}

# Teste 4: Verificar métricas
Write-Host "`n[TESTE 4] Verificando métricas do servidor..." -ForegroundColor Cyan
try {
    $metrics = Invoke-RestMethod -Uri "http://localhost:3002/metrics" -Method Get -TimeoutSec 2
    Write-Host "✅ Métricas obtidas" -ForegroundColor Green
    Write-Host "   Total de requisições: $($metrics.total_requests)" -ForegroundColor White
    Write-Host "   Requisições bem-sucedidas: $($metrics.successful_requests)" -ForegroundColor White
    Write-Host "   Requisições com erro: $($metrics.error_requests)" -ForegroundColor White
    if ($metrics.average_latency_ms) {
        Write-Host "   Latência média: $([math]::Round($metrics.average_latency_ms, 0)) ms" -ForegroundColor White
    }
} catch {
    Write-Host "⚠️  Não foi possível obter métricas" -ForegroundColor Yellow
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  DIAGNÓSTICO CONCLUÍDO" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nArquivos gerados:" -ForegroundColor Cyan
Write-Host "  📁 diagnose_piper_only.wav" -ForegroundColor White
Write-Host "     → Áudio apenas do Piper (sem SoVITS)" -ForegroundColor Gray
Write-Host "     → Use para verificar se Piper está funcionando" -ForegroundColor Gray
Write-Host "`n  📁 diagnose_full_pipeline.wav" -ForegroundColor White
Write-Host "     → Áudio completo (Piper + SoVITS)" -ForegroundColor Gray
Write-Host "     → Compare com o anterior para verificar se SoVITS está convertendo" -ForegroundColor Gray
Write-Host "`n📋 PRÓXIMOS PASSOS:" -ForegroundColor Yellow
Write-Host "  1. Ouça diagnose_piper_only.wav" -ForegroundColor White
Write-Host "     → Se estiver ruim: problema no Piper/phonemização" -ForegroundColor Gray
Write-Host "     → Se estiver bom: problema no SoVITS" -ForegroundColor Gray
Write-Host "`n  2. Compare os dois arquivos" -ForegroundColor White
Write-Host "     → Se forem idênticos: SoVITS não está convertendo" -ForegroundColor Gray
Write-Host "     → Se forem diferentes: SoVITS está funcionando" -ForegroundColor Gray
Write-Host "`n  3. Verifique os logs do servidor para mais detalhes" -ForegroundColor White

