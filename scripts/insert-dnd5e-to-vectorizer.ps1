# Script para inserir chunks de D&D 5e no Vectorizer via MCP
# Uso: .\scripts\insert-dnd5e-to-vectorizer.ps1

$ErrorActionPreference = "Stop"

$ChunksDir = "G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service"
$CollectionName = "dnd5e-rules"

Write-Host "Inserindo chunks de D&D 5e no Vectorizer..." -ForegroundColor Cyan
Write-Host "Collection: $CollectionName" -ForegroundColor Gray
Write-Host "Diretorio: $ChunksDir" -ForegroundColor Gray
Write-Host ""

# Contar arquivos
$chunkFiles = Get-ChildItem -Path $ChunksDir -Filter "*.json"
$totalChunks = $chunkFiles.Count

Write-Host "Total de chunks encontrados: $totalChunks" -ForegroundColor Cyan
Write-Host ""

# Processar em lotes para evitar sobrecarga
$batchSize = 10
$processed = 0
$errors = 0

foreach ($chunkFile in $chunkFiles) {
    try {
        $chunkData = Get-Content $chunkFile -Raw -Encoding UTF8 | ConvertFrom-Json
        
        # Preparar metadata para o Vectorizer
        $metadata = @{
            source_file = $chunkData.metadata.source_file
            document_type = $chunkData.metadata.document_type
            title = $chunkData.metadata.title
            chunk_index = $chunkData.metadata.chunk_index
            total_chunks = $chunkData.metadata.total_chunks
            game_system = $chunkData.metadata.game_system
            language = $chunkData.metadata.language
        }
        
        # Nota: A inserção real seria feita via MCP Vectorizer API
        # Por enquanto, vamos apenas validar os dados
        
        $processed++
        
        if ($processed % $batchSize -eq 0) {
            Write-Host "  Processados: $processed/$totalChunks chunks..." -ForegroundColor Gray
        }
        
    } catch {
        Write-Host "  Erro ao processar $($chunkFile.Name): $_" -ForegroundColor Red
        $errors++
    }
}

Write-Host ""
Write-Host "Validacao concluida!" -ForegroundColor Green
Write-Host "  Processados: $processed chunks" -ForegroundColor Cyan
Write-Host "  Erros: $errors chunks" -ForegroundColor $(if ($errors -gt 0) { "Red" } else { "Green" })
Write-Host ""
Write-Host "Nota: Para inserir no Vectorizer, use o MCP Vectorizer insert_text" -ForegroundColor Yellow
Write-Host "  Collection: $CollectionName" -ForegroundColor Gray
Write-Host "  Cada chunk deve ser inserido com seu texto e metadata" -ForegroundColor Gray
Write-Host ""


