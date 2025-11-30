#!/usr/bin/env python3
"""
Análise automática de qualidade de áudio e sugestões de correção
Analisa o áudio e sugere ajustes específicos baseados em problemas detectados
"""

import sys
import os
from pathlib import Path
import numpy as np

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent
sovits_dir = script_dir.parent.parent.parent.parent / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

try:
    import soundfile as sf
    from scipy import signal
    from scipy.fft import fft, fftfreq
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("   Instale: pip install soundfile scipy", file=sys.stderr)
    sys.exit(1)


def analyze_audio_quality(audio_path: Path):
    """Analisa qualidade do áudio e sugere correções"""
    print("\n" + "="*70)
    print("  ANÁLISE AUTOMÁTICA DE QUALIDADE DE ÁUDIO")
    print("="*70 + "\n")
    
    if not audio_path.exists():
        print(f"❌ ERRO: Arquivo não encontrado: {audio_path}")
        sys.exit(1)
    
    print(f"📁 Analisando: {audio_path.name}\n")
    
    # Carregar áudio
    try:
        audio, sr = sf.read(str(audio_path))
        
        # Converter para mono se necessário
        if len(audio.shape) > 1:
            audio = np.mean(audio, axis=1)
        
        audio = audio.astype(np.float32)
        
        print(f"✅ Áudio carregado:")
        print(f"   - Duração: {len(audio) / sr:.2f}s")
        print(f"   - Sample rate: {sr} Hz")
        print(f"   - Amostras: {len(audio)}\n")
        
    except Exception as e:
        print(f"❌ ERRO ao carregar áudio: {e}")
        sys.exit(1)
    
    # Análise espectral
    print("🔍 Realizando análise espectral...\n")
    
    nyquist = sr / 2
    n = len(audio)
    
    # FFT para análise de frequências
    fft_vals = fft(audio)
    fft_freq = fftfreq(n, 1/sr)
    
    # Apenas frequências positivas
    positive_freq_idx = fft_freq > 0
    fft_freq = fft_freq[positive_freq_idx]
    fft_magnitude = np.abs(fft_vals[positive_freq_idx])
    
    # Normalizar magnitude
    fft_magnitude_db = 20 * np.log10(fft_magnitude + 1e-10)
    
    # Análise por faixas de frequência
    def get_energy_in_band(freq_low, freq_high):
        """Calcula energia em uma faixa de frequência"""
        mask = (fft_freq >= freq_low) & (fft_freq <= freq_high)
        if np.any(mask):
            return np.mean(fft_magnitude_db[mask])
        return -np.inf
    
    # Faixas de frequência importantes
    bands = {
        "Baixas (20-200Hz)": (20, 200),
        "Médias-baixas (200-1000Hz)": (200, 1000),
        "Médias (1-3kHz)": (1000, 3000),
        "Médias-altas (3-6kHz)": (3000, 6000),
        "Altas (6-10kHz)": (6000, 10000),
        "Muito altas (10-12kHz)": (10000, 12000),
    }
    
    band_energies = {}
    for name, (low, high) in bands.items():
        if high <= nyquist:
            energy = get_energy_in_band(low, high)
            band_energies[name] = energy
    
    # Detectar problemas
    problems = []
    suggestions = []
    
    print("📊 Análise por faixas de frequência:")
    print("-" * 70)
    
    # Calcular energia média geral para comparação
    avg_energy = np.mean(fft_magnitude_db)
    
    for name, energy in band_energies.items():
        relative_energy = energy - avg_energy
        status = "✅"
        
        # Detectar problemas específicos (thresholds ajustados)
        if "3-6kHz" in name and relative_energy > 0:  # Mais sensível
            problems.append("Som metálico detectado (alta energia em 3-6kHz)")
            suggestions.append("Aplicar redução de 35-45% em 2-6kHz")
            status = "⚠️"
        elif "6-10kHz" in name and relative_energy > -2:  # Mais sensível
            problems.append("Chiado detectado (alta energia em 6-10kHz)")
            suggestions.append("Aplicar redução de 40-50% em 6-12kHz")
            status = "⚠️"
        elif "10-12kHz" in name and relative_energy > -5:  # Mais sensível
            problems.append("Chiado muito alto detectado (alta energia em 10-12kHz)")
            suggestions.append("Aplicar filtro passa-baixa em 9kHz com mix de 25-30%")
            status = "⚠️"
        elif "1-3kHz" in name and relative_energy < -5:
            problems.append("Voz abafada (baixa energia em 1-3kHz)")
            suggestions.append("Reduzir filtros agressivos ou aumentar ganho em 1-3kHz")
            status = "⚠️"
        
        print(f"{status} {name:30s}: {energy:6.1f} dB (relativo: {relative_energy:+6.1f} dB)")
    
    print("-" * 70)
    
    # Análise de DC offset
    dc_offset = np.mean(audio)
    if abs(dc_offset) > 0.001:
        problems.append(f"DC offset detectado ({dc_offset:.6f})")
        suggestions.append("Remover DC offset: audio = audio - np.mean(audio)")
    
    # Análise de clipping
    max_val = np.max(np.abs(audio))
    if max_val > 0.95:
        problems.append(f"Clipping detectado (pico: {max_val:.3f})")
        suggestions.append("Normalizar para 0.90-0.95 para evitar clipping")
    
    # Análise de ruído de fundo
    # Calcular energia em silêncio (últimos 10% do áudio, assumindo que pode ter silêncio)
    silence_samples = int(len(audio) * 0.1)
    if silence_samples > 0:
        silence_energy = np.mean(np.abs(audio[-silence_samples:]))
        speech_energy = np.mean(np.abs(audio))
        noise_ratio = silence_energy / speech_energy if speech_energy > 0 else 0
        
        if noise_ratio > 0.1:
            problems.append(f"Ruído de fundo detectado (ratio: {noise_ratio:.2%})")
            suggestions.append("Aplicar redução de ruído ou filtros high-pass/low-pass")
    
    # Análise de reverb (detectar cauda longa)
    # Calcular envelope do áudio
    envelope = np.abs(audio)
    # Suavizar envelope
    window_size = int(sr * 0.01)  # 10ms
    if window_size > 0:
        envelope_smooth = np.convolve(envelope, np.ones(window_size)/window_size, mode='same')
        # Verificar decaimento lento (característico de reverb)
        decay_rate = np.mean(np.diff(envelope_smooth[-int(sr*0.5):]))  # Últimos 0.5s
        if decay_rate > -0.0001:  # Decaimento muito lento
            problems.append("Reverb detectado (cauda longa)")
            suggestions.append("Aplicar redução de reverb: 15-20% na cauda + 8-10% em ressonâncias 1-4kHz")
    
    # Análise de distorção (harmônicos não naturais)
    # Verificar se há picos em frequências específicas que indicam distorção
    peak_freqs = []
    for i in range(1, len(fft_magnitude_db) - 1):
        if (fft_magnitude_db[i] > fft_magnitude_db[i-1] and 
            fft_magnitude_db[i] > fft_magnitude_db[i+1] and
            fft_magnitude_db[i] > avg_energy + 15):  # Pico significativo
            freq = fft_freq[i]
            if 2000 < freq < 8000:  # Faixa onde distorção é mais perceptível
                peak_freqs.append(freq)
    
    if len(peak_freqs) > 5:
        problems.append(f"Distorção/drive detectado ({len(peak_freqs)} picos em 2-8kHz)")
        suggestions.append("Aplicar soft clipping + redução de harmônicos em 8-12kHz (12-15%)")
    
    # Resumo de problemas
    print(f"\n🔍 Problemas detectados: {len(problems)}")
    print("-" * 70)
    
    if problems:
        for i, problem in enumerate(problems, 1):
            print(f"{i}. ⚠️  {problem}")
    else:
        print("✅ Nenhum problema crítico detectado!")
    
    # Sugestões de correção
    print(f"\n💡 Sugestões de correção:")
    print("-" * 70)
    
    if suggestions:
        for i, suggestion in enumerate(suggestions, 1):
            print(f"{i}. {suggestion}")
    else:
        print("✅ Áudio parece estar em boa qualidade!")
    
    # Parâmetros recomendados
    print(f"\n📋 Parâmetros recomendados baseados na análise:")
    print("-" * 70)
    
    # Calcular parâmetros baseados nos problemas detectados
    recommended_params = {
        "redução_chiado": 0,
        "redução_metálico": 0,
        "redução_reverb": 0,
        "redução_drive": False,
        "passa_baixa": 0,
    }
    
    if any("Chiado" in p for p in problems):
        if any("muito alto" in p.lower() for p in problems):
            recommended_params["redução_chiado"] = 50
            recommended_params["passa_baixa"] = 25
        else:
            recommended_params["redução_chiado"] = 40
            recommended_params["passa_baixa"] = 20
    
    if any("metálico" in p.lower() for p in problems):
        recommended_params["redução_metálico"] = 35
    
    if any("reverb" in p.lower() for p in problems):
        recommended_params["redução_reverb"] = 15
    
    if any("distorção" in p.lower() or "drive" in p.lower() for p in problems):
        recommended_params["redução_drive"] = True
    
    if recommended_params["redução_chiado"] > 0:
        print(f"   - Redução de chiado: {recommended_params['redução_chiado']}% (6-12kHz)")
    if recommended_params["redução_metálico"] > 0:
        print(f"   - Redução metálico: {recommended_params['redução_metálico']}% (2-6kHz)")
    if recommended_params["redução_reverb"] > 0:
        print(f"   - Redução reverb: {recommended_params['redução_reverb']}% (cauda) + 8% (ressonâncias)")
    if recommended_params["redução_drive"]:
        print(f"   - Redução drive: soft clipping + 12% harmônicos")
    if recommended_params["passa_baixa"] > 0:
        print(f"   - Passa-baixa: 9kHz ({recommended_params['passa_baixa']}% mix)")
    
    if all(v == 0 or v == False for v in recommended_params.values()):
        print("   - Processamento mínimo recomendado (áudio já está bom)")
    
    print("="*70 + "\n")
    
    return problems, suggestions, recommended_params


def main():
    """Função principal"""
    if len(sys.argv) < 2:
        # Se não passou arquivo, usar o latest
        audio_path = script_dir / "test_book_paragraph_xtts_custom_voice_latest.wav"
        if not audio_path.exists():
            print("❌ ERRO: Nenhum arquivo especificado e 'latest' não encontrado")
            print("   Uso: python analyze_audio_quality.py <caminho_do_audio.wav>")
            sys.exit(1)
    else:
        audio_path = Path(sys.argv[1])
        if not audio_path.is_absolute():
            audio_path = script_dir / audio_path
    
    analyze_audio_quality(audio_path)


if __name__ == "__main__":
    main()

