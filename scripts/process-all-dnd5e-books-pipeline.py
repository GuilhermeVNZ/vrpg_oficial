#!/usr/bin/env python3
"""
Pipeline completo para processar TODOS os PDFs de D&D 5e:
1. Transmutation: Converter PDFs para Markdown
2. Classify: Classificar o conteúdo  
3. Vectorizer: Indexar no Vectorizer via MCP

Uso: python scripts/process-all-dnd5e-books-pipeline.py
"""

import json
import os
import sys
import asyncio
import subprocess
import aiohttp
from pathlib import Path
from typing import Dict, List, Optional, Tuple
import hashlib
import re
import math
from collections import Counter, defaultdict

# Configuração
SOURCE_DIR = Path(r"G:\vrpg\vrpg-client\assets-and-models\books")
OUTPUT_DIR = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service\specs\rules5e-service")
COLLECTION_NAME = "dnd5e-rules-new"
CHUNK_SIZE = 2000  # Tamanho dos chunks em caracteres
VECTORIZER_URL = "http://127.0.0.1:15002"

# Lista de PDFs para processar (6 livros)
PDFS = [
    {
        "file": "old-dd-5e-manual-dos-monstros-biblioteca-elfica.pdf",
        "type": "monster_manual",
        "title": "D&D 5e - Manual dos Monstros"
    },
    {
        "file": "dd-5e-livro-do-jogador-fundo-branco-biblioteca-elfica.pdf",
        "type": "player_handbook",
        "title": "D&D 5e - Livro do Jogador"
    },
    {
        "file": "dd-5e-guia-do-mestre-biblioteca-elfica.pdf",
        "type": "dungeon_master_guide",
        "title": "D&D 5e - Guia do Mestre"
    },
    {
        "file": "dd-5e-ficha-de-personagem-completavel-biblioteca-elfica.pdf",
        "type": "character_sheet",
        "title": "D&D 5e - Ficha de Personagem"
    },
    {
        "file": "dd-5e-guia-de-xanathar-para-todas-as-coisas-fundo-branco-biblioteca-elfica.pdf",
        "type": "supplement",
        "title": "D&D 5e - Guia de Xanathar para Todas as Coisas"
    },
    {
        "file": "dd-5e-guia-do-volo-para-monstros-v-alta-resolucao-biblioteca-elfica.pdf",
        "type": "monster_manual",
        "title": "D&D 5e - Guia do Volo para Monstros"
    }
]


def check_transmutation_available() -> bool:
    """Verifica se transmutation CLI está disponível"""
    try:
        result = subprocess.run(
            ["transmutation", "--version"],
            capture_output=True,
            text=True,
            timeout=5
        )
        return result.returncode == 0
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return False


def extract_pdf_with_transmutation(pdf_path: Path, output_md: Path) -> Optional[Dict]:
    """Extrai texto de PDF usando Transmutation CLI"""
    try:
        result = subprocess.run(
            [
                "transmutation", "convert",
                str(pdf_path),
                "-o", str(output_md),
                "-f", "markdown",
                "--optimize-llm",
                "--normalize-whitespace"
            ],
            capture_output=True,
            text=True,
            timeout=300  # 5 minutos timeout
        )
        
        if result.returncode == 0 and output_md.exists():
            markdown = output_md.read_text(encoding='utf-8')
            pages = markdown.count("# Página") or markdown.count("# Page") or 1
            return {
                "success": True,
                "pages": pages,
                "markdown": markdown,
                "method": "transmutation"
            }
        else:
            print(f"  [ERRO] Transmutation falhou: {result.stderr}")
            return None
    except subprocess.TimeoutExpired:
        print("  [ERRO] Transmutation timeout")
        return None
    except Exception as e:
        print(f"  [ERRO] Erro ao usar Transmutation: {e}")
        return None


