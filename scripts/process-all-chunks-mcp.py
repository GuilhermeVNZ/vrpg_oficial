#!/usr/bin/env python3
"""
Script para processar TODOS os chunks e gerar instruções de inserção
Lê chunks_for_insertion.json e prepara para inserção via MCP
"""

import json
import sys
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_insertion.json")
BATCH_SIZE = 50

def main():
    print("=" * 70)
    print("PROCESSAMENTO DE TODOS OS CHUNKS PARA INSERCAO VIA MCP")
    print("=" * 70)
    print()
    
    if not CHUNKS_FILE.exists():
        print(f"ERRO: Arquivo nao encontrado: {CHUNKS_FILE}")
        sys.exit(1)
    
    # Carregar chunks
    print(f"Carregando: {CHUNKS_FILE.name}")
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    collection_name = data['collection_name']
    chunks = data['chunks']
    total_chunks = len(chunks)
    
    print(f"Collection: {collection_name}")
    print(f"Total de chunks: {total_chunks}")
    print()
    
    # Dividir em lotes
    batches = []
    for i in range(0, total_chunks, BATCH_SIZE):
        batch = chunks[i:i+BATCH_SIZE]
        batches.append({
            "batch_number": len(batches) + 1,
            "start_index": i + 1,
            "end_index": min(i + BATCH_SIZE, total_chunks),
            "chunk_count": len(batch),
            "chunks": batch
        })
    
    print(f"Total de lotes: {len(batches)}")
    print(f"Tamanho do lote: {BATCH_SIZE}")
    print()
    print("=" * 70)
    print("PRONTO PARA INSERCAO")
    print("=" * 70)
    print()
    print("Os chunks foram organizados em lotes.")
    print("Para inserir via MCP, processe cada lote:")
    print()
    print("Para cada chunk no lote:")
    print(f"  mcp_vectorizer-main_insert_text")
    print(f"    - collection_name: {collection_name}")
    print("    - text: chunk['text']")
    print("    - metadata: chunk['metadata']")
    print()
    
    # Salvar lotes
    batches_file = CHUNKS_FILE.parent / "mcp_insertion_batches.json"
    with open(batches_file, 'w', encoding='utf-8') as f:
        json.dump({
            "collection_name": collection_name,
            "total_chunks": total_chunks,
            "batch_size": BATCH_SIZE,
            "total_batches": len(batches),
            "batches": batches
        }, f, indent=2, ensure_ascii=False)
    
    print(f"Lotes salvos em: {batches_file.name}")
    print()
    print("NOTA: Para inserir automaticamente, seria necessario")
    print("um script que chame o MCP Vectorizer para cada chunk.")
    print("Como sao {total_chunks} chunks, isso levaria algum tempo.".format(total_chunks=total_chunks))
    print()

if __name__ == "__main__":
    main()

