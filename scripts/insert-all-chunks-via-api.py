#!/usr/bin/env python3
"""
Script para inserir TODOS os chunks no Vectorizer via API REST
Usa batch_insert para inserir em lotes eficientes
"""

import json
import requests
import sys
from pathlib import Path
from typing import List, Dict

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer.json")
COLLECTION_NAME = "dnd5e-rules"
VECTORIZER_URL = "http://localhost:8002"
BATCH_SIZE = 100  # Inserir em lotes de 100

def load_chunks():
    """Carrega os chunks do arquivo JSON"""
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    return data["chunks"]


def insert_batch(collection: str, batch: List[Dict]) -> bool:
    """Insere um lote de chunks via API REST"""
    url = f"{VECTORIZER_URL}/collections/{collection}/batch_insert"
    
    # Preparar dados para batch insert
    vectors = []
    for chunk in batch:
        vectors.append({
            "text": chunk["text"],
            "metadata": chunk["metadata"]
        })
    
    payload = {
        "vectors": vectors
    }
    
    try:
        response = requests.post(url, json=payload, timeout=300)
        if response.status_code == 200:
            return True
        else:
            print(f"  ERRO: Status {response.status_code}: {response.text}")
            return False
    except Exception as e:
        print(f"  ERRO: {e}")
        return False


def main():
    print("="*60)
    print("Inserindo TODOS os chunks no Vectorizer via API REST")
    print("="*60)
    
    if not CHUNKS_FILE.exists():
        print(f"ERRO: Arquivo nao encontrado: {CHUNKS_FILE}")
        sys.exit(1)
    
    print("Carregando chunks...")
    chunks = load_chunks()
    total = len(chunks)
    
    print(f"Total de chunks: {total}")
    print(f"Collection: {COLLECTION_NAME}")
    print(f"URL: {VECTORIZER_URL}")
    print(f"Batch size: {BATCH_SIZE}")
    print()
    
    # Verificar se Vectorizer está rodando
    try:
        health = requests.get(f"{VECTORIZER_URL}/health", timeout=5)
        if health.status_code != 200:
            print(f"ERRO: Vectorizer nao esta respondendo em {VECTORIZER_URL}")
            sys.exit(1)
        print("Vectorizer esta rodando")
    except Exception as e:
        print(f"ERRO: Nao foi possivel conectar ao Vectorizer: {e}")
        print(f"Certifique-se de que o Vectorizer esta rodando em {VECTORIZER_URL}")
        sys.exit(1)
    
    print()
    print("Iniciando insercao em lotes...")
    print()
    
    # Processar em lotes
    inserted = 0
    failed = 0
    
    for i in range(0, total, BATCH_SIZE):
        batch = chunks[i:i+BATCH_SIZE]
        batch_num = (i // BATCH_SIZE) + 1
        total_batches = (total + BATCH_SIZE - 1) // BATCH_SIZE
        
        print(f"Lote {batch_num}/{total_batches}: Inserindo {len(batch)} chunks...", end=" ")
        
        if insert_batch(COLLECTION_NAME, batch):
            inserted += len(batch)
            print(f"[OK] {inserted}/{total} inseridos")
        else:
            failed += len(batch)
            print(f"[ERRO] Falha no lote")
    
    print()
    print("="*60)
    print("RESUMO")
    print("="*60)
    print(f"Total de chunks: {total}")
    print(f"Inseridos com sucesso: {inserted}")
    print(f"Falhas: {failed}")
    print()
    
    if failed == 0:
        print("[OK] Todos os chunks foram inseridos com sucesso!")
    else:
        print(f"[ATENCAO] {failed} chunks falharam na insercao")
    print()


if __name__ == "__main__":
    main()

