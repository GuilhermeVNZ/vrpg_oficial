#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Extrai texto de PDFs usando OCR (Tesseract)
Para PDFs escaneados ou com texto como imagem
"""

import sys
from pathlib import Path

# Configurar encoding UTF-8
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')

def extract_pdf_with_ocr(pdf_path: Path) -> dict:
    """Extrai texto de PDF usando OCR (Tesseract)"""
    try:
        import pdf2image
        from PIL import Image
        import pytesseract
        import os
        
        # Configurar caminho do Tesseract no Windows se necessário
        if sys.platform == 'win32':
            tesseract_paths = [
                r"C:\Program Files\Tesseract-OCR\tesseract.exe",
                r"C:\Program Files (x86)\Tesseract-OCR\tesseract.exe",
            ]
            for tesseract_path in tesseract_paths:
                if os.path.exists(tesseract_path):
                    pytesseract.pytesseract.tesseract_cmd = tesseract_path
                    print(f"  [OCR] Tesseract encontrado: {tesseract_path}")
                    break
            else:
                print("  [AVISO] Tesseract não encontrado. Tentando usar do PATH...")
        
        print(f"  [OCR] Convertendo PDF para imagens...")
        # Converter PDF para imagens (uma por página)
        images = pdf2image.convert_from_path(str(pdf_path), dpi=300)
        print(f"  [OCR] {len(images)} páginas convertidas")
        
        pages = []
        text_parts = []
        
        for i, image in enumerate(images, 1):
            print(f"  [OCR] Processando página {i}/{len(images)}...", end='\r')
            
            # Extrair texto usando Tesseract
            # Configurar para português
            text = pytesseract.image_to_string(image, lang='por+eng')
            
            if text:
                text = text.strip()
                if text:  # Só adicionar se tiver conteúdo
                    pages.append({"number": i, "text": text})
                    text_parts.append(f"# Página {i}\n\n{text}\n\n")
        
        print(f"\n  [OCR] {len(pages)} páginas com texto extraído")
        
        markdown = '\n'.join(text_parts)
        return {
            "success": True,
            "pages": len(pages),
            "markdown": markdown,
            "method": "ocr-tesseract"
        }
    except ImportError as e:
        print(f"  [ERRO] Dependências OCR não instaladas: {e}")
        print("  [INFO] Instale com: pip install pdf2image pytesseract pillow")
        print("  [INFO] E instale o Tesseract: https://github.com/UB-Mannheim/tesseract/wiki")
        return None
    except Exception as e:
        print(f"  [ERRO] Erro ao usar OCR: {e}")
        import traceback
        traceback.print_exc()
        return None


def main():
    """Testa OCR em um PDF específico"""
    pdf_path = Path(r"G:\vrpg\vrpg-client\assets-and-models\books\a-guerra-dos-tronos-01-biblioteca-elfica.pdf")
    output_md = Path(r"G:\vrpg\vrpg-client\assets-and-models\books\processed\a-guerra-dos-tronos-01.md")
    
    print("="*60)
    print("Extraindo texto com OCR (Tesseract)")
    print("="*60)
    print(f"PDF: {pdf_path.name}")
    
    if not pdf_path.exists():
        print(f"❌ ERRO: PDF não encontrado: {pdf_path}")
        return
    
    print("Iniciando extração com OCR...")
    print("(Isso pode levar vários minutos dependendo do tamanho do PDF)")
    print()
    
    result = extract_pdf_with_ocr(pdf_path)
    
    if result and result.get('success'):
        markdown = result['markdown']
        pages = result['pages']
        
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
            print("Preview (primeiros 500 caracteres):")
            print("-" * 60)
            print(markdown[:500])
            print("-" * 60)
    else:
        error = result.get('error', 'Erro desconhecido') if result else 'Nenhum resultado retornado'
        print(f"\n❌ FALHA: {error}")


if __name__ == "__main__":
    main()

