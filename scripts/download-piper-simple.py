#!/usr/bin/env python3
"""Script simples para baixar modelos Piper TTS"""

import urllib.request
import os
from pathlib import Path

# URLs dos modelos (usando HuggingFace API)
# Alternativa: usar git-lfs ou baixar manualmente do HuggingFace
MODELS = {
    "piper-pt-br.onnx": {
        "repo": "rhasspy/piper-voices",
        "path": "pt/pt_BR/lessac/medium/pt_BR_lessac_medium.onnx"
    },
    "piper-en-us.onnx": {
        "repo": "rhasspy/piper-voices", 
        "path": "en/en_US/lessac/medium/en_US_lessac_medium.onnx"
    }
}

def get_huggingface_url(repo: str, path: str) -> str:
    """Gera URL do HuggingFace para download direto"""
    # Tentar diferentes formatos de URL
    base_urls = [
        f"https://huggingface.co/{repo}/resolve/main/{path}",
        f"https://huggingface.co/{repo}/raw/main/{path}",
    ]
    return base_urls[0]  # Retornar primeira opção

# Diretório de destino
TTS_DIR = Path(__file__).parent.parent / "assets-and-models" / "models" / "tts"
TTS_DIR.mkdir(parents=True, exist_ok=True)

def download_file(url: str, output_path: Path):
    """Baixa um arquivo com barra de progresso"""
    print(f"[DOWNLOAD] {output_path.name}...")
    
    def show_progress(block_num, block_size, total_size):
        downloaded = block_num * block_size
        percent = min(100, (downloaded / total_size) * 100) if total_size > 0 else 0
        mb_downloaded = downloaded / (1024 * 1024)
        mb_total = total_size / (1024 * 1024) if total_size > 0 else 0
        print(f"\r  Progresso: {percent:.1f}% ({mb_downloaded:.2f}/{mb_total:.2f} MB)", end="", flush=True)
    
    try:
        urllib.request.urlretrieve(url, output_path, show_progress)
        size_mb = output_path.stat().st_size / (1024 * 1024)
        print(f"\n[OK] {output_path.name} baixado com sucesso! ({size_mb:.2f} MB)")
        return True
    except Exception as e:
        print(f"\n[ERRO] Erro ao baixar {output_path.name}: {e}")
        return False

if __name__ == "__main__":
    import sys
    import io
    
    # Fix encoding for Windows
    if sys.platform == "win32":
        sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    
    print("========================================")
    print("  Download Piper TTS Models")
    print("========================================")
    print()
    
    success_count = 0
    for filename, url in MODELS.items():
        output_path = TTS_DIR / filename
        if output_path.exists():
            size_mb = output_path.stat().st_size / (1024 * 1024)
            print(f"[OK] {filename} ja existe ({size_mb:.2f} MB)")
            success_count += 1
        else:
            if download_file(url, output_path):
                success_count += 1
        print()
    
    print("========================================")
    print("  Resumo")
    print("========================================")
    print()
    print(f"Modelos baixados: {success_count}/{len(MODELS)}")
    
    if success_count == len(MODELS):
        print("[OK] Todos os modelos Piper foram baixados!")
    else:
        print("[AVISO] Alguns modelos falharam. Tente novamente.")

