#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Cria uma collection separada para cada livro e indexa os chunks correspondentes
USANDO MCP - Este script deve ser executado através do Cursor IDE que tem acesso às ferramentas MCP
"""

import json
import sys
import uuid
from pathlib import Path
from typing import List, Dict
from collections import defaultdict
import re

# Configurar encoding UTF-8 para stdout
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')

# Configuração
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")
BATCH_SIZE = 50


def sanitize_collection_name(name: str) -> str:
    """Sanitiza o nome para ser usado como collection name"""
    name = name.replace(".pdf", "")
    name = re.sub(r'[^a-zA-Z0-9_-]', '-', name)
    name = re.sub(r'-+', '-', name)
    name = name.strip('-')
    if len(name) > 50:
        name = name[:50]
    return name.lower()


def get_collection_name_from_metadata(metadata: Dict) -> str:
    """Extrai o nome da collection do metadata do chunk"""
    title = metadata.get("title", "")
    if title:
        title = title.replace("D&D 5e - ", "").strip()
        collection_name = sanitize_collection_name(title)
        if collection_name:
            return f"dnd5e-{collection_name}"
    
    source_file = metadata.get("source_file", "")
    if source_file:
        collection_name = sanitize_collection_name(source_file)
        return f"dnd5e-{collection_name}"
    
    return "dnd5e-unknown"


def main():
    """Função principal - Este script prepara os dados para inserção via MCP"""
    print("="*70)
    print("Preparando dados para inserção via MCP")
    print("="*70)
    print()
    print("NOTA: Este script prepara os dados.")
    print("Para inserir via MCP, use as ferramentas MCP do Cursor:")
    print("  - mcp_vectorizer_vectorizer_insert_texts")
    print()
    print("Ou execute este script através do Cursor IDE que tem acesso às ferramentas MCP.")
    print()
    
    # Carregar chunks
    print(f"Carregando chunks de: {CHUNKS_FILE}")
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
    
    # Preparar dados para inserção
    print(f"\nPreparando dados para inserção via MCP...")
    for collection_name, chunks in sorted(chunks_by_book.items()):
        title = chunks[0]["metadata"].get("title", "Sem título")
        print(f"\n[{collection_name}] {title}")
        print(f"  Total de chunks: {len(chunks)}")
        print(f"  Lotes de {BATCH_SIZE}: {(len(chunks) + BATCH_SIZE - 1) // BATCH_SIZE}")
        
        # Preparar vetores para cada lote
        for i in range(0, len(chunks), BATCH_SIZE):
            batch = chunks[i:i + BATCH_SIZE]
            batch_num = (i // BATCH_SIZE) + 1
            
            vectors = []
            for chunk in batch:
                text = chunk["text"]
                metadata = chunk["metadata"]
                chunk_id = str(uuid.uuid4())
                
                vectors.append({
                    "id": chunk_id,
                    "text": text,
                    "metadata": metadata
                })
            
            print(f"  Lote {batch_num}: {len(vectors)} vetores prontos para inserção")
    
    print("\n" + "="*70)
    print("Para inserir via MCP, use:")
    print("  mcp_vectorizer_vectorizer_insert_texts")
    print("  com collection='nome_da_collection' e vectors=[...]")
    print("="*70)


if __name__ == "__main__":
    main()








