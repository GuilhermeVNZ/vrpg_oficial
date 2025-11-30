#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Cria uma collection separada para cada livro e indexa os chunks correspondentes
USANDO MCP (não REST API)
"""

import json
import asyncio
import sys
import uuid
import subprocess
from pathlib import Path
from typing import List, Dict
from collections import defaultdict
import re

# Configurar encoding UTF-8 para stdout
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')

# Configuração
CHUNKS_FILE = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service\chunks_for_vectorizer_all_books.json")
VECTORIZER_MCP_SCRIPT = Path(r"G:\vrpg\vrpg-client\vectorizer-mcp.py")
BATCH_SIZE = 50


def sanitize_collection_name(name: str) -> str:
    """Sanitiza o nome para ser usado como collection name"""
    name = name.replace(".pdf", "")
    name = re.sub(r'[^a-zA-Z0-9_-]', '-', name)
    name = re.sub(r'-+', '-', name)
    name = name.strip('-')
    if len(name) > 50:
        name = name[:50]
    return name.lower()


def get_collection_name_from_metadata(metadata: Dict) -> str:
    """Extrai o nome da collection do metadata do chunk"""
    title = metadata.get("title", "")
    if title:
        title = title.replace("D&D 5e - ", "").strip()
        collection_name = sanitize_collection_name(title)
        if collection_name:
            return f"dnd5e-{collection_name}"
    
    source_file = metadata.get("source_file", "")
    if source_file:
        collection_name = sanitize_collection_name(source_file)
        return f"dnd5e-{collection_name}"
    
    return "dnd5e-unknown"


def call_mcp_tool(tool_name: str, arguments: Dict) -> Dict:
    """Chama uma ferramenta MCP via JSON-RPC"""
    request = {
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": arguments
        },
        "id": 1
    }
    
    try:
        process = subprocess.Popen(
            ["python", str(VECTORIZER_MCP_SCRIPT)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        request_json = json.dumps(request) + "\n"
        stdout, stderr = process.communicate(input=request_json, timeout=30)
        
        if process.returncode == 0 and stdout:
            response = json.loads(stdout.strip())
            if "result" in response:
                content = response["result"].get("content", [])
                if content and len(content) > 0:
                    result_text = content[0].get("text", "{}")
                    return json.loads(result_text)
            elif "error" in response:
                print(f"    [ERRO] MCP: {response['error'].get('message', 'Erro desconhecido')}")
                return None
        else:
            print(f"    [ERRO] MCP process: {stderr}")
            return None
    except Exception as e:
        print(f"    [ERRO] Exceção ao chamar MCP: {e}")
        return None


def create_collection(collection_name: str, dimension: int = 512) -> bool:
    """Cria a collection no Vectorizer via MCP"""
    try:
        # Verificar se já existe via MCP
        collections_data = call_mcp_tool("vectorizer_list_collections", {})
        if collections_data:
            collections = collections_data.get("collections", [])
            if isinstance(collections, list):
                for coll in collections:
                    if isinstance(coll, dict) and coll.get("name") == collection_name:
                        print(f"[INFO] Collection '{collection_name}' já existe")
                        return True
        
        # Criar collection via REST (MCP não tem tool para criar)
        import requests
        url = "http://127.0.0.1:15002/collections"
        payload = {
            "name": collection_name,
            "dimension": dimension,
            "metric": "cosine"
        }
        
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


def insert_batch_via_mcp(collection: str, batch: List[Dict]) -> int:
    """Insere um lote de chunks no Vectorizer via MCP"""
    try:
        # Preparar vetores para MCP
        vectors = []
        for chunk in batch:
            text = chunk["text"]
            metadata = chunk["metadata"]
            chunk_id = str(uuid.uuid4())
            
            vectors.append({
                "id": chunk_id,
                "text": text,
                "metadata": metadata
            })
        
        # Chamar MCP tool
        result = call_mcp_tool("vectorizer_insert_texts", {
            "collection": collection,
            "vectors": vectors
        })
        
        if result:
            return len(vectors)
        else:
            return 0
    except Exception as e:
        print(f"    [ERRO] Exceção MCP batch: {e}")
        return 0


def index_book(collection_name: str, chunks: List[Dict]) -> int:
    """Indexa todos os chunks de um livro via MCP"""
    total_chunks = len(chunks)
    print(f"\n  Indexando {total_chunks} chunks em lotes de {BATCH_SIZE}...")
    
    inserted_total = 0
    for i in range(0, total_chunks, BATCH_SIZE):
        batch = chunks[i:i + BATCH_SIZE]
        batch_num = (i // BATCH_SIZE) + 1
        total_batches = (total_chunks + BATCH_SIZE - 1) // BATCH_SIZE
        
        print(f"    [{batch_num}/{total_batches}] Inserindo lote de {len(batch)} chunks...", end=" ")
        inserted = insert_batch_via_mcp(collection_name, batch)
        inserted_total += inserted
        print(f"[OK] {inserted}/{len(batch)} inseridos (Total: {inserted_total}/{total_chunks})")
        
        # Pequeno delay entre batches
        if i + BATCH_SIZE < total_chunks:
            import time
            time.sleep(1)
    
    return inserted_total


def main():
    """Função principal"""
    print("="*70)
    print("Criando Collections por Livro e Indexando Chunks (via MCP)")
    print("="*70)
    
    # Verificar Vectorizer via MCP
    print("\nVerificando Vectorizer via MCP...")
    try:
        health = call_mcp_tool("vectorizer_health_check", {})
        if health and health.get("status") == "healthy":
            print("[OK] Vectorizer está disponível via MCP")
        else:
            print("[ERRO] Vectorizer não está disponível via MCP")
            return
    except Exception as e:
        print(f"[ERRO] Não foi possível conectar ao Vectorizer via MCP: {e}")
        print("[INFO] Verifique se o MCP está configurado corretamente")
        return
    
    # Carregar chunks
    print(f"\n[1/4] Carregando chunks de: {CHUNKS_FILE}")
    if not CHUNKS_FILE.exists():
        print(f"[ERRO] Arquivo não encontrado: {CHUNKS_FILE}")
        return
    
    with open(CHUNKS_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    all_chunks = data.get("chunks", [])
    total_chunks = len(all_chunks)
    print(f"[OK] Carregados {total_chunks} chunks")
    
    # Agrupar chunks por livro
    print(f"\n[2/4] Agrupando chunks por livro...")
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
    print(f"\n[3/4] Criando collections...")
    collections_created = 0
    for collection_name in sorted(chunks_by_book.keys()):
        if create_collection(collection_name, dimension=512):
            collections_created += 1
    
    print(f"[OK] {collections_created}/{len(chunks_by_book)} collections criadas/verificadas")
    
    # Indexar cada livro
    print(f"\n[4/4] Indexando chunks por livro via MCP...")
    total_inserted = 0
    results = {}
    
    for collection_name in sorted(chunks_by_book.keys()):
        chunks = chunks_by_book[collection_name]
        title = chunks[0]["metadata"].get("title", "Sem título")
        print(f"\n[{collection_name}] {title}")
        
        inserted = index_book(collection_name, chunks)
        total_inserted += inserted
        results[collection_name] = {
            "title": title,
            "chunks": len(chunks),
            "inserted": inserted
        }
    
    # Verificação final
    print("\n" + "="*70)
    print("VERIFICAÇÃO FINAL")
    print("="*70)
    
    for collection_name, result in sorted(results.items()):
        print(f"\n{collection_name} ({result['title']}):")
        print(f"  Chunks preparados: {result['chunks']}")
        print(f"  Chunks inseridos: {result['inserted']}")
        
        # Verificar no Vectorizer via MCP
        try:
            coll_info = call_mcp_tool("vectorizer_get_collection_info", {"collection": collection_name})
            if coll_info:
                vector_count = coll_info.get("vector_count", 0)
                print(f"  Vetores na collection: {vector_count}")
                
                if vector_count >= result['inserted']:
                    print(f"  [OK] Status: OK")
                else:
                    print(f"  [AVISO] Status: Parcial ({vector_count}/{result['inserted']})")
            else:
                print(f"  [AVISO] Não foi possível verificar a collection")
        except Exception as e:
            print(f"  [AVISO] Erro ao verificar: {e}")
    
    print(f"\n{'='*70}")
    print(f"RESUMO GERAL")
    print(f"{'='*70}")
    print(f"Total de livros: {len(chunks_by_book)}")
    print(f"Total de chunks preparados: {total_chunks}")
    print(f"Total de chunks inseridos: {total_inserted}")
    print(f"Taxa de sucesso: {(total_inserted/total_chunks*100):.1f}%")
    print()


if __name__ == "__main__":
    main()
