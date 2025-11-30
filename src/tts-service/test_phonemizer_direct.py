#!/usr/bin/env python3
"""
Teste direto do script de phonemização
"""

import sys
from pathlib import Path

# Adicionar scripts ao path
scripts_dir = Path(__file__).parent / "scripts"
sys.path.insert(0, str(scripts_dir))

from phonemize_for_piper import phonemize_text

if __name__ == "__main__":
    test_texts = [
        "Hello world.",
        "This is a test.",
        "In the dim light of the ancient library.",
    ]
    
    print("=" * 60)
    print("TESTE DIRETO DO PHONEMIZER")
    print("=" * 60)
    
    for text in test_texts:
        print(f"\nTexto: {text}")
        try:
            phonemes = phonemize_text(text, "en-us")
            print(f"  Phonemes ({len(phonemes)}): {' '.join(phonemes[:20])}")
            if len(phonemes) > 20:
                print(f"  ... e mais {len(phonemes) - 20} phonemes")
            print("  ✅ Sucesso")
        except Exception as e:
            print(f"  ❌ Erro: {e}")
            import traceback
            traceback.print_exc()
    
    print("\n" + "=" * 60)
    print("TESTE CONCLUÍDO")
    print("=" * 60)

