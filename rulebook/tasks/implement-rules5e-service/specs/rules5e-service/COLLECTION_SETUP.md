# Setup da Collection dnd5e-rules

## Status Atual

- **Collection antiga**: `dnd5e-rules` (dimensão 384) - **VAZIA** (0 vetores)
- **Collection temporária**: `dnd5e-rules-temp` (dimensão 512) - **CRIADA** ✅

## Problema

A collection `dnd5e-rules` foi criada com dimensão 384, mas o Vectorizer sempre deve usar vetores de 512 dimensões.

## Solução

### Opção 1: Deletar e Recriar (Recomendado)

Quando o Vectorizer estiver rodando, execute:

```powershell
# Deletar collection antiga via API REST
Invoke-WebRequest -Uri "http://localhost:8002/collections/dnd5e-rules" -Method DELETE

# Criar nova collection com dimensão 512 via MCP
# Use: mcp_vectorizer-main_create_collection
# - name: dnd5e-rules
# - dimension: 512
# - metric: cosine
```

Ou use o script automatizado:

```powershell
.\scripts\fix-dnd5e-collection.ps1
```

### Opção 2: Usar Collection Temporária

Por enquanto, você pode usar a collection `dnd5e-rules-temp` que já está criada com dimensão 512:

- **Nome**: `dnd5e-rules-temp`
- **Dimensão**: 512 ✅
- **Métrica**: cosine
- **Status**: Pronta para uso

Depois, quando deletar a antiga, pode renomear ou recriar com o nome correto.

## Verificação

Para verificar as collections disponíveis:

```powershell
# Via MCP
mcp_vectorizer-main_list_collections

# Via API REST
Invoke-WebRequest -Uri "http://localhost:8002/collections" -Method GET
```

## Inserção dos Chunks

Após corrigir a collection, você pode inserir os 1,908 chunks usando:

1. **Via MCP Vectorizer** (recomendado):
   ```python
   # Para cada chunk em rulebook/tasks/implement-rules5e-service/specs/rules5e-service/*.json
   mcp_vectorizer-main_insert_text
   - collection_name: dnd5e-rules (ou dnd5e-rules-temp)
   - text: chunk['text']
   - metadata: chunk['metadata']
   ```

2. **Via Script Batch**:
   - Criar script Python/Node.js para processar todos os chunks
   - Inserir em lotes para melhor performance

## Nota Importante

⚠️ **SEMPRE use dimensão 512 no Vectorizer** - Esta é a dimensão padrão do modelo de embedding usado.


