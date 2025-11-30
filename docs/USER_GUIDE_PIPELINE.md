# Guia de Usuário - Pipeline de 3 Agentes

## Visão Geral

O VRPG Client utiliza uma arquitetura de **Pipeline de 3 Agentes** para garantir respostas rápidas e de alta qualidade durante o jogo.

### O que é o Pipeline de 3 Agentes?

O sistema divide o processamento em três componentes principais:

1. **Orquestrador**: Lógica determinística que coordena tudo
2. **Qwen-1.5B**: Modelo rápido para reações imediatas (< 1.2s)
3. **Qwen-14B**: Modelo completo para narrativa detalhada (< 6s)

## Como Funciona

### Fluxo Normal de Interação

```
Você fala → ASR → Orquestrador classifica → 
  ├─ Pergunta objetiva? → Resposta direta (sem LLM)
  ├─ Regra simples? → Vectorizer + 1.5B
  └─ Ação narrativa? → 1.5B (reação) → 14B (narrativa completa)
```

### Exemplos Práticos

#### Pergunta Objetiva (Resposta Instantânea)
- **Você**: "Quantos HP eu tenho?"
- **Sistema**: Responde diretamente do estado do jogo (< 50ms)
- **Sem uso de LLM**: Resposta determinística e instantânea

#### Regra Simples (Resposta Rápida)
- **Você**: "Stealth usa Destreza?"
- **Sistema**: Consulta Vectorizer + converte com 1.5B (< 1.5s)
- **Sem uso do 14B**: Resposta rápida e técnica

#### Ação Narrativa (Resposta Completa)
- **Você**: "Eu quero atacar o goblin com minha espada"
- **1.5B** (reação imediata, < 1.2s): "Você empunha a espada, pronta para o golpe."
- **14B** (narrativa completa, < 6s): "Sua espada corta o ar, encontrando o goblin que tenta desviar..."

## Configuração dos Modelos

### Requisitos de Hardware

- **Mínimo**: 16GB RAM, GPU com 8GB VRAM
- **Recomendado**: 32GB RAM, GPU com 16GB+ VRAM
- **Ideal**: 64GB RAM, GPU com 24GB+ VRAM (para carregar ambos modelos simultaneamente)

### Arquivos de Modelo Necessários

Você precisa ter os seguintes modelos GGUF:

1. **Qwen-1.5B**: `qwen2.5-1.5b-instruct-q4_k_m.gguf`
2. **Qwen-14B**: `qwen2.5-14b-instruct-q4_k_m.gguf`

### Localização dos Modelos

Coloque os modelos na pasta:
```
vrpg-client/assets-and-models/models/llm/
```

### Configuração em `config/llm_config.json`

```json
{
  "models": {
    "1_5b": {
      "path": "assets-and-models/models/llm/qwen2.5-1.5b-instruct-q4_k_m.gguf",
      "max_tokens": 40,
      "temperature": 0.8,
      "top_p": 0.9
    },
    "14b": {
      "path": "assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf",
      "max_tokens": 512,
      "temperature": 0.7,
      "top_p": 0.95
    }
  },
  "memory": {
    "keep_both_loaded": true,
    "preload_on_startup": true
  }
}
```

## Troubleshooting

### Problema: "Modelo 1.5B não encontrado"

**Solução**:
1. Verifique se o arquivo do modelo está no caminho correto
2. Verifique se o nome do arquivo corresponde ao configurado
3. Verifique permissões de leitura do arquivo

### Problema: Latência alta nas respostas

**Solução**:
1. Verifique se ambos os modelos estão carregados na GPU (não CPU)
2. Verifique uso de memória RAM/VRAM
3. Reduza a quantidade de contexto usado
4. Verifique se há outros processos usando GPU

### Problema: "Pipeline state error"

**Solução**:
1. Verifique se os serviços estão rodando (ASR, LLM Core, TTS)
2. Verifique logs em `logs/orchestrator.log`
3. Reinicie os serviços

### Problema: Respostas objetivas demoram muito

**Solução**:
- Respostas objetivas não deveriam demorar. Se estiver demorando:
  1. Verifique se o Intent Router está funcionando
  2. Verifique se o Game State Cache está sendo usado
  3. Veja logs para identificar onde está o gargalo

## Dicas de Uso

### Para Melhor Performance

1. **Mantenha ambos modelos carregados**: Configure `keep_both_loaded: true`
2. **Use respostas objetivas**: Faça perguntas diretas sobre estado do jogo
3. **Evite interrupções**: Deixe o sistema processar completamente antes de falar novamente

### Para Melhor Qualidade Narrativa

1. **Seja específico**: Quanto mais específico, melhor a resposta do 14B
2. **Dê contexto**: Mencione elementos da cena para melhor contexto
3. **Use ações claras**: Ações claras resultam em narrativas mais ricas

## Latências Esperadas

| Tipo de Resposta | Latência Esperada | Modelo Usado |
|-----------------|-------------------|--------------|
| Pergunta Objetiva | < 50ms | Nenhum (Orquestrador) |
| Regra Simples | < 1.5s | Vectorizer + 1.5B |
| Reação Inicial | < 1.2s | 1.5B |
| Narrativa Completa | < 6s | 1.5B + 14B |

## Migração de Versões Antigas

Se você estava usando uma versão anterior do VRPG Client:

1. **Backup de dados**: Faça backup da pasta `saves/` se existir
2. **Modelos**: Certifique-se de ter ambos modelos (1.5B e 14B)
3. **Configuração**: Atualize `config/llm_config.json` com a nova estrutura
4. **Sessões antigas**: Sessões antigas serão migradas automaticamente quando carregadas

## Suporte

Para mais informações, consulte:
- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura técnica completa
- [ORCHESTRATOR.md](ORCHESTRATOR.md) - Detalhes do orquestrador
- [Troubleshooting](#troubleshooting) - Seção acima













