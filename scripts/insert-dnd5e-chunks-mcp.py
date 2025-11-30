#!/usr/bin/env python3
"""
Script para inserir chunks de D&D 5e no Vectorizer via MCP
Processa os chunks em lotes e insere via MCP Vectorizer API
"""

import json
import os
import sys
from pathlib import Path

# Configuração
CHUNKS_DIR = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service")
COLLECTION_NAME = "dnd5e-rules"
BATCH_SIZE = 10  # Processar em lotes de 10

def load_chunk(chunk_file):
    """Carrega um chunk do arquivo JSON"""
    try:
        with open(chunk_file, 'r', encoding='utf-8') as f:
            return json.load(f)
    except Exception as e:
        print(f"Erro ao carregar {chunk_file.name}: {e}")
        return None

def main():
    print("Inserindo chunks de D&D 5e no Vectorizer via MCP...")
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Diretorio: {CHUNKS_DIR}")
    print()
    
    if not CHUNKS_DIR.exists():
        print(f"Diretorio nao encontrado: {CHUNKS_DIR}")
        sys.exit(1)
    
    # Listar todos os arquivos JSON
    chunk_files = sorted(CHUNKS_DIR.glob("*.json"))
    total_chunks = len(chunk_files)
    
    print(f"Total de chunks encontrados: {total_chunks}")
    print()
    
    # Agrupar por tipo de documento para processar em ordem
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
    
    # Preparar lista de chunks para inserção
    print("Preparando chunks para insercao...")
    print()
    print("NOTA: Este script prepara os dados para insercao via MCP.")
    print("Para inserir, use o MCP Vectorizer insert_text para cada chunk.")
    print()
    
    # Criar arquivo de instruções
    instructions = {
        "collection_name": COLLECTION_NAME,
        "total_chunks": total_chunks,
        "insertion_instructions": []
    }
    
    chunk_count = 0
    for doc_type, chunks in sorted(by_type.items()):
        print(f"Processando {doc_type}...")
        for chunk_file, chunk in chunks:
            chunk_count += 1
            instructions["insertion_instructions"].append({
                "chunk_number": chunk_count,
                "file": chunk_file.name,
                "text_preview": chunk['text'][:100] + "..." if len(chunk['text']) > 100 else chunk['text'],
                "metadata": chunk['metadata'],
                "text_length": len(chunk['text'])
            })
            
            if chunk_count % 100 == 0:
                print(f"  Preparados: {chunk_count}/{total_chunks} chunks...")
    
    # Salvar instruções
    instructions_file = CHUNKS_DIR / "insertion_instructions.json"
    with open(instructions_file, 'w', encoding='utf-8') as f:
        json.dump(instructions, f, indent=2, ensure_ascii=False)
    
    print()
    print(f"Instrucoes salvas em: {instructions_file}")
    print()
    print("Resumo:")
    print(f"   Total de chunks: {total_chunks}")
    print(f"   Collection: {COLLECTION_NAME}")
    print()
    print("Para inserir os chunks, use o MCP Vectorizer:")
    print("   mcp_vectorizer-main_insert_text")
    print("   - collection_name: dnd5e-rules")
    print("   - text: conteudo do chunk")
    print("   - metadata: metadata do chunk")
    print()
    print("Ou use um script de batch insertion via API REST do Vectorizer.")
    print()

if __name__ == "__main__":
    main()


