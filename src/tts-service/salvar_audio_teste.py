#!/usr/bin/env python3
"""Script para testar TTS e salvar áudio com correção aplicada"""

import json
import struct
import urllib.request
import urllib.parse
import wave
import sys

def save_wav(audio_samples, sample_rate, filename):
    """Salva array de floats como arquivo WAV"""
    # Normalizar para int16
    max_val = max(abs(s) for s in audio_samples) if audio_samples else 1.0
    if max_val > 1.0:
        audio_samples = [s / max_val for s in audio_samples]
    
    # Converter para int16
    int16_samples = [int(s * 32767) for s in audio_samples]
    
    # Salvar como WAV
    with wave.open(filename, 'wb') as wav_file:
        wav_file.setnchannels(1)  # Mono
        wav_file.setsampwidth(2)   # 16-bit
        wav_file.setframerate(sample_rate)
        wav_file.writeframes(struct.pack('<' + 'h' * len(int16_samples), *int16_samples))
    
    print(f"Audio salvo: {filename}")
    print(f"   Duração: {len(audio_samples) / sample_rate:.2f}s")
    print(f"   Amostras: {len(audio_samples)}")
    print(f"   Sample rate: {sample_rate} Hz")

def main():
    url = "http://localhost:3002/speak"
    text = "Hello World"
    language = "en"
    
    payload = {
        "text": text,
        "language": language
    }
    
    print(f"Enviando requisição para: {url}")
    print(f"Texto: '{text}'")
    print(f"Idioma: {language}")
    print()
    
    try:
        # Preparar requisição
        data_json = json.dumps(payload).encode('utf-8')
        req = urllib.request.Request(url, data=data_json, headers={'Content-Type': 'application/json'})
        
        # Fazer requisição
        with urllib.request.urlopen(req, timeout=30) as response:
            data = json.loads(response.read().decode('utf-8'))
        
        print("Resposta recebida:")
        print(f"   Duração: {data['duration_ms']}ms")
        print(f"   Amostras: {len(data['audio'])}")
        print(f"   Sample rate: {data['sample_rate']} Hz")
        print(f"   Canais: {data['channels']}")
        print()
        
        # Salvar áudio
        filename = "test_hello_world_corrigido.wav"
        save_wav(data['audio'], data['sample_rate'], filename)
        
    except urllib.error.HTTPError as e:
        print(f"❌ Erro HTTP: {e}")
        sys.exit(1)
    except urllib.error.URLError as e:
        print(f"❌ Erro na URL: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"Erro: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()

