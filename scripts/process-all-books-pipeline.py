#!/usr/bin/env python3
"""
Pipeline completo para processar TODOS os PDFs de livros:
1. Transmutation: Converter PDFs para Markdown
2. Classify: Classificar o conteúdo  
3. Salvar arquivos MD processados
4. Indexar no Vectorizer via workspace

Uso: python scripts/process-all-books-pipeline.py
"""

import json
import os
import sys
import asyncio
import subprocess
from pathlib import Path
from typing import Dict, List, Optional
import re

# Configuração
SOURCE_DIR = Path(r"G:\vrpg\vrpg-client\assets-and-models\books")
OUTPUT_DIR = Path(r"G:\vrpg\vrpg-client\assets-and-models\books\processed")
CHUNK_SIZE = 2000  # Tamanho dos chunks em caracteres


def find_transmutation_path() -> Optional[str]:
    """Tenta encontrar o caminho do transmutation"""
    # Caminhos absolutos primeiro (mais confiáveis)
    possible_paths = [
        r"G:\vrpg\transmutation-main\target\release\transmutation.exe",
        r"G:\vrpg\transmutation-main\target\debug\transmutation.exe",
        # Caminhos relativos
        Path(__file__).parent.parent.parent.parent / "transmutation-main" / "target" / "release" / "transmutation.exe",
        Path(__file__).parent.parent.parent.parent / "transmutation-main" / "target" / "debug" / "transmutation.exe",
        # No PATH
        "transmutation",
    ]
    
    for path in possible_paths:
        try:
            path_str = str(path) if isinstance(path, Path) else path
            if path_str == "transmutation":
                # Tentar executar do PATH
                result = subprocess.run(
                    ["transmutation", "--version"],
                    capture_output=True,
                    text=True,
                    timeout=5
                )
                if result.returncode == 0:
                    return "transmutation"
            else:
                # Verificar se arquivo existe e testar execução
                if Path(path_str).exists():
                    # Testar se o binário funciona
                    try:
                        result = subprocess.run(
                            [path_str, "--version"],
                            capture_output=True,
                            text=True,
                            timeout=5
                        )
                        if result.returncode == 0:
                            print(f"  [INFO] Transmutation encontrado em: {path_str}")
                            return path_str
                    except Exception as e:
                        print(f"  [AVISO] Transmutation em {path_str} não executável: {e}")
                        continue
        except Exception as e:
            continue
    
    return None

def check_transmutation_available() -> bool:
    """Verifica se transmutation CLI está disponível"""
    return find_transmutation_path() is not None


