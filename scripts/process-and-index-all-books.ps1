# Script completo: Processa todos os PDFs e indexa no Vectorizer
# Uso: .\scripts\process-and-index-all-books.ps1

$ErrorActionPreference = "Stop"

Write-Host "=== PROCESSAMENTO E INDEXAÇÃO COMPLETA DE LIVROS ===" -ForegroundColor Cyan
Write-Host ""

# 1. Verificar Transmutation
Write-Host "[1/5] Verificando Transmutation..." -ForegroundColor Yellow
$transmutationPath = "G:\vrpg\transmutation-main\target\release\transmutation.exe"

if (-not (Test-Path $transmutationPath)) {
    Write-Host "  ⏳ Transmutation não encontrado. Compilando..." -ForegroundColor Yellow
    Push-Location "G:\vrpg\transmutation-main"
    cargo build --release
    Pop-Location
    
    if (-not (Test-Path $transmutationPath)) {
        Write-Host "  ❌ Falha ao compilar transmutation" -ForegroundColor Red
        exit 1
    }
}
Write-Host "  ✅ Transmutation pronto" -ForegroundColor Green
Write-Host ""

# 2. Verificar Classify
Write-Host "[2/5] Verificando Classify..." -ForegroundColor Yellow
$classifyPath = "G:\vrpg\classify-main\dist\cli.js"
if (-not (Test-Path $classifyPath)) {
    Write-Host "  ⏳ Classify não encontrado. Compilando..." -ForegroundColor Yellow
    Push-Location "G:\vrpg\classify-main"
    npm run build
    Pop-Location
}
Write-Host "  ✅ Classify pronto" -ForegroundColor Green
Write-Host ""

# 3. Verificar Vectorizer
Write-Host "[3/5] Verificando Vectorizer..." -ForegroundColor Yellow
try {
    $null = Invoke-RestMethod -Uri "http://localhost:15002/health" -Method Get -TimeoutSec 5
    Write-Host "  ✅ Vectorizer online" -ForegroundColor Green
} catch {
    Write-Host "  ⏳ Vectorizer offline. Iniciando..." -ForegroundColor Yellow
    $vectorizerPath = "G:\vrpg\vectorizer-feature-native-engine-optimization\target\release\vectorizer.exe"
    if (Test-Path $vectorizerPath) {
        Start-Process -FilePath $vectorizerPath -WindowStyle Hidden
        Start-Sleep -Seconds 10
        
        try {
            $null = Invoke-RestMethod -Uri "http://localhost:15002/health" -Method Get -TimeoutSec 5
            Write-Host "  ✅ Vectorizer iniciado" -ForegroundColor Green
        } catch {
            Write-Host "  ❌ Falha ao iniciar Vectorizer" -ForegroundColor Red
            exit 1
        }
    } else {
        Write-Host "  ❌ Vectorizer não encontrado" -ForegroundColor Red
        exit 1
    }
}
Write-Host ""

# 4. Processar PDFs
Write-Host "[4/5] Processando PDFs com Transmutation + Classify..." -ForegroundColor Yellow
$processedDir = "G:\vrpg\vrpg-client\assets-and-models\books\processed"
$sourceDir = "G:\vrpg\vrpg-client\assets-and-models\books"

# Contar arquivos
$totalPDFs = (Get-ChildItem "$sourceDir\*.pdf" -ErrorAction SilentlyContinue).Count
$processedMDs = 0
if (Test-Path $processedDir) {
    $processedMDs = (Get-ChildItem "$processedDir\*.md" -ErrorAction SilentlyContinue).Count
}

Write-Host "  Total PDFs: $totalPDFs" -ForegroundColor White
Write-Host "  Já processados: $processedMDs" -ForegroundColor White
Write-Host "  Faltam: $($totalPDFs - $processedMDs)" -ForegroundColor White
Write-Host ""

if ($processedMDs -lt $totalPDFs) {
    Write-Host "  Executando script de processamento..." -ForegroundColor Cyan
    Push-Location "G:\vrpg\vrpg-client"
    $env:PYTHONIOENCODING = "utf-8"
    python scripts/process-all-books-pipeline.py
    
    # Aguardar processamento completar
    $maxWait = 7200 # 2 horas máximo
    $waitTime = 0
    $checkInterval = 30 # verificar a cada 30 segundos
    
    while ($waitTime -lt $maxWait) {
        Start-Sleep -Seconds $checkInterval
        $waitTime += $checkInterval
        
        if (Test-Path $processedDir) {
            $currentProcessed = (Get-ChildItem "$processedDir\*.md" -ErrorAction SilentlyContinue).Count
            $progress = [math]::Round(($currentProcessed / $totalPDFs) * 100, 1)
            
            Write-Host "  Progresso: $currentProcessed/$totalPDFs ($progress%)" -ForegroundColor Gray
            
            if ($currentProcessed -ge $totalPDFs) {
                Write-Host "  ✅ Processamento completo!" -ForegroundColor Green
                break
            }
        }
    }
    Pop-Location
} else {
    Write-Host "  ✅ Todos os PDFs já foram processados" -ForegroundColor Green
}
Write-Host ""

# 5. Forçar indexação no Vectorizer
Write-Host "[5/5] Forçando indexação no Vectorizer..." -ForegroundColor Yellow

# Reiniciar Vectorizer para recarregar workspace
Write-Host "  Reiniciando Vectorizer para recarregar workspace..." -ForegroundColor Cyan
Get-Process | Where-Object { $_.ProcessName -like "*vectorizer*" } | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 3

$vectorizerPath = "G:\vrpg\vectorizer-feature-native-engine-optimization\target\release\vectorizer.exe"
if (Test-Path $vectorizerPath) {
    Start-Process -FilePath $vectorizerPath -WindowStyle Hidden
    Start-Sleep -Seconds 15
    
    try {
        $null = Invoke-RestMethod -Uri "http://localhost:15002/health" -Method Get -TimeoutSec 10
        Write-Host "  ✅ Vectorizer reiniciado" -ForegroundColor Green
        
        # Verificar workspace
        $workspace = Invoke-RestMethod -Uri "http://localhost:15002/api/workspace/config" -Method Get -TimeoutSec 10
        $booksCollection = $workspace.projects | Where-Object { $_.name -eq "vrpg-client" } | 
                           Select-Object -ExpandProperty collections | 
                           Where-Object { $_.name -eq "books-processed" }
        
        if ($booksCollection) {
            Write-Host "  ✅ Coleção 'books-processed' configurada" -ForegroundColor Green
            Write-Host "  ✅ Indexação automática iniciada" -ForegroundColor Green
        }
    } catch {
        Write-Host "  ⚠️  Erro ao reiniciar Vectorizer: $_" -ForegroundColor Yellow
    }
} else {
    Write-Host "  ❌ Vectorizer não encontrado" -ForegroundColor Red
}
Write-Host ""

# Resumo final
Write-Host "=== RESUMO ===" -ForegroundColor Cyan
if (Test-Path $processedDir) {
    $finalCount = (Get-ChildItem "$processedDir\*.md" -ErrorAction SilentlyContinue).Count
    Write-Host "Arquivos MD processados: $finalCount/$totalPDFs" -ForegroundColor White
    Write-Host "Diretório: $processedDir" -ForegroundColor White
}
Write-Host "Vectorizer: http://localhost:15002" -ForegroundColor White
Write-Host "Coleção: books-processed" -ForegroundColor White
Write-Host ""
Write-Host "✅ Processo completo!" -ForegroundColor Green




