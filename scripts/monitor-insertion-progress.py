#!/usr/bin/env python3
"""
Script para monitorar o progresso da insercao de chunks
Verifica periodicamente quantos vetores foram inseridos
"""

import json
import sys
import requests
import time
from pathlib import Path

VECTORIZER_URL = "http://127.0.0.1:15002"
COLLECTION_NAME = "dnd5e-rules-new"
TARGET_COUNT = 1908
CHECK_INTERVAL = 30  # Verificar a cada 30 segundos

def get_vector_count():
    """Obtem o numero de vetores na collection"""
    try:
        response = requests.get(
            f"{VECTORIZER_URL}/collections/{COLLECTION_NAME}",
            timeout=15
        )
        if response.status_code == 200:
            return response.json().get('vector_count', 0)
        return None
    except Exception:
        return None

def main():
    print("=" * 70)
    print("MONITORAMENTO DE INSERCAO DE CHUNKS")
    print("=" * 70)
    print()
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Target: {TARGET_COUNT} chunks")
    print(f"Verificando a cada {CHECK_INTERVAL} segundos...")
    print()
    
    last_count = 0
    stalled_count = 0
    
    while True:
        count = get_vector_count()
        
        if count is None:
            print(f"[{time.strftime('%H:%M:%S')}] Vectorizer nao respondeu")
            stalled_count += 1
            if stalled_count >= 10:
                print("Vectorizer nao respondeu por muito tempo. Verifique manualmente.")
                break
        else:
            stalled_count = 0
            remaining = TARGET_COUNT - count
            progress = (count / TARGET_COUNT) * 100
            
            if count != last_count:
                print(f"[{time.strftime('%H:%M:%S')}] {count}/{TARGET_COUNT} ({progress:.1f}%) - Restantes: {remaining}")
                last_count = count
            
            if count >= TARGET_COUNT:
                print()
                print("=" * 70)
                print("CONCLUIDO!")
                print("=" * 70)
                print(f"Todos os {TARGET_COUNT} chunks foram inseridos!")
                break
        
        time.sleep(CHECK_INTERVAL)

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\nMonitoramento interrompido pelo usuario.")
        sys.exit(0)

