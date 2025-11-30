#!/usr/bin/env python3
"""
Script para inserir TODOS os chunks no Vectorizer
Lê chunks_for_vectorizer.json e prepara para inserção via MCP
"""

import json
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer.json")
COLLECTION_NAME = "dnd5e-rules"

def main():
    print("Carregando chunks...")
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    chunks = data["chunks"]
    total = len(chunks)
    
    print(f"Total de chunks: {total}")
    print(f"Collection: {COLLECTION_NAME}")
    print()
    print("Chunks prontos para insercao via MCP")
    print(f"Use mcp_vectorizer-main_insert_text para cada chunk")
    print()
    
    # Mostrar estatísticas
    by_type = {}
    for chunk in chunks:
        doc_type = chunk["metadata"]["document_type"]
        by_type[doc_type] = by_type.get(doc_type, 0) + 1
    
    print("Chunks por tipo de documento:")
    for doc_type, count in by_type.items():
        print(f"  - {doc_type}: {count} chunks")
    
    print()
    print(f"Total: {total} chunks")

if __name__ == "__main__":
    main()

