#!/usr/bin/env python3
"""
Test script for Coqui XTTS integration
Validates that Coqui TTS is properly installed and can generate audio
"""

import sys
import json
import numpy as np
from pathlib import Path

def test_coqui_tts_installation():
    """Test if Coqui TTS is installed"""
    try:
        from TTS.api import TTS
        print("✅ Coqui TTS is installed")
        return True
    except ImportError as e:
        print(f"❌ Coqui TTS not installed: {e}")
        print("   Install with: pip install TTS")
        return False

def test_xtts_model_loading():
    """Test loading XTTS model"""
    try:
        from TTS.api import TTS
        
        print("📦 Loading XTTS model (this may take a while on first run)...")
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=False)
        print("✅ XTTS model loaded successfully")
        return tts
    except Exception as e:
        print(f"❌ Failed to load XTTS model: {e}")
        return None

def test_xtts_synthesis(tts, text="Hello, this is a test of XTTS.", language="en", speaker=None):
    """Test XTTS synthesis"""
    try:
        print(f"🎤 Synthesizing: '{text}' (lang: {language}, speaker: {speaker})")
        
        audio = tts.tts(
            text=text,
            speaker=speaker,
            language=language,
        )
        
        # Convert to numpy array if needed
        if isinstance(audio, list):
            audio = np.array(audio)
        
        print(f"✅ Generated {len(audio)} samples")
        print(f"   Sample rate: {tts.synthesizer.output_sample_rate} Hz")
        print(f"   Duration: {len(audio) / tts.synthesizer.output_sample_rate:.2f}s")
        print(f"   Range: [{audio.min():.4f}, {audio.max():.4f}]")
        
        return audio, tts.synthesizer.output_sample_rate
    except Exception as e:
        print(f"❌ Synthesis failed: {e}")
        return None, None

def test_xtts_multilingual(tts):
    """Test multilingual support"""
    test_cases = [
        ("Hello, world!", "en"),
        ("Olá, mundo!", "pt"),
        ("Hola, mundo!", "es"),
    ]
    
    print("\n🌍 Testing multilingual support:")
    for text, lang in test_cases:
        audio, sample_rate = test_xtts_synthesis(tts, text, lang)
        if audio is not None:
            print(f"   ✅ {lang}: OK")
        else:
            print(f"   ❌ {lang}: Failed")

def test_xtts_output_format():
    """Test output format for Rust integration"""
    try:
        from TTS.api import TTS
        
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=False)
        audio = tts.tts(text="Test", language="en")
        
        # Convert to format expected by Rust
        if isinstance(audio, np.ndarray):
            audio_list = audio.tolist()
        else:
            audio_list = list(audio)
        
        output = {
            "samples": audio_list,
            "sample_rate": tts.synthesizer.output_sample_rate,
            "channels": 1
        }
        
        # Test JSON serialization (what Rust will receive)
        json_output = json.dumps(output)
        parsed = json.loads(json_output)
        
        assert "samples" in parsed
        assert "sample_rate" in parsed
        assert "channels" in parsed
        assert len(parsed["samples"]) > 0
        
        print("✅ Output format is compatible with Rust integration")
        return True
    except Exception as e:
        print(f"❌ Output format test failed: {e}")
        return False

def main():
    """Run all tests"""
    print("🧪 Testing Coqui XTTS Integration\n")
    
    # Test 1: Installation
    if not test_coqui_tts_installation():
        sys.exit(1)
    
    print()
    
    # Test 2: Model loading
    tts = test_xtts_model_loading()
    if tts is None:
        sys.exit(1)
    
    print()
    
    # Test 3: Basic synthesis
    audio, sample_rate = test_xtts_synthesis(tts)
    if audio is None:
        sys.exit(1)
    
    print()
    
    # Test 4: Multilingual (optional, may take time)
    if "--multilingual" in sys.argv:
        test_xtts_multilingual(tts)
        print()
    
    # Test 5: Output format
    if not test_xtts_output_format():
        sys.exit(1)
    
    print("\n✅ All tests passed!")

if __name__ == "__main__":
    main()


