# Script PowerShell para executar grid de testes de qualidade SoVITS

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  GRID DE TESTES: Qualidade SoVITS" -ForegroundColor Green
Write-Host "========================================`n" -ForegroundColor Green

# Caminhos
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$testsDir = Split-Path -Parent $scriptDir
$ttsServiceDir = Split-Path -Parent $testsDir
$vrpgClientDir = Split-Path -Parent (Split-Path -Parent $ttsServiceDir)
$sovitsDir = Join-Path $vrpgClientDir "assets-and-models\models\tts\sovits"
$venvPython = Join-Path $sovitsDir "venv310\Scripts\python.exe"
$testScript = Join-Path $scriptDir "test_sovits_quality_grid.py"

# Verificar se o venv existe
if (-not (Test-Path $venvPython)) {
    Write-Host "ERRO: Ambiente virtual do SoVITS nao encontrado!" -ForegroundColor Red
    Write-Host "   Caminho esperado: $venvPython" -ForegroundColor Yellow
    exit 1
}

# Verificar se o script Python existe
if (-not (Test-Path $testScript)) {
    Write-Host "ERRO: Script de teste nao encontrado!" -ForegroundColor Red
    Write-Host "   Caminho esperado: $testScript" -ForegroundColor Yellow
    exit 1
}

Write-Host "Diretorios:" -ForegroundColor Cyan
Write-Host "   SoVITS: $sovitsDir" -ForegroundColor White
Write-Host "   Python: $venvPython" -ForegroundColor White
Write-Host "   Script: $testScript" -ForegroundColor White
Write-Host ""

# Executar o teste
Write-Host "Executando grid de testes...`n" -ForegroundColor Yellow

& $venvPython $testScript
$exitCode = $LASTEXITCODE

if ($exitCode -eq 0) {
    Write-Host "`nTeste concluido com sucesso!" -ForegroundColor Green
    Write-Host "`nArquivos gerados em:" -ForegroundColor Cyan
    Write-Host "   $scriptDir\sovits_quality_tests\" -ForegroundColor White
    Write-Host "`nOuça cada arquivo e identifique qual soa melhor!" -ForegroundColor Yellow
} else {
    Write-Host "`nTeste falhou com codigo de saida: $exitCode" -ForegroundColor Red
    exit $exitCode
}

