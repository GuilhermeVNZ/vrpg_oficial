#!/usr/bin/env python3
"""Verifica o status da indexação no Vectorizer"""

import requests
import json
from pathlib import Path

VECTORIZER_URL = "http://127.0.0.1:15002"
COLLECTION_NAME = "dnd5e-rules-new"
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")

def main():
    print("="*60)
    print("Status da Indexação - Vectorizer")
    print("="*60)
    
    # Verificar collection
    try:
        r = requests.get(f"{VECTORIZER_URL}/collections/{COLLECTION_NAME}", timeout=10)
        if r.status_code == 200:
            data = r.json()
            vector_count = data.get("vector_count", 0)
            dimension = data.get("dimension", "N/A")
            status = data.get("status", "N/A")
            
            print(f"\nCollection: {COLLECTION_NAME}")
            print(f"Status: {status}")
            print(f"Dimension: {dimension}")
            print(f"Vetores inseridos: {vector_count}")
        else:
            print(f"[ERRO] Não foi possível acessar a collection: {r.status_code}")
            print(r.text[:200])
            return
    except Exception as e:
        print(f"[ERRO] Erro ao verificar collection: {e}")
        return
    
    # Verificar chunks preparados
    if CHUNKS_FILE.exists():
        with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
            chunks_data = json.load(f)
        
        total_chunks = chunks_data.get("total_chunks", 0)
        total_inserted = chunks_data.get("total_inserted", 0)
        
        print(f"\nChunks preparados: {total_chunks}")
        print(f"Chunks marcados como inseridos: {total_inserted}")
        
        # Comparar
        print("\n" + "="*60)
        print("COMPARAÇÃO")
        print("="*60)
        print(f"Vetores na collection: {vector_count}")
        print(f"Chunks preparados: {total_chunks}")
        
        if vector_count >= total_chunks:
            print("\n[OK] Todos os chunks foram indexados!")
            print(f"   {vector_count}/{total_chunks} vetores na collection")
        elif vector_count > 0:
            missing = total_chunks - vector_count
            percentage = (vector_count / total_chunks) * 100
            print(f"\n[AVISO] Indexação parcial")
            print(f"   {vector_count}/{total_chunks} vetores inseridos ({percentage:.1f}%)")
            print(f"   Faltam {missing} chunks para completar")
        else:
            print("\n[ERRO] Nenhum vetor foi inserido ainda")
    else:
        print(f"\n[AVISO] Arquivo de chunks não encontrado: {CHUNKS_FILE}")

if __name__ == "__main__":
    main()










