#!/usr/bin/env python3
"""
Gera áudios de interjeição usando XTTS com voz da DM (Ana Florence)
Versão 2: Sem limite de duração, textos corrigidos, mais variedade
"""

import sys
import os
import time
from pathlib import Path
import yaml

# Configurar encoding para UTF-8
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

script_dir = Path(__file__).parent
base_dir = script_dir.parent.parent.parent.parent

try:
    import soundfile as sf
    import torch
    import torchaudio
    from TTS.api import TTS
    import numpy as np
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("   Instale: pip install TTS soundfile torch torchaudio pyyaml", file=sys.stderr)
    sys.exit(1)

# --- Fix para PyTorch 2.6+ ---
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load

# --- Monkey Patch para torchaudio.load ---
_original_torchaudio_load = torchaudio.load

def patched_torchaudio_load(filepath, *args, **kwargs):
    try:
        return _original_torchaudio_load(filepath, *args, **kwargs)
    except (RuntimeError, ImportError, OSError) as e:
        error_str = str(e).lower()
        if any(keyword in error_str for keyword in ["torchcodec", "ffmpeg", "dll", "libtorchcodec"]):
            try:
                audio, sr = sf.read(filepath)
                if len(audio.shape) == 1:
                    audio = audio.reshape(1, -1)
                elif len(audio.shape) == 2 and audio.shape[0] > audio.shape[1]:
                    audio = audio.T
                audio_tensor = torch.from_numpy(audio.copy()).float()
                return audio_tensor, int(sr)
            except Exception as sf_error:
                raise RuntimeError(
                    f"Failed to load audio with both torchcodec and soundfile. "
                    f"torchcodec error: {e}, soundfile error: {sf_error}"
                ) from e
        else:
            raise

torchaudio.load = patched_torchaudio_load
# --- Fim Monkey Patch ---

# Textos de interjeição e frases curtas (variados, sem repetição excessiva)
INTERJECTION_TEXTS = {
    # Interjeições curtas
    "dm_hmm_01": "Hmm...",
    "dm_hmm_02": "Hmm.",  # Simplificado para evitar final estranho
    "dm_hmm_03": "Hmm.",  # Simplificado para evitar duplicação
    "dm_hmm_04": "Hmmmm...",
    "dm_thinking_01": "Let me think...",
    "dm_thinking_02": "Thinking...",
    "dm_ah_01": "Ah.",  # Simplificado para evitar ruído ao fim
    "dm_ah_02": "Ah, I see...",
    "dm_well_01": "Well...",
    "dm_well_02": "Well...",
    "dm_okay_01": "Okay...",
    "dm_okay_02": "Okay...",
    "dm_right_01": "Right...",
    "dm_right_02": "Right...",
    "dm_so_01": "So.",  # Simplificado para evitar duplicação
    "dm_so_02": "So...",
    "dm_so_03": "So.",  # Segunda versão do "so" (splitado)
    "dm_uh_01": "Uh.",  # Simplificado para evitar segunda coisa sem sentido
    "dm_uh_02": "Uh...",
    "dm_um_01": "Um...",
    "dm_um_02": "Um.",  # Simplificado para evitar segunda coisa sem sentido
    "dm_hm_01": "Hm...",
    "dm_hm_02": "Hm...",
    "dm_hm_03": "Hm.",  # Simplificado para evitar final estranho
    
    # Frases curtas de resposta
    "dm_that_new": "That's new.",
    "dm_that_cool": "That's cool.",
    "dm_let_me_read": "Let me read about it.",
    "dm_let_me_think_about": "Let me think about it.",
    "dm_let_me_check_this": "Let me check this.",
    "dm_so_i_see_01": "So, I see what you're doing.",
    "dm_i_see_what": "I see what you're doing.",
    "dm_i_understand": "I understand.",
    "dm_got_it": "Got it.",
    "dm_ok_got_it": "Okay, got it.",
    "dm_ok_think": "Okay, let me think.",
    "dm_ok_check": "Okay, let me check.",
    "dm_ok_hmm": "Okay, hmm...",
    "dm_hmm_ok": "Hmm, okay.",
    "dm_that_clever": "That's clever.",
    "dm_interesting": "Interesting.",
    "dm_hmm_interesting": "Hmm, interesting.",
    "dm_interesting_huh": "Interesting, huh?",
    "dm_ah_i_see": "Ah, I see.",
    "dm_so_that_what": "So, that's what you want to do? Okay.",
    "dm_if_that_what": "If that's what you want.",
    "dm_if_you_say": "If you say so, but let me check if you can.",
    "dm_i_got_point": "I got your point, let me think a little.",
    "dm_let_me_check_something": "Let me check just something here.",
    "dm_think_before": "Let me think a little bit before answering this.",
    "dm_haha_smart": "Haha, that's smart, but let me check.",
    "dm_you_got_me": "You got me on this one, let me think longer before understanding how this works.",
    "dm_tricky_one": "This is a tricky one, not easy to answer.",
    "dm_alright": "Alright.",
    "dm_right": "Right.",  # Já está simplificado, mas pode precisar de ajuste
    
    # Sons não-verbais (usar texto descritivo que XTTS pode interpretar melhor)
    # Para "breath" e "sigh", vamos usar pausas e sons mais naturais
    "dm_sigh_01": "Hmm... *sigh*",
    "dm_sigh_02": "*sigh*",
    "dm_breath_01": "*breath*",
    "dm_breath_02": "*takes a breath*",
}

