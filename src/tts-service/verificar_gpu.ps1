# Script para verificar se o TTS está usando GPU

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  VERIFICACAO DE GPU/CUDA" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

# 1. Verificar cuDNN
Write-Host "1. Verificando cuDNN..." -ForegroundColor Cyan
$releasePath = "G:\vrpg\vrpg-client\target\release"
$cudnnDll = Join-Path $releasePath "cudnn64_9.dll"
if (Test-Path $cudnnDll) {
    $sizeMB = (Get-Item $cudnnDll).Length / 1MB
    Write-Host "   OK cuDNN encontrado: $cudnnDll" -ForegroundColor Green
    Write-Host "   Tamanho: $([math]::Round($sizeMB, 2)) MB" -ForegroundColor White
} else {
    Write-Host "   FALHOU cuDNN nao encontrado em: $releasePath" -ForegroundColor Red
    Write-Host "   Execute: .\instalar_cudnn_automatico.ps1" -ForegroundColor Yellow
}

# 2. Verificar se nvidia-smi está disponível
Write-Host ""
Write-Host "2. Verificando nvidia-smi..." -ForegroundColor Cyan
try {
    $nvidiaSmi = nvidia-smi --query-gpu=name,memory.total,memory.used,utilization.gpu --format=csv,noheader 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   OK nvidia-smi disponivel" -ForegroundColor Green
        Write-Host "   GPU Info:" -ForegroundColor White
        $nvidiaSmi | ForEach-Object { Write-Host "     $_" -ForegroundColor White }
    } else {
        Write-Host "   AVISO nvidia-smi nao retornou dados validos" -ForegroundColor Yellow
    }
} catch {
    Write-Host "   AVISO nvidia-smi nao encontrado (normal se nao tiver NVIDIA GPU)" -ForegroundColor Yellow
}

# 3. Verificar se o servidor está rodando
Write-Host ""
Write-Host "3. Verificando servidor TTS..." -ForegroundColor Cyan
try {
    $health = Invoke-RestMethod -Uri "http://localhost:3002/health" -Method Get -TimeoutSec 2 -ErrorAction Stop
    Write-Host "   OK Servidor esta respondendo" -ForegroundColor Green
} catch {
    Write-Host "   FALHOU Servidor nao esta respondendo" -ForegroundColor Red
    Write-Host "   Inicie o servidor primeiro!" -ForegroundColor Yellow
    exit 1
}

# 4. Teste rápido de síntese e medir tempo
Write-Host ""
Write-Host "4. Testando sintese (medindo tempo de inferencia)..." -ForegroundColor Cyan
$testText = "Hello world"
$body = @{
    text = '<VOICE actor="piper_only_test" emotion="neutral" style="narrative">' + $testText + '</VOICE>'
    language = "en"
} | ConvertTo-Json

try {
    $startTime = Get-Date
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 30 -ErrorAction Stop
    $endTime = Get-Date
    $totalTime = ($endTime - $startTime).TotalMilliseconds
    
    Write-Host "   OK Sintese concluida" -ForegroundColor Green
    Write-Host "   Tempo total: $([math]::Round($totalTime, 0)) ms" -ForegroundColor White
    Write-Host "   Duracao do audio: $($response.duration_ms) ms" -ForegroundColor White
    
    # Diagnosticar baseado no tempo
    # GPU inference geralmente < 200ms, CPU > 500ms
    if ($totalTime -lt 200) {
        Write-Host "   DIAGNOSTICO: Tempo muito rapido - PROVAVELMENTE USANDO GPU" -ForegroundColor Green
    } elseif ($totalTime -lt 500) {
        Write-Host "   DIAGNOSTICO: Tempo medio - VERIFICAR LOGS DO SERVIDOR" -ForegroundColor Yellow
    } else {
        Write-Host "   DIAGNOSTICO: Tempo lento - PROVAVELMENTE USANDO CPU" -ForegroundColor Red
    }
} catch {
    Write-Host "   FALHOU Sintese falhou: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  VERIFICACAO CONCLUIDA" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Para verificar uso de GPU em tempo real:" -ForegroundColor Cyan
Write-Host "  nvidia-smi -l 1" -ForegroundColor White
Write-Host ""
Write-Host "Verifique os logs do servidor TTS para:" -ForegroundColor Cyan
Write-Host "  - 'Successfully registered CUDAExecutionProvider' (CUDA OK)" -ForegroundColor Green
Write-Host "  - 'Adding default CPU execution provider' (CUDA falhou)" -ForegroundColor Red
Write-Host "  - Tempo de inferencia ONNX (GPU < 200ms, CPU > 500ms)" -ForegroundColor Yellow



