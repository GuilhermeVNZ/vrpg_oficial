#!/usr/bin/env python3
"""
Script para processar e inserir TODOS os chunks via MCP
Lê chunks_for_vectorizer.json e prepara para inserção completa
"""

import json
import sys
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer.json")
COLLECTION_NAME = "dnd5e-rules"

def main():
    print("="*60)
    print("Processando chunks para insercao completa")
    print("="*60)
    
    if not CHUNKS_FILE.exists():
        print(f"ERRO: Arquivo nao encontrado: {CHUNKS_FILE}")
        sys.exit(1)
    
    print("Carregando chunks...")
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    chunks = data["chunks"]
    total = len(chunks)
    
    print(f"Total de chunks: {total}")
    print(f"Collection: {COLLECTION_NAME}")
    print()
    
    # Estatísticas
    by_type = {}
    for chunk in chunks:
        doc_type = chunk["metadata"]["document_type"]
        by_type[doc_type] = by_type.get(doc_type, 0) + 1
    
    print("Distribuicao por tipo:")
    for doc_type, count in sorted(by_type.items()):
        print(f"  - {doc_type}: {count} chunks")
    print()
    
    print("NOTA: Para inserir todos os chunks via MCP:")
    print("  Use mcp_vectorizer-main_insert_text para cada chunk")
    print(f"  Collection: {COLLECTION_NAME}")
    print(f"  Total: {total} chunks")
    print()
    print("Arquivo pronto para processamento completo!")
    print()

if __name__ == "__main__":
    main()