def generate_interjections():
    """Gera todos os áudios de interjeição"""
    print("\n" + "="*70)
    print("  GERAÇÃO DE INTERJEIÇÕES V2 - VOZ DA DM")
    print("="*70)
    
    # Verificar GPU
    use_gpu = torch.cuda.is_available()
    if use_gpu:
        gpu_name = torch.cuda.get_device_name(0)
        print(f"\n🎮 GPU: {gpu_name}")
    else:
        print("\n💻 Usando CPU")
    
    # Carregar modelo XTTS
    print("\n📥 Carregando modelo XTTS v2...")
    try:
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=False)
        print("✅ Modelo XTTS carregado!")
        
        # Configurar FP16
        if use_gpu and torch.cuda.is_available():
            try:
                if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
                    tts.synthesizer.model = tts.synthesizer.model.half().cuda()
                    print("✅ Modelo configurado para FP16")
            except Exception as e:
                print(f"⚠️  Não foi possível configurar FP16: {e}")
        
        # Warm-up
        print("🔥 Executando warm-up...")
        with torch.inference_mode():
            _ = tts.tts("Warmup", speaker="Ana Florence", language="en")
        print("✅ Warm-up concluído")
        
    except Exception as e:
        print(f"❌ ERRO ao carregar modelo: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Criar diretório de saída (pasta separada para correções)
    # Usar: vrpg-client/assets-and-models/voices/interjections_fix
    output_dir = Path(r"G:\vrpg\vrpg-client\assets-and-models\voices\interjections_fix")
    output_dir.mkdir(parents=True, exist_ok=True)
    print(f"\n📁 Diretório de saída: {output_dir}")
    
    # Gerar cada interjeição
    print(f"\n🎵 Gerando {len(INTERJECTION_TEXTS)} interjeições e frases...\n")
    
    results = []
    
    for clip_id, text in INTERJECTION_TEXTS.items():
        print(f"🎤 {clip_id}: '{text}'")
        
        try:
            # Gerar áudio
            start_time = time.time()
            with torch.inference_mode():
                audio = tts.tts(text, speaker="Ana Florence", language="en")
            gen_time = time.time() - start_time
            
            # Converter para numpy se necessário
            if isinstance(audio, list):
                audio = np.array(audio)
            
            # Converter para mono se necessário
            if len(audio.shape) > 1:
                audio = np.mean(audio, axis=0)
            
            # Obter sample rate
            sample_rate = tts.synthesizer.output_sample_rate
            
            # Calcular duração
            duration = len(audio) / sample_rate
            
            print(f"   ✅ Duração: {duration:.2f}s")
            
            # Salvar arquivo
            output_file = output_dir / f"{clip_id}.wav"
            
            # Salvar como Float32 mono (qualidade máxima)
            sf.write(
                str(output_file),
                audio.astype(np.float32),
                sample_rate,
                subtype='FLOAT'
            )
            
            print(f"   💾 Salvo: {output_file}")
            print(f"   ⏱️  Tempo de geração: {gen_time:.3f}s\n")
            
            results.append({
                'id': clip_id,
                'text': text,
                'duration': duration,
                'file': str(output_file),
                'gen_time': gen_time,
            })
            
        except Exception as e:
            print(f"   ❌ Erro ao gerar {clip_id}: {e}\n")
            import traceback
            traceback.print_exc()
    
    # Resumo
    print("="*70)
    print("  RESUMO")
    print("="*70)
    
    total_duration = sum(r['duration'] for r in results)
    total_gen_time = sum(r['gen_time'] for r in results)
    avg_duration = total_duration / len(results) if results else 0
    max_duration_found = max((r['duration'] for r in results), default=0)
    min_duration_found = min((r['duration'] for r in results), default=0)
    
    print(f"\n📊 Estatísticas:")
    print(f"   Total de interjeições: {len(results)}")
    print(f"   Duração média: {avg_duration:.2f}s")
    print(f"   Duração mínima: {min_duration_found:.2f}s")
    print(f"   Duração máxima: {max_duration_found:.2f}s")
    print(f"   Total de áudio: {total_duration:.2f}s")
    print(f"   Tempo total de geração: {total_gen_time:.2f}s")
    
    print(f"\n📁 Arquivos salvos em: {output_dir}")
    print("\n" + "="*70 + "\n")
    
    return results

if __name__ == "__main__":
    results = generate_interjections()
    
    # Criar arquivo de resumo
    summary_file = Path(__file__).parent / "interjections_generated_summary_v2.md"
    with open(summary_file, 'w', encoding='utf-8') as f:
        f.write("# Interjeições Geradas V2\n\n")
        f.write(f"**Data**: {time.strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        f.write(f"**Total**: {len(results)} interjeições e frases\n\n")
        f.write("| ID | Texto | Duração (s) | Arquivo |\n")
        f.write("|----|-------|-------------|----------|\n")
        for r in results:
            f.write(f"| {r['id']} | {r['text']} | {r['duration']:.2f} | {Path(r['file']).name} |\n")

