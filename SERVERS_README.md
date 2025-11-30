# VRPG Servers Launcher

Este executável (`servers.exe`) permite iniciar, parar e verificar o status dos servidores Vectorizer e Synap necessários para o projeto VRPG.

## 🚀 Como usar

### Iniciar os servidores
```bash
.\servers.exe
```
- **Lógica inteligente**: Verifica automaticamente quais servidores já estão rodando
- **Inicia apenas os necessários**: 
  - Se ambos estão parados → inicia ambos
  - Se apenas um está rodando → inicia apenas o que falta
  - Se ambos estão rodando → mostra status e não faz nada
- Cada servidor é iniciado em terminal separado para facilitar debug
- Configura automaticamente as variáveis de ambiente necessárias (CMAKE, NASM)

### Verificar status dos servidores
```bash
.\servers.exe --status
```
- Verifica se os servidores estão rodando nas portas corretas
- Mostra status individual de cada servidor

### Parar todos os servidores
```bash
.\servers.exe --stop
```
- Para todos os processos relacionados aos servidores
- Inclui processos do Cargo que podem estar compilando

### Ajuda
```bash
.\servers.exe --help
```
- Mostra todas as opções disponíveis

## 📡 Servidores

### Synap (Porta 15500)
- **Função**: Sistema de conversação entre modelos, KV store, filas
- **Endpoint**: http://127.0.0.1:15500
- **Compilação**: Rápida (~1-2 minutos)
- **Status**: ✅ Funcionando perfeitamente

### Vectorizer (Porta 15002)
- **Função**: Banco de dados vetorial para busca semântica
- **Endpoint**: http://127.0.0.1:15002
- **Compilação**: Lenta (primeira vez pode demorar 5-10 minutos)
- **Status**: ⚠️ Requer dependências (cmake, NASM)

## ⚠️ Notas importantes

1. **Primeira execução**: A primeira compilação do Vectorizer pode demorar vários minutos
2. **Dependências**: O Vectorizer requer cmake e NASM instalados no sistema
3. **Terminais**: Os servidores são executados em terminais separados para facilitar o debug
4. **Logs**: Cada servidor mostra seus logs no próprio terminal

## 🔧 Arquivos incluídos

- `servers.exe` - Executável principal
- `servers.bat` - Script batch (usado internamente pelo .exe)
- `servers.ps1` - Script PowerShell (alternativo)
- `ServersLauncher.cs` - Código fonte do executável

## 🐛 Solução de problemas

### Vectorizer não compila
- Instale cmake: `winget install Kitware.CMake`
- Instale NASM: `winget install NASM.NASM`
- Ou baixe NASM manualmente e adicione ao PATH

### Portas ocupadas
- Use `.\servers.exe --stop` para parar todos os servidores
- Verifique se não há outros processos usando as portas 15002 e 15500

### Permissões do PowerShell
- Se houver erro de execução, execute: `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser`

## 🔌 Configuração MCP

Os servidores são automaticamente detectados pelo Cursor como serviços MCP quando estão rodando nas portas corretas:

- **Vectorizer**: `http://127.0.0.1:15002` (MCP nativo)
- **Synap**: `http://127.0.0.1:15500` (via bridge MCP)

### Synap MCP Bridge

O Synap requer um bridge MCP para integração com Cursor. O bridge está disponível em:
- **Script**: `synap-mcp-simple.py`
- **Executável**: `synap-mcp-server.bat`

Para configurar manualmente no Cursor, adicione ao arquivo de configuração MCP:

```json
{
  "mcpServers": {
    "synap": {
      "command": "G:\\vrpg\\vrpg-client\\synap-mcp-server.bat",
      "args": []
    }
  }
}
```

### Ferramentas MCP Disponíveis

**Synap** (3 ferramentas):
- `synap_kv_get` - Recuperar valores do KV store
- `synap_kv_set` - Armazenar valores no KV store  
- `synap_queue_publish` - Publicar mensagens em filas

**Vectorizer** (20+ ferramentas):
- Busca semântica e vetorial
- Gerenciamento de coleções
- Indexação de documentos

## 📋 Status atual

✅ **Synap**: Funcionando perfeitamente  
⚠️ **Vectorizer**: Compilação pendente (dependências)  
✅ **Launcher**: Funcionando perfeitamente  
✅ **MCP Bridge**: Synap integrado via bridge Python  

---

**Desenvolvido para o projeto VRPG Client**  
*Sistema modular de RPG virtual com IA local*
