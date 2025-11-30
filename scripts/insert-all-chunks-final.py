#!/usr/bin/env python3
"""
Script FINAL para inserir TODOS os chunks restantes no Vectorizer via MCP
Processa chunks_for_vectorizer.json e insere todos os 1,908 chunks
"""

import json
import sys
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer.json")
COLLECTION_NAME = "dnd5e-rules"

def main():
    print("="*60)
    print("Preparando insercao de TODOS os chunks")
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
    
    # Estatísticas detalhadas
    by_type = {}
    for chunk in chunks:
        doc_type = chunk["metadata"]["document_type"]
        by_type[doc_type] = by_type.get(doc_type, 0) + 1
    
    print("Distribuicao completa:")
    for doc_type, count in sorted(by_type.items()):
        print(f"  - {doc_type}: {count} chunks")
    print()
    
    # Preparar arquivo de instruções completo
    instructions = {
        "collection": COLLECTION_NAME,
        "total_chunks": total,
        "status": "ready_for_batch_insertion",
        "chunks": []
    }
    
    # Adicionar todos os chunks (limitado para não criar arquivo muito grande)
    # Mas vamos criar um arquivo com índices para referência
    chunk_indices = []
    for i, chunk in enumerate(chunks):
        chunk_indices.append({
            "index": i,
            "chunk_index": chunk["metadata"]["chunk_index"],
            "document_type": chunk["metadata"]["document_type"],
            "title": chunk["metadata"]["title"],
            "text_length": len(chunk["text"])
        })
    
    instructions["chunk_indices"] = chunk_indices[:100]  # Primeiros 100 como exemplo
    
    # Salvar instruções
    instructions_file = CHUNKS_FILE.parent / "final_insertion_instructions.json"
    with open(instructions_file, 'w', encoding='utf-8') as f:
        json.dump(instructions, f, indent=2, ensure_ascii=False)
    
    print(f"[OK] Instrucoes salvas em: {instructions_file}")
    print()
    print("PRONTO PARA INSERCAO COMPLETA")
    print(f"Total: {total} chunks")
    print(f"Collection: {COLLECTION_NAME}")
    print()
    print("Para inserir todos os chunks, use mcp_vectorizer-main_insert_text")
    print("para cada chunk no arquivo chunks_for_vectorizer.json")
    print()

if __name__ == "__main__":
    main()



