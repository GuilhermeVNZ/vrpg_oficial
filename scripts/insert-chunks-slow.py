#!/usr/bin/env python3
"""
Script para inserir chunks via API REST com pausas maiores
Evita sobrecarregar o Vectorizer
"""

import json
import sys
import requests
import time
from pathlib import Path

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_insertion.json")
VECTORIZER_URL = "http://127.0.0.1:15002"
START_FROM_INDEX = 18
PAUSE_BETWEEN_CHUNKS = 0.5  # 500ms entre chunks
PAUSE_EVERY_N = 10  # Pausa maior a cada N chunks
PAUSE_EVERY_N_SECONDS = 2  # 2 segundos a cada N chunks

def insert_single_chunk(collection_name: str, chunk: dict) -> bool:
    """Insere um único chunk via API REST"""
    url = f"{VECTORIZER_URL}/insert"
    
    payload = {
        "collection": collection_name,
        "text": chunk['text'],
        "metadata": chunk['metadata']
    }
    
    try:
        response = requests.post(url, json=payload, timeout=60)
        if response.status_code == 200:
            return True
        else:
            print(f"  ERRO: Status {response.status_code}")
            return False
    except requests.exceptions.Timeout:
        print(f"  TIMEOUT: Requisicao demorou muito")
        return False
    except Exception as e:
        print(f"  ERRO: {type(e).__name__}")
        return False

def main():
    print("=" * 70)
    print("INSERCAO DE CHUNKS VIA API REST (MODO LENTO)")
    print("=" * 70)
    print()
    
    # Verificar Vectorizer
    print("Verificando Vectorizer...")
    max_retries = 5
    for retry in range(max_retries):
        try:
            health_response = requests.get(f"{VECTORIZER_URL}/health", timeout=10)
            if health_response.status_code == 200:
                print("  Vectorizer esta rodando!")
                break
        except Exception as e:
            if retry < max_retries - 1:
                print(f"  Tentativa {retry + 1}/{max_retries} falhou, aguardando...")
                time.sleep(5)
            else:
                print(f"  ERRO: Vectorizer nao esta respondendo")
                sys.exit(1)
    
    # Carregar chunks
    print(f"\nCarregando chunks...")
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    collection_name = data['collection_name']
    all_chunks = data['chunks']
    chunks_to_insert = all_chunks[START_FROM_INDEX:]
    total = len(all_chunks)
    remaining = len(chunks_to_insert)
    
    print(f"Collection: {collection_name}")
    print(f"Total: {total} chunks")
    print(f"Ja inseridos: {START_FROM_INDEX}")
    print(f"Restantes: {remaining}")
    print(f"Pausa entre chunks: {PAUSE_BETWEEN_CHUNKS}s")
    print()
    
    print("Iniciando insercao (modo lento para evitar sobrecarga)...")
    print()
    
    inserted = 0
    errors = 0
    
    for i, chunk in enumerate(chunks_to_insert):
        chunk_num = START_FROM_INDEX + i + 1
        print(f"[{chunk_num}/{total}] ", end="", flush=True)
        
        if insert_single_chunk(collection_name, chunk):
            inserted += 1
            print("OK")
        else:
            errors += 1
            print("FALHOU")
            # Pausa maior após erro
            time.sleep(2)
        
        # Progresso a cada 50
        if (i + 1) % 50 == 0:
            print(f"\n>>> Progresso: {inserted} OK, {errors} erros, {remaining - (i + 1)} restantes <<<\n")
        
        # Pausa entre chunks
        if (i + 1) % PAUSE_EVERY_N == 0:
            time.sleep(PAUSE_EVERY_N_SECONDS)
        else:
            time.sleep(PAUSE_BETWEEN_CHUNKS)
    
    print()
    print("=" * 70)
    print("CONCLUIDO")
    print("=" * 70)
    print(f"Inseridos: {inserted}/{remaining}")
    print(f"Erros: {errors}")
    print()

if __name__ == "__main__":
    main()

