#!/usr/bin/env python3
"""
Teste: Converter XTTS para 44100 Hz ANTES do SoVITS

Este teste resolve o sample rate mismatch convertendo o áudio do XTTS
para 44100 Hz antes de passar para o SoVITS, evitando re-amostragem.
"""

import sys
import os
import time
from pathlib import Path
import soundfile as sf
import numpy as np
from scipy import signal

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent
tests_dir = script_dir.parent
tts_service_dir = tests_dir.parent
vrpg_client_dir = tts_service_dir.parent.parent
sovits_dir = vrpg_client_dir / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

try:
    from inference import infer_tool
    from inference.infer_tool import Svc
    import torch
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    sys.exit(1)


def resample_audio(audio: np.ndarray, orig_sr: int, target_sr: int) -> np.ndarray:
    """Re-amostra áudio usando scipy (melhor qualidade que simples interpolação)"""
    if orig_sr == target_sr:
        return audio
    
    # Calcular número de amostras na saída
    num_samples = int(len(audio) * target_sr / orig_sr)
    
    # Usar scipy.signal.resample (usa FFT, melhor qualidade)
    resampled = signal.resample(audio, num_samples)
    
    return resampled.astype(np.float32)


def test_with_resampled_input():
    """Testa SoVITS com áudio XTTS re-amostrado para 44100 Hz"""
    print("\n" + "="*70)
    print("  TESTE: Converter XTTS para 44100 Hz ANTES do SoVITS")
    print("="*70 + "\n")
    
    # 1. Encontrar arquivos
    input_audio_path = script_dir / "test_hello_world_xtts_real.wav"
    model_path = sovits_dir / "dungeon_master_en.pth"
    config_path = sovits_dir / "config.json"
    
    if not input_audio_path.exists():
        print("❌ Áudio de entrada não encontrado!", file=sys.stderr)
        sys.exit(1)
    
    # 2. Carregar e re-amostrar áudio XTTS
    print("📥 Carregando áudio XTTS...")
    audio_xtts, sr_xtts = sf.read(str(input_audio_path))
    if len(audio_xtts.shape) > 1:
        audio_xtts = np.mean(audio_xtts, axis=1)
    
    print(f"   Sample rate original: {sr_xtts} Hz")
    print(f"   Duração: {len(audio_xtts) / sr_xtts:.2f}s")
    
    # Re-amostrar para 44100 Hz
    target_sr = 44100
    print(f"\n🔄 Re-amostrando para {target_sr} Hz...")
    audio_resampled = resample_audio(audio_xtts, sr_xtts, target_sr)
    print(f"   ✅ Re-amostrado: {len(audio_resampled)} amostras")
    print(f"   Nova duração: {len(audio_resampled) / target_sr:.2f}s")
    
    # Salvar áudio re-amostrado temporariamente
    temp_resampled = script_dir / "temp_xtts_44100.wav"
    sf.write(str(temp_resampled), audio_resampled, target_sr)
    print(f"   💾 Salvo em: {temp_resampled.name}\n")
    
    # 3. Carregar modelo SoVITS
    use_gpu = torch.cuda.is_available()
    device = "cuda" if use_gpu else "cpu"
    print(f"🔧 Dispositivo: {device} ({'GPU' if use_gpu else 'CPU'})\n")
    
    print("📦 Carregando modelo SoVITS...")
    start_time = time.time()
    
    original_cwd = os.getcwd()
    os.chdir(sovits_dir)
    
    try:
        svc_model = Svc(
            str(model_path),
            str(config_path),
            device,
            cluster_model_path="",
            nsf_hifigan_enhance=False,
            diffusion_model_path="",
            diffusion_config_path="",
            shallow_diffusion=False,
            only_diffusion=False,
            spk_mix_enable=False,
            feature_retrieval=False,
        )
        
        load_time = time.time() - start_time
        print(f"✅ Modelo carregado em {load_time:.2f}s\n")
        
        # Verificar sample rate do modelo
        model_sr = svc_model.target_sample
        print(f"📊 Sample rate do modelo: {model_sr} Hz")
        print(f"📊 Sample rate do input: {target_sr} Hz")
        
        if model_sr == target_sr:
            print(f"✅ Sample rates compatíveis! Sem re-amostragem no SoVITS.\n")
        else:
            print(f"⚠️  Ainda há mismatch, mas input já está em {target_sr} Hz\n")
        
        # 4. Determinar speaker
        speakers = list(svc_model.spk2id.keys())
        speaker = "dungeon_master_en" if "dungeon_master_en" in speakers else speakers[0]
        print(f"🎤 Speaker: {speaker}\n")
        
        # 5. Preparar arquivo para SoVITS
        raw_dir = sovits_dir / "raw"
        raw_dir.mkdir(exist_ok=True)
        temp_input = raw_dir / "test_resampled_input.wav"
        
        infer_tool.format_wav(str(temp_resampled))
        sf.write(str(temp_input), audio_resampled, target_sr)
        
        # 6. Converter com parâmetros otimizados
        print("🔄 Convertendo com SoVITS (parâmetros otimizados)...")
        conversion_start = time.time()
        
        kwarg = {
            "raw_audio_path": str(temp_input),
            "spk": speaker,
            "tran": 0,
            "slice_db": -35,
            "cluster_infer_ratio": 0,
            "auto_predict_f0": True,
            "noice_scale": 0.2,  # Otimizado
            "pad_seconds": 0.8,  # Otimizado
            "clip_seconds": 0,
            "lg_num": 0,
            "lgr_num": 0.75,
            "f0_predictor": "rmvpe",
            "enhancer_adaptive_key": 0,
            "cr_threshold": 0.05,
            "k_step": 100,
            "use_spk_mix": False,
            "second_encoding": False,
            "loudness_envelope_adjustment": 1.0,
        }
        
        converted_audio = svc_model.slice_inference(**kwarg)
        conversion_time = time.time() - conversion_start
        
        print(f"✅ Conversão concluída em {conversion_time:.2f}s\n")
        
        # 7. Salvar resultado
        output_path = script_dir / "sovits_quality_tests" / "FIXED_sample_rate_44100.wav"
        output_path.parent.mkdir(exist_ok=True)
        sf.write(str(output_path), converted_audio, svc_model.target_sample)
        
        # 8. Estatísticas
        duration = len(converted_audio) / svc_model.target_sample
        max_amp = np.max(np.abs(converted_audio))
        rms = np.sqrt(np.mean(converted_audio**2))
        
        print("="*70)
        print("  RESULTADO")
        print("="*70)
        print(f"✅ Áudio convertido: {output_path.name}")
        print(f"✅ Sample rate: {svc_model.target_sample} Hz")
        print(f"✅ Duração: {duration:.2f}s")
        print(f"✅ Max amplitude: {max_amp:.4f}")
        print(f"✅ RMS: {rms:.4f}")
        print(f"\n🎧 Ouça e compare com os testes anteriores!")
        print(f"   Se este soar melhor, o problema era o sample rate mismatch.")
        print("="*70 + "\n")
        
    finally:
        os.chdir(original_cwd)


if __name__ == "__main__":
    test_with_resampled_input()

