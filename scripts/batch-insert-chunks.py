#!/usr/bin/env python3
"""
Script para inserir TODOS os chunks no Vectorizer
Lê o arquivo JSON e prepara para inserção batch via MCP
"""

import json
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer.json")
COLLECTION_NAME = "dnd5e-rules"

def main():
    print("Carregando chunks do arquivo JSON...")
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    chunks = data["chunks"]
    total = len(chunks)
    
    print(f"Total de chunks: {total}")
    print(f"Collection: {COLLECTION_NAME}")
    print()
    print("Preparando para insercao via MCP...")
    print()
    print("NOTA: Para inserir todos os chunks, use mcp_vectorizer-main_insert_text")
    print("para cada chunk no arquivo JSON.")
    print()
    print(f"Arquivo: {CHUNKS_FILE}")
    print(f"Total: {total} chunks prontos para insercao")

if __name__ == "__main__":
    main()
