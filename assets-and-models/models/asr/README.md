# Modelos ASR (Automatic Speech Recognition)

Este diretório contém os modelos Whisper para reconhecimento de voz.

## Modelo Principal

**Whisper Large V3**

- **Arquivo**: `whisper-large-v3.bin` ou `ggml-large-v3.bin`
- **Tamanho**: ~3GB
- **Formato**: GGML/GGUF
- **Idioma**: Multi-idioma (auto-detect)

### Onde Baixar

1. **Hugging Face (ggerganov/whisper.cpp)**:
   - Repositório: https://huggingface.co/ggerganov/whisper.cpp
   - Arquivo específico: `ggml-large-v3.bin`

2. **Whisper.cpp Releases**:
   - Repositório: https://github.com/ggerganov/whisper.cpp/releases
   - Modelos disponíveis: base, small, medium, large-v2, large-v3

### Estrutura Esperada

```
assets-and-models/
└── models/
    └── asr/
        └── whisper-large-v3.bin  (ou ggml-large-v3.bin)
```

## Modelos Alternativos

- **Whisper Medium**: Mais leve (~1.5GB), boa qualidade
- **Whisper Base**: Muito leve (~500MB), qualidade básica
- **Whisper Small**: Balanceado (~500MB), boa para uso geral

## Configuração

O caminho do modelo é configurado em:
- `config/vrpg.json` → `services.asr_service.model_path`
- `config/services.json` → `asr.model`










