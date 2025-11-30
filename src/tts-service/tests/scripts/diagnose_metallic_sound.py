#!/usr/bin/env python3
"""
Diagnóstico: Som Metálico em Todos os Testes

Se todos os testes soam igualmente metálicos, o problema não é dos parâmetros,
mas algo mais fundamental. Este script investiga as possíveis causas.
"""

import sys
import os
from pathlib import Path
import soundfile as sf
import numpy as np
import json

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent
tests_dir = script_dir.parent
tts_service_dir = tests_dir.parent
vrpg_client_dir = tts_service_dir.parent.parent
sovits_dir = vrpg_client_dir / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

print("\n" + "="*70)
print("  DIAGNÓSTICO: Som Metálico em Todos os Testes")
print("="*70 + "\n")

# 1. Verificar áudio de entrada (XTTS)
print("1️⃣  VERIFICANDO ÁUDIO DE ENTRADA (XTTS)")
print("-" * 70)
input_audio_path = script_dir / "test_hello_world_xtts_real.wav"

if input_audio_path.exists():
    audio_input, sr_input = sf.read(str(input_audio_path))
    if len(audio_input.shape) > 1:
        audio_input = np.mean(audio_input, axis=1)
    
    print(f"   ✅ Áudio encontrado: {input_audio_path.name}")
    print(f"   📊 Sample rate: {sr_input} Hz")
    print(f"   📊 Duração: {len(audio_input) / sr_input:.2f}s")
    print(f"   📊 Amostras: {len(audio_input)}")
    
    # Análise espectral básica
    max_amp = np.max(np.abs(audio_input))
    rms = np.sqrt(np.mean(audio_input**2))
    zero_crossings = np.sum(np.diff(np.sign(audio_input)) != 0)
    zcr = zero_crossings / len(audio_input)
    
    print(f"   📊 Max amplitude: {max_amp:.4f}")
    print(f"   📊 RMS: {rms:.4f}")
    print(f"   📊 Zero crossing rate: {zcr:.4f}")
    
    # Verificar clipping
    clipped_samples = np.sum(np.abs(audio_input) >= 0.99)
    if clipped_samples > 0:
        print(f"   ⚠️  CLIPPING DETECTADO: {clipped_samples} amostras ({clipped_samples/len(audio_input)*100:.2f}%)")
    else:
        print(f"   ✅ Sem clipping")
    
    # Verificar se já está metálico (alta frequência excessiva)
    # FFT básico
    fft = np.fft.rfft(audio_input)
    freqs = np.fft.rfftfreq(len(audio_input), 1/sr_input)
    magnitude = np.abs(fft)
    
    # Energia em diferentes bandas
    low_freq = np.sum(magnitude[freqs < 1000])  # < 1kHz
    mid_freq = np.sum(magnitude[(freqs >= 1000) & (freqs < 5000)])  # 1-5kHz
    high_freq = np.sum(magnitude[freqs >= 5000])  # > 5kHz
    
    total_energy = low_freq + mid_freq + high_freq
    if total_energy > 0:
        low_pct = (low_freq / total_energy) * 100
        mid_pct = (mid_freq / total_energy) * 100
        high_pct = (high_freq / total_energy) * 100
        
        print(f"   📊 Distribuição espectral:")
        print(f"      Baixas (< 1kHz): {low_pct:.1f}%")
        print(f"      Médias (1-5kHz): {mid_pct:.1f}%")
        print(f"      Altas (> 5kHz): {high_pct:.1f}%")
        
        if high_pct > 30:
            print(f"   ⚠️  ALTA ENERGIA EM FREQUÊNCIAS ALTAS - pode indicar som metálico")
else:
    print(f"   ❌ Áudio de entrada não encontrado!")
    audio_input = None
    sr_input = None

print()

# 2. Verificar áudio de saída (SoVITS)
print("2️⃣  VERIFICANDO ÁUDIO DE SAÍDA (SoVITS)")
print("-" * 70)
output_dir = script_dir / "sovits_quality_tests"
test_output = output_dir / "09_optimized.wav"

