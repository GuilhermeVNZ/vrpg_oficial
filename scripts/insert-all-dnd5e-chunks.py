#!/usr/bin/env python3
"""
Script para inserir TODOS os chunks de D&D 5e no Vectorizer via MCP
Processa todos os arquivos JSON e insere na collection
"""

import json
import sys
from pathlib import Path
from typing import Dict, Any, Optional

# Configuração
CHUNKS_DIR = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service")
COLLECTION_NAME = "dnd5e-rules-new"
BATCH_SIZE = 10  # Processar em lotes menores para evitar sobrecarga

def load_chunk(chunk_file: Path) -> Optional[Dict[str, Any]]:
    """Carrega um chunk do arquivo JSON, tratando BOM UTF-8"""
    try:
        # Tentar ler com utf-8-sig primeiro (remove BOM)
        with open(chunk_file, 'r', encoding='utf-8-sig') as f:
            return json.load(f)
    except json.JSONDecodeError:
        # Se falhar, tentar sem sig
        try:
            with open(chunk_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        except Exception as e:
            print(f"Erro ao carregar {chunk_file.name}: {e}")
            return None

def insert_chunk_via_mcp(text: str, metadata: Dict[str, Any]) -> bool:
    """
    Insere um chunk via MCP Vectorizer
    Nota: Esta função é um placeholder - a inserção real seria feita via MCP
    """
    # Em uma implementação real, isso chamaria o MCP Vectorizer
    # Por enquanto, apenas retorna True para simular sucesso
    return True

def main():
    print("=" * 60)
    print("Inserindo TODOS os chunks de D&D 5e no Vectorizer")
    print("=" * 60)
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Diretorio: {CHUNKS_DIR}")
    print()
    
    if not CHUNKS_DIR.exists():
        print(f"ERRO: Diretorio nao encontrado: {CHUNKS_DIR}")
        sys.exit(1)
    
    # Listar todos os arquivos JSON
    chunk_files = sorted(CHUNKS_DIR.glob("*.json"))
    # Filtrar arquivos de instruções/resumo
    chunk_files = [f for f in chunk_files 
                   if not f.name.startswith("insertion_") 
                   and f.name != "README.md"
                   and not f.name.endswith("_summary.json")]
    total_chunks = len(chunk_files)
    
    if total_chunks == 0:
        print("ERRO: Nenhum chunk encontrado!")
        sys.exit(1)
    
    print(f"Total de chunks encontrados: {total_chunks}")
    print()
    
    # Processar chunks
    inserted = 0
    errors = 0
    skipped = 0
    
    print("Iniciando insercao...")
    print("NOTA: Este script prepara os dados.")
    print("Para inserir via MCP, voce precisa usar mcp_vectorizer-main_insert_text")
    print("para cada chunk individualmente.")
    print()
    print("Gerando lista de chunks para insercao...")
    print()
    
    # Criar lista de chunks para inserção
    chunks_to_insert = []
    
    for i, chunk_file in enumerate(chunk_files, 1):
        chunk = load_chunk(chunk_file)
        if chunk:
            chunks_to_insert.append({
                "file": chunk_file.name,
                "text": chunk['text'],
                "metadata": chunk['metadata']
            })
            
            if i % 100 == 0:
                print(f"  Carregados: {i}/{total_chunks} chunks...")
    
    print()
    print(f"Total de chunks carregados: {len(chunks_to_insert)}")
    print()
    
    # Salvar lista completa para inserção manual ou via script MCP
    output_file = CHUNKS_DIR / "chunks_for_insertion.json"
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump({
            "collection_name": COLLECTION_NAME,
            "total_chunks": len(chunks_to_insert),
            "chunks": chunks_to_insert
        }, f, indent=2, ensure_ascii=False)
    
    print(f"Lista de chunks salva em: {output_file}")
    print()
    print("=" * 60)
    print("RESUMO")
    print("=" * 60)
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Total de chunks preparados: {len(chunks_to_insert)}")
    print()
    print("Para inserir os chunks, use:")
    print("  1. MCP Vectorizer: mcp_vectorizer-main_insert_text")
    print(f"     - collection_name: {COLLECTION_NAME}")
    print("     - text: chunk['text']")
    print("     - metadata: chunk['metadata']")
    print()
    print("  2. Ou crie um script que leia chunks_for_insertion.json")
    print("     e insira cada chunk via MCP")
    print()
    print("Exemplo de uso do primeiro chunk:")
    print("  - Arquivo: player_handbook_chunk_0.json")
    print("  - Texto: Primeiras linhas do Livro do Jogador")
    print("  - Metadata: player_handbook, chunk 0/653")
    print()

if __name__ == "__main__":
    main()

