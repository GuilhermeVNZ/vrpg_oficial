#!/usr/bin/env python3
"""
Script para inserir chunks no Vectorizer via MCP em lotes
Lê chunks_for_vectorizer.json e insere via MCP Vectorizer
"""

import json
import sys
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer.json")
COLLECTION_NAME = "dnd5e-rules"
BATCH_SIZE = 10  # Processar em lotes menores para evitar timeout

def load_chunks():
    """Carrega os chunks do arquivo JSON"""
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    return data["chunks"]


def main():
    print("="*60)
    print("Inserindo chunks no Vectorizer via MCP")
    print("="*60)
    
    if not CHUNKS_FILE.exists():
        print(f"ERRO: Arquivo nao encontrado: {CHUNKS_FILE}")
        sys.exit(1)
    
    chunks = load_chunks()
    
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Total de chunks: {len(chunks)}")
    print(f"Processando em lotes de {BATCH_SIZE}")
    print()
    print("NOTA: Este script prepara os dados para insercao.")
    print("Os chunks serao inseridos usando mcp_vectorizer-main_insert_text")
    print()
    
    # Preparar dados para inserção
    total = len(chunks)
    processed = 0
    
    print(f"Preparando {total} chunks para insercao...")
    print()
    print("Para inserir, execute via MCP:")
    print(f"  mcp_vectorizer-main_insert_text")
    print(f"  - collection_name: {COLLECTION_NAME}")
    print(f"  - text: <texto do chunk>")
    print(f"  - metadata: <metadata do chunk>")
    print()
    
    # Salvar resumo
    summary = {
        "collection": COLLECTION_NAME,
        "total_chunks": total,
        "status": "ready_for_insertion",
        "note": "Use mcp_vectorizer-main_insert_text para inserir cada chunk"
    }
    
    summary_file = CHUNKS_FILE.parent / "insertion_summary.json"
    with open(summary_file, 'w', encoding='utf-8') as f:
        json.dump(summary, f, indent=2, ensure_ascii=False)
    
    print(f"[OK] Resumo salvo em: {summary_file}")
    print()
    print(f"Total: {total} chunks prontos para insercao")
    print()


if __name__ == "__main__":
    main()

