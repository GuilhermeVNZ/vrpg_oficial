#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Testa OCR em um PDF específico
"""

import sys
from pathlib import Path

# Configurar encoding UTF-8
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')

# Adicionar scripts ao path
script_dir = Path(__file__).parent
sys.path.insert(0, str(script_dir))

# Importar função de extração OCR
import importlib.util
pipeline_file = script_dir / "process-all-books-pipeline.py"
if not pipeline_file.exists():
    print(f"ERRO: Arquivo não encontrado: {pipeline_file}")
    sys.exit(1)

spec = importlib.util.spec_from_file_location(
    "process_all_books_pipeline", 
    pipeline_file
)
pipeline_module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pipeline_module)
extract_pdf_with_ocr = pipeline_module.extract_pdf_with_ocr

def main():
    pdf_path = Path(r"G:\vrpg\vrpg-client\assets-and-models\books\a-guerra-dos-tronos-01-biblioteca-elfica.pdf")
    output_md = Path(r"G:\vrpg\vrpg-client\assets-and-models\books\processed\a-guerra-dos-tronos-01.md")
    
    print("="*60)
    print("Testando OCR no PDF 1")
    print("="*60)
    print(f"PDF: {pdf_path.name}")
    print(f"Tamanho: {pdf_path.stat().st_size / (1024*1024):.2f} MB")
    print()
    print("Isso pode levar vários minutos (32 páginas x OCR)...")
    print()
    
    result = extract_pdf_with_ocr(pdf_path)
    
    if result and result.get("success"):
        markdown = result["markdown"]
        pages = result["pages"]
        
        print()
        print(f"✅ SUCESSO!")
        print(f"  Páginas processadas: {pages}")
        print(f"  Caracteres extraídos: {len(markdown):,}")
        print()
        
        # Salvar arquivo
        output_md.parent.mkdir(parents=True, exist_ok=True)
        output_md.write_text(markdown, encoding='utf-8')
        print(f"✅ Arquivo salvo: {output_md}")
        print()
        
        # Preview
        if len(markdown) > 0:
            print("Preview (primeiras 500 caracteres):")
            print("-" * 60)
            print(markdown[:500])
            print("-" * 60)
    else:
        error = result.get('error', 'Erro desconhecido') if result else 'Nenhum resultado retornado'
        print(f"\n❌ FALHA: {error}")
        print()
        print("Possíveis causas:")
        print("  - Tesseract OCR não instalado")
        print("  - Dependências Python faltando")
        print("  - Erro durante processamento")

if __name__ == "__main__":
    main()

