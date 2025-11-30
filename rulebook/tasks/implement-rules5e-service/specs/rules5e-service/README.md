# D&D 5e Rules - Vectorizer Collection

Este diretório contém os chunks processados dos 4 PDFs de D&D 5e que foram extraídos e preparados para inserção no Vectorizer.

## Documentos Processados

1. **Livro do Jogador** (`player_handbook`)
   - Arquivo: `dd-5e-livro-do-jogador-fundo-branco-biblioteca-elfica.pdf`
   - Páginas: 314
   - Chunks: 653
   - Tamanho: ~1.3 MB de texto

2. **Guia do Mestre** (`dungeon_master_guide`)
   - Arquivo: `dd-5e-guia-do-mestre-biblioteca-elfica.pdf`
   - Páginas: 318
   - Chunks: 616
   - Tamanho: ~1.2 MB de texto

3. **Manual dos Monstros** (`monster_manual`)
   - Arquivo: `old-dd-5e-manual-dos-monstros-biblioteca-elfica.pdf`
   - Páginas: 349
   - Chunks: 638
   - Tamanho: ~1.3 MB de texto

4. **Ficha de Personagem** (`character_sheet`)
   - Arquivo: `dd-5e-ficha-de-personagem-completavel-biblioteca-elfica.pdf`
   - Páginas: 3
   - Chunks: 1
   - Tamanho: ~1.7 KB de texto

## Estatísticas Totais

- **Total de documentos**: 4
- **Total de páginas**: 984
- **Total de chunks**: 1,908
- **Tamanho total**: ~3.8 MB de texto

## Estrutura dos Chunks

Cada arquivo JSON contém:
```json
{
    "text": "Conteúdo do chunk em Markdown",
    "metadata": {
        "source_file": "nome-do-arquivo.pdf",
        "document_type": "player_handbook|dungeon_master_guide|monster_manual|character_sheet",
        "title": "Título do documento",
        "chunk_index": 0,
        "total_chunks": 653,
        "game_system": "dnd5e",
        "language": "pt-BR"
    }
}
```

## Collection Vectorizer

- **Nome**: `dnd5e-rules`
- **Dimensão**: 384 (atual) ou 512 (se usar modelo diferente)
- **Métrica**: Cosine
- **Status**: Criada, aguardando inserção dos chunks

## Inserção no Vectorizer

Para inserir os chunks no Vectorizer, você pode:

1. **Via MCP Vectorizer** (recomendado):
   ```python
   # Use mcp_vectorizer-main_insert_text para cada chunk
   # Collection: dnd5e-rules
   # Text: chunk['text']
   # Metadata: chunk['metadata']
   ```

2. **Via API REST do Vectorizer**:
   ```bash
   curl -X POST http://localhost:8002/collections/dnd5e-rules/insert \
     -H "Content-Type: application/json" \
     -d @chunk_file.json
   ```

3. **Via Script Batch**:
   - Use o script `scripts/insert-dnd5e-chunks-mcp.py` como base
   - Processe os chunks em lotes para melhor performance

## Notas Importantes

- Os arquivos foram salvos **sem BOM UTF-8** para compatibilidade
- Cada chunk tem aproximadamente 2000 caracteres
- Os chunks mantêm a estrutura de páginas do PDF original
- A metadata inclui informações para filtragem e busca

## Próximos Passos

1. ✅ PDFs processados e convertidos para Markdown
2. ✅ Chunks criados e salvos em JSON
3. ✅ Collection criada no Vectorizer
4. ⏳ Inserir chunks no Vectorizer (via MCP ou API)
5. ⏳ Testar busca semântica na collection
6. ⏳ Integrar com o `rules5e-service` para consultas

## Scripts Disponíveis

- `scripts/process-dnd5e-pdfs-simple.ps1` - Processa PDFs e cria chunks
- `scripts/insert-dnd5e-chunks-mcp.py` - Prepara dados para inserção
- `scripts/insert-dnd5e-to-vectorizer.ps1` - Valida chunks (não usado)


