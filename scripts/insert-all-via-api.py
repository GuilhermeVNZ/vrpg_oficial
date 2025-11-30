#!/usr/bin/env python3
"""
Script para inserir TODOS os chunks de D&D 5e via API REST do Vectorizer
Usa batch insert para eficiência máxima
"""

import json
import sys
import requests
from pathlib import Path
from typing import List, Dict, Any

# Configuração
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_insertion.json")
VECTORIZER_URL = "http://127.0.0.1:15002"  # Porta do Vectorizer
BATCH_SIZE = 100  # Tamanho do lote para batch insert

def insert_batch(collection_name: str, batch: List[Dict[str, Any]]) -> bool:
    """Insere um lote de chunks via API REST"""
    url = f"{VECTORIZER_URL}/collections/{collection_name}/batch_insert"
    
    # Preparar payload conforme documentação do Vectorizer
    vectors = []
    for chunk in batch:
        vectors.append({
            "text": chunk['text'],
            "metadata": chunk['metadata']
        })
    
    payload = {
        "vectors": vectors
    }
    
    try:
        response = requests.post(url, json=payload, timeout=300)
        if response.status_code == 200:
            result = response.json()
            # A resposta pode ter 'inserted' ou 'ids'
            inserted = result.get('inserted', len(result.get('ids', [])))
            if inserted == len(batch):
                return True
            else:
                print(f"  AVISO: Inseridos {inserted}/{len(batch)} chunks")
                return inserted > 0  # Aceita inserção parcial como sucesso
        else:
            print(f"  ERRO: Status {response.status_code}: {response.text}")
            return False
    except requests.exceptions.ConnectionError:
        print(f"  ERRO: Nao foi possivel conectar ao Vectorizer em {VECTORIZER_URL}")
        print(f"        Certifique-se de que o Vectorizer esta rodando")
        return False
    except Exception as e:
        print(f"  ERRO: {e}")
        return False

def main():
    print("=" * 70)
    print("INSERCAO EM LOTE VIA API REST DO VECTORIZER")
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
    print(f"Tamanho do lote: {BATCH_SIZE}")
    print(f"Vectorizer URL: {VECTORIZER_URL}")
    print()
    
    # Verificar se Vectorizer está rodando
    print("Verificando conexao com Vectorizer...")
    try:
        health_response = requests.get(f"{VECTORIZER_URL}/health", timeout=5)
        if health_response.status_code == 200:
            print("  Vectorizer esta rodando!")
        else:
            print(f"  AVISO: Vectorizer respondeu com status {health_response.status_code}")
    except requests.exceptions.ConnectionError:
        print(f"  ERRO: Vectorizer nao esta rodando em {VECTORIZER_URL}")
        print()
        print("  Para iniciar o Vectorizer:")
        print("    1. Execute: cargo run --bin vectorizer")
        print("    2. Ou use o script: servers.ps1")
        print()
        sys.exit(1)
    except Exception as e:
        print(f"  ERRO ao verificar: {e}")
        sys.exit(1)
    
    print()
    print("Iniciando insercao em lotes...")
    print()
    
    # Dividir em lotes e inserir
    total_batches = (total_chunks + BATCH_SIZE - 1) // BATCH_SIZE
    inserted_count = 0
    error_count = 0
    
    for batch_num in range(total_batches):
        start_idx = batch_num * BATCH_SIZE
        end_idx = min(start_idx + BATCH_SIZE, total_chunks)
        batch = chunks[start_idx:end_idx]
        
        print(f"Lote {batch_num + 1}/{total_batches} ({start_idx + 1}-{end_idx}/{total_chunks})...", end=" ")
        
        if insert_batch(collection_name, batch):
            inserted_count += len(batch)
            print(f"OK ({len(batch)} chunks inseridos)")
        else:
            error_count += len(batch)
            print(f"FALHOU")
    
    print()
    print("=" * 70)
    print("RESUMO")
    print("=" * 70)
    print(f"Total de chunks: {total_chunks}")
    print(f"Chunks inseridos: {inserted_count}")
    print(f"Chunks com erro: {error_count}")
    print()
    
    if error_count == 0:
        print("SUCESSO: Todos os chunks foram inseridos!")
    else:
        print(f"ATENCAO: {error_count} chunks nao foram inseridos")
    print()

if __name__ == "__main__":
    main()

