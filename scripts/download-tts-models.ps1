# Script para baixar modelos TTS (Piper + SoVITS)
# Executa: .\scripts\download-tts-models.ps1

param(
    [switch]$PiperOnly,
    [switch]$SoVITSOnly,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

$TTS_DIR = "$PSScriptRoot\..\assets-and-models\models\tts"
$PIPER_DIR = $TTS_DIR
$SOVITS_DIR = "$TTS_DIR\sovits"

Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  📦 Download TTS Models (Piper + SoVITS) ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Criar diretórios se não existirem
if (-not (Test-Path $TTS_DIR)) {
    New-Item -ItemType Directory -Path $TTS_DIR -Force | Out-Null
    Write-Host "✅ Criado diretório: $TTS_DIR" -ForegroundColor Green
}

if (-not (Test-Path $SOVITS_DIR)) {
    New-Item -ItemType Directory -Path $SOVITS_DIR -Force | Out-Null
    Write-Host "✅ Criado diretório: $SOVITS_DIR" -ForegroundColor Green
}

# Função para baixar arquivo
function Download-File {
    param(
        [string]$Url,
        [string]$OutputPath,
        [string]$Description
    )
    
    if ((Test-Path $OutputPath) -and -not $Force) {
        Write-Host "  ✅ $Description já existe: $(Split-Path $OutputPath -Leaf)" -ForegroundColor Green
        return $true
    }
    
    Write-Host "  📥 Baixando $Description..." -ForegroundColor Yellow
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $Url -OutFile $OutputPath -UseBasicParsing
        Write-Host "  ✅ $Description baixado com sucesso!" -ForegroundColor Green
        return $true
    }
    catch {
        Write-Host "  ❌ Erro ao baixar $Description : $_" -ForegroundColor Red
        return $false
    }
}

# ============================================
# PIPER TTS MODELS
# ============================================
if (-not $SoVITSOnly) {
    Write-Host "🔊 Piper TTS Models" -ForegroundColor Cyan
    Write-Host "─────────────────────────────────────" -ForegroundColor Gray
    
    # Piper PT-BR
    $PIPER_PT_URL = "https://huggingface.co/rhasspy/piper-voices/resolve/main/pt/pt_BR/lessac/medium/pt_BR_lessac_medium.onnx"
    $PIPER_PT_PATH = "$PIPER_DIR\piper-pt-br.onnx"
    
    # Piper EN-US
    $PIPER_EN_URL = "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/lessac/medium/en_US_lessac_medium.onnx"
    $PIPER_EN_PATH = "$PIPER_DIR\piper-en-us.onnx"
    
    $piperPtOk = Download-File -Url $PIPER_PT_URL -OutputPath $PIPER_PT_PATH -Description "Piper PT-BR"
    $piperEnOk = Download-File -Url $PIPER_EN_URL -OutputPath $PIPER_EN_PATH -Description "Piper EN-US"
    
    if ($piperPtOk -and $piperEnOk) {
        Write-Host "✅ Modelos Piper baixados com sucesso!" -ForegroundColor Green
    }
    else {
        Write-Host "⚠️  Alguns modelos Piper falharam ao baixar" -ForegroundColor Yellow
    }
    Write-Host ""
}

# ============================================
# XTTS EMBEDDINGS
# ============================================
if (-not $PiperOnly) {
    Write-Host "🎭 XTTS Embeddings" -ForegroundColor Cyan
    Write-Host "─────────────────────────────────────" -ForegroundColor Gray
    Write-Host "📝 Embeddings XTTS são arquivos WAV de referência para cada personagem" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "💡 Para criar embeddings:" -ForegroundColor Cyan
    Write-Host "   1. Colete 5-10 minutos de áudio limpo do personagem" -ForegroundColor Gray
    Write-Host "   2. Use create_clean_xtts_embedding.py para processar" -ForegroundColor Gray
    Write-Host "   3. Salve o embedding em: xtts_embeddings\<character_id>_xtts_reference_clean.wav" -ForegroundColor Gray
    Write-Host ""
    Write-Host "📁 Estrutura esperada:" -ForegroundColor Cyan
    $EMBEDDINGS_DIR = Join-Path $TTS_DIR "xtts_embeddings"
    Write-Host "   $EMBEDDINGS_DIR\" -ForegroundColor Gray
    Write-Host "   ├── narrator_default_xtts_reference_clean.wav" -ForegroundColor Gray
    Write-Host "   ├── npc_guard_xtts_reference_clean.wav" -ForegroundColor Gray
    Write-Host "   └── npc_barkeep_xtts_reference_clean.wav" -ForegroundColor Gray
    Write-Host ""
    Write-Host "📚 Recursos:" -ForegroundColor Cyan
    Write-Host "   - Guia completo: assets-and-models/models/tts/COMO_CRIAR_EMBEDDINGS_XTTS.md" -ForegroundColor Gray
    Write-Host "   - Script: src/tts-service/tests/scripts/create_clean_xtts_embedding.py" -ForegroundColor Gray
    Write-Host ""
}

# ============================================
# RESUMO
# ============================================
Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  📊 Resumo                              ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$piperPtExists = Test-Path "$PIPER_DIR\piper-pt-br.onnx"
$piperEnExists = Test-Path "$PIPER_DIR\piper-en-us.onnx"
$sovitsExists = (Get-ChildItem -Path $SOVITS_DIR -Filter "*.pth" -ErrorAction SilentlyContinue).Count -gt 0

Write-Host "Piper PT-BR:  $(if ($piperPtExists) { '✅ Instalado' } else { '❌ Não encontrado' })" -ForegroundColor $(if ($piperPtExists) { 'Green' } else { 'Red' })
Write-Host "Piper EN-US:  $(if ($piperEnExists) { '✅ Instalado' } else { '❌ Não encontrado' })" -ForegroundColor $(if ($piperEnExists) { 'Green' } else { 'Red' })
Write-Host "SoVITS:       $(if ($sovitsExists) { '✅ Modelos encontrados' } else { '⚠️  Nenhum modelo (requer treinamento)' })" -ForegroundColor $(if ($sovitsExists) { 'Green' } else { 'Yellow' })
Write-Host ""

if ($piperPtExists -and $piperEnExists) {
    Write-Host "✅ Piper está pronto para uso!" -ForegroundColor Green
}
else {
    Write-Host "⚠️  Execute novamente com -Force para re-baixar modelos Piper" -ForegroundColor Yellow
}

if (-not $sovitsExists) {
    Write-Host "💡 Para usar SoVITS, você precisa treinar modelos para seus personagens" -ForegroundColor Cyan
    Write-Host "   Consulte a documentação em: docs/AUDIO_PIPELINE.md" -ForegroundColor Gray
}

Write-Host ""