def extract_pdf_with_python(pdf_path: Path) -> Optional[Dict]:
    """Fallback: Extrai texto de PDF usando Python (pdfplumber)"""
    try:
        import pdfplumber
        
        pages = []
        text_parts = []
        
        with pdfplumber.open(str(pdf_path)) as pdf:
            for i, page in enumerate(pdf.pages, 1):
                text = page.extract_text()
                if text:
                    text = text.strip()
                    pages.append({"number": i, "text": text})
                    text_parts.append(f"# Página {i}\n\n{text}\n\n")
        
        markdown = '\n'.join(text_parts)
        return {
            "success": True,
            "pages": len(pages),
            "markdown": markdown,
            "method": "pdfplumber"
        }
    except ImportError:
        print("  [ERRO] pdfplumber não está instalado. Execute: pip install pdfplumber")
        return None
    except Exception as e:
        print(f"  [ERRO] Erro ao extrair PDF: {e}")
        return None


def classify_text_simple(text: str, doc_type: str) -> Dict:
    """Classificação simples baseada no tipo de documento"""
    categories_map = {
        "player_handbook": ["dnd5e", "rules", "player_guide", "game_rules"],
        "dungeon_master_guide": ["dnd5e", "rules", "dm_guide", "game_master"],
        "monster_manual": ["dnd5e", "monsters", "creatures", "bestiary"],
        "character_sheet": ["dnd5e", "character", "sheet", "form"],
        "supplement": ["dnd5e", "rules", "supplement", "expansion"]
    }
    
    return {
        "domain": "gaming",
        "doc_type": doc_type,
        "categories": categories_map.get(doc_type, ["dnd5e", "rules"]),
        "confidence": 0.95,
        "metadata": {
            "game_system": "dnd5e",
            "language": "pt-BR",
            "source": "biblioteca_elfica"
        }
    }


def split_into_chunks(text: str, chunk_size: int) -> List[Dict]:
    """Divide o texto em chunks inteligentes"""
    chunks = []
    paragraphs = text.split('\n\n')
    
    current_chunk = ""
    current_size = 0
    chunk_index = 0
    
    for para in paragraphs:
        para_size = len(para)
        
        if current_size + para_size > chunk_size and current_chunk:
            chunks.append({
                "text": current_chunk.strip(),
                "index": chunk_index,
                "start": len(chunks) * chunk_size,
                "end": len(chunks) * chunk_size + len(current_chunk)
            })
            chunk_index += 1
            current_chunk = para + "\n\n"
            current_size = para_size
        else:
            current_chunk += para + "\n\n"
            current_size += para_size
    
    if current_chunk.strip():
        chunks.append({
            "text": current_chunk.strip(),
            "index": chunk_index,
            "start": len(chunks) * chunk_size,
            "end": len(chunks) * chunk_size + len(current_chunk)
        })
    
    return chunks


async def insert_batch_to_vectorizer(collection: str, chunks_batch: List[Dict]) -> int:
    """Insere um lote de chunks no Vectorizer via API REST usando /collections/{name}/batch_insert"""
    if not chunks_batch:
        return 0
    
    try:
        async with aiohttp.ClientSession() as session:
            # Preparar vetores no formato correto para batch_insert
            vectors = []
            for chunk in chunks_batch:
                text = chunk["text"]
                metadata = chunk["metadata"]
                
                # Gerar ID único baseado no hash do texto
                text_hash = hashlib.md5(text.encode('utf-8')).hexdigest()
                chunk_id = f"{metadata['source_file']}_{metadata['chunk_index']}_{text_hash[:8]}"
                
                # Formato para batch_insert: apenas text e metadata (id opcional)
                vector_item = {
                    "text": text,
                    "metadata": metadata
                }
                # Adicionar id se quiser controle sobre o ID gerado
                if chunk_id:
                    vector_item["id"] = chunk_id
                
                vectors.append(vector_item)
            
            # Endpoint correto: POST /collections/{name}/batch_insert
            payload = {"vectors": vectors}
            url = f"{VECTORIZER_URL}/collections/{collection}/batch_insert"
            
            async with session.post(
                url, 
                json=payload, 
                headers={"Content-Type": "application/json"},
                timeout=aiohttp.ClientTimeout(total=180)
            ) as response:
                if response.status == 200:
                    result = await response.json()
                    # A resposta pode ter "inserted" ou "ids" ou apenas sucesso
                    inserted = result.get("inserted", result.get("count", len(vectors)))
                    return inserted
                else:
                    error_text = await response.text()
                    print(f"  [ERRO] Batch insert falhou: {response.status} - {error_text[:300]}")
                    return 0
    except Exception as e:
        print(f"  [ERRO] Exceção no batch insert: {e}")
        return 0


