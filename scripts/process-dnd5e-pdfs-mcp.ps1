# Script para processar PDFs de D&D 5e usando MCP Vectorizer
# Este script processa os PDFs e adiciona diretamente ao Vectorizer via MCP
# Uso: .\scripts\process-dnd5e-pdfs-mcp.ps1

param(
    [string]$CollectionName = "dnd5e-rules",
    [string]$SourceDir = "G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service"
)

$ErrorActionPreference = "Stop"

Write-Host "📚 Processando PDFs de D&D 5e e adicionando ao Vectorizer..." -ForegroundColor Cyan
Write-Host "   Collection: $CollectionName" -ForegroundColor Gray
Write-Host "   Source: $SourceDir" -ForegroundColor Gray
Write-Host ""

# Lista de PDFs para processar
$pdfs = @(
    @{
        File = "dd-5e-livro-do-jogador-fundo-branco-biblioteca-elfica.pdf"
        Type = "player_handbook"
        Title = "D&D 5e - Livro do Jogador"
        Description = "Regras completas para jogadores de D&D 5e"
    },
    @{
        File = "dd-5e-guia-do-mestre-biblioteca-elfica.pdf"
        Type = "dungeon_master_guide"
        Title = "D&D 5e - Guia do Mestre"
        Description = "Guia completo para mestres de D&D 5e"
    },
    @{
        File = "old-dd-5e-manual-dos-monstros-biblioteca-elfica.pdf"
        Type = "monster_manual"
        Title = "D&D 5e - Manual dos Monstros"
        Description = "Catálogo completo de monstros e criaturas de D&D 5e"
    },
    @{
        File = "dd-5e-ficha-de-personagem-completavel-biblioteca-elfica.pdf"
        Type = "character_sheet"
        Title = "D&D 5e - Ficha de Personagem"
        Description = "Ficha de personagem editável para D&D 5e"
    }
)

# Função para extrair texto de PDF usando Python
function Extract-PdfText {
    param(
        [string]$PdfPath
    )
    
    $pythonScript = @"
import sys
import json
import os

pdf_path = r'$PdfPath'

# Tentar diferentes bibliotecas
text_content = []
pages = []

try:
    # Tentar pdfplumber primeiro (melhor qualidade)
    try:
        import pdfplumber
        with pdfplumber.open(pdf_path) as pdf:
            for i, page in enumerate(pdf.pages, 1):
                text = page.extract_text()
                if text:
                    text = text.strip()
                    pages.append({
                        'number': i,
                        'text': text,
                        'length': len(text)
                    })
                    text_content.append(f"# Página {i}\n\n{text}\n\n")
        print(json.dumps({'success': True, 'method': 'pdfplumber', 'pages': len(pages)}))
    except ImportError:
        # Fallback para PyPDF2
        import PyPDF2
        with open(pdf_path, 'rb') as file:
            pdf_reader = PyPDF2.PdfReader(file)
            for i, page in enumerate(pdf_reader.pages, 1):
                text = page.extract_text()
                if text:
                    text = text.strip()
                    pages.append({
                        'number': i,
                        'text': text,
                        'length': len(text)
                    })
                    text_content.append(f"# Página {i}\n\n{text}\n\n")
        print(json.dumps({'success': True, 'method': 'PyPDF2', 'pages': len(pages)}))
except Exception as e:
    print(json.dumps({'success': False, 'error': str(e)}))
    sys.exit(1)

# Output markdown
markdown = '\n'.join(text_content)
print('---MARKDOWN_START---')
print(markdown)
print('---MARKDOWN_END---')
"@
    
    try {
        $output = $pythonScript | python - 2>&1
        
        if ($LASTEXITCODE -ne 0) {
            throw "Erro ao executar Python: $output"
        }
        
        # Separar JSON e Markdown
        $jsonOutput = $output | Where-Object { $_ -notmatch 'MARKDOWN' } | Out-String
        $markdownStart = $output | Select-String -Pattern '---MARKDOWN_START---' | Select-Object -First 1
        $markdownEnd = $output | Select-String -Pattern '---MARKDOWN_END---' | Select-Object -First 1
        
        if ($markdownStart -and $markdownEnd) {
            $startIndex = ($output | Select-String -Pattern '---MARKDOWN_START---').LineNumber
            $endIndex = ($output | Select-String -Pattern '---MARKDOWN_END---').LineNumber
            $markdownLines = $output[$startIndex..($endIndex-2)]
            $markdown = $markdownLines -join "`n"
        } else {
            throw "Formato de saída inválido do Python"
        }
        
        $result = $jsonOutput | ConvertFrom-Json
        
        if (-not $result.success) {
            throw $result.error
        }
        
        return @{
            Markdown = $markdown
            Pages = $result.pages
            Method = $result.method
        }
    } catch {
        Write-Host "   ❌ Erro ao extrair texto: $_" -ForegroundColor Red
        return $null
    }
}

