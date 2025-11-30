# Script PowerShell para testar pipeline completa: XTTS → SoVITS (1000 steps)
# Gera áudio "Hello World" com XTTS e converte usando modelo SoVITS treinado com 1000 steps

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE: XTTS → SoVITS (1000 steps)" -ForegroundColor Green
Write-Host "  Texto: 'Hello World'" -ForegroundColor Green
Write-Host "========================================`n" -ForegroundColor Green

# Encontrar diretórios
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$testsDir = Split-Path -Parent $scriptDir
$ttsServiceDir = Split-Path -Parent $testsDir
$vrpgClientDir = Split-Path -Parent (Split-Path -Parent $ttsServiceDir)
$sovitsDir = Join-Path $vrpgClientDir "assets-and-models\models\tts\sovits"

# Verificar se o diretório do SoVITS existe
if (-not (Test-Path $sovitsDir)) {
    Write-Host "[ERRO] Diretorio do SoVITS nao encontrado: $sovitsDir" -ForegroundColor Red
    exit 1
}

# Verificar se o modelo G_1000.pth existe
$modelPath = Join-Path $sovitsDir "logs\44k\G_1000.pth"
if (-not (Test-Path $modelPath)) {
    Write-Host "[ERRO] Modelo G_1000.pth nao encontrado!" -ForegroundColor Red
    Write-Host "   Procurando em: $modelPath" -ForegroundColor Yellow
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

Write-Host "[OK] Modelo encontrado: G_1000.pth" -ForegroundColor Green

# Verificar se o ambiente virtual existe
$venvPath = Join-Path $sovitsDir "venv310"
if (-not (Test-Path $venvPath)) {
    Write-Host "[ERRO] Ambiente virtual nao encontrado: $venvPath" -ForegroundColor Red
    Write-Host "   Crie o ambiente virtual primeiro:" -ForegroundColor Yellow
    Write-Host "   cd $sovitsDir" -ForegroundColor Gray
    Write-Host "   python -m venv venv310" -ForegroundColor Gray
    exit 1
}

# Ativar ambiente virtual
$activateScript = Join-Path $venvPath "Scripts\Activate.ps1"
if (-not (Test-Path $activateScript)) {
    Write-Host "[ERRO] Script de ativacao nao encontrado: $activateScript" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[INFO] Ativando ambiente virtual do SoVITS..." -ForegroundColor Cyan
& $activateScript

# Verificar se o Coqui TTS esta instalado
Write-Host ""
Write-Host "[INFO] Verificando dependencias..." -ForegroundColor Cyan
try {
    $ttsCheck = python -c "from TTS.api import TTS; print('OK')" 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[ERRO] Coqui TTS nao esta instalado!" -ForegroundColor Red
        Write-Host "   Instale com: pip install TTS" -ForegroundColor Yellow
        exit 1
    }
    Write-Host "[OK] Coqui TTS instalado" -ForegroundColor Green
} catch {
    Write-Host "[ERRO] Erro ao verificar Coqui TTS: $_" -ForegroundColor Red
    exit 1
}

# Executar script Python
Write-Host ""
Write-Host "[INFO] Executando pipeline completa..." -ForegroundColor Cyan
Write-Host "   (Isso pode levar alguns minutos na primeira vez)" -ForegroundColor Gray
Write-Host ""

$pythonScript = Join-Path $scriptDir "test_hello_world_pipeline_1000.py"

try {
    python $pythonScript
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "[SUCESSO] TESTE CONCLUIDO COM SUCESSO!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Arquivos gerados:" -ForegroundColor Cyan
        Write-Host "   - test_hello_world_xtts_output.wav (audio XTTS)" -ForegroundColor White
        Write-Host "   - test_hello_world_sovits_1000_output.wav (audio final SoVITS)" -ForegroundColor White
        Write-Host ""
        Write-Host "Ouça o resultado final em:" -ForegroundColor Cyan
        $outputPath = Join-Path $scriptDir "test_hello_world_sovits_1000_output.wav"
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

