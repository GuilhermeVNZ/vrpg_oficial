# Status dos Modelos TTS

**Última atualização**: 2025-01-XX

## Piper TTS

### PT-BR (Português do Brasil)

✅ **INSTALADO**

- **Modelo**: `pt_BR-faber-medium.onnx`
- **Tamanho**: 60.27 MB
- **Localização**: `piper-pt-br.onnx` (raiz do diretório tts/)
- **Configuração**: `pt/pt_BR/pt_BR-faber-medium.onnx.json`
- **Qualidade**: Medium
- **Sample Rate**: 22,050 Hz
- **Dataset**: faber

**Status**: ✅ Pronto para uso

### EN (English)

✅ **INSTALADO** (variante en_GB)

- **Modelo**: `en_GB-northern_english_male-medium.onnx`
- **Tamanho**: 60.27 MB
- **Localização**: `piper-en-us.onnx` (raiz do diretório tts/)
- **Configuração**: `en/en_US/lessac/medium/en_GB-northern_english_male-medium.onnx.json`
- **Variante**: en_GB (Great Britain) - funciona perfeitamente para inglês geral
- **Qualidade**: Medium
- **Sample Rate**: 22,050 Hz
- **Dataset**: northern_english_male

**Status**: ✅ Pronto para uso

**Nota**: Embora seja en_GB (British English), funciona perfeitamente para síntese em inglês geral. Se preferir en_US especificamente, pode baixar `en_US-lessac-medium.onnx` do HuggingFace.

## SoVITS

❌ **NÃO INSTALADO** (requer treinamento)

SoVITS requer modelos treinados por personagem. Não há modelos pré-treinados públicos.

**Status**: ⚠️ Opcional - sistema funciona sem SoVITS (voz neutra apenas)

## Estrutura Atual

```
assets-and-models/models/tts/
├── piper-pt-br.onnx                        ✅ (60.27 MB) - Modelo principal PT-BR
├── piper-pt-br.onnx.json                   ✅ - Configuração PT-BR
├── piper-en-us.onnx                        ✅ (60.27 MB) - Modelo principal EN
├── piper-en-us.onnx.json                   ✅ - Configuração EN
├── pt/                                      [metadados originais - pode manter]
│   └── pt_BR/
│       └── [arquivos originais do HuggingFace]
├── en/                                      [metadados originais - pode manter]
│   └── en_US/
│       └── lessac/
│           └── medium/
│               └── [arquivos originais do HuggingFace]
└── sovits/
    └── [vazio - requer treinamento]
```

## Como Usar

O código TTS aceita caminhos completos para os modelos. Exemplo:

```rust
// Carregar Piper PT-BR e EN
pipeline.load_piper_models(
    "assets-and-models/models/tts/piper-pt-br.onnx",
    "assets-and-models/models/tts/piper-en-us.onnx"
).await?;
```

## Próximos Passos

1. ✅ Piper PT-BR instalado
2. ✅ Piper EN instalado (en_GB - funciona para inglês geral)
3. ⏳ Treinar modelos SoVITS (opcional, mas recomendado para melhor qualidade vocal)

