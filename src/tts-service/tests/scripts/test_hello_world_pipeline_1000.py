#!/usr/bin/env python3
"""
Teste completo da pipeline: XTTS → SoVITS (1000 steps)
Gera áudio "Hello World" com XTTS e converte usando modelo SoVITS treinado com 1000 steps

Execute com o ambiente virtual do SoVITS ativado:
  cd assets-and-models/models/tts/sovits
  .\venv310\Scripts\activate
  python ../../../../src/tts-service/tests/scripts/test_hello_world_pipeline_1000.py
"""

import sys
import os
import time
from pathlib import Path
import numpy as np

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent  # tests/scripts/
tests_dir = script_dir.parent  # tests/
tts_service_dir = tests_dir.parent  # tts-service/
vrpg_client_dir = tts_service_dir.parent.parent  # vrpg-client/
sovits_dir = vrpg_client_dir / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

try:
    from TTS.api import TTS
    import torch
    import soundfile as sf
    from inference import infer_tool
    from inference.infer_tool import Svc
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("Certifique-se de que está no ambiente virtual do SoVITS", file=sys.stderr)
    print("E que o Coqui TTS está instalado: pip install TTS", file=sys.stderr)
    sys.exit(1)

# Fix para PyTorch 2.6+ - desabilitar weights_only para TTS
# O TTS precisa carregar classes customizadas que não estão na lista de safe globals
original_load = torch.load
def patched_load(*args, **kwargs):
    # Se weights_only não foi especificado, definir como False
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load


