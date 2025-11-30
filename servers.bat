@echo off
setlocal enabledelayedexpansion

REM VRPG Servers Launcher
REM Inicia Vectorizer e Synap em terminais separados

set VECTORIZER_PATH=G:\vrpg\vectorizer-feature-native-engine-optimization
set SYNAP_PATH=G:\vrpg\synap-main
set VECTORIZER_PORT=15002
set SYNAP_PORT=15500

REM Parse argumentos
if "%1"=="--help" goto :help
if "%1"=="-h" goto :help
if "%1"=="--stop" goto :stop
if "%1"=="--status" goto :status
if "%1"=="" goto :start

:help
echo.
echo VRPG Servers Launcher
echo =====================
echo.
echo Usage: servers.bat [OPTIONS]
echo.
echo Options:
echo   (no args)    Start both servers in separate terminals
echo   --help, -h   Show this help message
echo   --stop       Stop all running servers
echo   --status     Check server status
echo.
echo Servers:
echo   • Vectorizer: http://127.0.0.1:%VECTORIZER_PORT%
echo   • Synap:      http://127.0.0.1:%SYNAP_PORT%
echo.
pause
exit /b 0

:status
echo.
echo VRPG Servers Status
echo ===================
echo.
echo Checking Vectorizer on port %VECTORIZER_PORT%...
powershell -Command "try { $result = Test-NetConnection -ComputerName 127.0.0.1 -Port %VECTORIZER_PORT% -InformationLevel Quiet -WarningAction SilentlyContinue; if ($result) { Write-Host '✅ Vectorizer is running on port %VECTORIZER_PORT%' -ForegroundColor Green } else { Write-Host '❌ Vectorizer is not running on port %VECTORIZER_PORT%' -ForegroundColor Red } } catch { Write-Host '❌ Vectorizer is not running on port %VECTORIZER_PORT%' -ForegroundColor Red }"

echo Checking Synap on port %SYNAP_PORT%...
powershell -Command "try { $result = Test-NetConnection -ComputerName 127.0.0.1 -Port %SYNAP_PORT% -InformationLevel Quiet -WarningAction SilentlyContinue; if ($result) { Write-Host '✅ Synap is running on port %SYNAP_PORT%' -ForegroundColor Green } else { Write-Host '❌ Synap is not running on port %SYNAP_PORT%' -ForegroundColor Red } } catch { Write-Host '❌ Synap is not running on port %SYNAP_PORT%' -ForegroundColor Red }"
echo.
pause
exit /b 0

:stop
echo.
echo Stopping VRPG Servers...
echo.
powershell -Command "Get-Process | Where-Object { $_.ProcessName -like '*vectorizer*' -or $_.ProcessName -like '*synap*' -or ($_.ProcessName -eq 'cargo' -and $_.CommandLine -like '*vectorizer*') -or ($_.ProcessName -eq 'cargo' -and $_.CommandLine -like '*synap*') } | ForEach-Object { Write-Host 'Stopping process:' $_.ProcessName 'PID:' $_.Id; Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue }"
echo.
echo ✅ Server shutdown complete
echo.
pause
exit /b 0

:start
echo.
echo 🚀 Starting VRPG Servers...
echo ============================
echo.

REM Verificar se os servidores já estão rodando
echo Checking server status...
powershell -Command "$vectorizerRunning = try { Test-NetConnection -ComputerName 127.0.0.1 -Port %VECTORIZER_PORT% -InformationLevel Quiet -WarningAction SilentlyContinue } catch { $false }; $synapRunning = try { Test-NetConnection -ComputerName 127.0.0.1 -Port %SYNAP_PORT% -InformationLevel Quiet -WarningAction SilentlyContinue } catch { $false }; if ($vectorizerRunning -and $synapRunning) { Write-Host '✅ Both servers are already running!' -ForegroundColor Green; Write-Host '   • Vectorizer: http://127.0.0.1:%VECTORIZER_PORT%' -ForegroundColor White; Write-Host '   • Synap: http://127.0.0.1:%SYNAP_PORT%' -ForegroundColor White; exit 2 } elseif ($vectorizerRunning) { Write-Host '⚠️  Vectorizer is already running, will start Synap only' -ForegroundColor Yellow; exit 3 } elseif ($synapRunning) { Write-Host '⚠️  Synap is already running, will start Vectorizer only' -ForegroundColor Yellow; exit 4 } else { Write-Host '🚀 No servers running, starting both...' -ForegroundColor Green; exit 0 }"

