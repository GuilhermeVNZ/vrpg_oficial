#!/usr/bin/env python3
"""
Script Python para salvar áudio em WAV usando soundfile (mais confiável que PowerShell)
"""

import sys
import json
import soundfile as sf
import numpy as np
import requests

def save_audio_from_api():
    """Busca áudio da API e salva em WAV usando soundfile"""
    
    url = "http://localhost:3002/speak"
    
    test_text = "In the dim light of the ancient library, dust motes danced in the air like forgotten memories. The old tome lay open on the mahogany desk, its pages yellowed with age and secrets."
    
    payload = {
        "text": f'<VOICE actor="dungeon_master_en" emotion="neutral" style="narrative">{test_text}</VOICE>',
        "language": "en"
    }
    
    print("Enviando requisição para TTS service...")
    response = requests.post(url, json=payload, timeout=120)
    
    if response.status_code != 200:
        print(f"ERRO: {response.status_code}")
        print(response.text)
        return False
    
    data = response.json()
    
    print(f"\nAudio recebido:")
    print(f"  Samples: {len(data['audio'])}")
    print(f"  Sample rate: {data['sample_rate']} Hz")
    print(f"  Channels: {data['channels']}")
    print(f"  Duration: {data['duration_ms']} ms")
    
    # Converter para numpy array
    audio_array = np.array(data['audio'], dtype=np.float32)
    
    # Verificar estatísticas
    print(f"\nEstatísticas:")
    print(f"  Min: {audio_array.min()}")
    print(f"  Max: {audio_array.max()}")
    print(f"  Mean: {audio_array.mean()}")
    print(f"  Std: {audio_array.std()}")
    print(f"  Peak: {np.abs(audio_array).max()}")
    
    # Verificar se há clipping
    if np.abs(audio_array).max() > 1.0:
        print(f"\nAVISO: Valores fora do range [-1.0, 1.0]!")
        print(f"  Normalizando para evitar clipping...")
        audio_array = audio_array / np.abs(audio_array).max()
    
    # Salvar como WAV
    output_file = "test_audio_python.wav"
    sf.write(output_file, audio_array, data['sample_rate'], format='WAV', subtype='PCM_16')
    
    print(f"\nAudio salvo em: {output_file}")
    print(f"Tamanho: {len(audio_array) * 2 / 1024:.2f} KB")
    
    return True

if __name__ == "__main__":
    try:
        save_audio_from_api()
    except Exception as e:
        print(f"ERRO: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

