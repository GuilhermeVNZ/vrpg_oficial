#!/usr/bin/env python3
"""
Analisa arquivos de áudio dos benchmarks para comparar CPU vs GPU
"""

import soundfile as sf
from pathlib import Path
from collections import defaultdict

script_dir = Path(__file__).parent
old_dir = script_dir / "benchmark_5_voices"
new_dir = script_dir / "benchmark_5_voices_optimized"

# Timestamps
OLD_TIMESTAMP = "20251129_072746"  # CPU
NEW_TIMESTAMP = "20251129_084953"  # GPU

def analyze_audio_file(filepath):
    """Analisa um arquivo de áudio e retorna informações"""
    try:
        audio, sr = sf.read(str(filepath))
        duration = len(audio) / sr
        channels = 1 if len(audio.shape) == 1 else audio.shape[1]
        samples = len(audio)
        return {
            "duration": duration,
            "sample_rate": sr,
            "channels": channels,
            "samples": samples,
            "file_size_mb": filepath.stat().st_size / (1024 * 1024)
        }
    except Exception as e:
        return {"error": str(e)}

def extract_voice_and_text(filename):
    """Extrai nome da voz e tipo de texto do nome do arquivo"""
    # Formato: benchmark_<voice>_texto_<type>_<timestamp>.wav
    parts = filename.stem.replace("benchmark_", "").replace(f"_{OLD_TIMESTAMP}", "").replace(f"_{NEW_TIMESTAMP}", "").split("_")
    
    # Encontrar onde começa "texto"
    texto_idx = None
    for i, part in enumerate(parts):
        if part == "texto":
            texto_idx = i
            break
    
    if texto_idx:
        voice_parts = parts[:texto_idx]
        text_type = "_".join(parts[texto_idx+1:])
        voice = " ".join(voice_parts).title()
        return voice, text_type
    return None, None

def main():
    print("="*100)
    print("  ANÁLISE COMPARATIVA: BENCHMARK CPU vs GPU")
    print("="*100)
    print()
    
    # Encontrar arquivos
    old_files = sorted([f for f in old_dir.glob("*.wav") if OLD_TIMESTAMP in f.name])
    new_files = sorted([f for f in new_dir.glob("*.wav") if NEW_TIMESTAMP in f.name])
    
    print(f"📁 Arquivos encontrados:")
    print(f"   Benchmark antigo (CPU): {len(old_files)} arquivos")
    print(f"   Benchmark novo (GPU): {len(new_files)} arquivos")
    print()
    
    # Organizar por voz e tipo de texto
    old_data = defaultdict(dict)
    new_data = defaultdict(dict)
    
    print("📊 Analisando arquivos antigos (CPU)...")
    for f in old_files:
        voice, text_type = extract_voice_and_text(f)
        if voice and text_type:
            info = analyze_audio_file(f)
            old_data[voice][text_type] = info
            print(f"   ✅ {voice} - {text_type}: {info.get('duration', 'N/A'):.2f}s")
    
    print("\n📊 Analisando arquivos novos (GPU)...")
    for f in new_files:
        voice, text_type = extract_voice_and_text(f)
        if voice and text_type:
            info = analyze_audio_file(f)
            new_data[voice][text_type] = info
            print(f"   ✅ {voice} - {text_type}: {info.get('duration', 'N/A'):.2f}s")
    
    # Comparar
    print("\n" + "="*100)
    print("  COMPARAÇÃO: DURAÇÃO DOS ÁUDIOS GERADOS")
    print("="*100)
    print()
    
    # Texto curto
    print("📝 TEXTO CURTO (~5s):")
    print(f"{'Voz':<45} {'CPU (antigo)':<20} {'GPU (novo)':<20} {'Diferença':<15}")
    print("-" * 100)
    
    all_voices = set(list(old_data.keys()) + list(new_data.keys()))
    for voice in sorted(all_voices):
        old_short = old_data[voice].get("curto 5s", {})
        new_short = new_data[voice].get("curto 5s", {})
        
        old_dur = old_short.get("duration", None)
        new_dur = new_short.get("duration", None)
        
        old_str = f"{old_dur:.2f}s" if old_dur else "N/A ❌"
        new_str = f"{new_dur:.2f}s" if new_dur else "N/A ❌"
        
        if old_dur and new_dur:
            diff = new_dur - old_dur
            diff_str = f"{diff:+.2f}s ({diff/old_dur*100:+.1f}%)"
        else:
            diff_str = "N/A"
        
        print(f"{voice:<45} {old_str:<20} {new_str:<20} {diff_str:<15}")
    
    # Texto longo
    print("\n📝 TEXTO LONGO (~25s):")
    print(f"{'Voz':<45} {'CPU (antigo)':<20} {'GPU (novo)':<20} {'Diferença':<15}")
    print("-" * 100)
    
    for voice in sorted(all_voices):
        old_long = old_data[voice].get("longo 25s", {})
        new_long = new_data[voice].get("longo 25s", {})
        
        old_dur = old_long.get("duration", None)
        new_dur = new_long.get("duration", None)
        
        old_str = f"{old_dur:.2f}s" if old_dur else "N/A ❌"
        new_str = f"{new_dur:.2f}s" if new_dur else "N/A ❌"
        
        if old_dur and new_dur:
            diff = new_dur - old_dur
            diff_str = f"{diff:+.2f}s ({diff/old_dur*100:+.1f}%)"
        else:
            diff_str = "N/A"
        
        print(f"{voice:<45} {old_str:<20} {new_str:<20} {diff_str:<15}")
    
    # Resumo
    print("\n" + "="*100)
    print("  RESUMO")
    print("="*100)
    
    old_success = sum(1 for v in old_data.values() for t in v.values() if "error" not in t)
    new_success = sum(1 for v in new_data.values() for t in v.values() if "error" not in t)
    
    print(f"\n✅ Taxa de sucesso:")
    print(f"   CPU (antigo): {old_success}/10 arquivos ({old_success/10*100:.0f}%)")
    print(f"   GPU (novo): {new_success}/10 arquivos ({new_success/10*100:.0f}%)")
    print(f"   Melhoria: {new_success - old_success} arquivos adicionais funcionando")
    
    # Comparar apenas vozes que funcionaram nos dois
    print(f"\n🎯 Vozes que funcionaram nos dois benchmarks:")
    common_voices = set(old_data.keys()) & set(new_data.keys())
    for voice in sorted(common_voices):
        old_short = old_data[voice].get("curto 5s", {})
        new_short = new_data[voice].get("curto 5s", {})
        old_long = old_data[voice].get("longo 25s", {})
        new_long = new_data[voice].get("longo 25s", {})
        
        if old_short.get("duration") and new_short.get("duration"):
            print(f"   {voice}:")
            print(f"      Curto: CPU {old_short['duration']:.2f}s → GPU {new_short['duration']:.2f}s")
            if old_long.get("duration") and new_long.get("duration"):
                print(f"      Longo: CPU {old_long['duration']:.2f}s → GPU {new_long['duration']:.2f}s")
    
    print()

if __name__ == "__main__":
    main()



