#!/usr/bin/env python3
"""Script para baixar modelos Piper TTS usando huggingface_hub"""

import sys
from pathlib import Path

try:
    from huggingface_hub import hf_hub_download
except ImportError:
    print("[ERRO] Biblioteca 'huggingface_hub' nao encontrada!")
    print("Instale com: pip install huggingface_hub")
    sys.exit(1)

# Diretório de destino
TTS_DIR = Path(__file__).parent.parent / "assets-and-models" / "models" / "tts"
TTS_DIR.mkdir(parents=True, exist_ok=True)

# Modelos para baixar
MODELS = {
    "piper-pt-br.onnx": {
        "repo_id": "rhasspy/piper-voices",
        "filename": "pt/pt_BR/lessac/medium/pt_BR_lessac_medium.onnx"
    },
    "piper-en-us.onnx": {
        "repo_id": "rhasspy/piper-voices",
        "filename": "en/en_US/lessac/medium/en_US_lessac_medium.onnx"
    }
}

print("========================================")
print("  Download Piper TTS Models")
print("========================================")
print()

success_count = 0
for output_name, model_info in MODELS.items():
    output_path = TTS_DIR / output_name
    
    if output_path.exists():
        size_mb = output_path.stat().st_size / (1024 * 1024)
        print(f"[OK] {output_name} ja existe ({size_mb:.2f} MB)")
        success_count += 1
        continue
    
    print(f"[DOWNLOAD] {output_name}...")
    print(f"  Repo: {model_info['repo_id']}")
    print(f"  Arquivo: {model_info['filename']}")
    
    try:
        downloaded_path = hf_hub_download(
            repo_id=model_info["repo_id"],
            filename=model_info["filename"],
            local_dir=str(TTS_DIR),
            local_dir_use_symlinks=False
        )
        
        # Renomear para o nome esperado
        downloaded_file = Path(downloaded_path)
        if downloaded_file.name != output_name:
            final_path = TTS_DIR / output_name
            downloaded_file.rename(final_path)
            size_mb = final_path.stat().st_size / (1024 * 1024)
        else:
            size_mb = downloaded_file.stat().st_size / (1024 * 1024)
        
        print(f"[OK] {output_name} baixado com sucesso! ({size_mb:.2f} MB)")
        success_count += 1
    except Exception as e:
        print(f"[ERRO] Falha ao baixar {output_name}: {e}")
    
    print()

print("========================================")
print("  Resumo")
print("========================================")
print()
print(f"Modelos baixados: {success_count}/{len(MODELS)}")

if success_count == len(MODELS):
    print("[OK] Todos os modelos Piper foram baixados!")
    print()
    print("Os modelos estao prontos para uso em:")
    print(f"  {TTS_DIR}")
else:
    print("[AVISO] Alguns modelos falharam.")
    print()
    print("Alternativa: Baixe manualmente do HuggingFace:")
    print("  https://huggingface.co/rhasspy/piper-voices")
    print()
    print("Veja instrucoes em: assets-and-models/models/tts/COMO_BAIXAR.md")



