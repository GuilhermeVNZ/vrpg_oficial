#!/usr/bin/env python3
"""
Script para inserir todos os chunks no Vectorizer via MCP
Processa o arquivo chunks_for_vectorizer.json e insere via MCP
"""

import json
import sys
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer.json")
BATCH_SIZE = 50  # Processar em lotes de 50

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
    print(f"Processando em lotes de {BATCH_SIZE}")
    print()
    
    # Preparar instruções para inserção
    print("NOTA: Este script prepara os dados.")
    print("Para inserir via MCP, use mcp_vectorizer-main_insert_text para cada chunk.")
    print()
    
    # Salvar instruções detalhadas
    instructions = {
        "collection": collection_name,
        "total_chunks": len(chunks),
        "batch_size": BATCH_SIZE,
        "insertion_instructions": []
    }
    
    for i, chunk in enumerate(chunks):
        instructions["insertion_instructions"].append({
            "chunk_number": i + 1,
            "text_preview": chunk["text"][:100] + "..." if len(chunk["text"]) > 100 else chunk["text"],
            "metadata": chunk["metadata"]
        })
    
    # Salvar instruções
    instructions_file = CHUNKS_FILE.parent / "insertion_instructions_mcp.json"
    with open(instructions_file, 'w', encoding='utf-8') as f:
        json.dump(instructions, f, indent=2, ensure_ascii=False)
    
    print(f"[OK] Instrucoes salvas em: {instructions_file}")
    print()
    print("Para inserir os chunks, use:")
    print("  mcp_vectorizer-main_insert_text")
    print(f"  - collection_name: {collection_name}")
    print("  - text: chunk['text']")
    print("  - metadata: chunk['metadata']")
    print()
    print(f"Total: {len(chunks)} chunks para inserir")
    print()


if __name__ == "__main__":
    main()
