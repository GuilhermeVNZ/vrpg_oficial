#!/usr/bin/env python3
"""
Script para inserir chunks de D&D 5e no Vectorizer via MCP
Processa todos os chunks JSON e insere na collection especificada
"""

import json
import os
import sys
from pathlib import Path
from typing import Dict, Any

# Configuração
CHUNKS_DIR = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service")
COLLECTION_NAME = "dnd5e-rules-new"  # Usar collection com dimensão 512
BATCH_SIZE = 50  # Processar em lotes de 50

def load_chunk(chunk_file: Path) -> Dict[str, Any]:
    """Carrega um chunk do arquivo JSON, tratando BOM UTF-8"""
    try:
        # Tentar ler com utf-8-sig primeiro (remove BOM)
        with open(chunk_file, 'r', encoding='utf-8-sig') as f:
            return json.load(f)
    except json.JSONDecodeError:
        # Se falhar, tentar sem sig
        try:
            with open(chunk_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        except Exception as e:
            print(f"Erro ao carregar {chunk_file.name}: {e}")
            return None

def main():
    print("Inserindo chunks de D&D 5e no Vectorizer...")
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Diretorio: {CHUNKS_DIR}")
    print()
    
    if not CHUNKS_DIR.exists():
        print(f"Diretorio nao encontrado: {CHUNKS_DIR}")
        sys.exit(1)
    
    # Listar todos os arquivos JSON
    chunk_files = sorted(CHUNKS_DIR.glob("*.json"))
    # Filtrar arquivos de instruções/resumo
    chunk_files = [f for f in chunk_files if not f.name.startswith("insertion_") and not f.name == "README.md"]
    total_chunks = len(chunk_files)
    
    if total_chunks == 0:
        print("Nenhum chunk encontrado!")
        sys.exit(1)
    
    print(f"Total de chunks encontrados: {total_chunks}")
    print()
    
    # Agrupar por tipo de documento
    by_type = {}
    for chunk_file in chunk_files:
        chunk = load_chunk(chunk_file)
        if chunk:
            doc_type = chunk.get('metadata', {}).get('document_type', 'unknown')
            if doc_type not in by_type:
                by_type[doc_type] = []
            by_type[doc_type].append((chunk_file, chunk))
    
    print("Documentos encontrados:")
    for doc_type, chunks in by_type.items():
        print(f"   - {doc_type}: {len(chunks)} chunks")
    print()
    
    # Preparar dados para inserção
    print("Preparando chunks para insercao...")
    print()
    print("NOTA: Este script prepara os dados.")
    print("Para inserir via MCP, use mcp_vectorizer-main_insert_text para cada chunk.")
    print()
    
    # Criar arquivo de batch para inserção
    batch_file = CHUNKS_DIR / "insertion_batch.json"
    batches = []
    
    chunk_count = 0
    current_batch = []
    
    for doc_type, chunks in sorted(by_type.items()):
        print(f"Processando {doc_type}...")
        for chunk_file, chunk in chunks:
            chunk_count += 1
            
            # Preparar entrada para inserção
            entry = {
                "file": chunk_file.name,
                "text": chunk['text'],
                "metadata": chunk['metadata']
            }
            
            current_batch.append(entry)
            
            # Quando o batch estiver completo, adicionar à lista
            if len(current_batch) >= BATCH_SIZE:
                batches.append(current_batch.copy())
                current_batch = []
                print(f"  Batch {len(batches)} preparado ({chunk_count}/{total_chunks} chunks)...")
        
        # Adicionar batch final se houver itens restantes
        if current_batch:
            batches.append(current_batch.copy())
            current_batch = []
    
    # Salvar batches
    batch_data = {
        "collection_name": COLLECTION_NAME,
        "total_chunks": total_chunks,
        "total_batches": len(batches),
        "batch_size": BATCH_SIZE,
        "batches": batches
    }
    
    with open(batch_file, 'w', encoding='utf-8') as f:
        json.dump(batch_data, f, indent=2, ensure_ascii=False)
    
    print()
    print(f"Batches salvos em: {batch_file}")
    print()
    print("Resumo:")
    print(f"   Total de chunks: {total_chunks}")
    print(f"   Total de batches: {len(batches)}")
    print(f"   Collection: {COLLECTION_NAME}")
    print()
    print("Para inserir os chunks, use o MCP Vectorizer:")
    print("   mcp_vectorizer-main_insert_text")
    print(f"   - collection_name: {COLLECTION_NAME}")
    print("   - text: conteudo do chunk")
    print("   - metadata: metadata do chunk")
    print()
    print("Ou execute o script de insercao automatica (se disponivel).")
    print()

if __name__ == "__main__":
    main()

