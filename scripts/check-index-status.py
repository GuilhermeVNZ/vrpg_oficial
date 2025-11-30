#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Script rápido para verificar o status da indexação"""

import sys
import requests
import json
from pathlib import Path

# Configurar encoding UTF-8 para Windows
if sys.platform == "win32":
    sys.stdout.reconfigure(encoding='utf-8')

VECTORIZER_URL = "http://127.0.0.1:15002"
COLLECTION_NAME = "dnd5e-rules-new"
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")

def main():
    print("\n" + "="*60)
    print("STATUS DA INDEXAÇÃO - Vectorizer")
    print("="*60)
    
    # Verificar Vectorizer health
    try:
        health = requests.get(f"{VECTORIZER_URL}/health", timeout=30)
        if health.status_code == 200:
            health_data = health.json()
            print(f"\n[OK] Vectorizer: {health_data.get('status', 'OK')}")
        else:
            print(f"\n[ERRO] Vectorizer: Não disponível (status {health.status_code})")
            return
    except Exception as e:
        print(f"\n[ERRO] Vectorizer: Erro ao conectar - {e}")
        return
    
    # Verificar collection
    try:
        r = requests.get(f"{VECTORIZER_URL}/collections/{COLLECTION_NAME}", timeout=30)
        if r.status_code == 200:
            data = r.json()
            vector_count = data.get("vector_count", 0)
            dimension = data.get("dimension", "N/A")
            status = data.get("status", "N/A")
            
            print(f"\nCollection: {COLLECTION_NAME}")
            print(f"  Status: {status}")
            print(f"  Dimension: {dimension}d")
            print(f"  Vetores inseridos: {vector_count}")
        elif r.status_code == 404:
            print(f"\n[ERRO] Collection '{COLLECTION_NAME}' não encontrada")
            print("  Ainda não foi criada ou foi perdida após restart")
            vector_count = 0
        else:
            print(f"\n[ERRO] Erro ao verificar collection: {r.status_code}")
            print(f"  {r.text[:200]}")
            vector_count = 0
    except Exception as e:
        print(f"\n[ERRO] Erro ao verificar collection: {e}")
        vector_count = 0
    
    # Verificar chunks preparados
    if CHUNKS_FILE.exists():
        with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
            chunks_data = json.load(f)
        
        total_chunks = chunks_data.get("total_chunks", 0)
        
        print(f"\nChunks preparados: {total_chunks}")
        
        # Comparar
        print("\n" + "="*60)
        print("PROGRESSO")
        print("="*60)
        
        if vector_count > 0:
            percentage = (vector_count / total_chunks) * 100 if total_chunks > 0 else 0
            missing = total_chunks - vector_count
            
            if vector_count >= total_chunks:
                print(f"[COMPLETO] {vector_count}/{total_chunks} vetores (100%)")
                print("\n[OK] Todos os chunks foram indexados com sucesso!")
            else:
                print(f"[EM PROGRESSO] {vector_count}/{total_chunks} vetores ({percentage:.1f}%)")
                print(f"   Faltam: {missing} chunks")
                progress_bar = "=" * int(percentage/2) + "-" * (50 - int(percentage/2))
                print(f"   Progresso: [{progress_bar}] {percentage:.1f}%")
        else:
            print(f"[AGUARDANDO] Ainda não iniciado")
            print(f"   Esperando inserção dos {total_chunks} chunks...")
    else:
        print(f"\n[AVISO] Arquivo de chunks não encontrado: {CHUNKS_FILE}")
    
    print("\n" + "="*60)
    print("\nPara verificar novamente, execute:")
    print("   python scripts/check-index-status.py")
    print()

if __name__ == "__main__":
    main()

