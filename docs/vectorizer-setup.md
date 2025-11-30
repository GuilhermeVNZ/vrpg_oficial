# Configuração do Vectorizer

## Requisitos

- Docker e Docker Compose instalados
- Porta 15002 disponível

## Instalação e Configuração

### 1. Navegar para o diretório do Vectorizer

```powershell
cd G:\vrpg\vectorizer-feature-native-engine-optimization
```

### 2. Iniciar o Vectorizer

```powershell
docker-compose up -d
```

### 3. Verificar Status

```powershell
docker ps --filter "name=vectorizer"
docker-compose logs vectorizer
```

## Volumes e Persistência

O Vectorizer usa **volumes nomeados do Docker** para persistência de dados:

- `vectorizer-data`: Dados principais (collections, vetores)
- `vectorizer-storage`: Armazenamento adicional
- `vectorizer-snapshots`: Snapshots automáticos para backup
- `vectorizer-dashboard`: Dados do dashboard

### Verificar Volumes

```powershell
docker volume ls | Select-String -Pattern "vectorizer"
```

### Localização dos Volumes

Os volumes são gerenciados pelo Docker e armazenados em:
- **Windows (Docker Desktop)**: `\\wsl$\docker-desktop-data\data\docker\volumes\`
- **Linux**: `/var/lib/docker/volumes/`

## Variáveis de Ambiente

O Vectorizer está configurado com:

- `VECTORIZER_HOST=0.0.0.0`
- `VECTORIZER_PORT=15002`
- `DATA_DIR=/vectorizer/data`
- `TZ=America/Sao_Paulo`
- `RUN_MODE=production`

## Endpoints

- **REST API**: `http://localhost:15002`
- **MCP**: `http://localhost:15002` (mesma porta)
- **Health Check**: `http://localhost:15002/health`

## Comandos Úteis

### Parar o Vectorizer

```powershell
docker-compose stop vectorizer
```

### Reiniciar o Vectorizer

```powershell
docker-compose restart vectorizer
```

### Ver Logs

```powershell
docker-compose logs -f vectorizer
```

### Remover Container (mantém volumes)

```powershell
docker-compose down
```

### Remover Container e Volumes (⚠️ apaga todos os dados)

```powershell
docker-compose down -v
```

### Recriar Container e Volumes

```powershell
# Parar e remover container
docker-compose down

# Atualizar imagem (opcional)
docker-compose pull vectorizer

# Recriar container com todas as alterações
docker-compose up -d --force-recreate
```

**Nota**: Os volumes nomeados são preservados, então as collections e dados persistidos não serão perdidos.

## Indexação de Dados

### Indexar todos os livros (collections separadas)

Para criar collections separadas para cada livro e indexar:

```powershell
cd G:\vrpg\vrpg-client
python scripts/create-collections-per-book.py
```

### Indexar em uma única collection

Para indexar todos os livros em uma única collection:

```powershell
cd G:\vrpg\vrpg-client
python scripts/recreate-collection-and-index.py
```

## Verificação de Status

### Verificar status de todas as collections

```powershell
cd G:\vrpg\vrpg-client
python scripts/check-collections-status.py
```

### Verificar status de uma collection específica

```powershell
cd G:\vrpg\vrpg-client
python scripts/check-index-status.py
```

## Troubleshooting

### Container não inicia

1. Verificar logs: `docker-compose logs vectorizer`
2. Verificar se a porta 15002 está disponível
3. Verificar recursos do sistema (CPU/Memória)

### Container mostra "unhealthy"

O healthcheck pode falhar se `curl` não estiver disponível no container. O healthcheck foi configurado para usar `wget` como alternativa. Se ambos não estiverem disponíveis, você pode:

1. Verificar se o Vectorizer está realmente respondendo:
   ```powershell
   Invoke-WebRequest -Uri "http://localhost:15002/health" -UseBasicParsing
   ```

2. Se o Vectorizer responder, o problema é apenas o healthcheck. Você pode desabilitá-lo temporariamente removendo a seção `healthcheck` do `docker-compose.yml`.

### Timeout ao conectar

Se você receber timeouts ao tentar conectar:

1. Verificar se o container está rodando: `docker ps --filter "name=vectorizer"`
2. Reiniciar o container: `docker-compose restart vectorizer`
3. Aguardar alguns segundos após o restart antes de tentar novamente

### Dados não persistem

1. Verificar se os volumes foram criados: `docker volume ls`
2. Verificar se os volumes estão montados: `docker inspect vectorizer`
3. Verificar permissões dos volumes

### Collection não encontrada após restart

1. Verificar se os volumes existem
2. Recriar a collection usando o script de indexação
3. Verificar logs do Vectorizer para erros de carregamento

## Referências

- [Documentação de Persistência](./vectorizer-persistence.md)
- [API Reference](https://github.com/hivellm/vectorizer/docs)


