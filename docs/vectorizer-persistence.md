# Persistência do Vectorizer no Docker

## Configuração Atual

O Vectorizer está configurado com persistência através de **volumes nomeados do Docker** no arquivo `docker-compose.yml`:

```yaml
volumes:
  # Persistent data - using named volumes
  - vectorizer-data:/vectorizer/data
  - vectorizer-storage:/vectorizer/storage
  - vectorizer-snapshots:/vectorizer/snapshots
  - vectorizer-dashboard:/vectorizer/dashboard
```

Os volumes nomeados são definidos na seção `volumes`:

```yaml
volumes:
  vectorizer-data:
    driver: local
  vectorizer-storage:
    driver: local
  vectorizer-snapshots:
    driver: local
  vectorizer-dashboard:
    driver: local
```

## Variáveis de Ambiente

A variável `DATA_DIR` foi adicionada para garantir que o Vectorizer use o diretório correto:

```yaml
environment:
  - DATA_DIR=/vectorizer/data
```

## Localização dos Dados

Os dados persistidos estão em:
- **Volumes Docker**: Gerenciados pelo Docker (aparecem na interface de volumes)
  - `vectorizer-feature-native-engine-optimization_vectorizer-data`
  - `vectorizer-feature-native-engine-optimization_vectorizer-storage`
  - `vectorizer-feature-native-engine-optimization_vectorizer-snapshots`
  - `vectorizer-feature-native-engine-optimization_vectorizer-dashboard`
- **Container**: `/vectorizer/data/`, `/vectorizer/storage/`, etc.

## Arquivos de Persistência

O Vectorizer salva os dados em:
- `vectorizer.vecdb` - Banco de dados principal com vetores e collections
- `vectorizer.vecidx` - Índice dos dados
- `snapshots/` - Snapshots automáticos para backup

## Verificação

Para verificar se a persistência está funcionando:

1. **Verificar se os volumes foram criados:**
   ```powershell
   docker volume ls | Select-String -Pattern "vectorizer"
   ```
   
   Deve mostrar os 4 volumes:
   - `vectorizer-feature-native-engine-optimization_vectorizer-data`
   - `vectorizer-feature-native-engine-optimization_vectorizer-storage`
   - `vectorizer-feature-native-engine-optimization_vectorizer-snapshots`
   - `vectorizer-feature-native-engine-optimization_vectorizer-dashboard`

2. **Verificar se os volumes estão montados no container:**
   ```powershell
   docker inspect vectorizer --format='{{json .Mounts}}' | ConvertFrom-Json | Where-Object {$_.Type -eq 'volume'}
   ```

3. **Reiniciar o container:**
   ```powershell
   cd G:\vrpg\vectorizer-feature-native-engine-optimization
   docker-compose restart vectorizer
   ```

4. **Verificar se as collections foram preservadas:**
   ```powershell
   cd G:\vrpg\vrpg-client
   python scripts/check-index-status.py
   ```

## Notas Importantes

- ✅ **Volumes Nomeados**: Usamos volumes nomeados do Docker em vez de bind mounts. Isso permite que os volumes sejam gerenciados pelo Docker e apareçam na interface de gerenciamento de volumes.

- ⚠️ **Carregamento Automático**: O Vectorizer pode não carregar automaticamente as collections persistidas na inicialização. Pode ser necessário recriar as collections ou usar um comando específico para restaurar.

- 📦 **Backup**: Os snapshots automáticos estão no volume `vectorizer-snapshots` e podem ser usados para restaurar dados.

- 🔄 **Reinicialização**: Após reiniciar o container, as collections podem precisar ser recriadas se não forem carregadas automaticamente.

- 🗑️ **Remoção de Volumes**: Para remover completamente os dados, é necessário remover os volumes:
  ```powershell
  docker-compose down -v  # Remove containers e volumes
  ```

## Próximos Passos

Se as collections não forem carregadas automaticamente, pode ser necessário:
1. Verificar se há um comando de restore/load no Vectorizer
2. Implementar um script de inicialização que carrega as collections
3. Verificar a documentação do Vectorizer sobre persistência e restauração

## Refresh Completo do Container

Para fazer um refresh completo do container mantendo os volumes:

```powershell
cd G:\vrpg\vectorizer-feature-native-engine-optimization

# Parar e remover container
docker-compose down

# Atualizar imagem (opcional)
docker-compose pull vectorizer

# Recriar com todas as alterações
docker-compose up -d --force-recreate
```

**Importante**: Os volumes nomeados são preservados, então todas as collections e dados indexados serão mantidos após o refresh.

