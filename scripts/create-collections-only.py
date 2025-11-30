#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Cria apenas as collections para cada livro D&D 5e, sem indexar chunks
"""

import json
import requests
import sys
from pathlib import Path
from typing import Dict
from collections import defaultdict
import re

# Configuração
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")
VECTORIZER_URL = "http://127.0.0.1:15002"


def sanitize_collection_name(name: str) -> str:
    """Sanitiza o nome para ser usado como collection name"""
    # Remove extensão .pdf
    name = name.replace(".pdf", "")
    # Remove caracteres especiais e espaços
    name = re.sub(r'[^a-zA-Z0-9_-]', '-', name)
    # Remove múltiplos hífens
    name = re.sub(r'-+', '-', name)
    # Remove hífens no início/fim
    name = name.strip('-')
    # Limita tamanho
    if len(name) > 50:
        name = name[:50]
    return name.lower()


def get_collection_name_from_metadata(metadata: Dict) -> str:
    """Extrai o nome da collection do metadata do chunk"""
    # Tenta usar o título primeiro
    title = metadata.get("title", "")
    if title:
        # Remove "D&D 5e - " do início se existir
        title = title.replace("D&D 5e - ", "").strip()
        collection_name = sanitize_collection_name(title)
        if collection_name:
            return f"dnd5e-{collection_name}"
    
    # Fallback para source_file
    source_file = metadata.get("source_file", "")
    if source_file:
        collection_name = sanitize_collection_name(source_file)
        return f"dnd5e-{collection_name}"
    
    # Último fallback
    return "dnd5e-unknown"


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


def main():
    """Função principal"""
    # Configurar encoding UTF-8 para stdout
    if sys.platform == 'win32':
        sys.stdout.reconfigure(encoding='utf-8')
    
    print("="*70)
    print("Criando Collections por Livro (sem indexar)")
    print("="*70)
    
    # Verificar Vectorizer
    print("\nVerificando Vectorizer...")
    try:
        r = requests.get(f"{VECTORIZER_URL}/health", timeout=30)
        if r.status_code != 200:
            print(f"[ERRO] Vectorizer não está disponível - Status: {r.status_code}")
            return
        print("[OK] Vectorizer está disponível")
    except Exception as e:
        print(f"[ERRO] Não foi possível conectar ao Vectorizer: {e}")
        print("[INFO] Verifique se o container está rodando: docker ps --filter name=vectorizer")
        return
    
    # Carregar chunks para identificar os livros
    print(f"\n[1/2] Carregando chunks para identificar livros...")
    if not CHUNKS_FILE.exists():
        print(f"[ERRO] Arquivo não encontrado: {CHUNKS_FILE}")
        return
    
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    all_chunks = data.get("chunks", [])
    total_chunks = len(all_chunks)
    print(f"[OK] Carregados {total_chunks} chunks")
    
    # Agrupar chunks por livro
    print(f"\n[2/2] Identificando livros e criando collections...")
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
    print(f"\nCriando collections...")
    collections_created = 0
    collections_existing = 0
    collections_failed = 0
    
    for collection_name in sorted(chunks_by_book.keys()):
        chunks = chunks_by_book[collection_name]
        title = chunks[0]["metadata"].get("title", "Sem título")
        
        print(f"\n[{collection_name}] {title}")
        if create_collection(collection_name, dimension=512):
            collections_created += 1
        else:
            # Verificar se já existe
            try:
                r = requests.get(f"{VECTORIZER_URL}/collections/{collection_name}", timeout=10)
                if r.status_code == 200:
                    collections_existing += 1
                else:
                    collections_failed += 1
            except:
                collections_failed += 1
    
    # Resumo
    print("\n" + "="*70)
    print("RESUMO")
    print("="*70)
    print(f"Total de livros: {len(chunks_by_book)}")
    print(f"Collections criadas: {collections_created}")
    print(f"Collections já existentes: {collections_existing}")
    print(f"Collections com erro: {collections_failed}")
    print()
    
    # Verificar collections criadas
    print("Verificando collections criadas...")
    try:
        r = requests.get(f"{VECTORIZER_URL}/collections", timeout=30)
        if r.status_code == 200:
            collections_data = r.json()
            # Verificar se é lista ou dict
            if isinstance(collections_data, list):
                collections = collections_data
            elif isinstance(collections_data, dict):
                # Pode ser um dict com uma chave 'collections'
                collections = collections_data.get("collections", [collections_data])
            else:
                print(f"[AVISO] Formato de resposta inesperado: {type(collections_data)}")
                collections = []
            
            dnd5e_collections = [c for c in collections if isinstance(c, dict) and c.get("name", "").startswith("dnd5e-")]
            print(f"\nCollections D&D 5e encontradas: {len(dnd5e_collections)}")
            for coll in sorted(dnd5e_collections, key=lambda x: x.get("name", "")):
                name = coll.get("name", "unknown")
                count = coll.get("vector_count", 0)
                print(f"  - {name}: {count} vetores")
    except Exception as e:
        print(f"[AVISO] Não foi possível verificar collections: {e}")
    
    print("\n[OK] Processo concluído!")
    print("\nPara indexar os chunks, execute:")
    print("  python scripts/create-collections-per-book.py")
    print()


if __name__ == "__main__":
    main()