def find_classify_path() -> Optional[str]:
    """Encontra o caminho do classify"""
    classify_dir = Path(__file__).parent.parent.parent.parent / "classify-main"
    
    # Tentar usar o CLI compilado diretamente
    cli_path = classify_dir / "dist" / "cli.js"
    if cli_path.exists():
        try:
            # Testar se node consegue executar
            result = subprocess.run(
                ["node", "--version"],
                capture_output=True,
                text=True,
                timeout=5
            )
            if result.returncode == 0:
                return str(cli_path)
        except:
            pass
    
    # Fallback: tentar npx
    try:
        result = subprocess.run(
            ["npx", "--version"],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            return "npx"
    except:
        pass
    
    return None

def check_classify_available() -> bool:
    """Verifica se classify CLI está disponível"""
    return find_classify_path() is not None


def extract_pdf_with_transmutation(pdf_path: Path, output_md: Path) -> Optional[Dict]:
    """Extrai texto de PDF usando Transmutation CLI"""
    transmutation_path = find_transmutation_path()
    if not transmutation_path:
        return None
    
    try:
        # Comando básico do transmutation - tentar primeiro com precision mode
        # Se falhar, tentar com FFI mode, depois modo normal
        cmd = [transmutation_path, "convert", str(pdf_path), "-o", str(output_md), "-f", "markdown", "--precision"]
        
        # Tentar com precision mode primeiro (melhor qualidade)
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=600  # 10 minutos timeout
        )
        
        # Se precision mode falhou ou gerou arquivo vazio, tentar FFI mode
        if result.returncode != 0 or (output_md.exists() and len(output_md.read_text(encoding='utf-8').strip()) < 100):
            print(f"  [INFO] Precision mode falhou ou gerou arquivo vazio, tentando FFI mode...")
            cmd_ffi = [transmutation_path, "convert", str(pdf_path), "-o", str(output_md), "-f", "markdown", "--ffi"]
            result = subprocess.run(
                cmd_ffi,
                capture_output=True,
                text=True,
                timeout=600
            )
        
        # Se ainda falhou, tentar modo normal com optimize-llm
        if result.returncode != 0 or (output_md.exists() and len(output_md.read_text(encoding='utf-8').strip()) < 100):
            print(f"  [INFO] FFI mode falhou, tentando modo normal com optimize-llm...")
            cmd_normal = [transmutation_path, "convert", str(pdf_path), "-o", str(output_md), "-f", "markdown", "--optimize-llm"]
            result = subprocess.run(
                cmd_normal,
                capture_output=True,
                text=True,
                timeout=600
            )
        
        # Verificar resultado final
        if result.returncode == 0 and output_md.exists():
            markdown = output_md.read_text(encoding='utf-8')
            
            # Verificar se o arquivo não está vazio
            if len(markdown.strip()) < 100:
                print(f"  [AVISO] Transmutation gerou arquivo muito pequeno ({len(markdown)} caracteres)")
                print(f"  [INFO] Tentando fallback com pdfplumber...")
                return None  # Retorna None para tentar fallback
            
            pages = markdown.count("# Página") or markdown.count("# Page") or markdown.count("# Page ") or 1
            return {
                "success": True,
                "pages": pages,
                "markdown": markdown,
                "method": "transmutation"
            }
        else:
            print(f"  [AVISO] Transmutation falhou (returncode: {result.returncode})")
            print(f"  [INFO] Tentando fallback com pdfplumber...")
            return None  # Retorna None para tentar fallback
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


def extract_pdf_with_ocr(pdf_path: Path) -> Optional[Dict]:
    """Fallback OCR: Extrai texto de PDF usando OCR (Tesseract) para PDFs escaneados"""
    try:
        import pdf2image
        import pytesseract
        import os
        import sys
        
        # Configurar caminho do Tesseract no Windows se necessário
        if sys.platform == 'win32':
            tesseract_paths = [
                r"C:\Program Files\Tesseract-OCR\tesseract.exe",
                r"C:\Program Files (x86)\Tesseract-OCR\tesseract.exe",
            ]
            for tesseract_path in tesseract_paths:
                if os.path.exists(tesseract_path):
                    pytesseract.pytesseract.tesseract_cmd = tesseract_path
                    break
        
        print(f"  [OCR] Convertendo PDF para imagens...")
        # Converter PDF para imagens (uma por página)
        images = pdf2image.convert_from_path(str(pdf_path), dpi=300)
        print(f"  [OCR] {len(images)} páginas convertidas")
        
        pages = []
        text_parts = []
        
        for i, image in enumerate(images, 1):
            print(f"  [OCR] Processando página {i}/{len(images)}...", end='\r')
            
            # Extrair texto usando Tesseract (português + inglês)
            text = pytesseract.image_to_string(image, lang='por+eng')
            
            if text:
                text = text.strip()
                if text:  # Só adicionar se tiver conteúdo
                    pages.append({"number": i, "text": text})
                    text_parts.append(f"# Página {i}\n\n{text}\n\n")
        
        print(f"\n  [OCR] {len(pages)} páginas com texto extraído")
        
        if len(pages) == 0:
            return None
        
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
        return None
    except Exception as e:
        print(f"  [ERRO] Erro ao usar OCR: {e}")
        return None


