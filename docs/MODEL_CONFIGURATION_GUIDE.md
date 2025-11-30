# Guia de Configuração dos Modelos LLM

## Visão Geral

O VRPG Client utiliza dois modelos LLM para o pipeline de 3 agentes:
- **Qwen-1.5B**: Modelo rápido para reações imediatas
- **Qwen-14B**: Modelo completo para narrativa detalhada

## Download dos Modelos

### Qwen-1.5B

**Arquivo**: `qwen2.5-1.5b-instruct-q4_k_m.gguf`

**Tamanho**: ~1GB

**Download**:
- Hugging Face: [Qwen/Qwen2.5-1.5B-Instruct-GGUF](https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF)
- Ou use: `huggingface-cli download Qwen/Qwen2.5-1.5B-Instruct-GGUF qwen2.5-1.5b-instruct-q4_k_m.gguf`

### Qwen-14B

**Arquivo**: `qwen2.5-14b-instruct-q4_k_m.gguf`

**Tamanho**: ~8GB

**Download**:
- Hugging Face: [Qwen/Qwen2.5-14B-Instruct-GGUF](https://huggingface.co/Qwen/Qwen2.5-14B-Instruct-GGUF)
- Ou use: `huggingface-cli download Qwen/Qwen2.5-14B-Instruct-GGUF qwen2.5-14b-instruct-q4_k_m.gguf`

## Instalação

### 1. Criar Diretório para Modelos

```bash
mkdir -p assets-and-models/models/llm
```

### 2. Baixar Modelos

Copie os arquivos `.gguf` baixados para:
```
assets-and-models/models/llm/
```

### 3. Verificar Estrutura

Sua estrutura deve ficar assim:
```
assets-and-models/models/llm/
├── qwen2.5-1.5b-instruct-q4_k_m.gguf
└── qwen2.5-14b-instruct-q4_k_m.gguf
```

## Configuração

### Arquivo `config/llm_config.json`

```json
{
  "models": {
    "1_5b": {
      "path": "assets-and-models/models/llm/qwen2.5-1.5b-instruct-q4_k_m.gguf",
      "max_tokens": 40,
      "temperature": 0.8,
      "top_p": 0.9,
      "context_size": 2048
    },
    "14b": {
      "path": "assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf",
      "max_tokens": 512,
      "temperature": 0.7,
      "top_p": 0.95,
      "context_size": 8192
    }
  },
  "memory": {
    "keep_both_loaded": true,
    "preload_on_startup": true,
    "gpu_layers": -1
  },
  "performance": {
    "threads": 8,
    "use_mlock": true,
    "use_mmap": true
  }
}
```

### Parâmetros Importantes

#### Qwen-1.5B
- **max_tokens**: 40 (mantenha baixo para latência)
- **temperature**: 0.8 (mais criativo, menos determinístico)
- **top_p**: 0.9 (diversidade controlada)

#### Qwen-14B
- **max_tokens**: 512 (narrativa mais longa)
- **temperature**: 0.7 (mais consistente)
- **top_p**: 0.95 (alta qualidade)

#### Memória
- **keep_both_loaded**: `true` para melhor latência (requer mais RAM/VRAM)
- **preload_on_startup**: `true` para evitar delays na primeira resposta
- **gpu_layers**: `-1` para carregar tudo na GPU (se disponível)

## Verificação

### Teste de Carregamento

Inicie o LLM Core:
```bash
cargo run --bin llm-core
```

Você deve ver nos logs:
```
[INFO] Loading model 1.5B from: assets-and-models/models/llm/qwen2.5-1.5b-instruct-q4_k_m.gguf
[INFO] Model 1.5B loaded successfully
[INFO] Loading model 14B from: assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf
[INFO] Model 14B loaded successfully
[INFO] Both models loaded, ready for inference
```

### Teste de Latência

Execute os testes de performance:
```bash
cd src/orchestrator
cargo test --test pipeline_performance_test
```

Você deve ver latências dentro dos targets:
- 1.5B: < 1.2s
- 14B: < 6s
- Respostas objetivas: < 50ms

## Troubleshooting

### Erro: "Model file not found"

1. Verifique o caminho no `config/llm_config.json`
2. Verifique se o arquivo existe no caminho especificado
3. Verifique permissões de leitura

### Erro: "Out of memory"

1. Reduza `gpu_layers` ou desative `keep_both_loaded`
2. Feche outros aplicativos usando GPU
3. Use quantização menor (q4_k_m é recomendado)

### Latência alta

1. Verifique se modelos estão na GPU (não CPU)
2. Aumente `threads` se usando CPU
3. Verifique se `use_mlock` e `use_mmap` estão habilitados

## Alternativas de Quantização

Se você tiver limitações de VRAM, pode usar quantizações menores:

- **q4_k_m** (recomendado): Balance entre qualidade e tamanho
- **q3_k_m**: Menor, qualidade levemente inferior
- **q5_k_m**: Maior, qualidade superior

Para 14B especialmente, q4_k_m é o melhor balance.