if test_output.exists():
    audio_output, sr_output = sf.read(str(test_output))
    if len(audio_output.shape) > 1:
        audio_output = np.mean(audio_output, axis=1)
    
    print(f"   ✅ Áudio encontrado: {test_output.name}")
    print(f"   📊 Sample rate: {sr_output} Hz")
    print(f"   📊 Duração: {len(audio_output) / sr_output:.2f}s")
    print(f"   📊 Amostras: {len(audio_output)}")
    
    # Análise espectral
    max_amp = np.max(np.abs(audio_output))
    rms = np.sqrt(np.mean(audio_output**2))
    
    print(f"   📊 Max amplitude: {max_amp:.4f}")
    print(f"   📊 RMS: {rms:.4f}")
    
    # FFT
    fft = np.fft.rfft(audio_output)
    freqs = np.fft.rfftfreq(len(audio_output), 1/sr_output)
    magnitude = np.abs(fft)
    
    low_freq = np.sum(magnitude[freqs < 1000])
    mid_freq = np.sum(magnitude[(freqs >= 1000) & (freqs < 5000)])
    high_freq = np.sum(magnitude[freqs >= 5000])
    
    total_energy = low_freq + mid_freq + high_freq
    if total_energy > 0:
        low_pct = (low_freq / total_energy) * 100
        mid_pct = (mid_freq / total_energy) * 100
        high_pct = (high_freq / total_energy) * 100
        
        print(f"   📊 Distribuição espectral:")
        print(f"      Baixas (< 1kHz): {low_pct:.1f}%")
        print(f"      Médias (1-5kHz): {mid_pct:.1f}%")
        print(f"      Altas (> 5kHz): {high_pct:.1f}%")
        
        if high_pct > 30:
            print(f"   ⚠️  ALTA ENERGIA EM FREQUÊNCIAS ALTAS - confirma som metálico")
else:
    print(f"   ❌ Áudio de saída não encontrado!")
    audio_output = None
    sr_output = None

print()

# 3. Verificar sample rate mismatch
print("3️⃣  VERIFICANDO SAMPLE RATE MISMATCH")
print("-" * 70)
if audio_input is not None and audio_output is not None:
    print(f"   Input (XTTS): {sr_input} Hz")
    print(f"   Output (SoVITS): {sr_output} Hz")
    
    if sr_input != sr_output:
        print(f"   ⚠️  MISMATCH DETECTADO!")
        print(f"   ⚠️  SoVITS está re-amostrando de {sr_input} Hz → {sr_output} Hz")
        print(f"   ⚠️  Isso pode introduzir artefatos metálicos!")
        
        # Calcular fator de re-amostragem
        ratio = sr_output / sr_input
        print(f"   📊 Fator de re-amostragem: {ratio:.4f}")
        
        if ratio != 1.0:
            print(f"   💡 SOLUÇÃO: Converter XTTS para {sr_output} Hz ANTES do SoVITS")
    else:
        print(f"   ✅ Sample rates compatíveis")
else:
    print(f"   ⚠️  Não foi possível verificar (arquivos não encontrados)")

print()

# 4. Verificar modelo SoVITS
print("4️⃣  VERIFICANDO MODELO SOVITS")
print("-" * 70)
model_path = sovits_dir / "dungeon_master_en.pth"
config_path = sovits_dir / "config.json"

if model_path.exists():
    print(f"   ✅ Modelo encontrado: {model_path.name}")
    model_size = model_path.stat().st_size / (1024 * 1024)  # MB
    print(f"   📊 Tamanho: {model_size:.2f} MB")
    
    # Verificar se há checkpoints anteriores
    logs_dir = sovits_dir / "logs" / "44k"
    if logs_dir.exists():
        checkpoints = list(logs_dir.glob("G_*.pth"))
        if len(checkpoints) > 1:
            print(f"   📊 Checkpoints encontrados: {len(checkpoints)}")
            print(f"   💡 Dica: Teste com checkpoint anterior (pode ter menos overfitting)")
        else:
            print(f"   ⚠️  Apenas 1 checkpoint encontrado")
    else:
        print(f"   ⚠️  Diretório de logs não encontrado")
