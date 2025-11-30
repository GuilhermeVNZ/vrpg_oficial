#!/usr/bin/env python3
"""
Script para inserir TODOS os chunks restantes
Gera um arquivo com instruções para inserção via MCP
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
    
    print(f"Total: {total} chunks")
    print(f"Collection: {COLLECTION_NAME}")
    print()
    print("Todos os chunks estao prontos para insercao via MCP")
    print(f"Use mcp_vectorizer-main_insert_text para cada chunk")
    print()

if __name__ == "__main__":
    main()

