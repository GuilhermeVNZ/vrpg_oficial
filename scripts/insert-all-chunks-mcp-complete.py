#!/usr/bin/env python3
"""
Script para inserir TODOS os chunks via MCP
Processa chunks_for_insertion.json e insere todos via MCP Vectorizer
"""

import json
import sys
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_insertion.json")
COLLECTION_NAME = "dnd5e-rules-new"
START_FROM_INDEX = 18  # Já inserimos 18 chunks

def main():
    print("=" * 70)
    print("INSERCAO COMPLETA DE CHUNKS VIA MCP")
    print("=" * 70)
    print()
    
    if not CHUNKS_FILE.exists():
        print(f"ERRO: Arquivo nao encontrado: {CHUNKS_FILE}")
        sys.exit(1)
    
    # Carregar chunks
    print(f"Carregando chunks de: {CHUNKS_FILE.name}")
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    collection_name = data['collection_name']
    all_chunks = data['chunks']
    total_chunks = len(all_chunks)
    
    # Pular chunks já inseridos
    chunks_to_insert = all_chunks[START_FROM_INDEX:]
    remaining_count = len(chunks_to_insert)
    
    print(f"Collection: {collection_name}")
    print(f"Total de chunks no arquivo: {total_chunks}")
    print(f"Chunks ja inseridos: {START_FROM_INDEX}")
    print(f"Chunks restantes: {remaining_count}")
    print()
    print("NOTA: Este script prepara os dados para insercao via MCP.")
    print("A insercao real deve ser feita via mcp_vectorizer-main_insert_text")
    print()
    print("Para cada chunk, execute:")
    print(f"  mcp_vectorizer-main_insert_text")
    print(f"    - collection_name: {collection_name}")
    print("    - text: chunk['text']")
    print("    - metadata: chunk['metadata']")
    print()
    
    # Salvar chunks restantes em arquivo separado para facilitar processamento
    remaining_file = CHUNKS_FILE.parent / "chunks_remaining.json"
    with open(remaining_file, 'w', encoding='utf-8') as f:
        json.dump({
            "collection_name": collection_name,
            "start_index": START_FROM_INDEX,
            "total_chunks": total_chunks,
            "remaining_count": remaining_count,
            "chunks": chunks_to_insert
        }, f, indent=2, ensure_ascii=False)
    
    print(f"Chunks restantes salvos em: {remaining_file.name}")
    print()
    print(f"Total de chunks para inserir: {remaining_count}")
    print("Execute a insercao via MCP para cada chunk listado acima.")

if __name__ == "__main__":
    main()

