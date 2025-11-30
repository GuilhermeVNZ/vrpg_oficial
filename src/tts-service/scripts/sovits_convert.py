#!/usr/bin/env python3
"""
Script para converter áudio neutro do Piper usando modelo SoVITS treinado.
Este script recebe um arquivo WAV do Piper e converte usando o modelo SoVITS.
"""

import os
import sys
import argparse
import json
import tempfile
from pathlib import Path

# Adicionar o diretório do SoVITS ao path
# O script está em: vrpg-client/src/tts-service/scripts/sovits_convert.py
# O SoVITS está em: vrpg-client/assets-and-models/models/tts/sovits
script_dir = Path(__file__).parent  # scripts/
tts_service_dir = script_dir.parent  # tts-service/
vrpg_client_dir = tts_service_dir.parent.parent  # vrpg-client/
sovits_dir = vrpg_client_dir / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

try:
    import soundfile as sf
    import numpy as np
    from inference import infer_tool
    from inference.infer_tool import Svc
except ImportError as e:
    print(f"ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("Certifique-se de que está no ambiente virtual do SoVITS", file=sys.stderr)
    sys.exit(1)


def convert_audio_with_sovits(
    input_wav_path: str,
    model_path: str,
    config_path: str,
    speaker: str,
    output_wav_path: str,
    device: str = None,
    auto_predict_f0: bool = True,
    f0_predictor: str = "rmvpe",
    noice_scale: float = 0.4,
    trans: int = 0,
) -> bool:
    """
    Converte áudio neutro do Piper usando modelo SoVITS treinado.
    
    Args:
        input_wav_path: Caminho do arquivo WAV de entrada (do Piper)
        model_path: Caminho do modelo SoVITS (.pth)
        config_path: Caminho do arquivo de configuração (config.json)
        speaker: Nome do speaker (personagem)
        output_wav_path: Caminho do arquivo WAV de saída
        device: Dispositivo (cuda/cpu, None=auto)
        auto_predict_f0: Auto-prever F0
        f0_predictor: Tipo de preditor F0 (rmvpe, pm, dio, harvest, crepe, fcpe)
        noice_scale: Escala de ruído (afeta qualidade)
        trans: Transposição de tom (semitons)
    
    Returns:
        True se sucesso, False caso contrário
    """
    try:
        # Converter caminhos para absolutos antes de mudar o diretório
        model_path = os.path.abspath(model_path)
        config_path = os.path.abspath(config_path)
        input_wav_path = os.path.abspath(input_wav_path)
        output_wav_path = os.path.abspath(output_wav_path)
        
        # Mudar para o diretório do SoVITS para garantir que caminhos relativos funcionem
        original_cwd = os.getcwd()
        os.chdir(sovits_dir)
        
        try:
            # Carregar modelo SoVITS
            svc_model = Svc(
                model_path,
                config_path,
                device,
                cluster_model_path="",  # Sem clustering por enquanto
                nsf_hifigan_enhance=False,
                diffusion_model_path="",
                diffusion_config_path="",
                shallow_diffusion=False,
                only_diffusion=False,
                spk_mix_enable=False,
                feature_retrieval=False,
            )
        
            # Verificar se o speaker existe
            if speaker not in svc_model.spk2id:
                print(f"ERRO: Speaker '{speaker}' não encontrado!", file=sys.stderr)
                print(f"Speakers disponíveis: {list(svc_model.spk2id.keys())}", file=sys.stderr)
                return False
            
            # Preparar arquivo temporário no formato esperado pelo SoVITS
            # O SoVITS espera arquivos em raw/
            raw_dir = sovits_dir / "raw"
            raw_dir.mkdir(exist_ok=True)
            
            temp_input = raw_dir / "piper_input.wav"
            
            # Copiar/formatar o arquivo de entrada
            infer_tool.format_wav(str(input_wav_path))
            
            # Ler o áudio de entrada
            audio, sample_rate = sf.read(input_wav_path)
            
            # Garantir que está em mono
            if len(audio.shape) > 1:
                audio = np.mean(audio, axis=1)
            
            # Salvar no formato esperado
            sf.write(str(temp_input), audio, sample_rate)
            
            # Converter usando SoVITS
            kwarg = {
                "raw_audio_path": str(temp_input),
            "spk": speaker,
            "tran": trans,
            "slice_db": -40,
            "cluster_infer_ratio": 0,
            "auto_predict_f0": auto_predict_f0,
            "noice_scale": noice_scale,  # Recomendado: 0.1-0.3 para qualidade natural
            "pad_seconds": 0.5,
            "clip_seconds": 0,
            "lg_num": 0,
            "lgr_num": 0.75,
            "f0_predictor": f0_predictor,
            "enhancer_adaptive_key": 0,
            "cr_threshold": 0.05,
            "k_step": 100,
            "use_spk_mix": False,
            "second_encoding": False,
            "loudness_envelope_adjustment": 1.0,
        }
        
            converted_audio = svc_model.slice_inference(**kwarg)
            
            # Salvar áudio convertido
            sf.write(output_wav_path, converted_audio, svc_model.target_sample)
            
            # Limpar cache
            svc_model.clear_empty()
            
            return True
        finally:
            # Restaurar diretório de trabalho original
            os.chdir(original_cwd)
        
    except Exception as e:
        print(f"ERRO ao converter áudio: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return False


def main():
    parser = argparse.ArgumentParser(description='Converte áudio neutro do Piper usando SoVITS')
    parser.add_argument('input_wav', type=str, help='Caminho do arquivo WAV de entrada (do Piper)')
    parser.add_argument('output_wav', type=str, help='Caminho do arquivo WAV de saída')
    parser.add_argument('-m', '--model_path', type=str, 
                       default=str(sovits_dir / "dungeon_master_en.pth"),
                       help='Caminho do modelo SoVITS')
    parser.add_argument('-c', '--config_path', type=str,
                       default=str(sovits_dir / "config.json"),
                       help='Caminho do arquivo de configuração')
    parser.add_argument('-s', '--speaker', type=str,
                       default="dungeon_master_en",
                       help='Nome do speaker (personagem)')
    parser.add_argument('-d', '--device', type=str, default=None,
                       help='Dispositivo (cuda/cpu, None=auto detecta GPU)')
    parser.add_argument('--f0_predictor', type=str, default="rmvpe",
                       help='Tipo de preditor F0 (rmvpe, pm, dio, harvest, crepe, fcpe)')
    parser.add_argument('--noice_scale', type=float, default=0.2,
                       help='Escala de ruído (0.0-1.0). Valores menores (0.1-0.3) = mais natural, valores maiores (0.4-0.6) = mais robótico')
    parser.add_argument('--trans', type=int, default=0,
                       help='Transposição de tom (semitons)')
    parser.add_argument('--no-auto-f0', action='store_true',
                       help='Desabilitar auto-predição de F0')
    
    args = parser.parse_args()
    
    # Verificar se arquivo de entrada existe
    if not os.path.exists(args.input_wav):
        print(f"ERRO: Arquivo de entrada não encontrado: {args.input_wav}", file=sys.stderr)
        sys.exit(1)
    
    # Auto-detect GPU se não especificado
    import torch
    if args.device is None:
        args.device = "cuda" if torch.cuda.is_available() else "cpu"
        print(f"🔧 Auto-detected device: {args.device}", file=sys.stderr)
        if args.device == "cuda":
            print(f"   GPU: {torch.cuda.get_device_name(0)}", file=sys.stderr)
    elif args.device == "cuda" and not torch.cuda.is_available():
        print("⚠️  CUDA solicitado mas não disponível, usando CPU", file=sys.stderr)
        args.device = "cpu"
    
    # Converter
    success = convert_audio_with_sovits(
        input_wav_path=args.input_wav,
        model_path=args.model_path,
        config_path=args.config_path,
        speaker=args.speaker,
        output_wav_path=args.output_wav,
        device=args.device,
        auto_predict_f0=not args.no_auto_f0,
        f0_predictor=args.f0_predictor,
        noice_scale=args.noice_scale,
        trans=args.trans,
    )
    
    if success:
        print(json.dumps({
            "success": True,
            "output_path": args.output_wav,
            "speaker": args.speaker
        }))
        sys.exit(0)
    else:
        print(json.dumps({
            "success": False,
            "error": "Falha na conversão"
        }))
        sys.exit(1)


if __name__ == "__main__":
    main()

