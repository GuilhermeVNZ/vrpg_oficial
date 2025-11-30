#!/usr/bin/env python3
"""
Teste rápido do script de phonemização
"""

import sys
from pathlib import Path

# Adicionar scripts ao path
scripts_dir = Path(__file__).parent / "scripts"
sys.path.insert(0, str(scripts_dir))

from phonemize_for_piper import phonemize_text

if __name__ == "__main__":
    test_text = "In the dim light of the ancient library, dust motes danced in the air like forgotten memories."
    
    print("Testando phonemização...")
    print(f"Texto: {test_text}")
    print()
    
    try:
        phonemes = phonemize_text(test_text, "en-us")
        print(f"Phonemes encontrados: {len(phonemes)}")
        print("Phonemes:")
        for i, phoneme in enumerate(phonemes[:20]):  # Mostrar primeiros 20
            print(f"  {i+1}. {phoneme}")
        if len(phonemes) > 20:
            print(f"  ... e mais {len(phonemes) - 20} phonemes")
        print()
        print("✅ Phonemização funcionando!")
    except Exception as e:
        print(f"❌ Erro: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

