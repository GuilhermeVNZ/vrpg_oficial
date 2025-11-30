#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Separa os arquivos markdown por livro para indexação no workspace
"""

import json
from pathlib import Path

# Configuração
INPUT_DIR = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service")
CHUNKS_FILE = INPUT_DIR / "chunks_for_vectorizer_all_books.json"

# Mapeamento de tipos para nomes de arquivo
TYPE_TO_FILENAME = {
    "monster_manual": "monster_manual.md",
    "player_handbook": "player_handbook.md",
    "dungeon_master_guide": "dungeon_master_guide.md",
    "character_sheet": "character_sheet.md",
    "supplement": "supplement.md"
}

# Mapeamento de títulos para nomes de arquivo específicos
TITLE_TO_FILENAME = {
    "D&D 5e - Manual dos Monstros": "manual-dos-monstros.md",
    "D&D 5e - Livro do Jogador": "livro-do-jogador.md",
    "D&D 5e - Guia do Mestre": "guia-do-mestre.md",
    "D&D 5e - Ficha de Personagem": "ficha-de-personagem.md",
    "D&D 5e - Guia de Xanathar para Todas as Coisas": "guia-xanathar.md",
    "D&D 5e - Guia do Volo para Monstros": "guia-volo.md"
}


def main():
    """Separa chunks por livro e cria arquivos markdown individuais"""
    print("="*70)
    print("Separando chunks por livro para indexação no workspace")
    print("="*70)
    
    # Carregar chunks
    if not CHUNKS_FILE.exists():
        print(f"[ERRO] Arquivo não encontrado: {CHUNKS_FILE}")
        return
    
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    all_chunks = data.get("chunks", [])
    print(f"[OK] Carregados {len(all_chunks)} chunks")
    
    # Agrupar por livro
    from collections import defaultdict
    chunks_by_book = defaultdict(list)
    
    for chunk in all_chunks:
        metadata = chunk.get("metadata", {})
        title = metadata.get("title", "")
        chunks_by_book[title].append(chunk)
    
    print(f"[OK] Encontrados {len(chunks_by_book)} livros")
    
    # Criar arquivos markdown para cada livro
    for title, chunks in chunks_by_book.items():
        filename = TITLE_TO_FILENAME.get(title, f"{title.lower().replace(' ', '-')}.md")
        output_file = INPUT_DIR / filename
        
        print(f"\n[{title}]")
        print(f"  Chunks: {len(chunks)}")
        print(f"  Arquivo: {filename}")
        
        # Combinar todos os chunks em um único markdown
        markdown_content = []
        for i, chunk in enumerate(chunks):
            text = chunk.get("text", "")
            metadata = chunk.get("metadata", {})
            
            # Adicionar header com metadata
            markdown_content.append(f"## Chunk {i+1}\n")
            markdown_content.append(f"**Fonte:** {metadata.get('source_file', 'unknown')}\n")
            markdown_content.append(f"**Página:** {metadata.get('chunk_index', i)}\n\n")
            markdown_content.append(text)
            markdown_content.append("\n\n---\n\n")
        
        # Salvar arquivo
        output_file.write_text("".join(markdown_content), encoding='utf-8')
        print(f"  [OK] Arquivo criado: {output_file.name} ({len(''.join(markdown_content)):,} caracteres)")
    
    print("\n" + "="*70)
    print("[OK] Arquivos markdown criados para indexação no workspace")
    print("="*70)


if __name__ == "__main__":
    main()








