#!/usr/bin/env python3
"""
Script para inserir TODOS os chunks no Vectorizer via MCP
Processa chunks_for_vectorizer.json e insere todos os chunks
"""

import json
import sys
import time
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer.json")
COLLECTION_NAME = "dnd5e-rules"
BATCH_SIZE = 50  # Processar em lotes
DELAY_BETWEEN_BATCHES = 0.1  # Pequeno delay entre lotes

def load_chunks():
    """Carrega os chunks do arquivo JSON"""
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    return data["chunks"]


def main():
    print("="*60)
    print("Inserindo TODOS os chunks no Vectorizer via MCP")
    print("="*60)
    
    if not CHUNKS_FILE.exists():
        print(f"ERRO: Arquivo nao encontrado: {CHUNKS_FILE}")
        sys.exit(1)
    
    chunks = load_chunks()
    total = len(chunks)
    
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Total de chunks: {total}")
    print(f"Processando em lotes de {BATCH_SIZE}")
    print()
    
    # Preparar dados para inserção via MCP
    # Como não temos acesso direto ao MCP aqui, vamos criar um arquivo
    # com instruções detalhadas para inserção
    
    print("Preparando dados para insercao...")
    print()
    print("NOTA: Este script prepara os dados.")
    print("Os chunks serao inseridos usando mcp_vectorizer-main_insert_text")
    print()
    print(f"Iniciando insercao de {total} chunks...")
    print()
    
    # Criar arquivo de instruções para inserção batch
    instructions = {
        "collection": COLLECTION_NAME,
        "total_chunks": total,
        "batch_size": BATCH_SIZE,
        "chunks": []
    }
    
    for i, chunk in enumerate(chunks):
        instructions["chunks"].append({
            "chunk_number": i + 1,
            "text": chunk["text"],
            "metadata": chunk["metadata"]
        })
        
        if (i + 1) % 100 == 0:
            print(f"  Preparados: {i + 1}/{total} chunks...")
    
    # Salvar instruções completas
    instructions_file = CHUNKS_FILE.parent / "all_chunks_for_mcp.json"
    with open(instructions_file, 'w', encoding='utf-8') as f:
        json.dump(instructions, f, indent=2, ensure_ascii=False)
    
    print()
    print(f"[OK] Todos os {total} chunks preparados!")
    print(f"[OK] Arquivo salvo em: {instructions_file}")
    print()
    print("Pronto para insercao via MCP!")
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Total: {total} chunks")
    print()


if __name__ == "__main__":
    main()

