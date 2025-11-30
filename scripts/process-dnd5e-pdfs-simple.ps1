# Script simplificado para processar PDFs de D&D 5e e adicionar ao Vectorizer via MCP
# Uso: .\scripts\process-dnd5e-pdfs-simple.ps1

$ErrorActionPreference = "Stop"

$SourceDir = "G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service"
$OutputDir = "G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service"
$CollectionName = "dnd5e-rules"

Write-Host "Processando PDFs de D&D 5e..." -ForegroundColor Cyan
Write-Host "Collection: $CollectionName" -ForegroundColor Gray
Write-Host "Source: $SourceDir" -ForegroundColor Gray
Write-Host ""

# Criar diretório de output
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

# Lista de PDFs
$pdfs = @(
    @{ File = "dd-5e-livro-do-jogador-fundo-branco-biblioteca-elfica.pdf"; Type = "player_handbook"; Title = "Livro do Jogador" },
    @{ File = "dd-5e-guia-do-mestre-biblioteca-elfica.pdf"; Type = "dungeon_master_guide"; Title = "Guia do Mestre" },
    @{ File = "old-dd-5e-manual-dos-monstros-biblioteca-elfica.pdf"; Type = "monster_manual"; Title = "Manual dos Monstros" },
    @{ File = "dd-5e-ficha-de-personagem-completavel-biblioteca-elfica.pdf"; Type = "character_sheet"; Title = "Ficha de Personagem" }
)

# Processar cada PDF
foreach ($pdfInfo in $pdfs) {
    $pdfPath = Join-Path $SourceDir $pdfInfo.File
    
    if (-not (Test-Path $pdfPath)) {
        Write-Host "PDF nao encontrado: $pdfPath" -ForegroundColor Red
        continue
    }
    
    Write-Host ""
    Write-Host "Processando: $($pdfInfo.Title)" -ForegroundColor Cyan
    
    # Extrair texto usando Python
    $pythonScript = @"
import sys
import json
import pdfplumber

pdf_path = r'$pdfPath'
pages = []
text_parts = []

try:
    with pdfplumber.open(pdf_path) as pdf:
        for i, page in enumerate(pdf.pages, 1):
            text = page.extract_text()
            if text:
                text = text.strip()
                pages.append({'number': i, 'text': text})
                text_parts.append(f"# Pagina {i}\n\n{text}\n\n")
    
    markdown = '\n'.join(text_parts)
    result = {
        'success': True,
        'pages': len(pages),
        'markdown': markdown
    }
    print(json.dumps(result))
except Exception as e:
    print(json.dumps({'success': False, 'error': str(e)}))
    sys.exit(1)
"@
    
    Write-Host "  Extraindo texto..." -ForegroundColor Yellow
    $jsonOutput = $pythonScript | python - 2>&1
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  Erro ao extrair texto" -ForegroundColor Red
        continue
    }
    
    $result = $jsonOutput | ConvertFrom-Json
    
    if (-not $result.success) {
        Write-Host "  Erro: $($result.error)" -ForegroundColor Red
        continue
    }
    
    Write-Host "  Extraido: $($result.pages) paginas" -ForegroundColor Green
    Write-Host "  Tamanho: $($result.markdown.Length) caracteres" -ForegroundColor Gray
    
    # Dividir em chunks
    $chunkSize = 2000
    $chunks = @()
    $markdown = $result.markdown
    
    for ($i = 0; $i -lt $markdown.Length; $i += $chunkSize) {
        $chunkText = $markdown.Substring($i, [Math]::Min($chunkSize, $markdown.Length - $i))
        $chunkIndex = [int]($i / $chunkSize)
        $totalChunks = [Math]::Ceiling($markdown.Length / $chunkSize)
        
        $metadata = @{
            source_file = $pdfInfo.File
            document_type = $pdfInfo.Type
            title = $pdfInfo.Title
            chunk_index = $chunkIndex
            total_chunks = $totalChunks
            game_system = "dnd5e"
            language = "pt-BR"
        }
        
        $chunks += @{
            text = $chunkText
            metadata = $metadata
        }
    }
    
    Write-Host "  Criados $($chunks.Count) chunks" -ForegroundColor Gray
    
    # Salvar chunks para inserção
    $chunkNum = 0
    foreach ($chunk in $chunks) {
        $chunkFile = Join-Path $OutputDir "$($pdfInfo.Type)_chunk_$chunkNum.json"
        $jsonContent = @{
            text = $chunk.text
            metadata = $chunk.metadata
        } | ConvertTo-Json -Depth 10
        [System.IO.File]::WriteAllText($chunkFile, $jsonContent, [System.Text.UTF8Encoding]::new($false))
        $chunkNum++
    }
    
    Write-Host "  Chunks salvos em: $OutputDir" -ForegroundColor Green
}

Write-Host ""
Write-Host "Processamento concluido!" -ForegroundColor Green
Write-Host ""
Write-Host "Os chunks foram salvos em: $OutputDir" -ForegroundColor Cyan
Write-Host "Use o MCP Vectorizer para inserir na collection '$CollectionName'" -ForegroundColor Cyan
Write-Host ""


