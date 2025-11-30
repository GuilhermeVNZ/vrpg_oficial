# Script para executar teste Hello World → XTTS → SoVITS (1000 steps)
# Pode ser executado de qualquer diretório

$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$vrpgRoot = Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $scriptPath)))
$sovitsDir = Join-Path $vrpgRoot "assets-and-models\models\tts\sovits"
$pythonScript = Join-Path $scriptPath "test_hello_world_pipeline_1000.py"
$venvPython = Join-Path $sovitsDir "venv310\Scripts\python.exe"

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  TESTE: XTTS → SoVITS (1000 steps)" -ForegroundColor Green
Write-Host "  Texto: 'Hello World'" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

# Verificar se o Python do venv existe
if (-not (Test-Path $venvPython)) {
    Write-Host "[ERRO] Python do ambiente virtual nao encontrado!" -ForegroundColor Red
    Write-Host "   Caminho esperado: $venvPython" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "   Certifique-se de que:" -ForegroundColor Yellow
    Write-Host "   1. O ambiente virtual existe em: $sovitsDir\venv310" -ForegroundColor Gray
    Write-Host "   2. Voce esta no diretorio correto do projeto" -ForegroundColor Gray
    exit 1
}

# Verificar se o script Python existe
if (-not (Test-Path $pythonScript)) {
    Write-Host "[ERRO] Script Python nao encontrado!" -ForegroundColor Red
    Write-Host "   Caminho esperado: $pythonScript" -ForegroundColor Yellow
    exit 1
}

# Verificar se o modelo G_1000.pth existe
$modelPath = Join-Path $sovitsDir "logs\44k\G_1000.pth"
if (-not (Test-Path $modelPath)) {
    Write-Host "[ERRO] Modelo G_1000.pth nao encontrado!" -ForegroundColor Red
    Write-Host "   Caminho esperado: $modelPath" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "   Arquivos .pth encontrados em logs/44k/:" -ForegroundColor Yellow
    $logsDir = Join-Path $sovitsDir "logs\44k"
    if (Test-Path $logsDir) {
        Get-ChildItem -Path $logsDir -Filter "G_*.pth" | ForEach-Object {
            Write-Host "     - $($_.Name)" -ForegroundColor Gray
        }
    }
    exit 1
}

Write-Host "[OK] Python encontrado: $venvPython" -ForegroundColor Green
Write-Host "[OK] Script encontrado: $pythonScript" -ForegroundColor Green
Write-Host "[OK] Modelo encontrado: G_1000.pth" -ForegroundColor Green
Write-Host ""
Write-Host "[INFO] Executando pipeline completa..." -ForegroundColor Cyan
Write-Host "   (Isso pode levar alguns minutos na primeira vez)" -ForegroundColor Gray
Write-Host ""

# Executar o script
try {
    & $venvPython $pythonScript
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "[SUCESSO] TESTE CONCLUIDO COM SUCESSO!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Arquivos gerados:" -ForegroundColor Cyan
        Write-Host "   - test_hello_world_xtts_output.wav (audio XTTS)" -ForegroundColor White
        Write-Host "   - test_hello_world_sovits_1000_output.wav (audio final SoVITS)" -ForegroundColor White
        Write-Host ""
        Write-Host "Ouça o resultado final em:" -ForegroundColor Cyan
        $outputPath = Join-Path $scriptPath "test_hello_world_sovits_1000_output.wav"
        Write-Host "   $outputPath" -ForegroundColor White
    } else {
        Write-Host ""
        Write-Host "[ERRO] Teste falhou com codigo de saida: $LASTEXITCODE" -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host ""
    Write-Host "[ERRO] Erro ao executar script: $_" -ForegroundColor Red
    exit 1
}



