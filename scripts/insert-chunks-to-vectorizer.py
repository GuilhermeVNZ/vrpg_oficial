#!/usr/bin/env python3
"""
Script para inserir chunks de D&D 5e no Vectorizer via MCP
Uso: python scripts/insert-chunks-to-vectorizer.py
"""

import json
import os
import sys
from pathlib import Path

# Diretório dos chunks
CHUNKS_DIR = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service")
COLLECTION_NAME = "dnd5e-rules"

def load_chunk(chunk_file):
    """Carrega um chunk do arquivo JSON"""
    try:
        with open(chunk_file, 'r', encoding='utf-8') as f:
            return json.load(f)
    except Exception as e:
        print(f"Erro ao carregar {chunk_file}: {e}")
        return None

def main():
    print("Inserindo chunks de D&D 5e no Vectorizer...")
    print(f"   Collection: {COLLECTION_NAME}")
    print(f"   Diretorio: {CHUNKS_DIR}")
    print()
    
    if not CHUNKS_DIR.exists():
        print(f"❌ Diretório não encontrado: {CHUNKS_DIR}")
        sys.exit(1)
    
    # Listar todos os arquivos JSON
    chunk_files = sorted(CHUNKS_DIR.glob("*.json"))
    total_chunks = len(chunk_files)
    
    print(f"Total de chunks encontrados: {total_chunks}")
    print()
    
    # Processar em lotes
    batch_size = 50
    processed = 0
    errors = 0
    
    # Agrupar por tipo de documento
    by_type = {}
    for chunk_file in chunk_files:
        chunk = load_chunk(chunk_file)
        if chunk:
            doc_type = chunk['metadata']['document_type']
            if doc_type not in by_type:
                by_type[doc_type] = []
            by_type[doc_type].append((chunk_file, chunk))
    
    print("Documentos encontrados:")
    for doc_type, chunks in by_type.items():
        print(f"   - {doc_type}: {len(chunks)} chunks")
    print()
    
    # Preparar dados para inserção
    print("Preparando dados para insercao...")
    print()
    print("NOTA: Este script prepara os dados.")
    print("   Para inserir no Vectorizer, use o MCP Vectorizer insert_text")
    print("   ou execute via API REST do Vectorizer")
    print()
    
    # Salvar resumo
    summary = {
        "collection": COLLECTION_NAME,
        "total_chunks": total_chunks,
        "documents": {}
    }
    
    for doc_type, chunks in by_type.items():
        summary["documents"][doc_type] = {
            "count": len(chunks),
            "chunks": []
        }
        
        for chunk_file, chunk in chunks[:5]:  # Primeiros 5 como exemplo
            summary["documents"][doc_type]["chunks"].append({
                "file": chunk_file.name,
                "text_length": len(chunk['text']),
                "metadata": chunk['metadata']
            })
    
    summary_file = CHUNKS_DIR / "insertion_summary.json"
    with open(summary_file, 'w', encoding='utf-8') as f:
        json.dump(summary, f, indent=2, ensure_ascii=False)
    
    print(f"Resumo salvo em: {summary_file}")
    print()
    print("Proximos passos:")
    print("   1. Use o MCP Vectorizer para inserir os chunks")
    print(f"   2. Collection: {COLLECTION_NAME}")
    print("   3. Para cada chunk, use insert_text com:")
    print("      - collection_name: dnd5e-rules")
    print("      - text: conteudo do chunk")
    print("      - metadata: metadata do chunk")
    print()

if __name__ == "__main__":
    main()

