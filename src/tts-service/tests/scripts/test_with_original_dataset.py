#!/usr/bin/env python3
"""
Teste: SoVITS com Áudio Original do Dataset

Este teste bypassa o XTTS e usa áudio original do dataset para verificar
se o problema é no modelo treinado ou no pipeline XTTS → SoVITS.
"""

import sys
import os
import time
from pathlib import Path
import soundfile as sf
import numpy as np

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


def test_with_original_dataset_audio():
    """Testa SoVITS com áudio original do dataset (bypass XTTS)"""
    print("\n" + "="*70)
    print("  TESTE: SoVITS com Áudio Original do Dataset")
    print("="*70 + "\n")
    
    # 1. Encontrar áudio original do dataset
    dataset_dir = sovits_dir / "dataset_raw" / "dungeon_master_en"
    
    if not dataset_dir.exists():
        print("❌ Dataset não encontrado!", file=sys.stderr)
        sys.exit(1)
    
    wav_files = list(dataset_dir.rglob("*.wav"))
    if not wav_files:
        print("❌ Nenhum arquivo WAV encontrado no dataset!", file=sys.stderr)
        sys.exit(1)
    
    # Pegar um arquivo do dataset (preferir um curto)
    dataset_audio_path = None
    for wav_file in sorted(wav_files):
        try:
            audio, sr = sf.read(str(wav_file))
            duration = len(audio) / sr if len(audio.shape) == 1 else len(audio) / sr
            if duration < 5.0:  # Preferir áudios curtos (< 5s)
                dataset_audio_path = wav_file
                break
        except:
            continue
    
    if not dataset_audio_path:
        dataset_audio_path = wav_files[0]  # Usar o primeiro se não encontrar curto
    
    print(f"📥 Usando áudio do dataset: {dataset_audio_path.name}")
    audio_original, sr_original = sf.read(str(dataset_audio_path))
    if len(audio_original.shape) > 1:
        audio_original = np.mean(audio_original, axis=1)
    
    print(f"   Sample rate: {sr_original} Hz")
    print(f"   Duração: {len(audio_original) / sr_original:.2f}s")
    print()
    
    # 2. Carregar modelo SoVITS
    model_path = sovits_dir / "dungeon_master_en.pth"
    config_path = sovits_dir / "config.json"
    
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
        
        # 3. Verificar sample rate
        model_sr = svc_model.target_sample
        print(f"📊 Sample rate do modelo: {model_sr} Hz")
        print(f"📊 Sample rate do input: {sr_original} Hz")
        
        if sr_original != model_sr:
            print(f"⚠️  Sample rates diferentes, mas vamos testar mesmo assim\n")
        else:
            print(f"✅ Sample rates compatíveis\n")
        
        # 4. Determinar speaker
        speakers = list(svc_model.spk2id.keys())
        speaker = "dungeon_master_en" if "dungeon_master_en" in speakers else speakers[0]
        print(f"🎤 Speaker: {speaker}\n")
        
        # 5. Preparar arquivo para SoVITS
        raw_dir = sovits_dir / "raw"
        raw_dir.mkdir(exist_ok=True)
        temp_input = raw_dir / "test_dataset_original.wav"
        
        # Garantir que está em mono e no sample rate correto
        if sr_original != model_sr:
            # Re-amostrar se necessário
            from scipy import signal
            num_samples = int(len(audio_original) * model_sr / sr_original)
            audio_original = signal.resample(audio_original, num_samples).astype(np.float32)
            sr_original = model_sr
        
        infer_tool.format_wav(str(dataset_audio_path))
        sf.write(str(temp_input), audio_original, sr_original)
        
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
            "noice_scale": 0.2,
            "pad_seconds": 0.8,
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
        output_dir = script_dir / "sovits_quality_tests"
        output_dir.mkdir(exist_ok=True)
        output_path = output_dir / "TEST_dataset_original.wav"
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
        print(f"\n🎧 INTERPRETAÇÃO:")
        print(f"   Se este arquivo TAMBÉM soa metálico:")
        print(f"   → Problema está no MODELO TREINADO ou DATASET")
        print(f"   Se este arquivo soa BEM:")
        print(f"   → Problema está no XTTS ou no pipeline XTTS → SoVITS")
        print("="*70 + "\n")
        
    finally:
        os.chdir(original_cwd)


if __name__ == "__main__":
    test_with_original_dataset_audio()

