#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Cria collections e indexa chunks via workspace do Vectorizer
"""

import json
import requests
import sys
from pathlib import Path
from typing import List, Dict
from collections import defaultdict

# Configurar encoding UTF-8 para stdout
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')

# Configuração
VECTORIZER_URL = "http://127.0.0.1:15002"
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")

# Mapeamento de livros para collections
BOOK_COLLECTIONS = {
    "D&D 5e - Manual dos Monstros": "dnd5e-manual-dos-monstros",
    "D&D 5e - Livro do Jogador": "dnd5e-livro-do-jogador",
    "D&D 5e - Guia do Mestre": "dnd5e-guia-do-mestre",
    "D&D 5e - Guia de Xanathar para Todas as Coisas": "dnd5e-guia-de-xanathar-para-todas-as-coisas",
    "D&D 5e - Guia do Volo para Monstros": "dnd5e-guia-do-volo-para-monstros",
    "D&D 5e - Ficha de Personagem": "dnd5e-ficha-de-personagem"
}


def get_collection_name_from_metadata(metadata: Dict) -> str:
    """Extrai o nome da collection do metadata do chunk"""
    title = metadata.get("title", "")
    return BOOK_COLLECTIONS.get(title, "dnd5e-unknown")


def create_collection(collection_name: str, dimension: int = 512) -> bool:
    """Cria uma collection no Vectorizer"""
    url = f"{VECTORIZER_URL}/collections"
    payload = {
        "name": collection_name,
        "dimension": dimension,
        "metric": "cosine"
    }
    
    try:
        r = requests.post(url, json=payload, timeout=10)
        if r.status_code in [200, 201]:
            print(f"[OK] Collection '{collection_name}' criada")
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


def add_workspace_collection(collection_name: str, file_path: str) -> bool:
    """Adiciona uma collection ao workspace"""
    url = f"{VECTORIZER_URL}/api/workspace/add"
    payload = {
        "path": file_path,
        "collection_name": collection_name
    }
    
    try:
        r = requests.post(url, json=payload, timeout=10)
        if r.status_code == 200:
            result = r.json()
            if result.get("success"):
                print(f"[OK] Workspace collection '{collection_name}' adicionada")
                return True
            else:
                print(f"[ERRO] Falha ao adicionar workspace: {result.get('message', 'Erro desconhecido')}")
                return False
        else:
            print(f"[ERRO] Status {r.status_code}: {r.text[:200]}")
            return False
    except Exception as e:
        print(f"[ERRO] Exceção ao adicionar workspace: {e}")
        return False


def main():
    """Função principal"""
    print("="*70)
    print("Criando Collections e Indexando via Workspace")
    print("="*70)
    
    # Verificar Vectorizer
    try:
        r = requests.get(f"{VECTORIZER_URL}/health", timeout=10)
        if r.status_code != 200:
            print(f"[ERRO] Vectorizer não está disponível - Status: {r.status_code}")
            return
        print("[OK] Vectorizer está disponível")
    except Exception as e:
        print(f"[ERRO] Não foi possível conectar ao Vectorizer: {e}")
        return
    
    # Carregar chunks
    print(f"\n[1/3] Carregando chunks...")
    if not CHUNKS_FILE.exists():
        print(f"[ERRO] Arquivo não encontrado: {CHUNKS_FILE}")
        return
    
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    all_chunks = data.get("chunks", [])
    total_chunks = len(all_chunks)
    print(f"[OK] Carregados {total_chunks} chunks")
    
    # Agrupar chunks por livro
    print(f"\n[2/3] Agrupando chunks por livro...")
    chunks_by_book = defaultdict(list)
    for chunk in all_chunks:
        metadata = chunk.get("metadata", {})
        collection_name = get_collection_name_from_metadata(metadata)
        chunks_by_book[collection_name].append(chunk)
    
    print(f"[OK] Encontrados {len(chunks_by_book)} livros:")
    for collection_name, chunks in sorted(chunks_by_book.items()):
        title = chunks[0]["metadata"].get("title", "Sem título")
        print(f"  - {collection_name}: {len(chunks)} chunks ({title})")
    
    # Criar collections
    print(f"\n[3/3] Criando collections...")
    collections_created = 0
    for collection_name in sorted(chunks_by_book.keys()):
        if create_collection(collection_name, dimension=512):
            collections_created += 1
    
    print(f"[OK] {collections_created}/{len(chunks_by_book)} collections criadas/verificadas")
    
    print("\n" + "="*70)
    print("NOTA: O Vectorizer indexa arquivos automaticamente via workspace")
    print("As collections foram criadas. O workspace deve indexar automaticamente.")
    print("="*70)


if __name__ == "__main__":
    main()








