#!/usr/bin/env python3
"""
Teste direto do script SoVITS
"""

import sys
import tempfile
from pathlib import Path
import numpy as np
import soundfile as sf

# Adicionar scripts ao path
scripts_dir = Path(__file__).parent / "scripts"
sys.path.insert(0, str(scripts_dir))

# Adicionar SoVITS ao path
script_dir = Path(__file__).parent
tts_service_dir = script_dir.parent
vrpg_client_dir = tts_service_dir.parent.parent
sovits_dir = vrpg_client_dir / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

print("=" * 60)
print("TESTE DIRETO DO SOVITS")
print("=" * 60)

# Criar áudio de teste (sine wave simples)
print("\n1. Criando áudio de teste (Piper simulado)...")
sample_rate = 22050
duration = 2.0  # 2 segundos
t = np.linspace(0, duration, int(sample_rate * duration))
# Gerar um tom simples (440 Hz = A4)
test_audio = np.sin(2 * np.pi * 440 * t).astype(np.float32)

# Normalizar
test_audio = test_audio / np.max(np.abs(test_audio)) * 0.8

# Salvar áudio de teste
with tempfile.NamedTemporaryFile(suffix='.wav', delete=False) as tmp_input:
    input_wav = tmp_input.name
    sf.write(input_wav, test_audio, sample_rate)
    print(f"   ✅ Áudio de teste salvo: {input_wav}")

# Testar conversão SoVITS
print("\n2. Testando conversão SoVITS...")
try:
    from sovits_convert import convert_audio_with_sovits
    
    model_path = str(sovits_dir / "dungeon_master_en.pth")
    config_path = str(sovits_dir / "config.json")
    speaker = "dungeon_master_en"
    
    with tempfile.NamedTemporaryFile(suffix='.wav', delete=False) as tmp_output:
        output_wav = tmp_output.name
    
    print(f"   Modelo: {model_path}")
    print(f"   Config: {config_path}")
    print(f"   Speaker: {speaker}")
    print(f"   Entrada: {input_wav}")
    print(f"   Saída: {output_wav}")
    
    success = convert_audio_with_sovits(
        input_wav_path=input_wav,
        model_path=model_path,
        config_path=config_path,
        speaker=speaker,
        output_wav_path=output_wav,
        device=None,
        auto_predict_f0=True,
        f0_predictor="rmvpe",
        noice_scale=0.4,
        trans=0,
    )
    
    if success:
        print(f"   ✅ Conversão SoVITS bem-sucedida!")
        print(f"   Áudio convertido salvo em: {output_wav}")
        
        # Verificar áudio convertido
        converted_audio, converted_sr = sf.read(output_wav)
        print(f"   Sample rate convertido: {converted_sr} Hz")
        print(f"   Duração: {len(converted_audio) / converted_sr:.2f} segundos")
        print(f"   Amplitudes: min={converted_audio.min():.4f}, max={converted_audio.max():.4f}")
    else:
        print("   ❌ Conversão SoVITS falhou")
        
except Exception as e:
    print(f"   ❌ Erro: {e}")
    import traceback
    traceback.print_exc()

print("\n" + "=" * 60)
print("TESTE CONCLUÍDO")
print("=" * 60)

