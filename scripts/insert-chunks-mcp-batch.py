#!/usr/bin/env python3
"""
Script para inserir chunks via MCP em lotes
Lê chunks_for_insertion.json e gera instruções para inserção via MCP
"""

import json
import sys
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_insertion.json")
BATCH_SIZE = 10

def main():
    print("Carregando chunks...")
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    collection_name = data['collection_name']
    chunks = data['chunks']
    total = len(chunks)
    
    print(f"Collection: {collection_name}")
    print(f"Total: {total} chunks")
    print(f"Lote: {BATCH_SIZE} chunks")
    print()
    print("NOTA: Este script prepara os dados.")
    print("A insercao real sera feita via MCP Vectorizer.")
    print()
    
    # Dividir em lotes
    batches = []
    for i in range(0, total, BATCH_SIZE):
        batch = chunks[i:i+BATCH_SIZE]
        batches.append({
            "batch_num": len(batches) + 1,
            "start": i + 1,
            "end": min(i + BATCH_SIZE, total),
            "chunks": batch
        })
    
    print(f"Total de lotes: {len(batches)}")
    print()
    print("Pronto para insercao via MCP!")
    print()

if __name__ == "__main__":
    main()

