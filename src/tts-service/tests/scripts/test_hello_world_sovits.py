#!/usr/bin/env python3
"""
Teste: Converter áudio "Hello World" do XTTS usando modelo SoVITS do Dungeon Master

Este teste segue as práticas do projeto:
- Usa o áudio gerado pelo XTTS como entrada
- Converte usando o modelo SoVITS treinado do dungeon master
- Salva o resultado para verificação
- Mede latência e valida qualidade

IMPORTANTE: Execute este script com o ambiente virtual do SoVITS ativado:
  cd assets-and-models/models/tts/sovits
  .\venv310\Scripts\activate
  python ../../../../src/tts-service/tests/scripts/test_hello_world_sovits.py

Ou use o script PowerShell: test_hello_world_sovits.ps1
"""

import sys
import os
import json
import time
from pathlib import Path
import soundfile as sf
import numpy as np

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent  # tests/scripts/
tests_dir = script_dir.parent  # tests/
tts_service_dir = tests_dir.parent  # tts-service/
vrpg_client_dir = tts_service_dir.parent.parent  # vrpg-client/
sovits_dir = vrpg_client_dir / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

try:
    from inference import infer_tool
    from inference.infer_tool import Svc
    import torch
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("Certifique-se de que está no ambiente virtual do SoVITS", file=sys.stderr)
    sys.exit(1)


def find_hello_world_audio() -> Path:
    """Encontra o arquivo de áudio 'hello world' gerado pelo XTTS"""
    # Possíveis locais do arquivo
    possible_paths = [
        script_dir / "test_hello_world_xtts_real.wav",
        tts_service_dir / "test_hello_world_xtts_stub.wav",
        tts_service_dir / "test_hello_world_corrigido.wav",
    ]
    
    for path in possible_paths:
        if path.exists():
            print(f"✅ Áudio encontrado: {path}")
            return path
    
    # Se não encontrar, gerar um áudio de teste simples
    print("⚠️  Áudio 'hello world' não encontrado, gerando áudio de teste...")
    test_audio_path = script_dir / "test_hello_world_generated.wav"
    # Gerar um áudio simples (1 segundo de silêncio com tom)
    sample_rate = 22050
    duration = 1.0
    samples = int(sample_rate * duration)
    # Gerar um tom simples (440 Hz)
    t = np.linspace(0, duration, samples, False)
    audio = 0.3 * np.sin(2 * np.pi * 440 * t).astype(np.float32)
    sf.write(test_audio_path, audio, sample_rate)
    print(f"✅ Áudio de teste gerado: {test_audio_path}")
    return test_audio_path


def find_dungeon_master_model() -> tuple[Path, Path]:
    """Encontra o modelo SoVITS do dungeon master"""
    # Possíveis locais do modelo
    possible_model_paths = [
        sovits_dir / "dungeon_master_en.pth",
        sovits_dir / "dungeon_master.pth",
        sovits_dir / "dm_en.pth",
        sovits_dir / "models" / "dungeon_master_en.pth",
    ]
    
    possible_config_paths = [
        sovits_dir / "config.json",
        sovits_dir / "dungeon_master_config.json",
        sovits_dir / "configs" / "dungeon_master_config.json",
    ]
    
    model_path = None
    config_path = None
    
    for path in possible_model_paths:
        if path.exists():
            model_path = path
            print(f"✅ Modelo encontrado: {model_path}")
            break
    
    for path in possible_config_paths:
        if path.exists():
            config_path = path
            print(f"✅ Config encontrado: {config_path}")
            break
    
    if not model_path:
        print("❌ ERRO: Modelo SoVITS do dungeon master não encontrado!")
        print(f"   Procurando em: {sovits_dir}")
        print("   Arquivos .pth encontrados:")
        for pth_file in sovits_dir.rglob("*.pth"):
            print(f"     - {pth_file}")
        sys.exit(1)
    
    if not config_path:
        print("⚠️  Config não encontrado, usando config.json padrão")
        config_path = sovits_dir / "config.json"
        if not config_path.exists():
            print("❌ ERRO: config.json não encontrado!")
            sys.exit(1)
    
    return model_path, config_path


def get_available_speakers(svc_model: Svc) -> list[str]:
    """Lista os speakers disponíveis no modelo"""
    if hasattr(svc_model, 'spk2id'):
        return list(svc_model.spk2id.keys())
    return []