async def process_pdf(pdf_info: Dict, use_vectorizer: bool = True) -> Dict:
    """Processa um PDF completo: transmutation -> classify -> vectorizer"""
    pdf_path = SOURCE_DIR / pdf_info["file"]
    
    if not pdf_path.exists():
        return {
            "success": False,
            "error": f"PDF não encontrado: {pdf_path}"
        }
    
    print(f"\n{'='*60}")
    print(f"Processando: {pdf_info['title']}")
    print(f"Arquivo: {pdf_info['file']}")
    print(f"{'='*60}")
    
    # Criar diretório de output
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    output_md = OUTPUT_DIR / f"{pdf_info['type']}.md"
    
    # Passo 1: Transmutation (extrair texto do PDF)
    print("\n[1/3] Transmutation: Extraindo texto do PDF...")
    
    extraction_result = None
    if check_transmutation_available():
        print("  [INFO] Usando Transmutation CLI...")
        extraction_result = extract_pdf_with_transmutation(pdf_path, output_md)
    else:
        print("  [INFO] Transmutation não disponível, usando fallback (pdfplumber)...")
        extraction_result = extract_pdf_with_python(pdf_path)
        if extraction_result and extraction_result.get("success"):
            # Salvar markdown
            output_md.write_text(extraction_result["markdown"], encoding='utf-8')
    
    if not extraction_result or not extraction_result.get("success"):
        return {
            "success": False,
            "error": "Falha na extração do PDF"
        }
    
    markdown = extraction_result["markdown"]
    pages = extraction_result["pages"]
    method = extraction_result.get("method", "unknown")
    
    print(f"  [OK] Extraido: {pages} paginas ({method})")
    print(f"  [OK] Tamanho: {len(markdown):,} caracteres")
    
    # Passo 2: Classify (classificar o conteúdo)
    print(f"\n[2/3] Classify: Classificando conteudo...")
    classification = classify_text_simple(markdown, pdf_info["type"])
    print(f"  [OK] Dominio: {classification['domain']}")
    print(f"  [OK] Tipo: {classification['doc_type']}")
    print(f"  [OK] Categorias: {', '.join(classification['categories'])}")
    print(f"  [OK] Confianca: {classification['confidence']:.2f}")
    
    # Passo 3: Dividir em chunks
    print(f"\n[3/3] Preparando chunks para Vectorizer...")
    chunks = split_into_chunks(markdown, CHUNK_SIZE)
    print(f"  [OK] Criados {len(chunks)} chunks")
    
    # Preparar dados para inserção
    chunks_data = []
    
    for chunk in chunks:
        chunk_metadata = {
            **classification["metadata"],
            "source_file": pdf_info["file"],
            "document_type": pdf_info["type"],
            "title": pdf_info["title"],
            "chunk_index": chunk["index"],
            "total_chunks": len(chunks),
            "categories": classification["categories"],
            "confidence": classification["confidence"]
        }
        
        chunks_data.append({
            "text": chunk["text"],
            "metadata": chunk_metadata
        })
    
    # Inserir no Vectorizer usando batch insert (mais eficiente)
    inserted_count = 0
    if use_vectorizer and chunks_data:
        print(f"  [INFO] Inserindo {len(chunks_data)} chunks via batch insert...")
        BATCH_SIZE = 100  # Processar em lotes de 100
        
        for i in range(0, len(chunks_data), BATCH_SIZE):
            batch = chunks_data[i:i + BATCH_SIZE]
            batch_inserted = await insert_batch_to_vectorizer(COLLECTION_NAME, batch)
            inserted_count += batch_inserted
            
            if (i + BATCH_SIZE) < len(chunks_data):
                print(f"    [PROGRESSO] {inserted_count}/{len(chunks_data)} chunks inseridos...")
                await asyncio.sleep(0.5)  # Pequeno delay entre batches
        
        print(f"  [OK] {inserted_count}/{len(chunks_data)} chunks inseridos no Vectorizer")
    else:
        print(f"  [OK] {len(chunks_data)} chunks preparados para inserção")
    
    return {
        "success": True,
        "pdf": pdf_info["file"],
        "pages": pages,
        "chunks": len(chunks),
        "inserted": inserted_count if use_vectorizer else 0,
        "classification": classification,
        "chunks_data": chunks_data
    }


