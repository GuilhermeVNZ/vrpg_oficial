# Configuração dos Servidores MCP

## Status dos Servidores

✅ **Vectorizer**: Online na porta 15002  
✅ **Synap**: Online na porta 15500  
✅ **Context7**: Configurado (requer API key opcional)

## Arquivos de Configuração

O arquivo `mcp_servers.json` já está configurado com os três servidores MCP:

```json
{
  "mcpServers": {
    "synap": {
      "command": "python",
      "args": ["G:\\vrpg\\vrpg-client\\synap-mcp-simple.py"],
      "cwd": "G:\\vrpg\\vrpg-client"
    },
    "vectorizer": {
      "command": "python",
      "args": ["G:\\vrpg\\vrpg-client\\vectorizer-mcp.py"],
      "cwd": "G:\\vrpg\\vrpg-client"
    },
    "context7": {
      "command": "python",
      "args": ["G:\\vrpg\\vrpg-client\\context7-mcp.py"],
      "cwd": "G:\\vrpg\\vrpg-client",
      "env": {
        "CONTEXT7_API_KEY": "${CONTEXT7_API_KEY}"
      }
    }
  }
}
```

## Como Configurar no Cursor

### Opção 1: Usar o arquivo local (Recomendado)

1. O arquivo `mcp_servers.json` está em `G:\vrpg\vrpg-client\mcp_servers.json`
2. Copie o conteúdo para o arquivo de configuração do Cursor:
   - **Windows**: `%APPDATA%\Cursor\User\globalStorage\rooveterinaryinc.roo-cline\settings\cline_mcp_settings.json`
   - Ou use: `Settings` → `Cursor Settings` → `MCP` → `Add new global MCP server`

### Opção 2: Configuração manual no Cursor

1. Abra o Cursor
2. Vá em `Settings` → `Cursor Settings` → `MCP`
3. Clique em `Add new global MCP server`
4. Adicione cada servidor:

#### Synap
- **Name**: `synap`
- **Command**: `python`
- **Args**: `G:\vrpg\vrpg-client\synap-mcp-simple.py`
- **Working Directory**: `G:\vrpg\vrpg-client`

#### Vectorizer
- **Name**: `vectorizer`
- **Command**: `python`
- **Args**: `G:\vrpg\vrpg-client\vectorizer-mcp.py`
- **Working Directory**: `G:\vrpg\vrpg-client`

#### Context7
- **Name**: `context7`
- **Command**: `python`
- **Args**: `G:\vrpg\vrpg-client\context7-mcp.py`
- **Working Directory**: `G:\vrpg\vrpg-client`
- **Environment Variables**: `CONTEXT7_API_KEY` (opcional)

## Verificação

Para verificar se os servidores estão funcionando:

1. **Vectorizer**:
   ```powershell
   Invoke-WebRequest -Uri "http://127.0.0.1:15002/health" -UseBasicParsing
   ```

2. **Synap**:
   ```powershell
   Invoke-WebRequest -Uri "http://127.0.0.1:15500/health" -UseBasicParsing
   ```

## Dependências

Certifique-se de ter instalado:

```bash
pip install -r requirements-mcp.txt
```

O arquivo `requirements-mcp.txt` contém:
- `mcp>=1.0.0`
- `aiohttp>=3.9.0`

## Reiniciar o Cursor

Após configurar, **reinicie o Cursor** para que as mudanças tenham efeito.

## Troubleshooting

### Se os MCPs não aparecerem:

1. Verifique se os containers Docker estão rodando:
   - Vectorizer na porta 15002
   - Synap na porta 15500

2. Verifique se Python está no PATH:
   ```powershell
   python --version
   ```

3. Verifique se aiohttp está instalado:
   ```powershell
   python -c "import aiohttp; print('OK')"
   ```

4. Verifique os logs do Cursor:
   - Abra o painel de desenvolvedor (Ctrl+Shift+I)
   - Procure por erros relacionados a MCP

### Erros comuns:

- **"Connection refused"**: Verifique se os servidores Docker estão rodando
- **"Module not found"**: Execute `pip install -r requirements-mcp.txt`
- **"Command not found"**: Verifique se Python está no PATH do sistema

## Ferramentas Disponíveis

### Vectorizer
- `vectorizer_search_vectors` - Busca semântica
- `vectorizer_intelligent_search` - Busca inteligente
- `vectorizer_list_collections` - Listar coleções
- `vectorizer_insert_texts` - Inserir textos
- `vectorizer_get_collection_info` - Info da coleção
- `vectorizer_health_check` - Health check

### Synap
- `synap_kv_get` - Obter valor do KV store
- `synap_kv_set` - Armazenar valor no KV store
- `synap_queue_publish` - Publicar mensagem na fila

### Context7
- `context7_resolve_library_id` - Resolver ID de biblioteca
- `context7_get_library_docs` - Obter documentação






























