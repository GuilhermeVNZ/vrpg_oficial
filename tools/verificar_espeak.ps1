# Script para verificar e configurar espeak-ng

Write-Host "========================================" -ForegroundColor Green
Write-Host "  VERIFICAR E CONFIGURAR ESPEAK-NG" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

# Verificar se esta no PATH
Write-Host "[1] Verificando PATH..." -ForegroundColor Cyan
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
$inPath = Get-Command espeak-ng -ErrorAction SilentlyContinue
if ($inPath) {
    Write-Host "  OK! Encontrado no PATH: $($inPath.Source)" -ForegroundColor Green
    Write-Host "  Testando..." -ForegroundColor Cyan
    $version = & $inPath.Source --version 2>&1
    Write-Host "  Versao: $version" -ForegroundColor White
    exit 0
} else {
    Write-Host "  Nao encontrado no PATH" -ForegroundColor Yellow
}

# Verificar locais comuns
Write-Host "`n[2] Verificando locais comuns..." -ForegroundColor Cyan
$commonPaths = @(
    "C:\Program Files\espeak-ng\espeak-ng.exe",
    "C:\Program Files (x86)\espeak-ng\espeak-ng.exe",
    "C:\Program Files\espeak\espeak.exe",
    "C:\Program Files (x86)\espeak\espeak.exe"
)

$found = $false
foreach ($path in $commonPaths) {
    if (Test-Path $path) {
        Write-Host "  Encontrado: $path" -ForegroundColor Green
        $found = $true
        
        Write-Host "  Testando..." -ForegroundColor Cyan
        $version = & $path --version 2>&1
        Write-Host "  Versao: $version" -ForegroundColor White
        
        Write-Host "`n[3] Adicionando ao PATH..." -ForegroundColor Cyan
        $espeakDir = Split-Path $path -Parent
        try {
            $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
            if ($currentPath -notlike "*$espeakDir*") {
                [Environment]::SetEnvironmentVariable("Path", $currentPath + ";$espeakDir", "Machine")
                Write-Host "  Adicionado ao PATH do sistema!" -ForegroundColor Green
                Write-Host "`nIMPORTANTE: Feche e abra um novo terminal para aplicar" -ForegroundColor Yellow
            } else {
                Write-Host "  Ja esta no PATH" -ForegroundColor Green
            }
        } catch {
            Write-Host "  ERRO: $_" -ForegroundColor Red
            Write-Host "  Execute como Administrador" -ForegroundColor Yellow
        }
        break
    }
}

if (-not $found) {
    Write-Host "  Nao encontrado nos locais comuns" -ForegroundColor Red
    Write-Host "`n[4] Procurando em todo o sistema..." -ForegroundColor Cyan
    Write-Host "  (Isso pode demorar...)" -ForegroundColor Yellow
    
    $drives = Get-PSDrive -PSProvider FileSystem | Select-Object -ExpandProperty Root
    $searchPaths = @()
    foreach ($drive in $drives) {
        $searchPaths += "$drive\Program Files\espeak-ng"
        $searchPaths += "$drive\Program Files (x86)\espeak-ng"
    }
    
    $foundAny = $false
    foreach ($searchPath in $searchPaths) {
        $exePath = Join-Path $searchPath "espeak-ng.exe"
        if (Test-Path $exePath) {
            Write-Host "  Encontrado: $exePath" -ForegroundColor Green
            $foundAny = $true
            break
        }
    }
    
    if (-not $foundAny) {
        Write-Host "`nESPEAK-NG NAO ENCONTRADO" -ForegroundColor Red
        Write-Host "`nOpcoes:" -ForegroundColor Yellow
        Write-Host "1. Baixe o instalador de: https://github.com/espeak-ng/espeak-ng/releases" -ForegroundColor White
        Write-Host "2. Execute o instalador .msi" -ForegroundColor White
        Write-Host "3. Ou me diga onde voce instalou e eu ajusto o codigo" -ForegroundColor White
    }
}

Write-Host "`n========================================" -ForegroundColor Green

