# Script para verificar espeak-ng após instalação

Write-Host "========================================" -ForegroundColor Green
Write-Host "  VERIFICANDO INSTALACAO ESPEAK-NG" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

# Atualizar PATH
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

# Verificar PATH
$inPath = Get-Command espeak-ng -ErrorAction SilentlyContinue
if ($inPath) {
    Write-Host "[OK] Encontrado no PATH: $($inPath.Source)" -ForegroundColor Green
    $espeakPath = $inPath.Source
} else {
    # Verificar locais comuns
    $commonPaths = @(
        "C:\Program Files\espeak-ng\espeak-ng.exe",
        "C:\Program Files (x86)\espeak-ng\espeak-ng.exe",
        "C:\Program Files\espeak\espeak.exe",
        "C:\Program Files (x86)\espeak\espeak.exe"
    )
    
    $found = $false
    foreach ($path in $commonPaths) {
        if (Test-Path $path) {
            Write-Host "[OK] Encontrado: $path" -ForegroundColor Green
            $espeakPath = $path
            $found = $true
            break
        }
    }
    
    if (-not $found) {
        Write-Host "[ERRO] espeak-ng nao encontrado!" -ForegroundColor Red
        Write-Host "Certifique-se de que a instalacao foi concluida." -ForegroundColor Yellow
        exit 1
    }
}

# Testar versão
Write-Host "`nTestando versao..." -ForegroundColor Cyan
$version = & $espeakPath --version 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "Versao: $version" -ForegroundColor White
} else {
    Write-Host "[ERRO] Falha ao executar espeak-ng" -ForegroundColor Red
    exit 1
}

# Testar phonemização
Write-Host "`nTestando phonemizacao..." -ForegroundColor Cyan
$testText = "Hello world"
$phonemes = & $espeakPath -q -x --phonout=- --phonout-ipa -v en-us $testText 2>&1

$phonemesStr = $phonemes -join ""
if ($phonemesStr -and $phonemesStr.Trim().Length -gt 0) {
    Write-Host "Phonemes gerados: $phonemesStr" -ForegroundColor White
    Write-Host "`n[SUCESSO] espeak-ng esta funcionando perfeitamente!" -ForegroundColor Green
    Write-Host "`nO TTS Service agora pode usar phonemizacao real!" -ForegroundColor Green
} else {
    Write-Host "[AVISO] Nenhum phoneme gerado" -ForegroundColor Yellow
}

# Verificar se precisa adicionar ao PATH
if (-not $inPath) {
    Write-Host "`nAdicionando ao PATH do sistema..." -ForegroundColor Cyan
    $espeakDir = Split-Path $espeakPath -Parent
    try {
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
        if ($currentPath -notlike "*$espeakDir*") {
            [Environment]::SetEnvironmentVariable("Path", $currentPath + ";$espeakDir", "Machine")
            Write-Host "[OK] Adicionado ao PATH!" -ForegroundColor Green
            Write-Host "IMPORTANTE: Feche e abra um novo terminal para aplicar" -ForegroundColor Yellow
        } else {
            Write-Host "[OK] Ja esta no PATH" -ForegroundColor Green
        }
    } catch {
        Write-Host "[AVISO] Nao foi possivel adicionar ao PATH automaticamente" -ForegroundColor Yellow
        Write-Host "Execute como Administrador ou adicione manualmente: $espeakDir" -ForegroundColor Yellow
    }
}

Write-Host "`n========================================" -ForegroundColor Green

