#!/usr/bin/env python3
"""
Análise de Qualidade do Dataset

Verifica problemas conhecidos que causam som metálico/robótico:
- Vocal extraído de música (reverb/artefatos)
- Compressão excessiva (MP3, baixo bitrate)
- Sample rate inconsistente
- Clipping/distorção
- Qualidade geral dos arquivos
"""

import sys
import os
from pathlib import Path
import soundfile as sf
import numpy as np
from collections import Counter

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent
tests_dir = script_dir.parent
tts_service_dir = tests_dir.parent
vrpg_client_dir = tts_service_dir.parent.parent
sovits_dir = vrpg_client_dir / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

print("\n" + "="*70)
print("  ANÁLISE DE QUALIDADE DO DATASET")
print("="*70 + "\n")

# 1. Encontrar dataset
dataset_dir = sovits_dir / "dataset_raw" / "dungeon_master_en"

if not dataset_dir.exists():
    print("❌ Dataset não encontrado!", file=sys.stderr)
    sys.exit(1)

wav_files = list(dataset_dir.rglob("*.wav"))

if not wav_files:
    print("❌ Nenhum arquivo WAV encontrado!", file=sys.stderr)
    sys.exit(1)

print(f"📁 Dataset encontrado: {len(wav_files)} arquivos WAV\n")

# 2. Análise dos arquivos
issues = {
    "sample_rate_inconsistent": [],
    "compression_artifacts": [],
    "clipping": [],
    "low_energy": [],
    "high_freq_energy": [],
    "short_duration": [],
    "mono_issues": [],
}

sample_rates = []
durations = []
total_duration = 0.0

print("🔍 Analisando arquivos...\n")

for i, wav_file in enumerate(wav_files[:50], 1):  # Analisar primeiros 50
    try:
        audio, sr = sf.read(str(wav_file))
        
        # Converter para mono se necessário
        if len(audio.shape) > 1:
            audio = np.mean(audio, axis=1)
            issues["mono_issues"].append(wav_file.name)
        
        duration = len(audio) / sr
        durations.append(duration)
        total_duration += duration
        sample_rates.append(sr)
        
        # Verificar clipping
        max_amp = np.max(np.abs(audio))
        if max_amp > 0.95:
            issues["clipping"].append((wav_file.name, max_amp))
        
        # Verificar energia baixa
        rms = np.sqrt(np.mean(audio**2))
        if rms < 0.01:
            issues["low_energy"].append((wav_file.name, rms))
        
        # Análise espectral (verificar artefatos de compressão)
        fft = np.fft.rfft(audio)
        freqs = np.fft.rfftfreq(len(audio), 1/sr)
        magnitude = np.abs(fft)
        
        # Energia em diferentes bandas
        low_freq = np.sum(magnitude[freqs < 1000])
        mid_freq = np.sum(magnitude[(freqs >= 1000) & (freqs < 5000)])
        high_freq = np.sum(magnitude[freqs >= 5000])
        total_energy = low_freq + mid_freq + high_freq
        
        if total_energy > 0:
            high_pct = (high_freq / total_energy) * 100
            # Alta energia em frequências altas pode indicar artefatos de compressão
            if high_pct > 25:
                issues["high_freq_energy"].append((wav_file.name, high_pct))
        
        # Verificar duração muito curta
        if duration < 0.5:
            issues["short_duration"].append((wav_file.name, duration))
        
        if i % 10 == 0:
            print(f"   Analisados: {i}/{min(50, len(wav_files))}...", end='\r')
            
    except Exception as e:
        print(f"\n   ⚠️  Erro ao analisar {wav_file.name}: {e}")

print(f"\n   ✅ Análise concluída: {min(50, len(wav_files))} arquivos\n")

# 3. Estatísticas gerais
print("="*70)
print("  ESTATÍSTICAS GERAIS")
print("="*70)
print()

# Sample rates
sr_counter = Counter(sample_rates)
print("📊 Sample Rates:")
for sr, count in sr_counter.most_common():
    pct = (count / len(sample_rates)) * 100
    print(f"   {sr} Hz: {count} arquivos ({pct:.1f}%)")
    if len(sr_counter) > 1:
        issues["sample_rate_inconsistent"].append(f"{sr} Hz ({count} arquivos)")

print()

