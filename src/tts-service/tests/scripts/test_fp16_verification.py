#!/usr/bin/env python3
"""
Teste para verificar se FP16 está totalmente ativo no modelo XTTS
"""

import sys
import torch

# --- Fix para PyTorch 2.6+ ---
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load

# Adicionar safe globals
try:
    from TTS.tts.configs.xtts_config import XttsConfig
    torch.serialization.add_safe_globals([XttsConfig])
except:
    pass

from TTS.api import TTS

def test_fp16_verification():
    """Testa se FP16 está realmente ativo no modelo"""
    print("\n" + "="*70)
    print("  TESTE: VERIFICAÇÃO FP16")
    print("="*70)
    
    use_gpu = torch.cuda.is_available()
    if not use_gpu:
        print("\n❌ GPU não disponível - FP16 requer GPU")
        return False
    
    gpu_name = torch.cuda.get_device_name(0)
    print(f"\n🎮 GPU: {gpu_name}")
    
    # Carregar modelo
    print("\n📥 Carregando modelo XTTS v2...")
    try:
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=False)
        print("✅ Modelo XTTS carregado!")
    except Exception as e:
        print(f"❌ ERRO ao carregar modelo: {e}")
        return False
    
    # Verificar dtype antes da conversão
    print("\n🔍 Verificando dtype ANTES da conversão...")
    try:
        # Tentar diferentes caminhos para acessar o modelo
        model = None
        if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
            model = tts.synthesizer.model
        elif hasattr(tts, 'model'):
            model = tts.model
        elif hasattr(tts, 'synthesizer'):
            # Tentar acessar através de outros atributos
            synth = tts.synthesizer
            if hasattr(synth, 'model'):
                model = synth.model
            elif hasattr(synth, 'tts_model'):
                model = synth.tts_model
        
        if model is None:
            print("⚠️  Não foi possível acessar o modelo diretamente")
            print("   Tentando método alternativo...")
            # Tentar através de warm-up para verificar dtype durante inferência
            print("   Executando warm-up para verificar dtype...")
            with torch.inference_mode():
                _ = tts.tts("Test", speaker="Ana Florence", language="en")
            # Tentar novamente após warm-up
            if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
                model = tts.synthesizer.model
        
        if model is not None:
            first_param = next(model.parameters())
            dtype_before = first_param.dtype
            device_before = first_param.device
            
            print(f"   Dtype: {dtype_before}")
            print(f"   Device: {device_before}")
            
            # Contar parâmetros por dtype
            param_count = {}
            for p in model.parameters():
                dt = str(p.dtype)
                param_count[dt] = param_count.get(dt, 0) + 1
            
            print(f"   Parâmetros por dtype: {param_count}")
        else:
            print("⚠️  Não foi possível acessar o modelo")
            print("   Vamos tentar converter mesmo assim...")
    except Exception as e:
        print(f"⚠️  Erro ao verificar modelo: {e}")
        print("   Continuando com conversão...")
        model = None
    
    # Converter para FP16
    print("\n🔧 Convertendo modelo para FP16...")
    try:
        # Tentar diferentes caminhos
        converted = False
        if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
            tts.synthesizer.model = tts.synthesizer.model.half().cuda()
            converted = True
            print("✅ Conversão aplicada (.half().cuda()) via synthesizer.model")
        elif hasattr(tts, 'model'):
            tts.model = tts.model.half().cuda()
            converted = True
            print("✅ Conversão aplicada via tts.model")
        else:
            print("⚠️  Não foi possível acessar o modelo para conversão")
            print("   O modelo pode ser convertido internamente durante inferência")
            converted = False
        
        if not converted:
            print("⚠️  Continuando sem conversão explícita (pode usar autocast)")
    except Exception as e:
        print(f"⚠️  Erro ao converter modelo: {e}")
        print("   Continuando com autocast como fallback")
        converted = False
    
    # Verificar dtype após conversão
    print("\n🔍 Verificando dtype APÓS a conversão...")
    try:
        # Tentar acessar modelo novamente
        model = None
        if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
            model = tts.synthesizer.model
        elif hasattr(tts, 'model'):
            model = tts.model
        
        if model is not None:
            first_param = next(model.parameters())
            dtype_after = first_param.dtype
            device_after = first_param.device
            
            print(f"   Dtype: {dtype_after}")
            print(f"   Device: {device_after}")
            
            # Verificar todos os parâmetros
            all_fp16 = True
            param_count_after = {}
            fp16_count = 0
            total_count = 0
            
            for p in model.parameters():
                dt = str(p.dtype)
                param_count_after[dt] = param_count_after.get(dt, 0) + 1
                total_count += 1
                if p.dtype == torch.float16:
                    fp16_count += 1
                else:
                    all_fp16 = False
            
            print(f"   Parâmetros por dtype: {param_count_after}")
            print(f"   Parâmetros FP16: {fp16_count}/{total_count} ({fp16_count/total_count*100:.1f}%)")
            
            if dtype_after == torch.float16 and all_fp16:
                print("\n✅ FP16 TOTALMENTE ATIVO!")
                print("   Todos os parâmetros estão em torch.float16")
                return True
            elif dtype_after == torch.float16:
                print("\n⚠️  FP16 PARCIALMENTE ATIVO")
                print(f"   Primeiro parâmetro em FP16, mas {total_count - fp16_count} parâmetros não estão")
                return False
            else:
                print("\n❌ FP16 NÃO ATIVO")
                print(f"   Dtype atual: {dtype_after}")
                return False
        else:
            print("⚠️  Não foi possível acessar o modelo após conversão")
            print("   Vamos testar com inferência para verificar comportamento")
            return None  # Indeterminado, precisa testar com inferência
    except Exception as e:
        print(f"⚠️  Erro ao verificar modelo: {e}")
        return None
    
    # Teste de inferência
    print("\n🧪 Testando inferência com FP16...")
    try:
        with torch.inference_mode():
            audio = tts.tts("Test line for FP16 verification", speaker="Ana Florence", language="en")
        print("✅ Inferência bem-sucedida")
        
        # Verificar dtype novamente após inferência
        model_dtype = next(tts.synthesizer.model.parameters()).dtype
        print(f"   Dtype após inferência: {model_dtype}")
        
        if model_dtype == torch.float16:
            print("✅ FP16 mantido após inferência")
            return True
        else:
            print("⚠️  FP16 não mantido após inferência")
            return False
    except Exception as e:
        print(f"❌ Erro na inferência: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = test_fp16_verification()
    print("\n" + "="*70)
    if success:
        print("✅ TESTE PASSOU - FP16 está totalmente ativo")
    else:
        print("❌ TESTE FALHOU - FP16 não está totalmente ativo")
    print("="*70 + "\n")
    sys.exit(0 if success else 1)

