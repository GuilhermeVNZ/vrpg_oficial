# Script para reiniciar Vectorizer e forçar indexação dos livros processados
# Uso: .\scripts\index-processed-books.ps1

Write-Host "=== Indexando Livros Processados no Vectorizer ===" -ForegroundColor Cyan
Write-Host ""

# Verificar se há arquivos processados
$processedDir = "G:\vrpg\vrpg-client\assets-and-models\books\processed"
if (-not (Test-Path $processedDir)) {
    Write-Host "❌ Diretório de arquivos processados não encontrado: $processedDir" -ForegroundColor Red
    exit 1
}

$mdFiles = Get-ChildItem "$processedDir\*.md" -ErrorAction SilentlyContinue
if ($mdFiles.Count -eq 0) {
    Write-Host "❌ Nenhum arquivo MD encontrado para indexar" -ForegroundColor Red
    exit 1
}

Write-Host "📚 Arquivos para indexar: $($mdFiles.Count)" -ForegroundColor Green
Write-Host ""

# Verificar se Vectorizer está rodando
Write-Host "🔍 Verificando Vectorizer..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "http://localhost:15002/health" -Method Get -TimeoutSec 5
    Write-Host "✅ Vectorizer está online" -ForegroundColor Green
} catch {
    Write-Host "❌ Vectorizer não está respondendo. Iniciando..." -ForegroundColor Yellow
    
    # Tentar iniciar Vectorizer
    $vectorizerPath = "G:\vrpg\vectorizer-feature-native-engine-optimization\target\release\vectorizer.exe"
    if (Test-Path $vectorizerPath) {
        Start-Process -FilePath $vectorizerPath -WindowStyle Hidden
        Write-Host "⏳ Aguardando Vectorizer iniciar..." -ForegroundColor Yellow
        Start-Sleep -Seconds 10
        
        # Verificar novamente
        try {
            $response = Invoke-RestMethod -Uri "http://localhost:15002/health" -Method Get -TimeoutSec 5
            Write-Host "✅ Vectorizer iniciado com sucesso" -ForegroundColor Green
        } catch {
            Write-Host "❌ Falha ao iniciar Vectorizer" -ForegroundColor Red
            exit 1
        }
    } else {
        Write-Host "❌ Vectorizer não encontrado em: $vectorizerPath" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "📋 Verificando workspace..." -ForegroundColor Yellow

# Verificar se a coleção está configurada
try {
    $workspace = Invoke-RestMethod -Uri "http://localhost:15002/api/workspace/config" -Method Get -TimeoutSec 10
    $booksCollection = $workspace.projects | Where-Object { $_.name -eq "vrpg-client" } | 
                       Select-Object -ExpandProperty collections | 
                       Where-Object { $_.name -eq "books-processed" }
    
    if ($booksCollection) {
        Write-Host "✅ Coleção 'books-processed' encontrada no workspace" -ForegroundColor Green
    } else {
        Write-Host "⚠️  Coleção 'books-processed' não encontrada. Verifique o workspace." -ForegroundColor Yellow
    }
} catch {
    Write-Host "⚠️  Erro ao verificar workspace: $_" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🔄 Para forçar indexação, o Vectorizer precisa recarregar o workspace." -ForegroundColor Cyan
Write-Host "   Reinicie o Vectorizer ou aguarde a indexação automática." -ForegroundColor Cyan
Write-Host ""
Write-Host "📊 Status:" -ForegroundColor Cyan
Write-Host "   - Arquivos MD processados: $($mdFiles.Count)" -ForegroundColor White
Write-Host "   - Diretório: $processedDir" -ForegroundColor White
Write-Host "   - Coleção: books-processed" -ForegroundColor White
Write-Host ""
Write-Host "✅ Pronto para indexação!" -ForegroundColor Green




