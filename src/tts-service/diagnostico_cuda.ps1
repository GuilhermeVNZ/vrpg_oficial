# Script para diagnosticar problemas com CUDA e ONNX Runtime
# Captura logs do servidor TTS e verifica se CUDA está funcionando

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  DIAGNÓSTICO CUDA/ONNX RUNTIME" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

# 1. Verificar se os DLLs do cuDNN estão no local correto
Write-Host "`n1. Verificando DLLs do cuDNN..." -ForegroundColor Cyan
$releasePath = "G:\vrpg\vrpg-client\target\release"
$cudnnDlls = @(
    "cudnn64_9.dll",
    "cudnn_adv64_9.dll",
    "cudnn_cnn64_9.dll",
    "cudnn_engines_precompiled64_9.dll",
    "cudnn_engines_runtime_compiled64_9.dll",
    "cudnn_graph64_9.dll",
    "cudnn_heuristic64_9.dll",
    "cudnn_ops64_9.dll"
)

$allPresent = $true
foreach ($dll in $cudnnDlls) {
    $dllPath = Join-Path $releasePath $dll
    if (Test-Path $dllPath) {
        $sizeMB = (Get-Item $dllPath).Length / 1MB
        $sizeStr = [math]::Round($sizeMB, 2).ToString()
        Write-Host "   OK $dll - $sizeStr MB" -ForegroundColor Green
    } else {
        Write-Host "   FALHOU $dll - NAO ENCONTRADO" -ForegroundColor Red
        $allPresent = $false
    }
}

if ($allPresent) {
    Write-Host "`nOK Todos os DLLs do cuDNN estao presentes!" -ForegroundColor Green
} else {
    Write-Host "`nFALHOU Alguns DLLs do cuDNN estao faltando!" -ForegroundColor Red
    Write-Host "   Execute: .\instalar_cudnn_automatico.ps1" -ForegroundColor Yellow
}

# 2. Verificar se o servidor está rodando
Write-Host "`n2. Verificando servidor TTS..." -ForegroundColor Cyan
try {
    $health = Invoke-RestMethod -Uri "http://localhost:3002/health" -Method Get -TimeoutSec 5 -ErrorAction Stop
    Write-Host "   OK Servidor esta respondendo" -ForegroundColor Green
} catch {
    Write-Host "   FALHOU Servidor nao esta respondendo: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "   Inicie o servidor primeiro!" -ForegroundColor Yellow
    exit 1
}

# 3. Teste rápido de síntese
Write-Host "`n3. Testando sintese (timeout 5s)..." -ForegroundColor Cyan
$testText = "Hello"
$body = @{
    text = '<VOICE actor="piper_only_test" emotion="neutral" style="narrative">' + $testText + '</VOICE>'
    language = "en"
} | ConvertTo-Json

try {
    $startTime = Get-Date
    $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 5 -ErrorAction Stop
    $endTime = Get-Date
    $duration = ($endTime - $startTime).TotalMilliseconds
    
    $durationStr = [math]::Round($duration, 0).ToString()
    Write-Host "   OK Sintese concluida em $durationStr ms" -ForegroundColor Green
    Write-Host "   Duracao do audio: $($response.duration_ms) ms" -ForegroundColor White
    Write-Host "   Amostras: $($response.audio.Count)" -ForegroundColor White
} catch {
    Write-Host "   FALHOU Sintese falhou ou timeout: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "`nPOSSIVEIS CAUSAS:" -ForegroundColor Yellow
    Write-Host "   1. Modelo ONNX esta sendo carregado pela primeira vez (pode demorar 30-60s)" -ForegroundColor White
    Write-Host "   2. Problema com CUDA/GPU (verifique os logs do servidor)" -ForegroundColor White
    Write-Host "   3. Deadlock no codigo (verifique os logs do servidor)" -ForegroundColor White
    Write-Host "`nSOLUCAO:" -ForegroundColor Cyan
    Write-Host "   - Verifique a janela do servidor TTS para ver os logs detalhados" -ForegroundColor White
    Write-Host "   - Procure por mensagens como:" -ForegroundColor White
    Write-Host "     * 'cuDNN version: 91600' (CUDA funcionando)" -ForegroundColor Green
    Write-Host "     * 'Successfully registered CUDAExecutionProvider' (CUDA funcionando)" -ForegroundColor Green
    Write-Host "     * 'Adding default CPU execution provider' (CUDA falhou, usando CPU)" -ForegroundColor Red
    Write-Host "     * 'Failed to load ONNX model' (erro no carregamento)" -ForegroundColor Red
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  DIAGNÓSTICO CONCLUÍDO" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