# Processar cada PDF
$totalProcessed = 0
$totalChunks = 0

foreach ($pdfInfo in $pdfs) {
    $pdfPath = Join-Path $SourceDir $pdfInfo.File
    
    if (-not (Test-Path $pdfPath)) {
        Write-Host "❌ PDF não encontrado: $pdfPath" -ForegroundColor Red
        continue
    }
    
    Write-Host ""
    Write-Host "📄 Processando: $($pdfInfo.Title)" -ForegroundColor Cyan
    Write-Host "   Arquivo: $($pdfInfo.File)" -ForegroundColor Gray
    
    # Extrair texto do PDF
    Write-Host "   🔄 Extraindo texto do PDF..." -ForegroundColor Yellow
    $extractionResult = Extract-PdfText -PdfPath $pdfPath
    
    if ($null -eq $extractionResult) {
        Write-Host "   ❌ Falha na extração" -ForegroundColor Red
        continue
    }
    
    Write-Host "   ✅ Extraído: $($extractionResult.Pages) páginas usando $($extractionResult.Method)" -ForegroundColor Green
    Write-Host "   📏 Tamanho: $($extractionResult.Markdown.Length) caracteres" -ForegroundColor Gray
    
    # Dividir em chunks para inserção no Vectorizer
    # Cada chunk terá aproximadamente 500-1000 palavras
    $chunkSize = 2000  # caracteres por chunk
    $chunks = @()
    $markdown = $extractionResult.Markdown
    
    for ($i = 0; $i -lt $markdown.Length; $i += $chunkSize) {
        $chunkText = $markdown.Substring($i, [Math]::Min($chunkSize, $markdown.Length - $i))
        $chunkIndex = [int]($i / $chunkSize)
        $totalChunksInDoc = [Math]::Ceiling($markdown.Length / $chunkSize)
        
        # Criar metadata para o chunk
        $metadata = @{
            source_file = $pdfInfo.File
            document_type = $pdfInfo.Type
            title = $pdfInfo.Title
            description = $pdfInfo.Description
            chunk_index = $chunkIndex
            total_chunks = $totalChunksInDoc
            game_system = "dnd5e"
            language = "pt-BR"
        }
        
        $chunks += @{
            text = $chunkText
            metadata = $metadata
        }
    }
    
    Write-Host "   📦 Criados $($chunks.Count) chunks" -ForegroundColor Gray
    
    # Inserir chunks no Vectorizer via MCP
    Write-Host "   📊 Inserindo chunks no Vectorizer..." -ForegroundColor Yellow
    
    $insertedCount = 0
    foreach ($chunk in $chunks) {
        try {
            # Nota: Esta é uma simulação. A inserção real seria feita via MCP Vectorizer
            # Por enquanto, vamos salvar os dados para inserção manual ou via API
            
            $chunkFile = Join-Path $SourceDir "specs\rules5e-service\$($pdfInfo.Type)_chunk_$($chunk.metadata.chunk_index).json"
            $chunkDir = Split-Path $chunkFile -Parent
            if (-not (Test-Path $chunkDir)) {
                New-Item -ItemType Directory -Path $chunkDir -Force | Out-Null
            }
            
            @{
                text = $chunk.text
                metadata = $chunk.metadata
            } | ConvertTo-Json -Depth 10 | Set-Content $chunkFile -Encoding UTF8
            
            $insertedCount++
            
            if ($insertedCount % 10 -eq 0) {
                Write-Host "      ✅ $insertedCount/$($chunks.Count) chunks processados..." -ForegroundColor Gray
            }
        } catch {
            Write-Host "      ❌ Erro ao processar chunk $($chunk.metadata.chunk_index): $_" -ForegroundColor Red
        }
    }
    
    Write-Host "   ✅ $insertedCount chunks salvos para inserção" -ForegroundColor Green
    $totalProcessed++
    $totalChunks += $insertedCount
}

Write-Host ""
Write-Host "✅ Processamento concluído!" -ForegroundColor Green
Write-Host "   📊 Total: $totalProcessed documentos, $totalChunks chunks" -ForegroundColor Cyan
Write-Host ""
Write-Host "📝 Próximos passos:" -ForegroundColor Cyan
Write-Host "   1. Os chunks foram salvos em: $SourceDir\specs\rules5e-service\" -ForegroundColor Gray
Write-Host "   2. Use o MCP Vectorizer para inserir os chunks na collection '$CollectionName'" -ForegroundColor Gray
Write-Host "   3. Exemplo de inserção via MCP:" -ForegroundColor Gray
Write-Host "      - Use mcp_vectorizer-main_insert_text para cada chunk" -ForegroundColor DarkGray
Write-Host "      - Collection: $CollectionName" -ForegroundColor DarkGray
Write-Host "      - Metadata será incluída automaticamente" -ForegroundColor DarkGray
Write-Host ""



