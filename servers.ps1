# VRPG Servers Launcher
# Inicia Vectorizer e Synap em terminais separados

param(
    [switch]$Help,
    [switch]$Stop,
    [switch]$Status
)

# Configurações
$VECTORIZER_PATH = "G:\vrpg\vectorizer-feature-native-engine-optimization"
$SYNAP_PATH = "G:\vrpg\synap-main"
$VECTORIZER_PORT = 15002
$SYNAP_PORT = 15500

function Show-Help {
    Write-Host "VRPG Servers Launcher" -ForegroundColor Cyan
    Write-Host "=====================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\servers.ps1 [OPTIONS]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Options:" -ForegroundColor Green
    Write-Host "  (no args)    Start both servers in separate terminals"
    Write-Host "  -Help        Show this help message"
    Write-Host "  -Stop        Stop all running servers"
    Write-Host "  -Status      Check server status"
    Write-Host ""
    Write-Host "Servers:" -ForegroundColor Green
    Write-Host "  • Vectorizer: http://127.0.0.1:$VECTORIZER_PORT"
    Write-Host "  • Synap:      http://127.0.0.1:$SYNAP_PORT"
    Write-Host ""
}

function Test-ServerStatus {
    param([int]$Port, [string]$Name)
    
    try {
        $connection = Test-NetConnection -ComputerName 127.0.0.1 -Port $Port -WarningAction SilentlyContinue
        if ($connection.TcpTestSucceeded) {
            Write-Host "✅ $Name is running on port $Port" -ForegroundColor Green
            return $true
        } else {
            Write-Host "❌ $Name is not running on port $Port" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "❌ $Name is not running on port $Port" -ForegroundColor Red
        return $false
    }
}

function Show-Status {
    Write-Host "VRPG Servers Status" -ForegroundColor Cyan
    Write-Host "===================" -ForegroundColor Cyan
    Write-Host ""
    
    $vectorizerRunning = Test-ServerStatus -Port $VECTORIZER_PORT -Name "Vectorizer"
    $synapRunning = Test-ServerStatus -Port $SYNAP_PORT -Name "Synap"
    
    Write-Host ""
    if ($vectorizerRunning -and $synapRunning) {
        Write-Host "🎉 All servers are running!" -ForegroundColor Green
    } elseif ($vectorizerRunning -or $synapRunning) {
        Write-Host "⚠️  Some servers are running" -ForegroundColor Yellow
    } else {
        Write-Host "🛑 No servers are running" -ForegroundColor Red
    }
}

function Stop-Servers {
    Write-Host "Stopping VRPG Servers..." -ForegroundColor Yellow
    Write-Host ""
    
    # Parar processos do Vectorizer
    $vectorizerProcesses = Get-Process | Where-Object { $_.ProcessName -like "*vectorizer*" -or $_.CommandLine -like "*vectorizer*" }
    if ($vectorizerProcesses) {
        Write-Host "Stopping Vectorizer processes..." -ForegroundColor Yellow
        $vectorizerProcesses | ForEach-Object { 
            Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
            Write-Host "  Stopped PID $($_.Id)" -ForegroundColor Gray
        }
    }
    
    # Parar processos do Synap
    $synapProcesses = Get-Process | Where-Object { $_.ProcessName -like "*synap*" -or $_.CommandLine -like "*synap*" }
    if ($synapProcesses) {
        Write-Host "Stopping Synap processes..." -ForegroundColor Yellow
        $synapProcesses | ForEach-Object { 
            Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
            Write-Host "  Stopped PID $($_.Id)" -ForegroundColor Gray
        }
    }
    
    # Parar processos do Cargo que podem estar compilando/executando
    $cargoProcesses = Get-Process | Where-Object { $_.ProcessName -eq "cargo" }
    if ($cargoProcesses) {
        Write-Host "Stopping Cargo processes..." -ForegroundColor Yellow
        $cargoProcesses | ForEach-Object { 
            Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
            Write-Host "  Stopped PID $($_.Id)" -ForegroundColor Gray
        }
    }
    
    Write-Host ""
    Write-Host "✅ Server shutdown complete" -ForegroundColor Green
}

function Start-Servers {
    Write-Host "🚀 Starting VRPG Servers..." -ForegroundColor Cyan
    Write-Host "============================" -ForegroundColor Cyan
    Write-Host ""
    
    # Verificar se os servidores já estão rodando
    $vectorizerRunning = Test-ServerStatus -Port $VECTORIZER_PORT -Name "Vectorizer"
    $synapRunning = Test-ServerStatus -Port $SYNAP_PORT -Name "Synap"
    
    if ($vectorizerRunning -and $synapRunning) {
        Write-Host ""
        Write-Host "⚠️  Both servers are already running!" -ForegroundColor Yellow
        Write-Host "Use -Stop to stop them first, or -Status to check status." -ForegroundColor Yellow
        return
    }
    
    Write-Host ""
    Write-Host "Starting servers in separate terminals..." -ForegroundColor Green
    Write-Host ""
    
    # Iniciar Synap (mais rápido de compilar)
    if (-not $synapRunning) {
        Write-Host "🔄 Starting Synap server..." -ForegroundColor Yellow
        $synapCommand = "Set-Location '$SYNAP_PATH'; Write-Host 'Starting Synap Server...' -ForegroundColor Cyan; cargo run --release --bin synap-server"
        Start-Process powershell -ArgumentList "-NoExit", "-Command", $synapCommand -WindowStyle Normal
        Write-Host "✅ Synap terminal started" -ForegroundColor Green
        Start-Sleep -Seconds 2
    } else {
        Write-Host "✅ Synap is already running" -ForegroundColor Green
    }
    
    # Iniciar Vectorizer
    if (-not $vectorizerRunning) {
        Write-Host "🔄 Starting Vectorizer server..." -ForegroundColor Yellow
        $vectorizerCommand = "Set-Location '$VECTORIZER_PATH'; Write-Host 'Starting Vectorizer Server...' -ForegroundColor Cyan; Write-Host 'Note: First compilation may take several minutes' -ForegroundColor Yellow; cargo run --release --bin vectorizer --no-default-features"
        Start-Process powershell -ArgumentList "-NoExit", "-Command", $vectorizerCommand -WindowStyle Normal
        Write-Host "✅ Vectorizer terminal started" -ForegroundColor Green
    } else {
        Write-Host "✅ Vectorizer is already running" -ForegroundColor Green
    }
    
    Write-Host ""
    Write-Host "🎉 Server startup initiated!" -ForegroundColor Green
    Write-Host "=============================" -ForegroundColor Green
    Write-Host ""
    Write-Host "📡 Expected endpoints:" -ForegroundColor Cyan
    Write-Host "  • Vectorizer: http://127.0.0.1:$VECTORIZER_PORT" -ForegroundColor White
    Write-Host "  • Synap:      http://127.0.0.1:$SYNAP_PORT" -ForegroundColor White
    Write-Host ""
    Write-Host "⏱️  Please wait for compilation to complete..." -ForegroundColor Yellow
    Write-Host "💡 Use 'servers.exe -Status' to check when servers are ready" -ForegroundColor Cyan
    Write-Host "🛑 Use 'servers.exe -Stop' to stop all servers" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Press any key to continue..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}

# Main execution
if ($Help) {
    Show-Help
} elseif ($Stop) {
    Stop-Servers
} elseif ($Status) {
    Show-Status
} else {
    Start-Servers
}