def generate_xtts_audio(text: str = "Hello World") -> tuple[np.ndarray, int]:
    """
    Gera áudio usando XTTS
    
    Returns:
        (audio_samples, sample_rate)
    """
    print("\n" + "="*70)
    print("  ETAPA 1: Geração de áudio com XTTS")
    print("="*70 + "\n")
    print(f"📝 Texto: '{text}'")
    
    print("\n📥 Carregando modelo XTTS v2...")
    print("   (Isso pode levar alguns minutos na primeira vez)")
    
    try:
        use_gpu = torch.cuda.is_available()
        device = "cuda" if use_gpu else "cpu"
        print(f"   Dispositivo: {device}")
        
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=True)
        print("✅ Modelo XTTS carregado!")
        
        print(f"\n🎙️  Sintetizando '{text}'...")
        start_time = time.time()
        
        # XTTS v2 requer um speaker
        speaker = "Ana Florence"  # Speaker padrão do XTTS v2
        print(f"   Usando speaker: {speaker}")
        
        audio = tts.tts(
            text=text,
            speaker=speaker,
            language="en",
        )
        
        synthesis_time = time.time() - start_time
        sample_rate = tts.synthesizer.output_sample_rate
        
        print(f"\n✅ Áudio XTTS gerado!")
        print(f"   - Amostras: {len(audio)}")
        print(f"   - Sample rate: {sample_rate} Hz")
        print(f"   - Duração: {len(audio) / sample_rate:.2f}s")
        print(f"   - Tempo de síntese: {synthesis_time:.2f}s")
        
        # Converter para numpy array se necessário
        if not isinstance(audio, np.ndarray):
            audio = np.array(audio, dtype=np.float32)
        
        # Garantir que está em mono
        if len(audio.shape) > 1:
            audio = np.mean(audio, axis=1)
        
        return audio, sample_rate
        
    except Exception as e:
        print(f"\n❌ ERRO ao gerar áudio XTTS: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


def convert_with_sovits_1000(
    xtts_audio: np.ndarray,
    xtts_sample_rate: int,
    output_path: Path
) -> None:
    """
    Converte áudio XTTS usando modelo SoVITS treinado com 1000 steps
    """
    print("\n" + "="*70)
    print("  ETAPA 2: Conversão com SoVITS (1000 steps)")
    print("="*70 + "\n")
    
    # Encontrar modelo de 1000 steps
    model_path = sovits_dir / "logs" / "44k" / "G_1000.pth"
    config_path = sovits_dir / "logs" / "44k" / "config.json"
    
    if not model_path.exists():
        print(f"❌ ERRO: Modelo G_1000.pth não encontrado!")
        print(f"   Procurando em: {model_path}")
        print("\n   Arquivos .pth encontrados em logs/44k/:")
        logs_dir = sovits_dir / "logs" / "44k"
        if logs_dir.exists():
            for pth_file in logs_dir.glob("G_*.pth"):
                print(f"     - {pth_file.name}")
        sys.exit(1)
    
    if not config_path.exists():
        # Tentar config padrão
        config_path = sovits_dir / "configs" / "config.json"
        if not config_path.exists():
            config_path = sovits_dir / "config.json"
            if not config_path.exists():
                print(f"❌ ERRO: config.json não encontrado!")
                sys.exit(1)
    
    print(f"✅ Modelo encontrado: {model_path.name}")
    print(f"✅ Config encontrado: {config_path.name}")
    
    # Auto-detectar GPU
    use_gpu = torch.cuda.is_available()
    device = "cuda" if use_gpu else "cpu"
    print(f"\n🔧 Dispositivo: {device} ({'GPU' if use_gpu else 'CPU'})")
    
    # Carregar modelo SoVITS
    print(f"\n📦 Carregando modelo SoVITS...")
    load_start = time.time()
    
    try:
        original_cwd = os.getcwd()
        os.chdir(sovits_dir)
        
        try:
            svc_model = Svc(
                str(model_path),
                str(config_path),
                device,
                cluster_model_path="",
                nsf_hifigan_enhance=False,  # Desabilitado temporariamente (arquivo config não encontrado)
                diffusion_model_path="",
                diffusion_config_path="",
                shallow_diffusion=False,
                only_diffusion=False,
                spk_mix_enable=False,
                feature_retrieval=False,
            )
            
            load_time = time.time() - load_start
            print(f"✅ Modelo carregado em {load_time:.2f}s")
            
            # Listar speakers disponíveis
            speakers = list(svc_model.spk2id.keys()) if hasattr(svc_model, 'spk2id') else []
            print(f"\n📢 Speakers disponíveis: {speakers}")
            
            # Determinar speaker (dungeon master)
            speaker = None
            possible_speaker_names = [
                "dungeon_master_en",
                "dungeon_master",
                "dm_en",
                "dm",
                "narrator",
            ]
            
            for name in possible_speaker_names:
                if name in speakers:
                    speaker = name
                    break
            
            if not speaker:
                if speakers:
                    speaker = speakers[0]
                    print(f"⚠️  Speaker 'dungeon_master' não encontrado, usando: {speaker}")
                else:
                    print("❌ ERRO: Nenhum speaker disponível no modelo!")
                    sys.exit(1)
            
            print(f"🎤 Usando speaker: {speaker}")
            
            # Preparar arquivo temporário
            raw_dir = sovits_dir / "raw"
            raw_dir.mkdir(exist_ok=True)
            temp_input = raw_dir / "hello_world_xtts_input.wav"
            
            # Salvar áudio XTTS temporariamente
            print(f"\n💾 Salvando áudio XTTS temporário...")
            sf.write(str(temp_input), xtts_audio, xtts_sample_rate)
            print(f"   Arquivo: {temp_input.name}")
            print(f"   Sample rate: {xtts_sample_rate} Hz")
            print(f"   Duração: {len(xtts_audio) / xtts_sample_rate:.2f}s")
            
            # Formatar para SoVITS
            infer_tool.format_wav(str(temp_input))
            
            # Converter usando SoVITS
            print(f"\n🔄 Convertendo áudio com SoVITS...")
            conversion_start = time.time()
            
            kwarg = {
                "raw_audio_path": str(temp_input),
                "spk": speaker,
                "tran": 0,  # Transposição de tom
                "slice_db": -40,  # Threshold de silêncio
                "cluster_infer_ratio": 0,
                "auto_predict_f0": True,  # Auto-detectar F0
                "noice_scale": 0.2,  # Escala de ruído (menor = mais natural)
                "pad_seconds": 0.5,
                "clip_seconds": 0,
                "lg_num": 0,
                "lgr_num": 0.75,
                "f0_predictor": "rmvpe",  # Melhor qualidade
                "enhancer_adaptive_key": 0,
                "cr_threshold": 0.05,
                "k_step": 100,
                "use_spk_mix": False,
                "second_encoding": False,
                "loudness_envelope_adjustment": 1.0,
            }
            
            converted_audio = svc_model.slice_inference(**kwarg)
            conversion_time = time.time() - conversion_start
            
            print(f"✅ Conversão concluída em {conversion_time:.2f}s")
            
            # Salvar áudio convertido
            print(f"\n💾 Salvando áudio convertido...")
            sf.write(str(output_path), converted_audio, svc_model.target_sample)
            
            # Limpar recursos
            svc_model.clear_empty()
            
            print(f"\n✅ Áudio convertido salvo em: {output_path}")
            print(f"   Sample rate: {svc_model.target_sample} Hz")
            print(f"   Duração: {len(converted_audio) / svc_model.target_sample:.2f}s")
            
        finally:
            os.chdir(original_cwd)
            
    except Exception as e:
        print(f"\n❌ ERRO durante conversão SoVITS: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


def main():
    """Executa a pipeline completa: XTTS → SoVITS"""
    print("\n" + "="*70)
    print("  TESTE COMPLETO: XTTS → SoVITS (1000 steps)")
    print("  Texto: 'Hello World'")
    print("="*70)
    
    text = "Hello World"
    total_start = time.time()
    
    # Etapa 1: Gerar áudio com XTTS
    xtts_audio, xtts_sample_rate = generate_xtts_audio(text)
    
    # Salvar áudio XTTS intermediário
    xtts_output = script_dir / "test_hello_world_xtts_output.wav"
    sf.write(str(xtts_output), xtts_audio, xtts_sample_rate)
    print(f"\n💾 Áudio XTTS salvo em: {xtts_output}")
    
    # Etapa 2: Converter com SoVITS
    sovits_output = script_dir / "test_hello_world_sovits_1000_output.wav"
    convert_with_sovits_1000(xtts_audio, xtts_sample_rate, sovits_output)
    
    total_time = time.time() - total_start
    
    # Resumo final
    print("\n" + "="*70)
    print("  RESULTADO FINAL")
    print("="*70)
    print(f"✅ Texto processado: '{text}'")
    print(f"✅ Áudio XTTS: {xtts_output.name}")
    print(f"✅ Áudio SoVITS (1000 steps): {sovits_output.name}")
    print(f"✅ Tempo total: {total_time:.2f}s")
    print(f"\n🎧 Ouça o resultado final em: {sovits_output}")
    print("="*70 + "\n")


if __name__ == "__main__":
    main()

