#!/usr/bin/env python3
"""
Teste Automatizado: Grid de Parâmetros SoVITS para Qualidade
Testa diferentes combinações de parâmetros para encontrar a melhor qualidade

Este script gera múltiplos arquivos de áudio com diferentes configurações.
Execute e depois valide qual soa melhor.
"""

import sys
import os
import json
import time
from pathlib import Path
import soundfile as sf
import numpy as np
from itertools import product

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


def find_hello_world_audio() -> Path:
    """Encontra o arquivo de áudio 'hello world' gerado pelo XTTS"""
    possible_paths = [
        script_dir / "test_hello_world_xtts_real.wav",
        tts_service_dir / "test_hello_world_xtts_stub.wav",
    ]
    
    for path in possible_paths:
        if path.exists():
            return path
    
    print("❌ ERRO: Áudio 'hello world' não encontrado!", file=sys.stderr)
    sys.exit(1)


def find_dungeon_master_model() -> tuple[Path, Path]:
    """Encontra o modelo SoVITS do dungeon master"""
    model_path = sovits_dir / "dungeon_master_en.pth"
    config_path = sovits_dir / "config.json"
    
    if not model_path.exists():
        print(f"❌ ERRO: Modelo não encontrado: {model_path}", file=sys.stderr)
        sys.exit(1)
    
    if not config_path.exists():
        print(f"❌ ERRO: Config não encontrado: {config_path}", file=sys.stderr)
        sys.exit(1)
    
    return model_path, config_path


def check_sample_rate(audio_path: Path, model: Svc) -> dict:
    """Verifica compatibilidade de sample rate"""
    audio, sr = sf.read(str(audio_path))
    model_sr = model.target_sample
    
    return {
        "input_sr": sr,
        "model_sr": model_sr,
        "match": sr == model_sr,
        "needs_resample": sr != model_sr,
    }


def test_configuration(
    svc_model: Svc,
    input_audio_path: Path,
    speaker: str,
    config: dict,
    output_path: Path,
) -> dict:
    """Testa uma configuração específica e retorna métricas"""
    raw_dir = sovits_dir / "raw"
    raw_dir.mkdir(exist_ok=True)
    temp_input = raw_dir / "test_input.wav"
    
    # Preparar áudio
    audio, sample_rate = sf.read(str(input_audio_path))
    if len(audio.shape) > 1:
        audio = np.mean(audio, axis=1)
    
    infer_tool.format_wav(str(input_audio_path))
    sf.write(str(temp_input), audio, sample_rate)
    
    # Converter com configuração específica
    start_time = time.time()
    
    kwarg = {
        "raw_audio_path": str(temp_input),
        "spk": speaker,
        "tran": config.get("trans", 0),
        "slice_db": config.get("slice_db", -40),
        "cluster_infer_ratio": config.get("cluster_infer_ratio", 0),
        "auto_predict_f0": config.get("auto_predict_f0", True),
        "noice_scale": config.get("noice_scale", 0.4),
        "pad_seconds": config.get("pad_seconds", 0.5),
        "clip_seconds": 0,
        "lg_num": 0,
        "lgr_num": 0.75,
        "f0_predictor": config.get("f0_predictor", "rmvpe"),
        "enhancer_adaptive_key": 0,
        "cr_threshold": 0.05,
        "k_step": 100,
        "use_spk_mix": False,
        "second_encoding": False,
        "loudness_envelope_adjustment": 1.0,
    }
    
    try:
        converted_audio = svc_model.slice_inference(**kwarg)
        conversion_time = time.time() - start_time
        
        # Salvar
        sf.write(str(output_path), converted_audio, svc_model.target_sample)
        
        # Métricas básicas
        duration = len(converted_audio) / svc_model.target_sample
        max_amp = np.max(np.abs(converted_audio))
        rms = np.sqrt(np.mean(converted_audio**2))
        
        return {
            "success": True,
            "conversion_time": conversion_time,
            "duration": duration,
            "max_amplitude": float(max_amp),
            "rms": float(rms),
            "samples": len(converted_audio),
            "sample_rate": svc_model.target_sample,
        }
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
        }


