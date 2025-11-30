#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Reprocessa PDFs específicos usando pdfplumber como fallback
"""

import sys
from pathlib import Path

# Adicionar o diretório do script ao path
sys.path.insert(0, str(Path(__file__).parent))

from process_all_books_pipeline import (
    extract_pdf_with_python,
    find_classify_path,
    classify_with_cli,
    classify_text_simple,
    sanitize_filename,
    OUTPUT_DIR
)

SOURCE_DIR = Path(r"G:\vrpg\vrpg-client\assets-and-models\books")
OUTPUT_DIR = Path(r"G:\vrpg\vrpg-client\assets-and-models\books\processed")


def reprocess_with_pdfplumber(pdf_path: Path):
    """Reprocessa um PDF usando pdfplumber como fallback"""
    pdf_filename = pdf_path.name
    print(f"\n{'='*60}")
    print(f"Reprocessando com pdfplumber: {pdf_filename}")
    print(f"{'='*60}")
    
    # Nome do arquivo MD de saída
    md_filename = sanitize_filename(pdf_filename) + ".md"
    output_md = OUTPUT_DIR / md_filename
    
    # Remover arquivo anterior
    if output_md.exists():
        output_md.unlink()
        metadata_file = OUTPUT_DIR / (md_filename.replace('.md', '.metadata.json'))
        if metadata_file.exists():
            metadata_file.unlink()
    
    # Extrair com pdfplumber
    print("\n[1/2] Extraindo com pdfplumber...")
    extraction_result = extract_pdf_with_python(pdf_path)
    
    if not extraction_result or not extraction_result.get("success"):
        print(f"  [ERRO] Falha na extração: {extraction_result.get('error', 'Erro desconhecido') if extraction_result else 'Nenhum resultado'}")
        return {"success": False, "error": "Falha na extração com pdfplumber"}
    
    markdown = extraction_result["markdown"]
    pages = extraction_result["pages"]
    
    print(f"  [OK] Extraido: {pages} paginas")
    print(f"  [OK] Tamanho: {len(markdown):,} caracteres")
    
    # Salvar arquivo MD
    output_md.write_text(markdown, encoding='utf-8')
    print(f"  [OK] Arquivo salvo: {output_md.name}")
    
    # Classify
    print(f"\n[2/2] Classify: Classificando conteudo...")
    
    classify_path = find_classify_path()
    classification_result = None
    
    if classify_path:
        classification_result = classify_with_cli(output_md)
    
    if not classification_result or not classification_result.get("success"):
        print("  [AVISO] Classify CLI não disponível, usando classificação simples...")
        classification = classify_text_simple(markdown, pdf_filename)
    else:
        classification_data = classification_result.get("classification", {})
        if classification_data:
            classification = {
                "domain": classification_data.get("classification", {}).get("domain", "unknown"),
                "doc_type": classification_data.get("classification", {}).get("doc_type", "book"),
                "categories": classification_data.get("classification", {}).get("categories", []),
                "confidence": classification_data.get("classification", {}).get("confidence", 0.85),
                "metadata": classification_data.get("classification", {}).get("metadata", {})
            }
        else:
            classification = classify_text_simple(markdown, pdf_filename)
    
    print(f"  [OK] Dominio: {classification.get('domain', 'unknown')}")
    print(f"  [OK] Tipo: {classification.get('doc_type', 'unknown')}")
    
    # Salvar metadados
    import json
    metadata_file = OUTPUT_DIR / (md_filename.replace('.md', '.metadata.json'))
    with open(metadata_file, 'w', encoding='utf-8') as f:
        json.dump({
            "source_pdf": pdf_filename,
            "output_md": md_filename,
            "pages": pages,
            "classification": classification,
            "extraction_method": "pdfplumber"
        }, f, indent=2, ensure_ascii=False)
    
    print(f"  [OK] Metadados salvos")
    
    return {
        "success": True,
        "pdf": pdf_filename,
        "md_file": md_filename,
        "pages": pages,
        "classification": classification
    }


def main():
    """Reprocessa os arquivos 1 e 2"""
    print("="*60)
    print("Reprocessamento com pdfplumber: A Guerra dos Tronos 1 e 2")
    print("="*60)
    
    pdfs = [
        SOURCE_DIR / "a-guerra-dos-tronos-01-biblioteca-elfica.pdf",
        SOURCE_DIR / "a-guerra-dos-tronos-02-biblioteca-elfica.pdf"
    ]
    
    results = []
    
    for pdf_path in pdfs:
        if not pdf_path.exists():
            print(f"\n[ERRO] PDF não encontrado: {pdf_path.name}")
            results.append({"success": False, "pdf": pdf_path.name, "error": "PDF não encontrado"})
            continue
        
        result = reprocess_with_pdfplumber(pdf_path)
        results.append(result)
        
        if result["success"]:
            print(f"\n[OK] {pdf_path.name} → {result['md_file']}")
        else:
            print(f"\n[ERRO] {pdf_path.name}: {result.get('error', 'Erro desconhecido')}")
    
    # Resumo
    print("\n" + "="*60)
    print("RESUMO")
    print("="*60)
    successful = sum(1 for r in results if r.get("success"))
    print(f"Sucessos: {successful}/{len(pdfs)}")
    print()


if __name__ == "__main__":
    main()




