#!/usr/bin/env python3
"""
Verifica o status de todas as collections de livros D&D 5e
"""

import json
import requests
import sys
from pathlib import Path

# Configuração
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")
VECTORIZER_URL = "http://127.0.0.1:15002"

# Mapeamento esperado de livros
EXPECTED_BOOKS = {
    "dnd5e-manual-dos-monstros": "Manual dos Monstros",
    "dnd5e-livro-do-jogador": "Livro do Jogador",
    "dnd5e-guia-do-mestre": "Guia do Mestre",
    "dnd5e-ficha-de-personagem": "Ficha de Personagem",
    "dnd5e-guia-de-xanathar-para-todas-as-coisas": "Guia de Xanathar",
    "dnd5e-guia-do-volo-para-monstros": "Guia do Volo para Monstros"
}


def get_chunks_by_book():
    """Agrupa chunks por livro"""
    if not CHUNKS_FILE.exists():
        return {}
    
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    chunks = data.get("chunks", [])
    chunks_by_book = {}
    
    for chunk in chunks:
        metadata = chunk.get("metadata", {})
        source_file = metadata.get("source_file", "")
        
        # Determinar nome da collection baseado no source_file
        if "manual-dos-monstros" in source_file.lower():
            collection_name = "dnd5e-manual-dos-monstros"
        elif "livro-do-jogador" in source_file.lower():
            collection_name = "dnd5e-livro-do-jogador"
        elif "guia-do-mestre" in source_file.lower():
            collection_name = "dnd5e-guia-do-mestre"
        elif "ficha-de-personagem" in source_file.lower():
            collection_name = "dnd5e-ficha-de-personagem"
        elif "xanathar" in source_file.lower():
            collection_name = "dnd5e-guia-de-xanathar-para-todas-as-coisas"
        elif "volo" in source_file.lower():
            collection_name = "dnd5e-guia-do-volo-para-monstros"
        else:
            continue
        
        if collection_name not in chunks_by_book:
            chunks_by_book[collection_name] = []
        chunks_by_book[collection_name].append(chunk)
    
    return chunks_by_book


def main():
    """Função principal"""
    sys.stdout.reconfigure(encoding='utf-8')
    
    print("="*70)
    print("STATUS DAS COLLECTIONS - D&D 5e Livros")
    print("="*70)
    print()
    
    # Verificar Vectorizer
    try:
        r = requests.get(f"{VECTORIZER_URL}/health", timeout=30)
        if r.status_code == 200:
            print("[OK] Vectorizer: healthy")
        else:
            print(f"[ERRO] Vectorizer: Status {r.status_code}")
            return
    except Exception as e:
        print(f"[ERRO] Vectorizer: Erro ao conectar - {e}")
        return
    
    print()
    
    # Carregar chunks esperados
    chunks_by_book = get_chunks_by_book()
    
    # Listar collections
    try:
        r = requests.get(f"{VECTORIZER_URL}/collections", timeout=30)
        if r.status_code != 200:
            print(f"[ERRO] Não foi possível listar collections: {r.status_code}")
            return
        
        collections_data = r.json()
        # Verificar se é lista ou dict
        if isinstance(collections_data, list):
            collections = collections_data
        elif isinstance(collections_data, dict):
            # Pode ser um dict com uma chave 'collections'
            collections = collections_data.get("collections", [collections_data])
        else:
            print(f"[ERRO] Formato de resposta inesperado: {type(collections_data)}")
            return
        
        collection_dict = {c["name"]: c for c in collections if isinstance(c, dict) and "name" in c}
        
        print("COLLECTIONS:")
        print("-"*70)
        
        total_expected = 0
        total_indexed = 0
        
        for collection_name, book_title in sorted(EXPECTED_BOOKS.items()):
            expected_chunks = len(chunks_by_book.get(collection_name, []))
            total_expected += expected_chunks
            
            if collection_name in collection_dict:
                collection = collection_dict[collection_name]
                vector_count = collection.get("vector_count", 0)
                total_indexed += vector_count
                
                status = "✓" if vector_count >= expected_chunks else "⚠"
                percentage = (vector_count / expected_chunks * 100) if expected_chunks > 0 else 0
                
                print(f"{status} {collection_name}")
                print(f"   Título: {book_title}")
                print(f"   Chunks esperados: {expected_chunks}")
                print(f"   Vetores indexados: {vector_count}")
                print(f"   Progresso: {percentage:.1f}%")
                print()
            else:
                print(f"✗ {collection_name} - NÃO CRIADA")
                print(f"   Título: {book_title}")
                print(f"   Chunks esperados: {expected_chunks}")
                print()
        
        print("="*70)
        print("RESUMO")
        print("="*70)
        print(f"Total de chunks esperados: {total_expected}")
        print(f"Total de vetores indexados: {total_indexed}")
        print(f"Progresso geral: {(total_indexed/total_expected*100):.1f}%")
        print()
        
        if total_indexed >= total_expected:
            print("[OK] Todas as collections foram indexadas com sucesso!")
        else:
            print(f"[AGUARDANDO] Indexação em progresso... ({total_indexed}/{total_expected})")
        
    except Exception as e:
        print(f"[ERRO] Erro ao verificar collections: {e}")


if __name__ == "__main__":
    main()