def classify_with_cli(md_path: Path) -> Optional[Dict]:
    """Classifica usando classify CLI"""
    classify_path = find_classify_path()
    if not classify_path:
        return None
    
    try:
        classify_dir = Path(__file__).parent.parent.parent.parent / "classify-main"
        
        # Se encontrou o CLI compilado, usar node diretamente
        if classify_path.endswith("cli.js"):
            cmd = ["node", classify_path, "document", str(md_path), "--output", "combined", "--no-cache"]
        else:
            # Usar npx
            cmd = ["npx", "@hivellm/classify", "document", str(md_path), "--output", "combined", "--no-cache"]
        
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=300,  # 5 minutos timeout
            cwd=str(classify_dir) if classify_dir.exists() else None
        )
        
        if result.returncode == 0:
            try:
                classification = json.loads(result.stdout)
                return {
                    "success": True,
                    "classification": classification
                }
            except json.JSONDecodeError:
                print(f"  [AVISO] Classify retornou output não-JSON, usando classificação simples")
                return None
        else:
            print(f"  [AVISO] Classify CLI falhou: {result.stderr[:300]}")
            return None
    except subprocess.TimeoutExpired:
        print("  [AVISO] Classify timeout, usando classificação simples")
        return None
    except Exception as e:
        print(f"  [AVISO] Erro ao usar Classify CLI: {e}")
        return None


def classify_text_simple(text: str, filename: str) -> Dict:
    """Classificação simples baseada no nome do arquivo e conteúdo"""
    filename_lower = filename.lower()
    
    # Detectar tipo de documento
    doc_type = "book"
    domain = "gaming"
    categories = ["book", "rpg"]
    
    # D&D 5e
    if "dd-5e" in filename_lower or "d&d" in filename_lower or "dnd" in filename_lower:
        domain = "gaming"
        categories = ["dnd5e", "dungeons_and_dragons", "rpg", "game_rules"]
        if "livro-do-jogador" in filename_lower or "player" in filename_lower:
            doc_type = "player_handbook"
        elif "guia-do-mestre" in filename_lower or "dm" in filename_lower or "master" in filename_lower:
            doc_type = "dungeon_master_guide"
        elif "monstros" in filename_lower or "monster" in filename_lower:
            doc_type = "monster_manual"
        elif "ficha" in filename_lower or "character" in filename_lower:
            doc_type = "character_sheet"
        elif "xanathar" in filename_lower:
            doc_type = "supplement"
        elif "volo" in filename_lower:
            doc_type = "monster_manual"
    
    # Forgotten Realms
    elif "forgotten-realms" in filename_lower or "forgotten_realms" in filename_lower:
        domain = "gaming"
        categories = ["forgotten_realms", "dnd", "campaign_setting", "rpg"]
        doc_type = "campaign_setting"
    
    # Ravenloft
    elif "ravenloft" in filename_lower:
        domain = "gaming"
        categories = ["ravenloft", "dnd", "campaign_setting", "rpg", "horror"]
        doc_type = "campaign_setting"
    
    # A Guerra dos Tronos / Game of Thrones
    elif "guerra-dos-tronos" in filename_lower or "game-of-thrones" in filename_lower:
        domain = "literature"
        categories = ["fantasy", "literature", "novel", "george_martin"]
        doc_type = "novel"
    
    # As Crônicas de Gelo e Fogo
    elif "cronicas-de-gelo" in filename_lower or "ice-and-fire" in filename_lower:
        domain = "literature"
        categories = ["fantasy", "literature", "novel", "george_martin"]
        doc_type = "novel"
    
    # A Lenda de Drizzt
    elif "drizzt" in filename_lower:
        domain = "literature"
        categories = ["fantasy", "literature", "novel", "forgotten_realms", "r_a_salvatore"]
        doc_type = "novel"
    
    # O Senhor dos Anéis
    elif "senhor-dos-aneis" in filename_lower or "lord-of-the-rings" in filename_lower:
        domain = "literature"
        categories = ["fantasy", "literature", "novel", "tolkien"]
        doc_type = "novel"
    
    # O Vale do Vento Gelido
    elif "vale-do-vento" in filename_lower:
        domain = "literature"
        categories = ["fantasy", "literature", "novel"]
        doc_type = "novel"
    
    # Lendas de Baldur's Gate
    elif "baldurs-gate" in filename_lower or "baldur" in filename_lower:
        domain = "gaming"
        categories = ["baldurs_gate", "dnd", "novel", "rpg"]
        doc_type = "novel"
    
    return {
        "domain": domain,
        "doc_type": doc_type,
        "categories": categories,
        "confidence": 0.85,
        "metadata": {
            "source": "biblioteca_elfica",
            "language": "pt-BR",
            "filename": filename
        }
    }


