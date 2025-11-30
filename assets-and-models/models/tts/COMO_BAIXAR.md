# Como Baixar Modelos Piper TTS

## Método 1: Download Manual (Recomendado)

### Passo a Passo

1. **Acesse o HuggingFace:**
   - URL: https://huggingface.co/rhasspy/piper-voices

2. **Baixe Piper PT-BR:**
   - Navegue até: `pt/pt_BR/faber/medium/`
   - Clique em `pt_BR-faber-medium.onnx` e baixe
   - Clique em `pt_BR-faber-medium.onnx.json` e baixe
   - Salve ambos na raiz: `assets-and-models/models/tts/`
   - **Renomeie para**: `piper-pt-br.onnx` e `piper-pt-br.onnx.json`

3. **Baixe Piper EN (EN-US ou EN-GB):**
   - **Opção A (EN-US)**: Navegue até `en/en_US/lessac/medium/` e baixe `en_US-lessac-medium.onnx`
   - **Opção B (EN-GB)**: Navegue até `en/en_GB/northern_english_male/medium/` e baixe `en_GB-northern_english_male-medium.onnx`
   - Baixe também o arquivo `.json` correspondente
   - Salve ambos na raiz: `assets-and-models/models/tts/`
   - **Renomeie para**: `piper-en-us.onnx` e `piper-en-us.onnx.json`

## Estrutura Final

Após baixar e renomear, a estrutura deve ser:

```
assets-and-models/models/tts/
├── piper-pt-br.onnx              ✅ (60.27 MB)
├── piper-pt-br.onnx.json         ✅
├── piper-en-us.onnx              ✅ (60.27 MB)
├── piper-en-us.onnx.json         ✅
└── sovits/                        [opcional - modelos SoVITS]
```

**Nota**: Os arquivos originais do HuggingFace podem ficar nas subpastas `pt/` e `en/`, mas os arquivos principais devem estar na raiz com os nomes padronizados acima.

## Método 2: Usando Git LFS

Se você tem `git-lfs` instalado:

```bash
cd assets-and-models/models/tts
git lfs install
git clone https://huggingface.co/rhasspy/piper-voices
# Depois copie e renomeie os arquivos .onnx necessários para a raiz
```

## Verificação

Após baixar e renomear, verifique se os arquivos existem:

```powershell
cd assets-and-models\models\tts
Test-Path "piper-pt-br.onnx"
Test-Path "piper-pt-br.onnx.json"
Test-Path "piper-en-us.onnx"
Test-Path "piper-en-us.onnx.json"
```

Os arquivos `.onnx` devem ter aproximadamente **60 MB** cada.

## Script de Reorganização Automática

Se você baixou os arquivos nas subpastas originais do HuggingFace, você pode usar este comando PowerShell para copiar e renomear automaticamente:

```powershell
cd assets-and-models\models\tts

# Copiar e renomear PT-BR
Copy-Item "pt\pt_BR\pt_BR-faber-medium.onnx" "piper-pt-br.onnx" -Force
Copy-Item "pt\pt_BR\pt_BR-faber-medium.onnx.json" "piper-pt-br.onnx.json" -Force

# Copiar e renomear EN
Copy-Item "en\en_US\lessac\medium\en_GB-northern_english_male-medium.onnx" "piper-en-us.onnx" -Force
Copy-Item "en\en_US\lessac\medium\en_GB-northern_english_male-medium.onnx.json" "piper-en-us.onnx.json" -Force
```

