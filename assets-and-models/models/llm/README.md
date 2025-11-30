# Modelos LLM

Este diretório contém os modelos de linguagem para o VRPG Client.

## Modelo Principal

**Qwen 2.5 14B (Q4_K_M quantization)**

- **Arquivo**: `qwen2.5-14b-instruct-q4_k_m.gguf`
- **Tamanho**: ~8GB
- **Formato**: GGUF
- **Quantização**: Q4_K_M

### Onde Baixar

1. **Hugging Face (TheBloke)**:
   - Repositório: https://huggingface.co/TheBloke/Qwen2.5-14B-Instruct-GGUF
   - Arquivo específico: `qwen2.5-14b-instruct.Q4_K_M.gguf`

2. **Hugging Face (Original)**:
   - Repositório: https://huggingface.co/Qwen/Qwen2.5-14B-Instruct-GGUF
   - Arquivo específico: `qwen2.5-14b-instruct-q4_k_m.gguf`

### Estrutura Esperada

```
assets-and-models/
└── models/
    └── llm/
        └── qwen2.5-14b-instruct-q4_k_m.gguf  (ou nome similar)
```

## Modelos Alternativos (Fallback)

- **Llama 3.1 13B Q4_K_M**: Alternativa robusta
- **Phi-3.5 Medium (14B)**: Opção mais leve

## Configuração

O caminho do modelo é configurado em:
- `config/vrpg.json` → `services.llm_core.model_path`
- `config/services.json` → `llm.model`

Certifique-se de que o caminho relativo está correto a partir da raiz do projeto `vrpg-client/`.


