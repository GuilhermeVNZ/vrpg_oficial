#!/usr/bin/env python3
"""
Whisper Transcription Bridge
Chama faster-whisper para transcrição com suporte GPU
"""

import json
import sys
import os
import tempfile
import numpy as np
import soundfile as sf
from pathlib import Path

# Configurar variável de ambiente para aceitar termos
os.environ["COQUI_TOS_AGREED"] = "1"

try:
    from faster_whisper import WhisperModel
    import torch
except ImportError:
    print(json.dumps({"error": "faster-whisper not installed. Install with: pip install faster-whisper"}), file=sys.stderr)
    sys.exit(1)


def transcribe_audio_with_whisper(
    audio_data: list,
    sample_rate: int,
    language: str = "auto",
    use_gpu: bool = True,
    model_size: str = "large-v3",
) -> dict:
    """
    Transcreve áudio usando Whisper large-v3 com GPU
    
    Args:
        audio_data: Lista de amostras de áudio (float32)
        sample_rate: Taxa de amostragem (Hz)
        language: Idioma ("auto", "en", "pt", etc.)
        use_gpu: Usar GPU se disponível
        model_size: Tamanho do modelo ("large-v3", "medium", "base", etc.)
    
    Returns:
        dict com texto transcrito, confiança, idioma detectado, etc.
    """
    try:
        # Detectar dispositivo
        if use_gpu and torch.cuda.is_available():
            device = "cuda"
            compute_type = "float16"  # Mais rápido na GPU
            print(f"🐍 Whisper Python: Usando GPU ({torch.cuda.get_device_name(0)})", file=sys.stderr)
        else:
            device = "cpu"
            compute_type = "int8"  # Mais rápido na CPU
            print("🐍 Whisper Python: Usando CPU", file=sys.stderr)
        
        # Carregar modelo
        print(f"🐍 Whisper Python: Carregando modelo {model_size}...", file=sys.stderr)
        model = WhisperModel(model_size, device=device, compute_type=compute_type)
        print("🐍 Whisper Python: Modelo carregado!", file=sys.stderr)
        
        # Converter lista para numpy array
        audio_array = np.array(audio_data, dtype=np.float32)
        
        # Normalizar se necessário
        if audio_array.max() > 1.0 or audio_array.min() < -1.0:
            audio_array = audio_array / np.max(np.abs(audio_array))
        
        # Transcrever
        print(f"🐍 Whisper Python: Transcrevendo áudio ({len(audio_array)} amostras, {sample_rate} Hz)...", file=sys.stderr)
        
        # faster-whisper aceita numpy array diretamente
        segments, info = model.transcribe(
            audio_array,
            beam_size=5,
            language=language if language != "auto" else None,
            task="transcribe",
            vad_filter=True,  # Voice Activity Detection
            vad_parameters=dict(min_silence_duration_ms=500),
        )
        
        # Coletar segmentos
        full_text = ""
        segments_list = []
        total_confidence = 0.0
        segment_count = 0
        
        for segment in segments:
            text = segment.text.strip()
            if text:
                full_text += text + " "
                segments_list.append({
                    "text": text,
                    "start": segment.start,
                    "end": segment.end,
                    "confidence": getattr(segment, 'avg_logprob', 0.0),
                })
                total_confidence += getattr(segment, 'avg_logprob', 0.0)
                segment_count += 1
        
        full_text = full_text.strip()
        
        # Calcular confiança média
        avg_confidence = total_confidence / segment_count if segment_count > 0 else 0.0
        # Converter logprob para confiança (aproximação)
        confidence = min(1.0, max(0.0, np.exp(avg_confidence) if avg_confidence < 0 else 1.0))
        
        detected_language = info.language if hasattr(info, 'language') else language
        language_probability = getattr(info, 'language_probability', 1.0)
        
        print(f"🐍 Whisper Python: Transcrição completa! Texto: '{full_text[:50]}...'", file=sys.stderr)
        print(f"🐍 Whisper Python: Idioma detectado: {detected_language} (confiança: {language_probability:.2f})", file=sys.stderr)
        
        return {
            "text": full_text,
            "confidence": float(confidence),
            "language": detected_language,
            "language_probability": float(language_probability),
            "segments": segments_list,
            "duration_ms": int((len(audio_array) / sample_rate) * 1000),
        }
        
    except Exception as e:
        print(json.dumps({"error": f"Whisper transcription failed: {str(e)}"}), file=sys.stderr)
        import traceback
        traceback.print_exc(file=sys.stderr)
        sys.exit(1)


def main():
    """Main entry point"""
    try:
        # Ler input JSON do stdin
        input_data = json.load(sys.stdin)
        
        audio_data = input_data.get("audio_data", [])
        sample_rate = input_data.get("sample_rate", 16000)
        language = input_data.get("language", "auto")
        use_gpu = input_data.get("use_gpu", True)
        model_size = input_data.get("model_size", "large-v3")
        
        if not audio_data:
            print(json.dumps({"error": "No audio data provided"}), file=sys.stderr)
            sys.exit(1)
        
        # Transcrever
        result = transcribe_audio_with_whisper(
            audio_data=audio_data,
            sample_rate=sample_rate,
            language=language,
            use_gpu=use_gpu,
            model_size=model_size,
        )
        
        # Output JSON
        print(json.dumps(result))
        sys.stdout.flush()
        
    except Exception as e:
        print(json.dumps({"error": f"Failed to process request: {str(e)}"}), file=sys.stderr)
        import traceback
        traceback.print_exc(file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()