set SERVER_STATUS=%errorlevel%

echo.
echo Starting required servers in separate terminals...
echo.

REM Iniciar Synap se não estiver rodando (apenas status 0)
if %SERVER_STATUS% equ 0 (
    echo 🔄 Starting Synap server...
    start "VRPG-Synap-Server" powershell -NoExit -Command "Write-Host 'VRPG Synap Server' -ForegroundColor Cyan; Write-Host '==================' -ForegroundColor Cyan; Write-Host ''; Set-Location '%SYNAP_PATH%'; cargo run --release --bin synap-server"
    echo ✅ Synap terminal started
    timeout /t 2 /nobreak >nul
)

REM Iniciar Vectorizer se não estiver rodando (status 0 ou 4)
if %SERVER_STATUS% equ 0 (
    echo 🔄 Starting Vectorizer server...
    start "VRPG-Vectorizer-Server" powershell -NoExit -Command "Write-Host 'VRPG Vectorizer Server' -ForegroundColor Cyan; Write-Host '=======================' -ForegroundColor Cyan; Write-Host ''; Write-Host 'Note: First compilation may take several minutes' -ForegroundColor Yellow; Write-Host 'Setting up environment...' -ForegroundColor Gray; $env:PATH += ';C:\Program Files\CMake\bin;C:\nasm\nasm-2.16.03'; $env:CMAKE = 'cmake'; $env:NASM = 'nasm'; Write-Host ''; Set-Location '%VECTORIZER_PATH%'; cargo run --release --bin vectorizer --no-default-features"
    echo ✅ Vectorizer terminal started
)
if %SERVER_STATUS% equ 4 (
    echo 🔄 Starting Vectorizer server...
    start "VRPG-Vectorizer-Server" powershell -NoExit -Command "Write-Host 'VRPG Vectorizer Server' -ForegroundColor Cyan; Write-Host '=======================' -ForegroundColor Cyan; Write-Host ''; Write-Host 'Note: First compilation may take several minutes' -ForegroundColor Yellow; Write-Host 'Setting up environment...' -ForegroundColor Gray; $env:PATH += ';C:\Program Files\CMake\bin;C:\nasm\nasm-2.16.03'; $env:CMAKE = 'cmake'; $env:NASM = 'nasm'; Write-Host ''; Set-Location '%VECTORIZER_PATH%'; cargo run --release --bin vectorizer --no-default-features"
    echo ✅ Vectorizer terminal started
)

REM Se ambos já estão rodando (status 2)
if %SERVER_STATUS% equ 2 (
    echo.
    echo 🎉 All servers are already running!
    echo =============================
    echo.
    echo 📡 Available endpoints:
    echo   • Vectorizer: http://127.0.0.1:%VECTORIZER_PORT%
    echo   • Synap:      http://127.0.0.1:%SYNAP_PORT%
    echo.
    echo Press any key to continue...
    pause >nul
    exit /b 0
)

echo.
echo 🎉 Server startup initiated!
echo =============================
echo.
echo 📡 Expected endpoints:
echo   • Vectorizer: http://127.0.0.1:%VECTORIZER_PORT%
echo   • Synap:      http://127.0.0.1:%SYNAP_PORT%
echo.
echo ⏱️  Please wait for compilation to complete...
echo 💡 Use 'servers.bat --status' to check when servers are ready
echo 🛑 Use 'servers.bat --stop' to stop all servers
echo.
echo Press any key to continue...
pause >nul
exit /b 0
