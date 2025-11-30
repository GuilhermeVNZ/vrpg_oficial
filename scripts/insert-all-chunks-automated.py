#!/usr/bin/env python3
"""
Script automatizado para inserir TODOS os chunks via MCP
Lê chunks_for_insertion.json e processa todos os chunks
"""

import json
import sys
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_insertion.json")

def main():
    print("=" * 70)
    print("INSERCAO AUTOMATIZADA DE CHUNKS VIA MCP")
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
    chunks = data['chunks']
    total_chunks = len(chunks)
    
    print(f"Collection: {collection_name}")
    print(f"Total de chunks: {total_chunks}")
    print()
    print("NOTA: Este script prepara os dados para insercao.")
    print("A insercao real deve ser feita via MCP Vectorizer.")
    print()
    print("Para inserir todos os chunks, voce precisa:")
    print("  1. Ler este arquivo JSON")
    print("  2. Para cada chunk, chamar mcp_vectorizer-main_insert_text")
    print(f"     - collection_name: {collection_name}")
    print("     - text: chunk['text']")
    print("     - metadata: chunk['metadata']")
    print()
    print("Ou usar um script que faca isso automaticamente.")
    print()
    
    # Salvar instruções
    instructions = {
        "collection_name": collection_name,
        "total_chunks": total_chunks,
        "chunks_ready": True,
        "insertion_method": "mcp_vectorizer-main_insert_text",
        "note": "Processar chunks em lotes de 10-20 para melhor performance"
    }
    
    instructions_file = CHUNKS_FILE.parent / "insertion_instructions.json"
    with open(instructions_file, 'w', encoding='utf-8') as f:
        json.dump(instructions, f, indent=2, ensure_ascii=False)
    
    print(f"Instrucoes salvas em: {instructions_file.name}")
    print()

if __name__ == "__main__":
    main()

