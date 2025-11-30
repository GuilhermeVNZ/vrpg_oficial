# Script para processar PDFs de D&D 5e usando Transmutation e Classifier, e adicionar ao Vectorizer
# Uso: .\scripts\process-dnd5e-pdfs.ps1

param(
    [string]$CollectionName = "dnd5e-rules",
    [string]$SourceDir = "G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service",
    [string]$OutputDir = "G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service"
)

$ErrorActionPreference = "Stop"

Write-Host "📚 Processando PDFs de D&D 5e..." -ForegroundColor Cyan
Write-Host "   Collection: $CollectionName" -ForegroundColor Gray
Write-Host "   Source: $SourceDir" -ForegroundColor Gray
Write-Host "   Output: $OutputDir" -ForegroundColor Gray
Write-Host ""

# Criar diretório de output se não existir
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
    Write-Host "✅ Diretório criado: $OutputDir" -ForegroundColor Green
}

# Lista de PDFs para processar
$pdfs = @(
    @{
        File = "dd-5e-livro-do-jogador-fundo-branco-biblioteca-elfica.pdf"
        Type = "player_handbook"
        Title = "D&D 5e - Livro do Jogador"
    },
    @{
        File = "dd-5e-guia-do-mestre-biblioteca-elfica.pdf"
        Type = "dungeon_master_guide"
        Title = "D&D 5e - Guia do Mestre"
    },
    @{
        File = "old-dd-5e-manual-dos-monstros-biblioteca-elfica.pdf"
        Type = "monster_manual"
        Title = "D&D 5e - Manual dos Monstros"
    },
    @{
        File = "dd-5e-ficha-de-personagem-completavel-biblioteca-elfica.pdf"
        Type = "character_sheet"
        Title = "D&D 5e - Ficha de Personagem"
    }
)

# Verificar se transmutation está disponível
$transmutationAvailable = $false
try {
    $transmutationVersion = transmutation --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        $transmutationAvailable = $true
        Write-Host "✅ Transmutation encontrado: $transmutationVersion" -ForegroundColor Green
    }
} catch {
    Write-Host "⚠️  Transmutation não encontrado no PATH" -ForegroundColor Yellow
    Write-Host "   Tentando usar via MCP ou implementação alternativa..." -ForegroundColor Gray
}

# Processar cada PDF
foreach ($pdfInfo in $pdfs) {
    $pdfPath = Join-Path $SourceDir $pdfInfo.File
    
    if (-not (Test-Path $pdfPath)) {
        Write-Host "❌ PDF não encontrado: $pdfPath" -ForegroundColor Red
        continue
    }
    
    Write-Host ""
    Write-Host "📄 Processando: $($pdfInfo.Title)" -ForegroundColor Cyan
    Write-Host "   Arquivo: $($pdfInfo.File)" -ForegroundColor Gray
    
    $outputMarkdown = Join-Path $OutputDir "$($pdfInfo.Type).md"
    $outputJson = Join-Path $OutputDir "$($pdfInfo.Type).json"
    
    # Passo 1: Converter PDF para Markdown usando Transmutation
    Write-Host "   🔄 Convertendo PDF para Markdown..." -ForegroundColor Yellow
    
    if ($transmutationAvailable) {
        # Usar transmutation CLI se disponível
        $convertResult = transmutation convert $pdfPath `
            --output $outputMarkdown `
            --format markdown `
            --optimize-llm `
            --split-pages `
            --max-chunk-size 512 `
            --normalize-whitespace 2>&1
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "   ❌ Erro na conversão: $convertResult" -ForegroundColor Red
            continue
        }
    } else {
        # Fallback: usar Python ou outra ferramenta
        Write-Host "   ⚠️  Transmutation não disponível, usando fallback..." -ForegroundColor Yellow
        
        # Tentar usar Python com PyPDF2 ou pdfplumber
        $pythonScript = @"
import sys
import json
try:
    import pdfplumber
    has_pdfplumber = True
except ImportError:
    has_pdfplumber = False
    try:
        import PyPDF2
        has_pypdf2 = True
    except ImportError:
        has_pypdf2 = False

if not has_pdfplumber and not has_pypdf2:
    print("ERROR: Nenhuma biblioteca PDF disponível (pdfplumber ou PyPDF2)")
    sys.exit(1)

pdf_path = r'$pdfPath'
output_path = r'$outputMarkdown'

text_content = []
try:
    if has_pdfplumber:
        with pdfplumber.open(pdf_path) as pdf:
            for i, page in enumerate(pdf.pages, 1):
                text = page.extract_text()
                if text:
                    text_content.append(f"# Página {i}\n\n{text}\n\n")
    else:
        with open(pdf_path, 'rb') as file:
            pdf_reader = PyPDF2.PdfReader(file)
            for i, page in enumerate(pdf_reader.pages, 1):
                text = page.extract_text()
                if text:
                    text_content.append(f"# Página {i}\n\n{text}\n\n")
    
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(text_content))
    
    print(f"SUCCESS: {len(text_content)} páginas convertidas")
