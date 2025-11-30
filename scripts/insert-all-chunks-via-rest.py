#!/usr/bin/env python3
"""
Script para inserir TODOS os chunks via API REST do Vectorizer
Usa o endpoint /insert com collection no body
"""

import json
import sys
import requests
import time
from pathlib import Path
from typing import List, Dict, Any

CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_insertion.json")
VECTORIZER_URL = "http://127.0.0.1:15002"
START_FROM_INDEX = 18  # Já inseridos
BATCH_SIZE = 1  # Inserir um por vez para evitar problemas

def insert_single_chunk(collection_name: str, chunk: Dict[str, Any]) -> bool:
    """Insere um único chunk via API REST"""
    url = f"{VECTORIZER_URL}/insert"
    
    payload = {
        "collection": collection_name,
        "text": chunk['text'],
        "metadata": chunk['metadata']
    }
    
    try:
        response = requests.post(url, json=payload, timeout=30)
        if response.status_code == 200:
            return True
        else:
            print(f"  ERRO: Status {response.status_code}: {response.text[:200]}")
            return False
    except requests.exceptions.ConnectionError:
        print(f"  ERRO: Nao foi possivel conectar ao Vectorizer em {VECTORIZER_URL}")
        return False
    except Exception as e:
        print(f"  ERRO: {e}")
        return False

def main():
    print("=" * 70)
    print("INSERCAO COMPLETA DE CHUNKS VIA API REST")
    print("=" * 70)
    print()
    
    if not CHUNKS_FILE.exists():
        print(f"ERRO: Arquivo nao encontrado: {CHUNKS_FILE}")
        sys.exit(1)
    
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
        sys.exit(1)
    except Exception as e:
        print(f"  ERRO ao verificar: {e}")
        sys.exit(1)
    
    # Carregar chunks
    print(f"\nCarregando chunks de: {CHUNKS_FILE.name}")
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    collection_name = data['collection_name']
    all_chunks = data['chunks']
    total_chunks = len(all_chunks)
    
    # Pular chunks já inseridos
    chunks_to_insert = all_chunks[START_FROM_INDEX:]
    remaining_count = len(chunks_to_insert)
    
    print(f"Collection: {collection_name}")
    print(f"Total de chunks no arquivo: {total_chunks}")
    print(f"Chunks ja inseridos: {START_FROM_INDEX}")
    print(f"Chunks restantes: {remaining_count}")
    print()
    
    print("Iniciando insercao...")
    print()
    
    # Inserir chunks
    inserted_count = 0
    error_count = 0
    
    for i, chunk in enumerate(chunks_to_insert):
        chunk_num = START_FROM_INDEX + i + 1
        print(f"Chunk {chunk_num}/{total_chunks}...", end=" ")
        
        if insert_single_chunk(collection_name, chunk):
            inserted_count += 1
            print("OK")
        else:
            error_count += 1
            print("FALHOU")
        
        # Mostrar progresso a cada 50 chunks
        if (i + 1) % 50 == 0:
            print(f"\nProgresso: {inserted_count} inseridos, {error_count} erros, {remaining_count - (i + 1)} restantes\n")
        
        # Pequena pausa para não sobrecarregar
        time.sleep(0.1)
    
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

