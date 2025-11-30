# Script PowerShell para testar Hello World -> SoVITS (Dungeon Master)
# Segue as praticas do projeto e rulebook

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  TESTE: Hello World -> SoVITS (DM)" -ForegroundColor Green
Write-Host "========================================`n" -ForegroundColor Green

# Caminhos
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$testsDir = Split-Path -Parent $scriptDir
$ttsServiceDir = Split-Path -Parent $testsDir
$vrpgClientDir = Split-Path -Parent (Split-Path -Parent $ttsServiceDir)
$sovitsDir = Join-Path $vrpgClientDir "assets-and-models\models\tts\sovits"
$venvPython = Join-Path $sovitsDir "venv310\Scripts\python.exe"
$testScript = Join-Path $scriptDir "test_hello_world_sovits.py"

# Verificar se o venv existe
if (-not (Test-Path $venvPython)) {
    Write-Host "ERRO: Ambiente virtual do SoVITS nao encontrado!" -ForegroundColor Red
    Write-Host "   Caminho esperado: $venvPython" -ForegroundColor Yellow
    Write-Host "`n   Certifique-se de que o SoVITS esta configurado corretamente." -ForegroundColor Yellow
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

# Executar o teste usando o Python do venv
Write-Host "Executando teste...`n" -ForegroundColor Yellow

& $venvPython $testScript
$exitCode = $LASTEXITCODE

if ($exitCode -eq 0) {
    Write-Host "`nTeste concluido com sucesso!" -ForegroundColor Green
    Write-Host "`nVerifique o arquivo de saida em:" -ForegroundColor Cyan
    Write-Host "   $scriptDir\test_hello_world_sovits_output.wav" -ForegroundColor White
} else {
    Write-Host "`nTeste falhou com codigo de saida: $exitCode" -ForegroundColor Red
    exit $exitCode
}