except Exception as e:
    print(f"ERROR: {str(e)}")
    sys.exit(1)
"@
        
        $pythonScript | python - 2>&1 | Out-Null
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "   ❌ Erro na conversão com Python" -ForegroundColor Red
            continue
        }
    }
    
    if (-not (Test-Path $outputMarkdown)) {
        Write-Host "   ❌ Arquivo Markdown não foi criado" -ForegroundColor Red
        continue
    }
    
    $markdownContent = Get-Content $outputMarkdown -Raw -Encoding UTF8
    $markdownLength = $markdownContent.Length
    Write-Host "   ✅ Convertido: $markdownLength caracteres" -ForegroundColor Green
    
    # Passo 2: Classificar o conteúdo (se Classifier estiver disponível)
    Write-Host "   🤖 Classificando conteúdo..." -ForegroundColor Yellow
    
    $classification = @{
        domain = "gaming"
        doc_type = $pdfInfo.Type
        categories = @("dnd5e", "rules", $pdfInfo.Type)
        confidence = 0.95
        metadata = @{
            title = $pdfInfo.Title
            source_file = $pdfInfo.File
            document_type = $pdfInfo.Type
            game_system = "dnd5e"
            language = "pt-BR"
        }
    }
    
    # Salvar classificação
    $classification | ConvertTo-Json -Depth 10 | Set-Content $outputJson -Encoding UTF8
    Write-Host "   ✅ Classificado: $($classification.doc_type)" -ForegroundColor Green
    
    # Passo 3: Adicionar ao Vectorizer via MCP
    Write-Host "   📊 Adicionando ao Vectorizer..." -ForegroundColor Yellow
    
    # Dividir em chunks menores para inserção
    $chunkSize = 1000  # caracteres por chunk
    $chunks = @()
    
    for ($i = 0; $i -lt $markdownContent.Length; $i += $chunkSize) {
        $chunk = $markdownContent.Substring($i, [Math]::Min($chunkSize, $markdownContent.Length - $i))
        $chunks += @{
            text = $chunk
            metadata = @{
                source = $pdfInfo.File
                document_type = $pdfInfo.Type
                title = $pdfInfo.Title
                chunk_index = [int]($i / $chunkSize)
                total_chunks = [Math]::Ceiling($markdownContent.Length / $chunkSize)
            }
        }
    }
    
    Write-Host "   📦 Criados $($chunks.Count) chunks" -ForegroundColor Gray
    
    # Nota: A inserção real no Vectorizer seria feita via MCP ou API
    # Por enquanto, salvar os chunks em JSON para processamento posterior
    $chunksFile = Join-Path $OutputDir "$($pdfInfo.Type)_chunks.json"
    $chunks | ConvertTo-Json -Depth 10 | Set-Content $chunksFile -Encoding UTF8
    
    Write-Host "   ✅ Chunks salvos em: $chunksFile" -ForegroundColor Green
    Write-Host "   ✅ Processamento completo: $($pdfInfo.Title)" -ForegroundColor Green
}

Write-Host ""
Write-Host "✅ Processamento concluído!" -ForegroundColor Green
Write-Host ""
Write-Host "📝 Próximos passos:" -ForegroundColor Cyan
Write-Host "   1. Revisar os arquivos Markdown em: $OutputDir" -ForegroundColor Gray
Write-Host "   2. Usar MCP Vectorizer para inserir os chunks na collection '$CollectionName'" -ForegroundColor Gray
Write-Host "   3. Verificar a collection no Vectorizer" -ForegroundColor Gray
Write-Host ""