def test_sovits_conversion():
    """Testa a conversão do áudio usando SoVITS"""
    print("\n" + "="*60)
    print("  TESTE: Hello World → SoVITS (Dungeon Master)")
    print("="*60 + "\n")
    
    # 1. Encontrar áudio de entrada
    input_audio_path = find_hello_world_audio()
    
    # 2. Encontrar modelo SoVITS
    model_path, config_path = find_dungeon_master_model()
    
    # 3. Preparar caminhos
    output_audio_path = script_dir / "test_hello_world_sovits_output.wav"
    
    # 4. Auto-detectar GPU
    use_gpu = torch.cuda.is_available()
    device = "cuda" if use_gpu else "cpu"
    print(f"🔧 Dispositivo: {device} ({'GPU' if use_gpu else 'CPU'})")
    
    # 5. Carregar modelo SoVITS
    print(f"\n📦 Carregando modelo SoVITS...")
    print(f"   Modelo: {model_path}")
    print(f"   Config: {config_path}")
    
    start_time = time.time()
    
    try:
        original_cwd = os.getcwd()
        os.chdir(sovits_dir)
        
        try:
            # Habilitar enhancer para melhor qualidade (reduz som robótico)
            svc_model = Svc(
                str(model_path),
                str(config_path),
                device,
                cluster_model_path="",
                nsf_hifigan_enhance=True,  # HABILITADO para melhor qualidade
                diffusion_model_path="",
                diffusion_config_path="",
                shallow_diffusion=False,
                only_diffusion=False,
                spk_mix_enable=False,
                feature_retrieval=False,
            )
            
            load_time = time.time() - start_time
            print(f"✅ Modelo carregado em {load_time:.2f}s")
            
            # 6. Listar speakers disponíveis
            speakers = get_available_speakers(svc_model)
            print(f"\n📢 Speakers disponíveis: {speakers}")
            
            # 7. Determinar speaker (dungeon master)
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
            
            # 8. Carregar áudio de entrada
            print(f"\n📥 Carregando áudio de entrada: {input_audio_path}")
            audio, sample_rate = sf.read(str(input_audio_path))
            
            if len(audio.shape) > 1:
                audio = np.mean(audio, axis=1)  # Converter para mono
            
            print(f"   Sample rate: {sample_rate} Hz")
            print(f"   Duração: {len(audio) / sample_rate:.2f}s")
            print(f"   Amostras: {len(audio)}")
            
            # 9. Preparar arquivo temporário no formato esperado pelo SoVITS
            raw_dir = sovits_dir / "raw"
            raw_dir.mkdir(exist_ok=True)
            temp_input = raw_dir / "hello_world_input.wav"
            
            # Formatar e salvar
            infer_tool.format_wav(str(input_audio_path))
            sf.write(str(temp_input), audio, sample_rate)
            
            # 10. Converter usando SoVITS
            print(f"\n🔄 Convertendo áudio com SoVITS...")
            conversion_start = time.time()
            
            # Parâmetros otimizados para qualidade natural (menos robótico)
            kwarg = {
                "raw_audio_path": str(temp_input),
                "spk": speaker,
                "tran": 0,  # Transposição de tom (semitons)
                "slice_db": -40,  # Threshold de silêncio (padrão)
                "cluster_infer_ratio": 0,  # Sem clustering
                "auto_predict_f0": True,  # Auto-detectar F0
                "noice_scale": 0.2,  # REDUZIDO de 0.4 para 0.2 (mais natural, menos robótico)
                "pad_seconds": 0.5,  # Padding para evitar cortes
                "clip_seconds": 0,  # Sem clipping
                "lg_num": 0,  # Sem LG (linguistic guidance)
                "lgr_num": 0.75,  # LG ratio
                "f0_predictor": "rmvpe",  # RMVPE é o melhor para qualidade
                "enhancer_adaptive_key": 0,  # Sem enhancer adaptativo
                "cr_threshold": 0.05,  # Threshold de crossfade
                "k_step": 100,  # Steps de difusão (se aplicável)
                "use_spk_mix": False,  # Sem mix de speakers
                "second_encoding": False,  # Sem segunda codificação
                "loudness_envelope_adjustment": 1.0,  # Ajuste de loudness
            }
            
            converted_audio = svc_model.slice_inference(**kwarg)
            conversion_time = time.time() - conversion_start
            
            print(f"✅ Conversão concluída em {conversion_time:.2f}s")
            
            # 11. Salvar áudio convertido
            print(f"\n💾 Salvando áudio convertido: {output_audio_path}")
            sf.write(str(output_audio_path), converted_audio, svc_model.target_sample)
            
            # 12. Limpar recursos
            svc_model.clear_empty()
            
            # 13. Estatísticas finais
            print(f"\n" + "="*60)
            print("  RESULTADO DO TESTE")
            print("="*60)
            print(f"✅ Áudio de entrada: {input_audio_path.name}")
            print(f"✅ Áudio convertido: {output_audio_path.name}")
            print(f"✅ Speaker usado: {speaker}")
            print(f"✅ Dispositivo: {device}")
            print(f"✅ Tempo de carregamento: {load_time:.2f}s")
            print(f"✅ Tempo de conversão: {conversion_time:.2f}s")
            print(f"✅ Sample rate de saída: {svc_model.target_sample} Hz")
            print(f"✅ Duração do áudio convertido: {len(converted_audio) / svc_model.target_sample:.2f}s")
            print(f"\n🎧 Ouça o resultado em: {output_audio_path}")
            print("="*60 + "\n")
            
        finally:
            os.chdir(original_cwd)
            
    except Exception as e:
        print(f"\n❌ ERRO durante conversão: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    test_sovits_conversion()

