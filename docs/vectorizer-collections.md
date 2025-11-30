# Collections do Vectorizer - D&D 5e

## Visão Geral

O sistema de indexação de livros D&D 5e utiliza **collections separadas** para cada livro, permitindo busca semântica específica por fonte.

## Collections Criadas

### 1. Manual dos Monstros
- **Collection**: `dnd5e-manual-dos-monstros`
- **Chunks**: 647
- **Tipo**: `monster_manual`
- **Fonte**: `old-dd-5e-manual-dos-monstros-biblioteca-elfica.pdf`

### 2. Livro do Jogador
- **Collection**: `dnd5e-livro-do-jogador`
- **Chunks**: 571
- **Tipo**: `player_handbook`
- **Fonte**: `dd-5e-livro-do-jogador-fundo-branco-biblioteca-elfica.pdf`

### 3. Guia do Mestre
- **Collection**: `dnd5e-guia-do-mestre`
- **Chunks**: 566
- **Tipo**: `dungeon_master_guide`
- **Fonte**: `dd-5e-guia-do-mestre-biblioteca-elfica.pdf`

### 4. Ficha de Personagem
- **Collection**: `dnd5e-ficha-de-personagem`
- **Chunks**: 1
- **Tipo**: `character_sheet`
- **Fonte**: `dd-5e-ficha-de-personagem-completavel-biblioteca-elfica.pdf`

### 5. Guia de Xanathar
- **Collection**: `dnd5e-guia-de-xanathar-para-todas-as-coisas`
- **Chunks**: 347
- **Tipo**: `supplement`
- **Fonte**: `dd-5e-guia-de-xanathar-para-todas-as-coisas-fundo-branco-biblioteca-elfica.pdf`

### 6. Guia do Volo para Monstros
- **Collection**: `dnd5e-guia-do-volo-para-monstros`
- **Chunks**: 436
- **Tipo**: `monster_manual`
- **Fonte**: `dd-5e-guia-do-volo-para-monstros-v-alta-resolucao-biblioteca-elfica.pdf`

## Total

- **Total de Collections**: 6
- **Total de Chunks**: 2568
- **Dimensão dos Vetores**: 512
- **Métrica**: cosine

## Scripts de Gerenciamento

### Criar Collections (sem indexar)

```powershell
cd G:\vrpg\vrpg-client
python scripts/create-collections-only.py
```

Este script:
1. Carrega chunks do arquivo JSON para identificar os livros
2. Agrupa chunks por livro usando metadata
3. Cria uma collection para cada livro (sem indexar chunks)

### Criar e Indexar Collections

```powershell
cd G:\vrpg\vrpg-client
python scripts/create-collections-per-book.py
```

Este script:
1. Carrega todos os chunks do arquivo JSON
2. Agrupa chunks por livro usando metadata
3. Cria uma collection para cada livro (se não existir)
4. Indexa todos os chunks em suas respectivas collections

### Verificar Status

```powershell
cd G:\vrpg\vrpg-client
python scripts/check-collections-status.py
```

Este script mostra:
- Status de cada collection
- Número de chunks esperados vs. indexados
- Progresso de indexação
- Resumo geral

## Busca em Collections

### Inserir Chunks em Collection

```python
import requests
import uuid

collection = "dnd5e-manual-dos-monstros"
chunk_id = str(uuid.uuid4())

payload = {
    "vectors": [{
        "id": chunk_id,
        "text": "Texto do chunk...",
        "metadata": {
            "source": "manual-dos-monstros",
            "page": 1
        }
    }]
}

response = requests.post(
    f"http://127.0.0.1:15002/collections/{collection}/vectors",
    json=payload
)
```

### Buscar em Collection Específica

```python
import requests

collection = "dnd5e-manual-dos-monstros"
query = "dragão vermelho"

response = requests.post(
    f"http://127.0.0.1:15002/collections/{collection}/search",
    json={
        "query": query,
        "limit": 10
    }
)
```

### Buscar em Múltiplas Collections

Para buscar em múltiplas collections, faça buscas separadas e combine os resultados:

```python
collections = [
    "dnd5e-manual-dos-monstros",
    "dnd5e-guia-do-volo-para-monstros"
]

results = []
for collection in collections:
    response = requests.post(
        f"http://127.0.0.1:15002/collections/{collection}/search",
        json={"query": query, "limit": 5}
    )
    results.extend(response.json())
```

## Estrutura dos Chunks

Cada chunk indexado contém:

```json
{
  "text": "Conteúdo do chunk...",
  "metadata": {
    "game_system": "dnd5e",
    "language": "pt-BR",
    "source": "biblioteca_elfica",
    "source_file": "nome-do-arquivo.pdf",
    "document_type": "monster_manual",
    "title": "D&D 5e - Manual dos Monstros",
    "chunk_index": 0,
    "total_chunks": 647,
    "categories": ["dnd5e", "monsters", "creatures", "bestiary"],
    "confidence": 0.95
  }
}
```

## Vantagens das Collections Separadas

1. **Busca Específica**: Permite buscar apenas em livros específicos
2. **Performance**: Buscas menores são mais rápidas
3. **Organização**: Dados organizados por fonte
4. **Manutenção**: Fácil adicionar/remover livros específicos
5. **Escalabilidade**: Cada collection pode ser gerenciada independentemente

## Troubleshooting

### Collection não encontrada após restart

Se as collections não aparecerem após reiniciar o container:

1. **Verificar se os volumes estão montados**:
   ```powershell
   docker volume ls | Select-String -Pattern "vectorizer"
   ```

2. **Verificar se o container está usando os volumes**:
   ```powershell
   docker inspect vectorizer --format='{{json .Mounts}}' | ConvertFrom-Json
   ```

3. **Recriar collections se necessário**:
   ```powershell
   cd G:\vrpg\vrpg-client
   python scripts/create-collections-per-book.py
   ```

### Verificar se collection existe

```powershell
# Verificar se a collection existe
Invoke-WebRequest -Uri "http://127.0.0.1:15002/collections/dnd5e-manual-dos-monstros" -UseBasicParsing
```

### Reindexar um livro específico

1. Identifique a collection do livro
2. Delete a collection (se necessário) via API ou script
3. Execute o script `create-collections-per-book.py` novamente

### Verificar progresso de indexação

```powershell
cd G:\vrpg\vrpg-client
python scripts/check-collections-status.py
```

### Refresh completo do container

Para fazer refresh mantendo os dados:

```powershell
cd G:\vrpg\vectorizer-feature-native-engine-optimization
docker-compose down
docker-compose pull vectorizer
docker-compose up -d --force-recreate
```

**Nota**: Os volumes nomeados são preservados, então as collections não serão perdidas.

## Referências

- [Vectorizer Setup](./vectorizer-setup.md)
- [Vectorizer Persistence](./vectorizer-persistence.md)