# Duração total
print(f"📊 Duração Total:")
print(f"   {total_duration/60:.1f} minutos (dos primeiros {min(50, len(wav_files))} arquivos)")
print(f"   Média: {np.mean(durations):.2f}s por arquivo")
print(f"   Mínimo: {np.min(durations):.2f}s")
print(f"   Máximo: {np.max(durations):.2f}s")
print()

# 4. Problemas encontrados
print("="*70)
print("  PROBLEMAS ENCONTRADOS")
print("="*70)
print()

problem_count = 0

if issues["sample_rate_inconsistent"]:
    problem_count += 1
    print(f"❌ 1. SAMPLE RATE INCONSISTENTE")
    print(f"   {len(issues['sample_rate_inconsistent'])} sample rates diferentes encontrados")
    print(f"   Isso pode causar problemas no treinamento!")
    print()

if issues["clipping"]:
    problem_count += 1
    print(f"❌ 2. CLIPPING DETECTADO")
    print(f"   {len(issues['clipping'])} arquivos com clipping")
    print(f"   Primeiros 5: {[f[0] for f in issues['clipping'][:5]]}")
    print(f"   Clipping pode causar distorção e som metálico!")
    print()

if issues["compression_artifacts"] or issues["high_freq_energy"]:
    problem_count += 1
    print(f"❌ 3. POSSÍVEIS ARTEFATOS DE COMPRESSÃO")
    print(f"   {len(issues['high_freq_energy'])} arquivos com alta energia em frequências altas")
    print(f"   Isso pode indicar:")
    print(f"   - Vocal extraído de música (UVR, etc.)")
    print(f"   - Compressão MP3 excessiva")
    print(f"   - Rips de YouTube/streaming")
    print(f"   → CAUSA COMUM DE SOM METÁLICO!")
    print()

if issues["low_energy"]:
    problem_count += 1
    print(f"⚠️  4. ENERGIA BAIXA")
    print(f"   {len(issues['low_energy'])} arquivos com energia muito baixa")
    print(f"   Pode afetar qualidade do treinamento")
    print()

if issues["short_duration"]:
    problem_count += 1
    print(f"⚠️  5. DURAÇÃO MUITO CURTA")
    print(f"   {len(issues['short_duration'])} arquivos com < 0.5s")
    print(f"   Arquivos muito curtos podem não ter contexto suficiente")
    print()

if issues["mono_issues"]:
    print(f"ℹ️  6. ARQUIVOS ESTÉREO CONVERTIDOS")
    print(f"   {len(issues['mono_issues'])} arquivos foram convertidos para mono")
    print(f"   (Normal, mas verifique se não há perda de qualidade)")
    print()

if problem_count == 0:
    print("✅ Nenhum problema óbvio encontrado nos primeiros 50 arquivos")
    print("   (Mas o som metálico pode ser de outras causas)")
    print()

# 5. Recomendações
print("="*70)
print("  RECOMENDAÇÕES")
print("="*70)
print()

if issues["sample_rate_inconsistent"]:
    print("1. ✅ PADRONIZAR SAMPLE RATE")
    print("   Todos os arquivos devem ter o mesmo sample rate (44.1k ou 48k)")
    print()

if issues["clipping"]:
    print("2. ✅ REMOVER CLIPPING")
    print("   Normalizar arquivos com clipping")
    print("   Reduzir ganho se necessário")
    print()

if issues["high_freq_energy"] or issues["compression_artifacts"]:
    print("3. ⚠️  VERIFICAR FONTE DO ÁUDIO")
    print("   Se o áudio foi extraído de música:")
    print("   - Usar apenas trechos MUITO limpos")
    print("   - Preferir gravações originais do locutor")
    print("   - Evitar rips de YouTube/streaming")
    print()

if total_duration < 20 * 60:
    print("4. ⚠️  DURAÇÃO TOTAL PODE SER INSUFICIENTE")
    print(f"   Atual: {total_duration/60:.1f} minutos")
    print("   Recomendado: 20-30 minutos mínimo")
    print("   Pouco áudio pode resultar em voz instável/robótica")
    print()

print("5. 💡 RE-TREINAR COM DATASET MELHOR")
print("   - 20-30 min mínimo de áudio limpo")
print("   - WAV 16-bit, 44.1k/48k mono, consistente")
print("   - Sem vocal extraído de música (ou apenas trechos muito limpos)")
print("   - Sem compressão excessiva (evitar MP3, rips)")
print("   - Sem clipping ou distorção")
print()

print("="*70)
print()

