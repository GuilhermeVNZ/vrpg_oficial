#!/usr/bin/env python3
"""
Teste do XTTS usando o arquivo de referência personalizado do dungeon master
"""

import sys
import os
from pathlib import Path
import numpy as np

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent
sovits_dir = script_dir.parent.parent.parent.parent / "assets-and-models" / "models" / "tts" / "sovits"
sys.path.insert(0, str(sovits_dir))

try:
    from TTS.api import TTS
    import torch
    import soundfile as sf
    import torchaudio
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    sys.exit(1)

# Monkey patch torchaudio.load para usar soundfile (evita problema do torchcodec)
original_torchaudio_load = torchaudio.load
def patched_torchaudio_load(filepath, *args, **kwargs):
    try:
        # Tentar carregar com soundfile primeiro
        audio, sr = sf.read(filepath)
        # Converter para tensor no formato esperado pelo torchaudio
        if len(audio.shape) == 1:
            audio = audio.reshape(1, -1)  # [channels, samples]
        audio_tensor = torch.from_numpy(audio).float()
        return audio_tensor, sr
    except:
        # Fallback para método original
        return original_torchaudio_load(filepath, *args, **kwargs)

torchaudio.load = patched_torchaudio_load

# Fix para PyTorch 2.6+
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load


def test_xtts_with_custom_voice():
    """Testa XTTS com voz personalizada do dungeon master"""
    print("\n" + "="*70)
    print("  TESTE: XTTS com Voz Personalizada (Dungeon Master)")
    print("="*70 + "\n")
    
    # Caminho do arquivo de referência
    reference_wav = script_dir / "dungeon_master_en_xtts_reference.wav"
    
    if not reference_wav.exists():
        print(f"❌ ERRO: Arquivo de referência não encontrado: {reference_wav}")
        print("   Execute primeiro: create_xtts_embedding.py")
        sys.exit(1)
    
    print(f"✅ Arquivo de referência encontrado: {reference_wav.name}")
    
    # Carregar modelo XTTS
    print("\n📥 Carregando modelo XTTS v2...")
    try:
        use_gpu = torch.cuda.is_available()
        device = "cuda" if use_gpu else "cpu"
        print(f"   Dispositivo: {device}")
        
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu, progress_bar=True)
        print("✅ Modelo XTTS carregado!\n")
    except Exception as e:
        print(f"❌ ERRO ao carregar modelo: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)
    
    # Testar síntese com voz personalizada
    test_text = "Hello World"
    print(f"🎙️  Sintetizando: '{test_text}'")
    print(f"   Usando voz personalizada: {reference_wav.name}\n")
    
    try:
        # Tentar usar speaker_wav diretamente
        # Se falhar, vamos tentar carregar o áudio e usar método alternativo
        try:
            audio = tts.tts(
                text=test_text,
                speaker_wav=str(reference_wav),  # Usar arquivo de referência personalizado
                language="en",
            )
        except Exception as e:
            if "torchcodec" in str(e).lower() or "ffmpeg" in str(e).lower():
                print("⚠️  Erro com torchcodec, tentando método alternativo...")
                # Carregar áudio com soundfile e passar como array
                ref_audio, ref_sr = sf.read(str(reference_wav))
                if len(ref_audio.shape) > 1:
                    ref_audio = np.mean(ref_audio, axis=1)
                
                # Usar método direto do modelo se disponível
                if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'tts_model'):
                    # Tentar usar get_conditioning_latents diretamente
                    model = tts.synthesizer.tts_model
                    ref_audio_tensor = torch.from_numpy(ref_audio).float().unsqueeze(0)
                    if torch.cuda.is_available():
                        ref_audio_tensor = ref_audio_tensor.cuda()
                    
                    with torch.no_grad():
                        gpt_cond_latent, speaker_embedding = model.get_conditioning_latents(
                            audio=ref_audio_tensor,
                            gpt_cond_len=model.config.gpt_cond_len,
                            max_ref_length=model.config.max_ref_len,
                            sound_norm_refs=model.config.sound_norm_refs
                        )
                    
                    # Agora sintetizar com os latents
                    audio = tts.tts(
                        text=test_text,
                        language="en",
                        gpt_cond_latent=gpt_cond_latent,
                        speaker_embedding=speaker_embedding,
                    )
                else:
                    raise e
            else:
                raise e
        
        print(f"✅ Áudio gerado com sucesso!")
        print(f"   - Amostras: {len(audio)}")
        print(f"   - Sample rate: {tts.synthesizer.output_sample_rate} Hz")
        print(f"   - Duração: {len(audio) / tts.synthesizer.output_sample_rate:.2f}s")
        
        # Salvar resultado
        output_path = script_dir / "test_hello_world_xtts_custom_voice.wav"
        sf.write(str(output_path), audio, tts.synthesizer.output_sample_rate)
        
        print(f"\n💾 Áudio salvo em: {output_path}")
        print(f"\n🎧 Ouça o resultado para verificar a qualidade da voz personalizada!")
        print("="*70 + "\n")
        
    except Exception as e:
        print(f"❌ ERRO ao sintetizar: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    test_xtts_with_custom_voice()

