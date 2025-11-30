#!/usr/bin/env python3
"""
Pipeline completo para processar PDFs de D&D 5e:
1. Transmutation: Converter PDFs para Markdown
2. Classify: Classificar o conteúdo
3. Vectorizer: Indexar no Vectorizer via MCP

Uso: python scripts/process-dnd5e-full-pipeline.py
"""

import json
import os
import sys
import asyncio
from pathlib import Path
from typing import Dict, List, Optional

# Configuração
SOURCE_DIR = Path(r"G:\vrpg\vrpg-client\rulebook\tasks\implement-rules5e-service")
COLLECTION_NAME = "dnd5e-rules-temp"  # Usar a collection temporária com dimensão 512
CHUNK_SIZE = 2000  # Tamanho dos chunks em caracteres

# Lista de PDFs para processar
PDFS = [
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
        "file": "old-dd-5e-manual-dos-monstros-biblioteca-elfica.pdf",
        "type": "monster_manual",
        "title": "D&D 5e - Manual dos Monstros"
    },
    {
        "file": "dd-5e-ficha-de-personagem-completavel-biblioteca-elfica.pdf",
        "type": "character_sheet",
        "title": "D&D 5e - Ficha de Personagem"
    }
]


def extract_pdf_with_python(pdf_path: Path) -> Optional[Dict]:
    """Extrai texto de PDF usando Python (pdfplumber)"""
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
        print("ERRO: pdfplumber não está instalado. Execute: pip install pdfplumber")
        return None
    except Exception as e:
        print(f"ERRO ao extrair PDF: {e}")
        return None


def classify_text_simple(text: str, doc_type: str) -> Dict:
    """Classificação simples baseada no tipo de documento"""
    # Classificação básica para D&D 5e
    categories = {
        "player_handbook": ["dnd5e", "rules", "player_guide", "game_rules"],
        "dungeon_master_guide": ["dnd5e", "rules", "dm_guide", "game_master"],
        "monster_manual": ["dnd5e", "monsters", "creatures", "bestiary"],
        "character_sheet": ["dnd5e", "character", "sheet", "form"]
    }
    
    return {
        "domain": "gaming",
        "doc_type": doc_type,
        "categories": categories.get(doc_type, ["dnd5e", "rules"]),
        "confidence": 0.95,
        "metadata": {
            "game_system": "dnd5e",
            "language": "pt-BR",
            "source": "biblioteca_elfica"
        }
    }


def split_into_chunks(text: str, chunk_size: int) -> List[Dict]:
    """Divide o texto em chunks"""
    chunks = []
    for i in range(0, len(text), chunk_size):
        chunk_text = text[i:min(i + chunk_size, len(text))]
        chunks.append({
            "text": chunk_text,
            "index": len(chunks),
            "start": i,
            "end": min(i + chunk_size, len(text))
        })
    return chunks


async def insert_chunk_to_vectorizer(collection: str, text: str, metadata: Dict) -> bool:
    """Insere um chunk no Vectorizer via MCP"""
    # Esta função será chamada via MCP Vectorizer
    # Por enquanto, retorna True (será implementada via MCP)
    return True


async def process_pdf(pdf_info: Dict) -> Dict:
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
    
    # Passo 1: Transmutation (extrair texto do PDF)
    print("\n[1/3] Transmutation: Extraindo texto do PDF...")
    extraction_result = extract_pdf_with_python(pdf_path)
    
    if not extraction_result or not extraction_result.get("success"):
        return {
            "success": False,
            "error": "Falha na extração do PDF"
        }
    
    markdown = extraction_result["markdown"]
    pages = extraction_result["pages"]
    print(f"  [OK] Extraido: {pages} paginas")
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
    
    return {
        "success": True,
        "pdf": pdf_info["file"],
        "pages": pages,
        "chunks": len(chunks),
        "classification": classification,
        "chunks_data": chunks_data
    }


async def main():
    """Função principal"""
    print("="*60)
    print("Pipeline D&D 5e: Transmutation -> Classify -> Vectorizer")
    print("="*60)
    print(f"Collection: {COLLECTION_NAME}")
    print(f"Diretório: {SOURCE_DIR}")
    print()
    
    all_chunks = []
    results = []
    
    # Processar cada PDF
    for pdf_info in PDFS:
        result = await process_pdf(pdf_info)
        results.append(result)
        
        if result["success"]:
            all_chunks.extend(result["chunks_data"])
            print(f"\n[OK] {pdf_info['title']}: {result['chunks']} chunks preparados")
        else:
            print(f"\n[ERRO] {pdf_info['title']}: {result.get('error', 'Erro desconhecido')}")
    
    # Resumo
    print("\n" + "="*60)
    print("RESUMO")
    print("="*60)
    total_chunks = len(all_chunks)
    successful = sum(1 for r in results if r.get("success"))
    
    print(f"PDFs processados: {successful}/{len(PDFS)}")
    print(f"Total de chunks: {total_chunks}")
    print(f"Collection: {COLLECTION_NAME}")
    print()
    
    # Salvar chunks para inserção
    output_file = SOURCE_DIR / "specs" / "rules5e-service" / "chunks_for_vectorizer.json"
    output_file.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump({
            "collection": COLLECTION_NAME,
            "total_chunks": total_chunks,
            "chunks": all_chunks
        }, f, indent=2, ensure_ascii=False)
    
    print(f"[OK] Chunks salvos em: {output_file}")
    print()
    print("PROXIMOS PASSOS:")
    print("1. Inserir chunks no Vectorizer via MCP")
    print(f"2. Collection: {COLLECTION_NAME}")
    print(f"3. Total: {total_chunks} chunks para inserir")
    print()
    print("Use mcp_vectorizer-main_insert_text para cada chunk")
    print()


if __name__ == "__main__":
    asyncio.run(main())

