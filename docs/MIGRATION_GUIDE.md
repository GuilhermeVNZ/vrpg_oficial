# Guia de Migração - Pipeline de 3 Agentes

## Para Usuários Existentes

Este guia ajuda você a migrar da versão anterior do VRPG Client para a nova arquitetura de Pipeline de 3 Agentes.

## O Que Mudou

### Antes (Arquitetura Antiga)
- **LLM único**: Apenas um modelo processava todas as respostas
- **Sem otimizações**: Todas as respostas passavam pelo mesmo modelo
- **Latência variável**: Dependia do tipo de pergunta

### Agora (Pipeline de 3 Agentes)
- **2 modelos LLM**: Qwen-1.5B (rápido) + Qwen-14B (completo)
- **Orquestrador inteligente**: Classifica e roteia para o melhor caminho
- **Latência otimizada**: Respostas objetivas instantâneas, narrativas em < 6s

## Passos de Migração

### 1. Backup de Dados

Antes de qualquer coisa, faça backup:

```bash
# Backup de sessões salvas (se existirem)
cp -r saves/ saves_backup/

# Backup de configuração
cp config/llm_config.json config/llm_config.json.backup
```

### 2. Atualizar Modelos

Você precisa ter **ambos** modelos:

1. **Baixe Qwen-1.5B**: `qwen2.5-1.5b-instruct-q4_k_m.gguf` (~1GB)
2. **Mantenha ou baixe Qwen-14B**: `qwen2.5-14b-instruct-q4_k_m.gguf` (~8GB)

Coloque ambos em:
```
assets-and-models/models/llm/
```

### 3. Atualizar Configuração

Atualize `config/llm_config.json`:

**Antes**:
```json
{
  "model": {
    "path": "assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf"
  }
}
```

**Agora**:
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

### 4. Migração de Sessões

Sessões antigas serão migradas automaticamente quando carregadas:

- **Formato antigo**: Será detectado e migrado para novo formato
- **Dados preservados**: Todos os dados serão preservados
- **Primeira carga**: Pode demorar um pouco mais (processo de migração)

### 5. Verificar Funcionamento

Após migrar, verifique:

```bash
# Iniciar LLM Core
cargo run --bin llm-core

# Você deve ver:
# [INFO] Loading model 1.5B...
# [INFO] Model 1.5B loaded successfully
# [INFO] Loading model 14B...
# [INFO] Model 14B loaded successfully
# [INFO] Both models loaded, ready for inference
```

### 6. Testar Pipeline

Execute testes para verificar:

```bash
cd src/orchestrator
cargo test --test pipeline_integration_test
cargo test --test pipeline_performance_test
```

Todos os testes devem passar.

## Mudanças de Comportamento

### O Que Esperar

1. **Respostas mais rápidas**: Perguntas objetivas são instantâneas
2. **Reações imediatas**: Você verá reações rápidas do 1.5B antes da narrativa completa
3. **Melhor qualidade**: 14B focando apenas em narrativas complexas

### O Que Não Mudou

1. **Compatibilidade**: Todas as funcionalidades anteriores continuam funcionando
2. **Dados**: Sessões e configurações são preservadas
3. **API**: APIs principais não mudaram

## Problemas Comuns na Migração

### "Modelo 1.5B não encontrado"

**Solução**: Baixe o modelo Qwen-1.5B e coloque no diretório correto.

### "Configuração inválida"

**Solução**: Use o novo formato de `llm_config.json` mostrado acima.

### "Sessão antiga não carrega"

**Solução**: Verifique logs para erros. Sessões muito antigas podem precisar de migração manual.

## Rollback (Se Necessário)

Se precisar voltar à versão anterior:

1. **Restaurar backup**:
   ```bash
   cp config/llm_config.json.backup config/llm_config.json
   ```

2. **Reverter código**: Use git para voltar ao commit anterior

3. **Restaurar sessões**:
   ```bash
   cp -r saves_backup/* saves/
   ```

## Suporte

Se encontrar problemas na migração:

1. Verifique logs em `logs/orchestrator.log`
2. Consulte [TROUBLESHOOTING_PIPELINE.md](TROUBLESHOOTING_PIPELINE.md)
3. Abra uma issue no GitHub com logs e detalhes