async def main():
    """Função principal"""
    print("="*60)
    print("Pipeline D&D 5e: Transmutation -> Classify -> Vectorizer")
    print("="*60)
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Source: {SOURCE_DIR}")
    print(f"Output: {OUTPUT_DIR}")
    print()
    
    # Verificar se Vectorizer está disponível
    vectorizer_available = False
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(f"{VECTORIZER_URL}/health", timeout=aiohttp.ClientTimeout(total=5)) as response:
                if response.status == 200:
                    vectorizer_available = True
                    print(f"[OK] Vectorizer está disponível em {VECTORIZER_URL}")
    except Exception as e:
        print(f"[AVISO] Vectorizer não disponível: {e}")
        print("  Chunks serão preparados mas NÃO inseridos automaticamente")
    
    print()
    
    all_chunks = []
    results = []
    
    # Processar cada PDF
    for pdf_info in PDFS:
        result = await process_pdf(pdf_info, use_vectorizer=vectorizer_available)
        results.append(result)
        
        if result["success"]:
            all_chunks.extend(result["chunks_data"])
            status = f"{result['inserted']}/{result['chunks']} inseridos" if vectorizer_available else f"{result['chunks']} preparados"
            print(f"\n[OK] {pdf_info['title']}: {status}")
        else:
            print(f"\n[ERRO] {pdf_info['title']}: {result.get('error', 'Erro desconhecido')}")
    
    # Resumo
    print("\n" + "="*60)
    print("RESUMO")
    print("="*60)
    total_chunks = len(all_chunks)
    successful = sum(1 for r in results if r.get("success"))
    total_inserted = sum(r.get("inserted", 0) for r in results if r.get("success"))
    
    print(f"PDFs processados: {successful}/{len(PDFS)}")
    print(f"Total de chunks: {total_chunks}")
    if vectorizer_available:
        print(f"Chunks inseridos no Vectorizer: {total_inserted}/{total_chunks}")
    print(f"Collection: {COLLECTION_NAME}")
    print()
    
    # Salvar chunks para inserção (backup)
    output_file = OUTPUT_DIR / "chunks_for_vectorizer_all_books.json"
    
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump({
            "collection": COLLECTION_NAME,
            "total_chunks": total_chunks,
            "total_inserted": total_inserted if vectorizer_available else 0,
            "chunks": all_chunks
        }, f, indent=2, ensure_ascii=False)
    
    print(f"[OK] Chunks salvos em: {output_file}")
    print()
    
    if not vectorizer_available:
        print("PROXIMOS PASSOS:")
        print("1. Certifique-se de que o Vectorizer está rodando")
        print("2. Execute este script novamente para inserir os chunks")
        print(f"3. Ou use os chunks salvos em: {output_file}")
    else:
        print("[OK] Processamento completo!")
        print(f"   Collection '{COLLECTION_NAME}' agora contém {total_inserted} vetores")


if __name__ == "__main__":
    asyncio.run(main())

