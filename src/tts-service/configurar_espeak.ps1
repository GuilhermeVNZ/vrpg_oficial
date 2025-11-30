# Script para configurar espeak-ng no Windows
# Execute este script APOS extrair o espeak-ng

Write-Host "========================================" -ForegroundColor Green
Write-Host "  CONFIGURAR ESPEAK-NG" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

# Perguntar onde o espeak-ng foi extraído
$espeakPath = Read-Host "Digite o caminho completo onde voce extraiu o espeak-ng (ex: C:\Program Files\espeak-ng ou G:\vrpg\vrpg-client\tools\espeak-ng)"

if (-not (Test-Path $espeakPath)) {
    Write-Host "ERRO: Caminho nao encontrado: $espeakPath" -ForegroundColor Red
    exit 1
}

# Verificar se tem o executavel
$espeakExe = Join-Path $espeakPath "espeak-ng.exe"
if (-not (Test-Path $espeakExe)) {
    $espeakExe = Join-Path $espeakPath "espeak.exe"
    if (-not (Test-Path $espeakExe)) {
        Write-Host "ERRO: Executavel nao encontrado em: $espeakPath" -ForegroundColor Red
        Write-Host "Procure por: espeak-ng.exe ou espeak.exe" -ForegroundColor Yellow
        exit 1
    }
}

Write-Host "`nExecutavel encontrado: $espeakExe" -ForegroundColor Green

# Testar se funciona
Write-Host "`nTestando espeak-ng..." -ForegroundColor Cyan
$testOutput = & $espeakExe --version 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "OK! espeak-ng esta funcionando" -ForegroundColor Green
    Write-Host "Versao: $testOutput" -ForegroundColor White
} else {
    Write-Host "AVISO: Nao foi possivel executar o espeak-ng" -ForegroundColor Yellow
}

# Perguntar se quer adicionar ao PATH
Write-Host "`nDeseja adicionar ao PATH do sistema? (S/N)" -ForegroundColor Cyan
$addToPath = Read-Host

if ($addToPath -eq "S" -or $addToPath -eq "s" -or $addToPath -eq "Y" -or $addToPath -eq "y") {
    Write-Host "`nAdicionando ao PATH..." -ForegroundColor Cyan
    
    try {
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
        if ($currentPath -notlike "*$espeakPath*") {
            [Environment]::SetEnvironmentVariable(
                "Path",
                $currentPath + ";$espeakPath",
                "Machine"
            )
            Write-Host "Adicionado ao PATH com sucesso!" -ForegroundColor Green
            Write-Host "`nIMPORTANTE: Feche e abra um novo terminal para aplicar as mudancas" -ForegroundColor Yellow
        } else {
            Write-Host "Ja esta no PATH!" -ForegroundColor Green
        }
    } catch {
        Write-Host "ERRO ao adicionar ao PATH: $_" -ForegroundColor Red
        Write-Host "Tente executar como Administrador" -ForegroundColor Yellow
    }
} else {
    Write-Host "`nOK, nao adicionado ao PATH." -ForegroundColor Yellow
    Write-Host "Voce pode:" -ForegroundColor Cyan
    Write-Host "1. Adicionar manualmente ao PATH depois" -ForegroundColor White
    Write-Host "2. Ou me dizer o caminho e eu ajusto o codigo para usar caminho absoluto" -ForegroundColor White
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  CONFIGURACAO CONCLUIDA" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

