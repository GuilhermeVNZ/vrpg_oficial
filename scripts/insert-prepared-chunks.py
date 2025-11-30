#!/usr/bin/env python3
"""
Script para inserir chunks já preparados no Vectorizer
"""

import json
import asyncio
import aiohttp
from pathlib import Path
from typing import List, Dict

# Configuração
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")
VECTORIZER_URL = "http://127.0.0.1:15002"
COLLECTION_NAME = "dnd5e-rules-new"
BATCH_SIZE = 50  # Processar em lotes menores para evitar timeout


async def insert_batch(collection: str, batch: List[Dict], session: aiohttp.ClientSession) -> int:
    """Insere um lote de chunks no Vectorizer"""
    vectors = []
    for chunk in batch:
        text = chunk["text"]
        metadata = chunk["metadata"]
        
        # Gerar ID único
        import hashlib
        text_hash = hashlib.md5(text.encode('utf-8')).hexdigest()
        chunk_id = f"{metadata.get('source_file', 'unknown')}_{metadata.get('chunk_index', 0)}_{text_hash[:8]}"
        
        vectors.append({
            "id": chunk_id,
            "text": text,
            "metadata": metadata
        })
    
    payload = {"vectors": vectors}
    url = f"{VECTORIZER_URL}/collections/{collection}/batch_insert"
    
    try:
        async with session.post(
            url,
            json=payload,
            headers={"Content-Type": "application/json"},
            timeout=aiohttp.ClientTimeout(total=300)
        ) as response:
            if response.status == 200:
                result = await response.json()
                inserted = result.get("inserted", result.get("count", len(vectors)))
                return inserted
            else:
                error_text = await response.text()
                print(f"  [ERRO] Status {response.status}: {error_text[:300]}")
                return 0
    except Exception as e:
        print(f"  [ERRO] Exceção: {e}")
        return 0


async def main():
    """Função principal"""
    print("="*60)
    print("Inserindo chunks preparados no Vectorizer")
    print("="*60)
    
    # Carregar chunks
    if not CHUNKS_FILE.exists():
        print(f"[ERRO] Arquivo não encontrado: {CHUNKS_FILE}")
        return
    
    print(f"[INFO] Carregando chunks de: {CHUNKS_FILE}")
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    chunks = data.get("chunks", [])
    total_chunks = len(chunks)
    
    print(f"[OK] Carregados {total_chunks} chunks")
    print(f"[INFO] Collection: {COLLECTION_NAME}")
    print(f"[INFO] Batch size: {BATCH_SIZE}")
    print()
    
    # Verificar Vectorizer
    async with aiohttp.ClientSession() as session:
        try:
            async with session.get(f"{VECTORIZER_URL}/health", timeout=aiohttp.ClientTimeout(total=5)) as response:
                if response.status != 200:
                    print("[ERRO] Vectorizer não está disponível")
                    return
        except Exception as e:
            print(f"[ERRO] Não foi possível conectar ao Vectorizer: {e}")
            return
        
        print("[OK] Vectorizer está disponível")
        print()
        
        # Inserir em lotes
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
        
        print()
        print("="*60)
        print("RESUMO")
        print("="*60)
        print(f"Total de chunks: {total_chunks}")
        print(f"Chunks inseridos: {inserted_total}")
        print(f"Taxa de sucesso: {(inserted_total/total_chunks*100):.1f}%")
        print(f"Collection: {COLLECTION_NAME}")


if __name__ == "__main__":
    asyncio.run(main())










