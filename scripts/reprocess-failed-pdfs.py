#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Reprocessa PDFs que falharam (geraram MDs vazios)
Tenta diferentes estratégias do transmutation
"""

import json
import subprocess
import sys
from pathlib import Path

# Adicionar o diretório do script ao path
sys.path.insert(0, str(Path(__file__).parent))

# Importar funções do script principal
from process_all_books_pipeline import (
    find_transmutation_path,
    find_classify_path,
    classify_with_cli,
    classify_text_simple,
    sanitize_filename,
    OUTPUT_DIR
)

SOURCE_DIR = Path(r"G:\vrpg\vrpg-client\assets-and-models\books")
OUTPUT_DIR = Path(r"G:\vrpg\vrpg-client\assets-and-models\books\processed")


def extract_with_transmutation_strategies(pdf_path: Path, output_md: Path) -> dict:
    """Tenta extrair PDF usando diferentes estratégias do transmutation"""
    transmutation_path = find_transmutation_path()
    if not transmutation_path:
        return {"success": False, "error": "Transmutation não encontrado"}
    
    strategies = [
        ("precision", ["--precision"]),
        ("ffi", ["--ffi"]),
        ("optimize-llm", ["--optimize-llm"]),
        ("normal", []),
    ]
    
    for strategy_name, extra_args in strategies:
        print(f"  [TENTATIVA] Modo: {strategy_name}")
        cmd = [transmutation_path, "convert", str(pdf_path), "-o", str(output_md), "-f", "markdown"] + extra_args
        
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=600
            )
            
            if result.returncode == 0 and output_md.exists():
                markdown = output_md.read_text(encoding='utf-8')
                
                if len(markdown.strip()) >= 100:
                    pages = markdown.count("# Página") or markdown.count("# Page") or markdown.count("# Page ") or 1
                    print(f"  [OK] Sucesso com modo {strategy_name}: {len(markdown):,} caracteres, {pages} páginas")
                    return {
                        "success": True,
                        "pages": pages,
                        "markdown": markdown,
                        "method": f"transmutation-{strategy_name}"
                    }
                else:
                    print(f"  [FALHOU] Modo {strategy_name} gerou arquivo muito pequeno ({len(markdown)} caracteres)")
            else:
                print(f"  [FALHOU] Modo {strategy_name} retornou código {result.returncode}")
                
        except subprocess.TimeoutExpired:
            print(f"  [FALHOU] Modo {strategy_name} timeout")
        except Exception as e:
            print(f"  [FALHOU] Modo {strategy_name} erro: {e}")
    
    return {"success": False, "error": "Todas as estratégias do transmutation falharam"}


def reprocess_failed_pdf(pdf_path: Path) -> dict:
    """Reprocessa um PDF que falhou anteriormente"""
    pdf_filename = pdf_path.name
    print(f"\n{'='*60}")
    print(f"Reprocessando: {pdf_filename}")
    print(f"{'='*60}")
    
    # Nome do arquivo MD de saída
    md_filename = sanitize_filename(pdf_filename) + ".md"
    output_md = OUTPUT_DIR / md_filename
    
    # Remover arquivo MD anterior se existir
    if output_md.exists():
        output_md.unlink()
        metadata_file = OUTPUT_DIR / (md_filename.replace('.md', '.metadata.json'))
        if metadata_file.exists():
            metadata_file.unlink()
    
    # Passo 1: Tentar extrair com diferentes estratégias
    print("\n[1/2] Transmutation: Tentando diferentes estratégias...")
    extraction_result = extract_with_transmutation_strategies(pdf_path, output_md)
    
    if not extraction_result or not extraction_result.get("success"):
        return {
            "success": False,
            "error": f"Falha na extração: {extraction_result.get('error', 'Erro desconhecido')}"
        }
    
    markdown = extraction_result["markdown"]
    pages = extraction_result["pages"]
    method = extraction_result.get("method", "unknown")
    
    print(f"  [OK] Extraido: {pages} paginas ({method})")
    print(f"  [OK] Tamanho: {len(markdown):,} caracteres")
    
    # Passo 2: Classify
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
    metadata_file = OUTPUT_DIR / (md_filename.replace('.md', '.metadata.json'))
    with open(metadata_file, 'w', encoding='utf-8') as f:
        json.dump({
            "source_pdf": pdf_filename,
            "output_md": md_filename,
            "pages": pages,
            "classification": classification,
            "extraction_method": method
        }, f, indent=2, ensure_ascii=False)
    
    return {
        "success": True,
        "pdf": pdf_filename,
        "md_file": md_filename,
        "pages": pages,
        "classification": classification,
        "output_path": str(output_md)
    }


def main():
    """Função principal"""
    print("="*60)
    print("Reprocessamento de PDFs que Falharam")
    print("="*60)
    
    # Carregar lista de arquivos que falharam
    failed_files_json = Path(__file__).parent.parent / "failed-files.json"
    
    if not failed_files_json.exists():
        print(f"[ERRO] Arquivo failed-files.json não encontrado")
        print("Execute primeiro o comando PowerShell para identificar os arquivos que falharam")
        return
    
    with open(failed_files_json, 'r', encoding='utf-8') as f:
        failed_files = json.load(f)
    
    if not failed_files:
        print("[OK] Nenhum arquivo falhou!")
        return
    
    print(f"\nEncontrados {len(failed_files)} arquivos para reprocessar\n")
    
    results = []
    successful = 0
    failed = 0
    
    # Reprocessar cada PDF
    for i, failed_file in enumerate(failed_files, 1):
        pdf_path = Path(failed_file.get("PDFPath", ""))
        
        if not pdf_path.exists():
            # Tentar encontrar o PDF pelo nome
            pdf_name = failed_file.get("PDF", "")
            pdf_path = SOURCE_DIR / pdf_name
            if not pdf_path.exists():
                print(f"\n[{i}/{len(failed_files)}] [ERRO] PDF não encontrado: {pdf_name}")
                failed += 1
                results.append({
                    "success": False,
                    "pdf": pdf_name,
                    "error": "PDF não encontrado"
                })
                continue
        
        print(f"\n[{i}/{len(failed_files)}]")
        result = reprocess_failed_pdf(pdf_path)
        results.append(result)
        
        if result["success"]:
            successful += 1
            print(f"\n[OK] {pdf_path.name} → {result['md_file']}")
        else:
            failed += 1
            print(f"\n[ERRO] {pdf_path.name}: {result.get('error', 'Erro desconhecido')}")
    
    # Resumo
    print("\n" + "="*60)
    print("RESUMO DO REPROCESSAMENTO")
    print("="*60)
    print(f"PDFs reprocessados: {successful}/{len(failed_files)}")
    print(f"Sucessos: {successful}")
    print(f"Falhas: {failed}")
    print()


if __name__ == "__main__":
    main()




