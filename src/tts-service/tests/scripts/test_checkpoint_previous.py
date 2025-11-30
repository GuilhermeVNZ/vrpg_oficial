#!/usr/bin/env python3
"""
Teste: SoVITS com Checkpoint Anterior

Overfitting pode causar som metálico. Este teste verifica se um checkpoint
anterior (menos treinado) soa melhor.
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


def test_with_previous_checkpoint():
    """Testa SoVITS com checkpoint anterior"""
    print("\n" + "="*70)
    print("  TESTE: SoVITS com Checkpoint Anterior")
    print("="*70 + "\n")
    
    # 1. Encontrar checkpoints
    logs_dir = sovits_dir / "logs" / "44k"
    
    if not logs_dir.exists():
        print("❌ Diretório de logs não encontrado!", file=sys.stderr)
        sys.exit(1)
    
    checkpoints = sorted(logs_dir.glob("G_*.pth"), key=lambda x: int(x.stem.split('_')[1]) if x.stem.split('_')[1].isdigit() else 0)
    
    if len(checkpoints) < 2:
        print("❌ Menos de 2 checkpoints encontrados!", file=sys.stderr)
        print(f"   Encontrados: {len(checkpoints)}")
        sys.exit(1)
    
    # Checkpoint atual (último)
    current_checkpoint = sovits_dir / "dungeon_master_en.pth"
    
    # Checkpoint anterior (penúltimo)
    previous_checkpoint = checkpoints[-2] if len(checkpoints) >= 2 else checkpoints[-1]
    
    print(f"📦 Checkpoints encontrados: {len(checkpoints)}")
    print(f"   Atual: {current_checkpoint.name}")
    print(f"   Anterior: {previous_checkpoint.name}")
    print()
    
    # 2. Usar áudio XTTS re-amostrado (44100 Hz)
    input_audio_path = script_dir / "temp_xtts_44100.wav"
    
    if not input_audio_path.exists():
        # Tentar criar se não existir
        original_xtts = script_dir / "test_hello_world_xtts_real.wav"
        if original_xtts.exists():
            from scipy import signal
            audio, sr = sf.read(str(original_xtts))
            if len(audio.shape) > 1:
                audio = np.mean(audio, axis=1)
            audio_resampled = signal.resample(audio, int(len(audio) * 44100 / sr)).astype(np.float32)
            sf.write(str(input_audio_path), audio_resampled, 44100)
            print(f"✅ Áudio re-amostrado criado: {input_audio_path.name}\n")
        else:
            print("❌ Áudio de entrada não encontrado!", file=sys.stderr)
            sys.exit(1)
    
    audio_input, sr_input = sf.read(str(input_audio_path))
    if len(audio_input.shape) > 1:
        audio_input = np.mean(audio_input, axis=1)
    
    print(f"📥 Áudio de entrada: {input_audio_path.name}")
    print(f"   Sample rate: {sr_input} Hz")
    print(f"   Duração: {len(audio_input) / sr_input:.2f}s")
    print()
    
    # 3. Carregar modelo com checkpoint anterior
    config_path = sovits_dir / "config.json"
    
    use_gpu = torch.cuda.is_available()
    device = "cuda" if use_gpu else "cpu"
    print(f"🔧 Dispositivo: {device} ({'GPU' if use_gpu else 'CPU'})\n")
    
    print(f"📦 Carregando modelo com checkpoint anterior: {previous_checkpoint.name}...")
    start_time = time.time()
    
    original_cwd = os.getcwd()
    os.chdir(sovits_dir)
    
    try:
        svc_model = Svc(
            str(previous_checkpoint),  # Usar checkpoint anterior
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
        
        # 4. Determinar speaker
        speakers = list(svc_model.spk2id.keys())
        speaker = "dungeon_master_en" if "dungeon_master_en" in speakers else speakers[0]
        print(f"🎤 Speaker: {speaker}\n")
        
        # 5. Preparar arquivo para SoVITS
        raw_dir = sovits_dir / "raw"
        raw_dir.mkdir(exist_ok=True)
        temp_input = raw_dir / "test_checkpoint_input.wav"
        
        infer_tool.format_wav(str(input_audio_path))
        sf.write(str(temp_input), audio_input, sr_input)
        
        # 6. Converter com parâmetros otimizados
        print("🔄 Convertendo com checkpoint anterior...")
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
        output_path = output_dir / f"TEST_checkpoint_{previous_checkpoint.stem}.wav"
        sf.write(str(output_path), converted_audio, svc_model.target_sample)
        
        # 8. Estatísticas
        duration = len(converted_audio) / svc_model.target_sample
        max_amp = np.max(np.abs(converted_audio))
        rms = np.sqrt(np.mean(converted_audio**2))
        
        print("="*70)
        print("  RESULTADO")
        print("="*70)
        print(f"✅ Áudio convertido: {output_path.name}")
        print(f"✅ Checkpoint usado: {previous_checkpoint.name}")
        print(f"✅ Sample rate: {svc_model.target_sample} Hz")
        print(f"✅ Duração: {duration:.2f}s")
        print(f"✅ Max amplitude: {max_amp:.4f}")
        print(f"✅ RMS: {rms:.4f}")
        print(f"\n🎧 INTERPRETAÇÃO:")
        print(f"   Se este arquivo soar MELHOR:")
        print(f"   → Problema é OVERFITTING (checkpoint atual treinado demais)")
        print(f"   Se este arquivo também soar metálico:")
        print(f"   → Problema é no DATASET ou TREINAMENTO INICIAL")
        print("="*70 + "\n")
        
    finally:
        os.chdir(original_cwd)


if __name__ == "__main__":
    test_with_previous_checkpoint()