def sanitize_filename(filename: str) -> str:
    """Sanitiza o nome do arquivo para usar como nome de coleção"""
    # Remove extensão
    name = Path(filename).stem
    # Remove sufixos comuns
    name = re.sub(r'-biblioteca-elfica$', '', name)
    name = re.sub(r'-fundo-branco$', '', name)
    name = re.sub(r'-v-alta-resolucao$', '', name)
    # Substitui espaços e caracteres especiais por hífens
    name = re.sub(r'[^\w\-]', '-', name)
    # Remove hífens múltiplos
    name = re.sub(r'-+', '-', name)
    # Remove hífens no início/fim
    name = name.strip('-')
    return name.lower()


def process_pdf(pdf_path: Path) -> Dict:
    """Processa um PDF completo: transmutation -> classify"""
    pdf_filename = pdf_path.name
    print(f"\n{'='*60}")
    print(f"Processando: {pdf_filename}")
    print(f"{'='*60}")
    
    # Criar diretório de output
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    
    # Nome do arquivo MD de saída
    md_filename = sanitize_filename(pdf_filename) + ".md"
    output_md = OUTPUT_DIR / md_filename
    
    # Passo 1: Transmutation (extrair texto do PDF) - OBRIGATÓRIO
    print("\n[1/2] Transmutation: Extraindo texto do PDF...")
    
    transmutation_path = find_transmutation_path()
    if not transmutation_path:
        return {
            "success": False,
            "error": "Transmutation não encontrado. Compile primeiro: cd G:\\vrpg\\transmutation-main && cargo build --release"
        }
    
    print(f"  [INFO] Usando Transmutation CLI: {transmutation_path}")
    extraction_result = extract_pdf_with_transmutation(pdf_path, output_md)
    
    # Se transmutation falhou ou gerou arquivo vazio, tentar pdfplumber
    if not extraction_result or not extraction_result.get("success"):
        print("  [INFO] Transmutation falhou, tentando pdfplumber...")
        extraction_result = extract_pdf_with_python(pdf_path)
    
    # Se pdfplumber também falhou, tentar OCR
    if not extraction_result or not extraction_result.get("success"):
        print("  [INFO] pdfplumber falhou, tentando OCR (Tesseract)...")
        print("  [AVISO] OCR pode levar vários minutos...")
        extraction_result = extract_pdf_with_ocr(pdf_path)
        if extraction_result and extraction_result.get("success"):
            # Salvar resultado do OCR
            output_md.write_text(extraction_result["markdown"], encoding='utf-8')
    
    if not extraction_result or not extraction_result.get("success"):
        return {
            "success": False,
            "error": f"Falha na extração do PDF: {extraction_result.get('error', 'Erro desconhecido') if extraction_result else 'Todas as tentativas falharam (transmutation, pdfplumber, OCR)'}"
        }
    
    markdown = extraction_result["markdown"]
    pages = extraction_result["pages"]
    method = extraction_result.get("method", "unknown")
    
    print(f"  [OK] Extraido: {pages} paginas ({method})")
    print(f"  [OK] Tamanho: {len(markdown):,} caracteres")
    
    # Passo 2: Classify (classificar o conteúdo) - OBRIGATÓRIO
    print(f"\n[2/2] Classify: Classificando conteudo...")
    
    classify_path = find_classify_path()
    classification_result = None
    
    if classify_path:
        if classify_path.endswith("cli.js"):
            print(f"  [INFO] Usando Classify CLI (node dist/cli.js)...")
        else:
            print(f"  [INFO] Usando Classify CLI (npx)...")
        classification_result = classify_with_cli(output_md)
    
    if not classification_result or not classification_result.get("success"):
        print("  [AVISO] Classify CLI não disponível ou falhou, usando classificação simples baseada em regras...")
        print("  [INFO] Para usar Classify CLI completo, certifique-se de que:")
        print("    - npm está instalado")
        print("    - Dependências do classify-main estão instaladas (npm install)")
        classification = classify_text_simple(markdown, pdf_filename)
    else:
        # Usar resultado do classify CLI
        classification_data = classification_result.get("classification", {})
        if classification_data:
            # Converter resultado do classify CLI para formato esperado
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
    print(f"  [OK] Categorias: {', '.join(classification.get('categories', []))}")
    
    # Salvar metadados de classificação
    metadata_file = OUTPUT_DIR / (md_filename.replace('.md', '.metadata.json'))
    with open(metadata_file, 'w', encoding='utf-8') as f:
        json.dump({
            "source_pdf": pdf_filename,
            "output_md": md_filename,
            "pages": pages,
            "classification": classification,
            "extraction_method": method
        }, f, indent=2, ensure_ascii=False)
    
    print(f"  [OK] Metadados salvos em: {metadata_file.name}")
    
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
    print("Pipeline de Livros: Transmutation -> Classify")
    print("="*60)
    print(f"Source: {SOURCE_DIR}")
    print(f"Output: {OUTPUT_DIR}")
    print()
    
    # Verificar ferramentas disponíveis
    transmutation_available = check_transmutation_available()
    classify_available = check_classify_available()
    
    print(f"Transmutation: {'[OK] Disponivel' if transmutation_available else '[FALLBACK] Nao disponivel (usando fallback)'}")
    print(f"Classify CLI: {'[OK] Disponivel' if classify_available else '[FALLBACK] Nao disponivel (usando classificacao simples)'}")
    print()
    
    # Encontrar todos os PDFs
    all_pdf_files = list(SOURCE_DIR.glob("*.pdf"))
    
    if not all_pdf_files:
        print(f"[ERRO] Nenhum PDF encontrado em {SOURCE_DIR}")
        return
    
    # Filtrar PDFs já processados
    pdf_files = []
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    
    for pdf_path in all_pdf_files:
        md_filename = sanitize_filename(pdf_path.name) + ".md"
        output_md = OUTPUT_DIR / md_filename
        if not output_md.exists():
            pdf_files.append(pdf_path)
    
    already_processed = len(all_pdf_files) - len(pdf_files)
    
    print(f"Encontrados {len(all_pdf_files)} arquivos PDF")
    if already_processed > 0:
        print(f"Já processados: {already_processed} arquivos (serão pulados)")
    print(f"A processar: {len(pdf_files)} arquivos")
    print()
    
    results = []
    successful = 0
    failed = 0
    
    # Processar cada PDF
    for i, pdf_path in enumerate(pdf_files, 1):
        print(f"\n[{i}/{len(pdf_files)}]")
        result = process_pdf(pdf_path)
        results.append(result)
        
        if result["success"]:
            successful += 1
            print(f"\n[OK] {pdf_path.name} → {result['md_file']}")
        else:
            failed += 1
            print(f"\n[ERRO] {pdf_path.name}: {result.get('error', 'Erro desconhecido')}")
    
    # Resumo
    print("\n" + "="*60)
    print("RESUMO")
    print("="*60)
    print(f"PDFs processados: {successful}/{len(pdf_files)}")
    print(f"Sucessos: {successful}")
    print(f"Falhas: {failed}")
    print(f"Arquivos MD gerados em: {OUTPUT_DIR}")
    print()
    
    # Salvar índice de processamento
    index_file = OUTPUT_DIR / "processing_index.json"
    with open(index_file, 'w', encoding='utf-8') as f:
        json.dump({
            "total_pdfs": len(pdf_files),
            "successful": successful,
            "failed": failed,
            "results": results
        }, f, indent=2, ensure_ascii=False)
    
    print(f"[OK] Índice de processamento salvo em: {index_file}")
    print()
    print("PRÓXIMOS PASSOS:")
    print("1. Verifique os arquivos MD gerados em:", OUTPUT_DIR)
    print("2. Atualize o workspace do vectorizer para incluir essas coleções")
    print("3. O vectorizer indexará automaticamente os arquivos configurados")


if __name__ == "__main__":
    main()


