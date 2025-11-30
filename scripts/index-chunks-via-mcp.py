#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Indexa chunks nos Vectorizer usando MCP diretamente
Este script carrega os chunks e os prepara para inserção via MCP
"""

import json
import sys
import uuid
from pathlib import Path
from typing import List, Dict
from collections import defaultdict

# Configurar encoding UTF-8 para stdout
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')

# Configuração
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")
BATCH_SIZE = 50


def get_collection_name_from_metadata(metadata: Dict) -> str:
    """Extrai o nome da collection do metadata do chunk"""
    title = metadata.get("title", "")
    if title:
        title = title.replace("D&D 5e - ", "").strip()
        # Sanitizar nome
        import re
        name = re.sub(r'[^a-zA-Z0-9_-]', '-', title.lower())
        name = re.sub(r'-+', '-', name)
        name = name.strip('-')
        if len(name) > 50:
            name = name[:50]
        return f"dnd5e-{name}"
    
    source_file = metadata.get("source_file", "")
    if source_file:
        import re
        name = source_file.replace(".pdf", "").lower()
        name = re.sub(r'[^a-zA-Z0-9_-]', '-', name)
        name = re.sub(r'-+', '-', name)
        name = name.strip('-')
        if len(name) > 50:
            name = name[:50]
        return f"dnd5e-{name}"
    
    return "dnd5e-unknown"


def main():
    """Função principal"""
    print("="*70)
    print("Preparando Chunks para Inserção via MCP")
    print("="*70)
    
    # Carregar chunks
    print(f"\nCarregando chunks de: {CHUNKS_FILE}")
    if not CHUNKS_FILE.exists():
        print(f"[ERRO] Arquivo não encontrado: {CHUNKS_FILE}")
        return
    
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    all_chunks = data.get("chunks", [])
    total_chunks = len(all_chunks)
    print(f"[OK] Carregados {total_chunks} chunks")
    
    # Agrupar chunks por livro
    print(f"\nAgrupando chunks por livro...")
    chunks_by_book = defaultdict(list)
    for chunk in all_chunks:
        metadata = chunk.get("metadata", {})
        collection_name = get_collection_name_from_metadata(metadata)
        chunks_by_book[collection_name].append(chunk)
    
    print(f"[OK] Encontrados {len(chunks_by_book)} livros:")
    for collection_name, chunks in sorted(chunks_by_book.items()):
        title = chunks[0]["metadata"].get("title", "Sem título")
        print(f"  - {collection_name}: {len(chunks)} chunks ({title})")
    
    # Preparar vetores para inserção
    print(f"\nPreparando vetores para inserção via MCP...")
    print(f"Tamanho do lote: {BATCH_SIZE} chunks")
    print()
    print("Para inserir via MCP, use:")
    print("  mcp_vectorizer_vectorizer_insert_texts")
    print("  com:")
    print("    collection='nome_da_collection'")
    print("    vectors=[lista de vetores com id, text e metadata]")
    print()
    
    # Gerar comandos de exemplo para cada livro
    for collection_name, chunks in sorted(chunks_by_book.items()):
        title = chunks[0]["metadata"].get("title", "Sem título")
        print(f"\n[{collection_name}] {title}")
        print(f"  Total: {len(chunks)} chunks")
        print(f"  Lotes: {(len(chunks) + BATCH_SIZE - 1) // BATCH_SIZE}")
        
        # Preparar primeiro lote como exemplo
        first_batch = chunks[:BATCH_SIZE]
        vectors_example = []
        for chunk in first_batch:
            vectors_example.append({
                "id": str(uuid.uuid4()),
                "text": chunk["text"],
                "metadata": chunk["metadata"]
            })
        
        print(f"  Exemplo de primeiro lote ({len(vectors_example)} vetores):")
        print(f"    Collection: {collection_name}")
        print(f"    Vetores: {len(vectors_example)}")
    
    print("\n" + "="*70)
    print("NOTA: Use as ferramentas MCP do Cursor para inserir os chunks")
    print("="*70)


if __name__ == "__main__":
    main()








