#!/usr/bin/env python3
"""
Recria a collection e indexa todos os chunks preparados
"""

import json
import asyncio
import aiohttp
import requests
from pathlib import Path
from typing import List, Dict
import hashlib

# Configuração
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")
VECTORIZER_URL = "http://127.0.0.1:15002"
COLLECTION_NAME = "dnd5e-rules-new"
BATCH_SIZE = 50


def create_collection(collection_name: str, dimension: int = 512) -> bool:
    """Cria a collection no Vectorizer"""
    url = f"{VECTORIZER_URL}/collections"
    payload = {
        "name": collection_name,
        "dimension": dimension,
        "metric": "cosine"
    }
    
    try:
        r = requests.post(url, json=payload, timeout=10)
        if r.status_code in [200, 201]:
            print(f"[OK] Collection '{collection_name}' criada com sucesso")
            return True
        elif r.status_code == 409:
            print(f"[INFO] Collection '{collection_name}' já existe")
            return True
        else:
            print(f"[ERRO] Falha ao criar collection: {r.status_code} - {r.text[:200]}")
            return False
    except Exception as e:
        print(f"[ERRO] Exceção ao criar collection: {e}")
        return False


async def insert_single(collection: str, chunk: Dict, session: aiohttp.ClientSession) -> bool:
    """Insere um único chunk no Vectorizer usando o endpoint /insert"""
    text = chunk["text"]
    metadata = chunk["metadata"]
    
    payload = {
        "collection": collection,
        "text": text,
        "metadata": metadata
    }
    
    url = f"{VECTORIZER_URL}/insert"
    
    try:
        async with session.post(
            url,
            json=payload,
            headers={"Content-Type": "application/json"},
            timeout=aiohttp.ClientTimeout(total=60)
        ) as response:
            if response.status == 200:
                return True
            else:
                error_text = await response.text()
                print(f"    [ERRO] Status {response.status}: {error_text[:200]}")
                return False
    except Exception as e:
        print(f"    [ERRO] Exceção: {e}")
        return False


async def insert_batch(collection: str, batch: List[Dict], session: aiohttp.ClientSession) -> int:
    """Insere um lote de chunks no Vectorizer usando processamento paralelo"""
    # Criar tasks para inserção paralela (máximo 5 simultâneos para não sobrecarregar)
    semaphore = asyncio.Semaphore(5)
    
    async def insert_with_semaphore(chunk):
        async with semaphore:
            return await insert_single(collection, chunk, session)
    
    tasks = [insert_with_semaphore(chunk) for chunk in batch]
    results = await asyncio.gather(*tasks, return_exceptions=True)
    
    # Contar sucessos
    inserted = sum(1 for r in results if r is True)
    return inserted


async def main():
    """Função principal"""
    print("="*60)
    print("Recriando Collection e Indexando Chunks")
    print("="*60)
    
    # Verificar Vectorizer
    try:
        r = requests.get(f"{VECTORIZER_URL}/health", timeout=10)
        if r.status_code != 200:
            print("[ERRO] Vectorizer não está disponível")
            return
        print("[OK] Vectorizer está disponível")
    except Exception as e:
        print(f"[ERRO] Não foi possível conectar ao Vectorizer: {e}")
        return
    
    # Criar collection
    print(f"\n[1/3] Criando collection '{COLLECTION_NAME}'...")
    if not create_collection(COLLECTION_NAME, dimension=512):
        print("[ERRO] Não foi possível criar a collection. Abortando.")
        return
    
    # Carregar chunks
    print(f"\n[2/3] Carregando chunks de: {CHUNKS_FILE}")
    if not CHUNKS_FILE.exists():
        print(f"[ERRO] Arquivo não encontrado: {CHUNKS_FILE}")
        return
    
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    chunks = data.get("chunks", [])
    total_chunks = len(chunks)
    print(f"[OK] Carregados {total_chunks} chunks")
    
    # Inserir em lotes
    print(f"\n[3/3] Inserindo {total_chunks} chunks em lotes de {BATCH_SIZE}...")
    async with aiohttp.ClientSession() as session:
        inserted_total = 0
        for i in range(0, total_chunks, BATCH_SIZE):
            batch = chunks[i:i + BATCH_SIZE]
            batch_num = (i // BATCH_SIZE) + 1
            total_batches = (total_chunks + BATCH_SIZE - 1) // BATCH_SIZE
            
            print(f"[{batch_num}/{total_batches}] Inserindo lote de {len(batch)} chunks...")
            inserted = await insert_batch(COLLECTION_NAME, batch, session)
            inserted_total += inserted
            
            print(f"  [OK] Inseridos {inserted}/{len(batch)} chunks (Total: {inserted_total}/{total_chunks})")
            
            # Pequeno delay entre batches
            if i + BATCH_SIZE < total_chunks:
                await asyncio.sleep(1)
    
    # Verificar resultado final
    print("\n" + "="*60)
    print("VERIFICAÇÃO FINAL")
    print("="*60)
    try:
        r = requests.get(f"{VECTORIZER_URL}/collections/{COLLECTION_NAME}", timeout=10)
        if r.status_code == 200:
            data = r.json()
            vector_count = data.get("vector_count", 0)
            print(f"Collection: {COLLECTION_NAME}")
            print(f"Vetores na collection: {vector_count}")
            print(f"Chunks preparados: {total_chunks}")
            
            if vector_count >= total_chunks:
                print("\n[OK] Todos os chunks foram indexados com sucesso!")
            else:
                print(f"\n[AVISO] Indexação parcial: {vector_count}/{total_chunks} ({(vector_count/total_chunks*100):.1f}%)")
    except Exception as e:
        print(f"[AVISO] Não foi possível verificar a collection: {e}")


if __name__ == "__main__":
    asyncio.run(main())