def run_quality_grid():
    """Executa grid de testes de qualidade"""
    print("\n" + "="*70)
    print("  TESTE AUTOMATIZADO: Grid de Qualidade SoVITS")
    print("="*70 + "\n")
    
    # 1. Encontrar arquivos
    input_audio = find_hello_world_audio()
    model_path, config_path = find_dungeon_master_model()
    
    # 2. Verificar GPU
    use_gpu = torch.cuda.is_available()
    device = "cuda" if use_gpu else "cpu"
    print(f"🔧 Dispositivo: {device} ({'GPU' if use_gpu else 'CPU'})\n")
    
    # 3. Carregar modelo
    print("📦 Carregando modelo SoVITS...")
    start_time = time.time()
    
    original_cwd = os.getcwd()
    os.chdir(sovits_dir)
    
    try:
        # Desabilitar enhancer se o modelo não estiver disponível
        # Focamos nos parâmetros de inferência primeiro
        svc_model = Svc(
            str(model_path),
            str(config_path),
            device,
            cluster_model_path="",
            nsf_hifigan_enhance=False,  # Desabilitado (modelo não disponível)
            diffusion_model_path="",
            diffusion_config_path="",
            shallow_diffusion=False,
            only_diffusion=False,
            spk_mix_enable=False,
            feature_retrieval=False,
        )
        
        load_time = time.time() - start_time
        print(f"✅ Modelo carregado em {load_time:.2f}s\n")
        
        # 4. Verificar sample rate
        print("🔍 Verificando compatibilidade de sample rate...")
        sr_info = check_sample_rate(input_audio, svc_model)
        print(f"   Input: {sr_info['input_sr']} Hz")
        print(f"   Modelo: {sr_info['model_sr']} Hz")
        if sr_info['needs_resample']:
            print(f"   ⚠️  ATENÇÃO: Sample rates diferentes! Pode causar problemas.")
        else:
            print(f"   ✅ Sample rates compatíveis")
        print()
        
        # 5. Determinar speaker
        speakers = list(svc_model.spk2id.keys())
        speaker = "dungeon_master_en" if "dungeon_master_en" in speakers else speakers[0]
        print(f"🎤 Speaker: {speaker}\n")
        
        # 6. Definir grid de testes
        # Baseado no guia fornecido pelo usuário
        test_configs = [
            # Teste 1: Baseline (parâmetros atuais)
            {
                "name": "01_baseline",
                "description": "Parâmetros atuais (noice_scale=0.4, auto_f0=True, rmvpe)",
                "f0_predictor": "rmvpe",
                "auto_predict_f0": True,
                "noice_scale": 0.4,
                "slice_db": -40,
                "pad_seconds": 0.5,
            },
            
            # Teste 2: noice_scale reduzido (mais natural)
            {
                "name": "02_noice_0.2",
                "description": "noice_scale=0.2 (mais natural)",
                "f0_predictor": "rmvpe",
                "auto_predict_f0": True,
                "noice_scale": 0.2,
                "slice_db": -40,
                "pad_seconds": 0.5,
            },
            
            # Teste 3: noice_scale muito baixo
            {
                "name": "03_noice_0.1",
                "description": "noice_scale=0.1 (muito natural, pode perder características)",
                "f0_predictor": "rmvpe",
                "auto_predict_f0": True,
                "noice_scale": 0.1,
                "slice_db": -40,
                "pad_seconds": 0.5,
            },
            
            # Teste 4: auto_predict_f0 OFF
            {
                "name": "04_no_auto_f0",
                "description": "auto_predict_f0=False (pode melhorar dinâmica)",
                "f0_predictor": "rmvpe",
                "auto_predict_f0": False,
                "noice_scale": 0.2,
                "slice_db": -40,
                "pad_seconds": 0.5,
            },
            
            # Teste 5: F0 predictor diferente (fcpe)
            {
                "name": "05_f0_fcpe",
                "description": "f0_predictor=fcpe (alternativa de alta qualidade)",
                "f0_predictor": "fcpe",
                "auto_predict_f0": True,
                "noice_scale": 0.2,
                "slice_db": -40,
                "pad_seconds": 0.5,
            },
            
            # Teste 6: F0 predictor crepe
            {
                "name": "06_f0_crepe",
                "description": "f0_predictor=crepe (boa qualidade, mais lento)",
                "f0_predictor": "crepe",
                "auto_predict_f0": True,
                "noice_scale": 0.2,
                "slice_db": -40,
                "pad_seconds": 0.5,
            },
            
            # Teste 7: pad_seconds aumentado
            {
                "name": "07_pad_0.8",
                "description": "pad_seconds=0.8 (evita cortes de fonemas)",
                "f0_predictor": "rmvpe",
                "auto_predict_f0": True,
                "noice_scale": 0.2,
                "slice_db": -40,
                "pad_seconds": 0.8,
            },
            
            # Teste 8: slice_db menos agressivo
            {
                "name": "08_slice_-35",
                "description": "slice_db=-35 (menos cortes agressivos)",
                "f0_predictor": "rmvpe",
                "auto_predict_f0": True,
                "noice_scale": 0.2,
                "slice_db": -35,
                "pad_seconds": 0.5,
            },
            
            # Teste 9: Combinação otimizada
            {
                "name": "09_optimized",
                "description": "Combinação otimizada (noice=0.2, pad=0.8, slice=-35)",
                "f0_predictor": "rmvpe",
                "auto_predict_f0": True,
                "noice_scale": 0.2,
                "slice_db": -35,
                "pad_seconds": 0.8,
            },
            
            # Teste 10: Combinação com fcpe
            {
                "name": "10_fcpe_optimized",
                "description": "fcpe + parâmetros otimizados",
                "f0_predictor": "fcpe",
                "auto_predict_f0": True,
                "noice_scale": 0.2,
                "slice_db": -35,
                "pad_seconds": 0.8,
            },
        ]
        
        # 7. Criar diretório de saída
        output_dir = script_dir / "sovits_quality_tests"
        output_dir.mkdir(exist_ok=True)
        
        # 8. Executar testes
        print("🧪 Executando grid de testes...\n")
        results = []
        
        for i, config in enumerate(test_configs, 1):
            print(f"[{i}/{len(test_configs)}] {config['name']}: {config['description']}")
            output_path = output_dir / f"{config['name']}.wav"
            
            result = test_configuration(
                svc_model,
                input_audio,
                speaker,
                config,
                output_path,
            )
            
            result["config"] = config
            result["output_path"] = str(output_path)
            results.append(result)
            
            if result["success"]:
                print(f"   ✅ Tempo: {result['conversion_time']:.2f}s | "
                      f"Duração: {result['duration']:.2f}s | "
                      f"RMS: {result['rms']:.4f}")
            else:
                print(f"   ❌ Erro: {result.get('error', 'Unknown')}")
            print()
        
        # 9. Salvar relatório
        report_path = output_dir / "test_report.json"
        with open(report_path, 'w', encoding='utf-8') as f:
            json.dump({
                "sample_rate_check": sr_info,
                "model_path": str(model_path),
                "input_audio": str(input_audio),
                "speaker": speaker,
                "device": device,
                "tests": results,
            }, f, indent=2, ensure_ascii=False)
        
        # 10. Resumo
        print("="*70)
        print("  RESUMO DOS TESTES")
        print("="*70)
        print(f"\n✅ {sum(1 for r in results if r['success'])}/{len(results)} testes concluídos")
        print(f"\n📁 Arquivos gerados em: {output_dir}")
        print(f"\n📊 Relatório salvo em: {report_path}")
        print("\n🎧 PRÓXIMO PASSO: Ouça cada arquivo e identifique qual soa melhor!")
        print("\nArquivos gerados:")
        for result in results:
            if result["success"]:
                print(f"  - {result['config']['name']}.wav: {result['config']['description']}")
        print("\n" + "="*70 + "\n")
        
    finally:
        os.chdir(original_cwd)


if __name__ == "__main__":
    run_quality_grid()

