# Script para reiniciar o servidor TTS e testar a velocidade corrigida

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  REINICIANDO SERVIDOR TTS E TESTANDO" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

# 1. Parar servidor TTS se estiver rodando
Write-Host "`n1. Parando servidor TTS (porta 3002)..." -ForegroundColor Cyan
$processes = Get-NetTCPConnection -LocalPort 3002 -ErrorAction SilentlyContinue | Select-Object -ExpandProperty OwningProcess -Unique
if ($processes) {
    foreach ($pid in $processes) {
        $proc = Get-Process -Id $pid -ErrorAction SilentlyContinue
        if ($proc) {
            Write-Host "   Parando processo: $($proc.Name) (PID: $pid)" -ForegroundColor Yellow
            Stop-Process -Id $pid -Force -ErrorAction SilentlyContinue
            Start-Sleep -Seconds 2
        }
    }
    Write-Host "   OK Servidor parado" -ForegroundColor Green
} else {
    Write-Host "   Info: Nenhum servidor rodando na porta 3002" -ForegroundColor Gray
}

# 2. Iniciar servidor TTS
Write-Host "`n2. Iniciando servidor TTS..." -ForegroundColor Cyan
$serverExe = "G:\vrpg\vrpg-client\target\release\tts-server.exe"
if (-not (Test-Path $serverExe)) {
    Write-Host "   ERRO: Executavel nao encontrado: $serverExe" -ForegroundColor Red
    Write-Host "   Compile o projeto primeiro: cargo build --release --bin tts-server" -ForegroundColor Yellow
    exit 1
}

Write-Host "   Iniciando: $serverExe" -ForegroundColor White
$serverProcess = Start-Process -FilePath $serverExe -PassThru -WindowStyle Minimized
Write-Host "   OK Servidor iniciado (PID: $($serverProcess.Id))" -ForegroundColor Green

# 3. Aguardar servidor ficar pronto
Write-Host "`n3. Aguardando servidor ficar pronto..." -ForegroundColor Cyan
$maxAttempts = 30
$attempt = 0
$ready = $false

while ($attempt -lt $maxAttempts -and -not $ready) {
    Start-Sleep -Seconds 2
    $attempt++
    try {
        $health = Invoke-RestMethod -Uri "http://localhost:3002/health" -Method Get -TimeoutSec 2 -ErrorAction Stop
        $ready = $true
        Write-Host "   OK Servidor pronto (tentativa $attempt/$maxAttempts)" -ForegroundColor Green
    } catch {
        Write-Host "   Aguardando... (tentativa $attempt/$maxAttempts)" -ForegroundColor Gray
    }
}

if (-not $ready) {
    Write-Host "   ERRO: Servidor nao respondeu apos $maxAttempts tentativas" -ForegroundColor Red
    Write-Host "   Verifique os logs do servidor manualmente" -ForegroundColor Yellow
    exit 1
}

# 4. Executar teste de velocidade
Write-Host "`n4. Executando teste de velocidade..." -ForegroundColor Cyan
Start-Sleep -Seconds 1
& "$PSScriptRoot\test_velocidade_corrigida.ps1"

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE CONCLUÍDO" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "`nPROXIMOS PASSOS:" -ForegroundColor Yellow
Write-Host "1. Ouca o audio gerado (test_velocidade_corrigida.wav)" -ForegroundColor White
Write-Host "2. Verifique se a velocidade esta adequada" -ForegroundColor White
Write-Host "3. Se ainda estiver rapido, podemos aumentar o length_scale para 2.0" -ForegroundColor White
Write-Host "4. Se estiver lento demais, podemos diminuir para 1.2" -ForegroundColor White

