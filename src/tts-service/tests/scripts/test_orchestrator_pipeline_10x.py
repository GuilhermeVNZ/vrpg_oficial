#!/usr/bin/env python3
"""
Teste de latência - 10 execuções para verificar melhoria com torch.compile
"""

import sys
import time
import statistics
from pathlib import Path

# Importar o módulo de teste original
sys.path.insert(0, str(Path(__file__).parent))
from test_orchestrator_pipeline import main as run_single_test, TEXT_1_5B, TEXT_14B

def run_10_tests():
    """Executa o teste 10 vezes e coleta métricas"""
    print("\n" + "="*70)
    print("  TESTE DE LATÊNCIA - 10 EXECUÇÕES")
    print("  Verificando melhoria com torch.compile")
    print("="*70 + "\n")
    
    latencies = []
    first_chunk_times = []
    qwen_1_5b_times = []
    qwen_14b_times = []
    
    for i in range(10):
        print(f"\n{'='*70}")
        print(f"  EXECUÇÃO {i+1}/10")
        print(f"{'='*70}\n")
        
        # Executar teste único
        # Nota: O teste original imprime muito, vamos capturar apenas o essencial
        start_time = time.time()
        
        # Importar e executar a função de teste diretamente
        from test_orchestrator_pipeline import test_orchestrator_pipeline
        import torch
        from TTS.api import TTS
        
        # Carregar modelo apenas na primeira execução
        if i == 0:
            use_gpu = torch.cuda.is_available()
            if use_gpu:
                gpu_name = torch.cuda.get_device_name(0)
                print(f"🎮 GPU: {gpu_name}")
            
            print("\n📥 Carregando modelo XTTS v2...")
            tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=False)
            print("✅ Modelo XTTS carregado!")
            
            # Aplicar otimizações (mesmo código do main)
            if use_gpu and torch.cuda.is_available():
                print("🔧 Configurando modelo para FP16...")
                try:
                    if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
                        current_dtype = next(tts.synthesizer.model.parameters()).dtype
                        print(f"   Dtype atual: {current_dtype}")
                        tts.synthesizer.model = tts.synthesizer.model.half().cuda()
                        model_dtype = next(tts.synthesizer.model.parameters()).dtype
                        if model_dtype == torch.float16:
                            print(f"✅ Modelo configurado para FP16 - dtype: {model_dtype}")
                        else:
                            print(f"⚠️  Modelo não está em FP16 - dtype: {model_dtype}")
                except Exception as e:
                    print(f"⚠️  Não foi possível configurar FP16: {e}")
                
                # Torch compile
                if hasattr(torch, 'compile'):
                    print("🔧 Compilando modelo com torch.compile...")
                    try:
                        if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
                            original_model = tts.synthesizer.model
                            try:
                                tts.synthesizer.model = torch.compile(original_model, mode="reduce-overhead")
                                print("✅ Modelo compilado com torch.compile")
                            except Exception as compile_error:
                                tts.synthesizer.model = original_model
                                print(f"⚠️  torch.compile não disponível: {compile_error}")
                    except Exception as e:
                        print(f"⚠️  Não foi possível compilar modelo: {e}")
                
                # Warm-up apenas na primeira execução
                print("🔥 Executando warm-up...")
                warmup_start = time.time()
                with torch.cuda.amp.autocast():
                    with torch.inference_mode():
                        _ = tts.tts("Warmup line for TTS", speaker="Ana Florence", language="en")
                warmup_time = time.time() - warmup_start
                print(f"✅ Warm-up concluído em {warmup_time:.3f}s")
                
                torch.cuda.empty_cache()
                torch.cuda.synchronize()
        
        # Executar teste
        result = test_orchestrator_pipeline(tts, TEXT_1_5B, TEXT_14B)
        
        if result and result.get('time_to_first_audio'):
            latency = result['time_to_first_audio']
            latencies.append(latency)
            
            if result.get('time_to_first_chunk'):
                first_chunk_times.append(result['time_to_first_chunk'])
            
            print(f"\n✅ Execução {i+1} concluída:")
            print(f"   Latência: {latency:.3f}s")
            if result.get('time_to_first_chunk'):
                print(f"   Primeiro chunk: {result['time_to_first_chunk']:.3f}s")
        else:
            print(f"\n⚠️  Execução {i+1} não retornou métricas")
        
        # Pequena pausa entre execuções (exceto na última)
        if i < 9:
            time.sleep(1)
    
    # Estatísticas finais
    print("\n" + "="*70)
    print("  ESTATÍSTICAS - 10 EXECUÇÕES")
    print("="*70)
    
    if latencies:
        print(f"\n📊 Latência (tempo até primeira reprodução):")
        print(f"   Média: {statistics.mean(latencies):.3f}s")
        print(f"   Mediana: {statistics.median(latencies):.3f}s")
        print(f"   Mínimo: {min(latencies):.3f}s")
        print(f"   Máximo: {max(latencies):.3f}s")
        print(f"   Desvio padrão: {statistics.stdev(latencies):.3f}s" if len(latencies) > 1 else "   Desvio padrão: N/A")
        
        print(f"\n📈 Evolução da latência:")
        for i, lat in enumerate(latencies, 1):
            trend = "📉" if i > 1 and lat < latencies[i-2] else "📈" if i > 1 and lat > latencies[i-2] else "➡️"
            print(f"   Execução {i}: {lat:.3f}s {trend}")
        
        # Análise de melhoria
        if len(latencies) >= 3:
            first_3_avg = statistics.mean(latencies[:3])
            last_3_avg = statistics.mean(latencies[-3:])
            improvement = first_3_avg - last_3_avg
            improvement_pct = (improvement / first_3_avg) * 100 if first_3_avg > 0 else 0
            
            print(f"\n🔍 Análise de melhoria:")
            print(f"   Primeiras 3 execuções (média): {first_3_avg:.3f}s")
            print(f"   Últimas 3 execuções (média): {last_3_avg:.3f}s")
            print(f"   Melhoria: {improvement:.3f}s ({improvement_pct:+.1f}%)")
            
            if improvement > 0.1:
                print(f"   ✅ torch.compile está melhorando a performance!")
            elif improvement < -0.1:
                print(f"   ⚠️  Latência aumentou (possível variação normal)")
            else:
                print(f"   ➡️  Latência estável (torch.compile já otimizado)")
        
        if first_chunk_times:
            print(f"\n📊 Tempo até primeiro chunk gerado:")
            print(f"   Média: {statistics.mean(first_chunk_times):.3f}s")
            print(f"   Mínimo: {min(first_chunk_times):.3f}s")
            print(f"   Máximo: {max(first_chunk_times):.3f}s")
    
    print("\n" + "="*70 + "\n")

if __name__ == "__main__":
    run_10_tests()