else:
    print(f"   ❌ Modelo não encontrado!")

if config_path.exists():
    print(f"   ✅ Config encontrado: {config_path.name}")
    try:
        with open(config_path, 'r', encoding='utf-8') as f:
            config = json.load(f)
            if 'data' in config and 'sampling_rate' in config['data']:
                model_sr = config['data']['sampling_rate']
                print(f"   📊 Sample rate do modelo: {model_sr} Hz")
                
                if audio_input is not None and sr_input != model_sr:
                    print(f"   ⚠️  MISMATCH: Input ({sr_input} Hz) ≠ Modelo ({model_sr} Hz)")
    except Exception:
        pass

print()

# 5. Verificar dataset original
print("5️⃣  VERIFICANDO DATASET ORIGINAL")
print("-" * 70)
dataset_dir = sovits_dir / "dataset_raw" / "dungeon_master_en"
if dataset_dir.exists():
    wav_files = list(dataset_dir.rglob("*.wav"))
    if wav_files:
        print(f"   ✅ Dataset encontrado: {len(wav_files)} arquivos WAV")
        
        # Verificar sample rate do primeiro arquivo
        try:
            sample_audio, sample_sr = sf.read(str(wav_files[0]))
            print(f"   📊 Sample rate do dataset: {sample_sr} Hz")
            
            if audio_output is not None and sample_sr != sr_output:
                print(f"   ⚠️  MISMATCH: Dataset ({sample_sr} Hz) ≠ Output ({sr_output} Hz)")
        except:
            print(f"   ⚠️  Não foi possível ler arquivo do dataset")
    else:
        print(f"   ⚠️  Dataset vazio ou sem arquivos WAV")
else:
    print(f"   ⚠️  Dataset não encontrado")

print()

# 6. Diagnóstico e recomendações
print("="*70)
print("  DIAGNÓSTICO E RECOMENDAÇÕES")
print("="*70)
print()

print("🔍 POSSÍVEIS CAUSAS DO SOM METÁLICO:")
print()

issues = []

if audio_input is not None and audio_output is not None:
    if sr_input != sr_output:
        issues.append("Sample rate mismatch (re-amostragem introduz artefatos)")
    
    # Comparar espectros
    if audio_input is not None:
        fft_in = np.fft.rfft(audio_input)
        freqs_in = np.fft.rfftfreq(len(audio_input), 1/sr_input)
        mag_in = np.abs(fft_in)
        high_in = np.sum(mag_in[freqs_in >= 5000])
        total_in = np.sum(mag_in)
        high_pct_in = (high_in / total_in * 100) if total_in > 0 else 0
        
        if high_pct_in > 30:
            issues.append("Áudio de entrada (XTTS) já tem características metálicas")

if len(issues) == 0:
    issues.append("Problema provavelmente no modelo treinado ou dataset")

for i, issue in enumerate(issues, 1):
    print(f"   {i}. {issue}")

print()
print("💡 SOLUÇÕES RECOMENDADAS:")
print()
print("   1. CONVERTER XTTS PARA 44100 Hz ANTES DO SOVITS")
print("      - Evita re-amostragem no SoVITS")
print("      - Pode eliminar artefatos metálicos")
print()
print("   2. TESTAR COM ÁUDIO ORIGINAL DO DATASET")
print("      - Se o áudio original soa bem, o problema é no XTTS")
print("      - Se o áudio original também soa metálico, problema é no modelo")
print()
print("   3. TESTAR COM CHECKPOINT ANTERIOR")
print("      - Overfitting pode causar som metálico")
print("      - Checkpoint anterior pode ser melhor")
print()
print("   4. VERIFICAR QUALIDADE DO DATASET")
print("      - Vocal extraído de música? (pode ter reverb/artefatos)")
print("      - Compressão excessiva? (MP3 baixo bitrate)")
print("      - Sample rate inconsistente?")
print()
print("   5. RE-TREINAR COM DATASET MELHOR")
print("      - 20-30 min mínimo de áudio limpo")
print("      - WAV 16-bit, 44.1k/48k mono")
print("      - Sem vocal extraído de música")
print()

print("="*70)
print()

