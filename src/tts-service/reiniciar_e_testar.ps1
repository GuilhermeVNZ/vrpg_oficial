# Script para reiniciar o servidor TTS e executar teste de diagnostico

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  REINICIANDO SERVIDOR TTS" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

# 1. Parar servidor existente (se estiver rodando)
Write-Host "`n1. Parando servidor existente..." -ForegroundColor Cyan
$processes = Get-Process -Name "tts-server" -ErrorAction SilentlyContinue
if ($processes) {
    foreach ($proc in $processes) {
        Write-Host "   Parando processo PID: $($proc.Id)" -ForegroundColor Yellow
        Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
    }
    Start-Sleep -Seconds 2
    Write-Host "   OK Servidor parado" -ForegroundColor Green
} else {
    Write-Host "   Info: Nenhum servidor rodando" -ForegroundColor Gray
}

# 2. Verificar se a porta 3002 esta livre
Write-Host "`n2. Verificando porta 3002..." -ForegroundColor Cyan
$portInUse = Get-NetTCPConnection -LocalPort 3002 -ErrorAction SilentlyContinue
if ($portInUse) {
    Write-Host "   AVISO: Porta 3002 ainda em uso, tentando liberar..." -ForegroundColor Yellow
    $portProcess = Get-Process -Id $portInUse.OwningProcess -ErrorAction SilentlyContinue
    if ($portProcess) {
        Stop-Process -Id $portProcess.Id -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 2
    }
} else {
    Write-Host "   OK Porta 3002 livre" -ForegroundColor Green
}

# 3. Iniciar servidor
Write-Host "`n3. Iniciando servidor TTS..." -ForegroundColor Cyan
$serverPath = "G:\vrpg\vrpg-client\target\release\tts-server.exe"
if (-not (Test-Path $serverPath)) {
    Write-Host "   ERRO: Servidor nao encontrado em: $serverPath" -ForegroundColor Red
    Write-Host "   Execute: cargo build --release --bin tts-server" -ForegroundColor Yellow
    exit 1
}

Write-Host "   Iniciando: $serverPath" -ForegroundColor White
$serverProcess = Start-Process -FilePath $serverPath -WorkingDirectory (Split-Path $serverPath) -WindowStyle Minimized -PassThru
Write-Host "   OK Servidor iniciado (PID: $($serverProcess.Id))" -ForegroundColor Green

# 4. Aguardar servidor ficar pronto
Write-Host "`n4. Aguardando servidor ficar pronto..." -ForegroundColor Cyan
$maxWait = 30
$waited = 0
$ready = $false

while ($waited -lt $maxWait -and -not $ready) {
    Start-Sleep -Seconds 1
    $waited++
    try {
        $health = Invoke-RestMethod -Uri "http://localhost:3002/health" -Method Get -TimeoutSec 2 -ErrorAction Stop
        $ready = $true
        Write-Host "   OK Servidor pronto apos $waited segundos" -ForegroundColor Green
    } catch {
        Write-Host "   Aguardando... ($waited/$maxWait)" -ForegroundColor Gray
    }
}

if (-not $ready) {
    Write-Host "   ERRO: Servidor nao respondeu apos $maxWait segundos" -ForegroundColor Red
    Write-Host "   Verifique os logs do servidor manualmente" -ForegroundColor Yellow
    exit 1
}

# 5. Executar teste de diagnostico
Write-Host "`n5. Executando teste de diagnostico..." -ForegroundColor Cyan
Write-Host "   (Isso pode demorar alguns segundos)" -ForegroundColor Gray

$testScript = Join-Path $PSScriptRoot "test_audio_speed_diagnosis.ps1"
if (Test-Path $testScript) {
    & $testScript
} else {
    Write-Host "   AVISO: Script de teste nao encontrado: $testScript" -ForegroundColor Yellow
    Write-Host "   Execute manualmente: .\test_audio_speed_diagnosis.ps1" -ForegroundColor White
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  PROCESSO CONCLUIDO" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nPROXIMOS PASSOS:" -ForegroundColor Yellow
Write-Host "1. Verifique os logs do servidor para ver:" -ForegroundColor White
Write-Host "   - Quantos fonemas foram mapeados vs pulados" -ForegroundColor White
Write-Host "   - A duracao real do audio gerado" -ForegroundColor White
Write-Host "   - Se ha avisos sobre muitos fonemas sendo pulados" -ForegroundColor White
Write-Host "2. Ouça os arquivos de audio gerados" -ForegroundColor White
Write-Host "3. Compare a duracao esperada vs obtida" -ForegroundColor White
