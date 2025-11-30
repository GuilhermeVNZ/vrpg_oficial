#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Testa extração de PDF usando pdfplumber
"""

import sys
from pathlib import Path

# Configurar encoding UTF-8
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')

def extract_pdf_with_pdfplumber(pdf_path: Path):
    """Extrai texto de PDF usando pdfplumber"""
    try:
        import pdfplumber
        
        pages = []
        text_parts = []
        
        print(f"Abrindo PDF: {pdf_path}")
        with pdfplumber.open(str(pdf_path)) as pdf:
            print(f"Total de páginas no PDF: {len(pdf.pages)}")
            
            for i, page in enumerate(pdf.pages, 1):
                text = page.extract_text()
                if text:
                    text = text.strip()
                    pages.append({"number": i, "text": text})
                    text_parts.append(f"# Página {i}\n\n{text}\n\n")
                    if i <= 3:
                        print(f"  Página {i}: {len(text)} caracteres extraídos")
                else:
                    print(f"  Página {i}: ⚠️ Nenhum texto extraído")
        
        markdown = '\n'.join(text_parts)
        return {
            "success": True,
            "pages": len(pages),
            "markdown": markdown,
            "method": "pdfplumber"
        }
    except ImportError:
        print("❌ ERRO: pdfplumber não está instalado. Execute: pip install pdfplumber")
        return None
    except Exception as e:
        print(f"❌ ERRO ao extrair PDF: {e}")
        import traceback
        traceback.print_exc()
        return None

def main():
    pdf_path = Path(r"G:\vrpg\vrpg-client\assets-and-models\books\a-guerra-dos-tronos-01-biblioteca-elfica.pdf")
    output_md = Path(r"G:\vrpg\vrpg-client\assets-and-models\books\processed\a-guerra-dos-tronos-01.md")
    
    print("="*60)
    print("Testando PDF com pdfplumber")
    print("="*60)
    print(f"PDF: {pdf_path.name}")
    
    if not pdf_path.exists():
        print(f"❌ ERRO: PDF não encontrado: {pdf_path}")
        return
    
    file_size_mb = pdf_path.stat().st_size / (1024*1024)
    print(f"Tamanho: {file_size_mb:.2f} MB")
    print()
    
    print("Extraindo texto com pdfplumber...")
    result = extract_pdf_with_pdfplumber(pdf_path)
    
    if result and result.get('success'):
        markdown = result['markdown']
        pages = result['pages']
        
        print()
        print(f"✅ SUCESSO!")
        print(f"  Páginas processadas: {pages}")
        print(f"  Caracteres extraídos: {len(markdown):,}")
        print(f"  Palavras aproximadas: ~{len(markdown.split()):,}")
        print()
        
        # Salvar arquivo
        output_md.parent.mkdir(parents=True, exist_ok=True)
        output_md.write_text(markdown, encoding='utf-8')
        print(f"✅ Arquivo salvo: {output_md}")
        print()
        
        # Mostrar preview
        print("="*60)
        print("PREVIEW DO CONTEÚDO (primeiras 800 caracteres):")
        print("="*60)
        preview = markdown[:800]
        print(preview)
        if len(markdown) > 800:
            print(f"\n... (mais {len(markdown) - 800:,} caracteres)")
        print("="*60)
        
    else:
        error = result.get('error', 'Erro desconhecido') if result else 'Nenhum resultado retornado'
        print(f"\n❌ FALHA: {error}")

if __name__ == "__main__":
    main()
