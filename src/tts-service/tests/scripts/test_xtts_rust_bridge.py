#!/usr/bin/env python3
"""
Test script that simulates the Python bridge used by Rust XTTS module
This validates the exact interface that Rust will use
"""

import sys
import json
import numpy as np
from TTS.api import TTS

def main():
    """Simulate Rust Python bridge call"""
    if len(sys.argv) < 2:
        print(json.dumps({"error": "Missing input JSON file"}))
        sys.exit(1)
    
    input_file = sys.argv[1]
    
    try:
        # Read input (as Rust would write it)
        with open(input_file, 'r', encoding='utf-8-sig') as f:
            data = json.load(f)
        
        text = data.get("text", "")
        language = data.get("language", "en")
        speaker = data.get("speaker", None)
        use_gpu = data.get("use_gpu", False)
        
        # Load XTTS model
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu)
        
        # Generate audio
        audio = tts.tts(
            text=text,
            speaker=speaker if speaker else None,
            language=language,
        )
        
        # Convert to list (JSON serializable)
        if isinstance(audio, np.ndarray):
            audio_list = audio.tolist()
        else:
            audio_list = list(audio)
        
        # Output format expected by Rust
        output = {
            "samples": audio_list,
            "sample_rate": tts.synthesizer.output_sample_rate,
            "channels": 1
        }
        
        # Print JSON (Rust will read from stdout)
        print(json.dumps(output))
        
    except Exception as e:
        # Error output (Rust will read from stderr)
        print(json.dumps({"error": str(e)}), file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()


