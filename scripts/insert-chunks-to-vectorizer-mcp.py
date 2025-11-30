#!/usr/bin/env python3
"""
Script para inserir chunks no Vectorizer via MCP
Lê o arquivo chunks_for_vectorizer.json e insere via MCP Vectorizer
"""

import json
import sys
from pathlib import Path

# Caminho do arquivo de chunks
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer.json")

def load_chunks():
    """Carrega os chunks do arquivo JSON"""
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    return data["collection"], data["chunks"]


def main():
    print("="*60)
    print("Inserindo chunks no Vectorizer via MCP")
    print("="*60)
    
    if not CHUNKS_FILE.exists():
        print(f"ERRO: Arquivo nao encontrado: {CHUNKS_FILE}")
        sys.exit(1)
    
    collection_name, chunks = load_chunks()
    
    print(f"Collection: {collection_name}")
    print(f"Total de chunks: {len(chunks)}")
    print()
    print("NOTA: Este script prepara os dados para insercao via MCP.")
    print("Os chunks serao inseridos usando mcp_vectorizer-main_insert_text")
    print()
    
    # Mostrar primeiros 3 chunks como exemplo
    print("Exemplo de chunks (primeiros 3):")
    for i, chunk in enumerate(chunks[:3], 1):
        print(f"\nChunk {i}:")
        print(f"  Texto (primeiros 100 chars): {chunk['text'][:100]}...")
        print(f"  Metadata: {json.dumps(chunk['metadata'], indent=4, ensure_ascii=False)}")
    
    print(f"\n... e mais {len(chunks) - 3} chunks")
    print()
    print("Pronto para insercao via MCP!")
    print(f"Collection: {collection_name}")
    print(f"Total: {len(chunks)} chunks")
    print()


if __name__ == "__main__":
    main()

