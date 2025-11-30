#!/usr/bin/env python3
"""
Gera áudios de interjeição usando XTTS com voz da DM (Ana Florence)
Garante que cada áudio tenha no máximo 1.5s de duração
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

# Textos de interjeição (curtos, naturais, máximo 1.5s)
INTERJECTION_TEXTS = {
    "dm_hmm_01": "Hmm...",
    "dm_hmm_02": "Hmm, let me see...",
    "dm_thinking_01": "Let me think...",
    "dm_thinking_02": "Thinking...",
    "dm_let_me_see_01": "Let me see...",
    "dm_let_me_see_02": "Let me check...",
    "dm_interesting_01": "Interesting...",
    "dm_interesting_02": "That's interesting...",
    "dm_ah_01": "Ah...",
    "dm_ah_02": "Ah, I see...",
    "dm_well_01": "Well...",
    "dm_well_02": "Well, let me see...",
    "dm_okay_01": "Okay...",
    "dm_okay_02": "Okay, let me think...",
    "dm_right_01": "Right...",
    "dm_right_02": "Right, let me see...",
    "dm_so_01": "So...",
    "dm_so_02": "So, let me think...",
    "dm_let_me_think_01": "Let me think...",
    "dm_let_me_think_02": "Let me think about that...",
    "dm_uh_01": "Uh...",
    "dm_uh_02": "Uh, let me see...",
    "dm_um_01": "Um...",
    "dm_um_02": "Um, let me think...",
    "dm_hm_01": "Hm...",
    "dm_hm_02": "Hm, interesting...",
    "dm_hm_03": "Hm, let me see...",
    "dm_sigh_01": "*sigh*",
    "dm_sigh_02": "*deep breath*",
    "dm_breath_01": "*breath*",
    "dm_breath_02": "*takes a breath*",
    "dm_very_well_01": "Very well...",
    "dm_very_well_02": "Very well, then...",
    "dm_let_me_check_01": "Let me check...",
    "dm_let_me_check_02": "Let me check that...",
    "dm_interesting_03": "That's quite interesting...",
    "dm_curious_01": "Curious...",
    "dm_curious_02": "How curious...",
    "dm_ah_interesting_01": "Ah, interesting...",
    "dm_ah_interesting_02": "Ah, that's interesting...",
}

def generate_interjections():
    """Gera todos os áudios de interjeição"""
    print("\n" + "="*70)
    print("  GERAÇÃO DE INTERJEIÇÕES - VOZ DA DM")
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
    
    # Criar diretório de saída
    output_dir = base_dir / "vrpg-client" / "assets-and-models" / "models" / "tts" / "interjections"
    output_dir.mkdir(parents=True, exist_ok=True)
    print(f"\n📁 Diretório de saída: {output_dir}")
    
    # Gerar cada interjeição
    print(f"\n🎵 Gerando {len(INTERJECTION_TEXTS)} interjeições...\n")
    
    results = []
    max_duration = 1.5  # Máximo de 1.5s
    
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
            
            # Verificar se está dentro do limite
            if duration > max_duration:
                print(f"   ⚠️  Duração: {duration:.2f}s (ACIMA DO LIMITE de {max_duration}s)")
                # Truncar para 1.5s
                max_samples = int(sample_rate * max_duration)
                audio = audio[:max_samples]
                duration = len(audio) / sample_rate
                print(f"   ✂️  Truncado para: {duration:.2f}s")
            else:
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
    
    # Verificar se alguma excedeu o limite
    exceeded = [r for r in results if r['duration'] > max_duration]
    if exceeded:
        print(f"\n⚠️  {len(exceeded)} interjeições excederam {max_duration}s:")
        for r in exceeded:
            print(f"   • {r['id']}: {r['duration']:.2f}s")
    else:
        print(f"\n✅ Todas as interjeições estão dentro do limite de {max_duration}s")
    
    print(f"\n📁 Arquivos salvos em: {output_dir}")
    print("\n" + "="*70 + "\n")
    
    return results

if __name__ == "__main__":
    results = generate_interjections()
    
    # Criar arquivo de resumo
    summary_file = Path(__file__).parent / "interjections_generated_summary.md"
    with open(summary_file, 'w', encoding='utf-8') as f:
        f.write("# Interjeições Geradas\n\n")
        f.write(f"**Data**: {time.strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        f.write(f"**Total**: {len(results)} interjeições\n\n")
        f.write("| ID | Texto | Duração (s) | Arquivo |\n")
        f.write("|----|-------|-------------|----------|\n")
        for r in results:
            f.write(f"| {r['id']} | {r['text']} | {r['duration']:.2f} | {Path(r['file']).name} |\n")
    
    print(f"📄 Resumo salvo em: {summary_file}")



