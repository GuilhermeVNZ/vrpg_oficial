#!/usr/bin/env python3
"""
Teste do Pipeline Completo com Sistema de Interjeições
Simula o fluxo: Jogador fala → ASR → LLM → Interjeição (se necessário) → TTS → Áudio
"""

import sys
import os
import time
from pathlib import Path
from datetime import datetime
import queue
import threading
import numpy as np

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
    from scipy import signal
    import yaml
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("   Instale: pip install TTS soundfile torch torchaudio scipy pyyaml", file=sys.stderr)
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

# Configuração de Interjeições (simplificada)
class InterjectionConfig:
    def __init__(self):
        self.enabled = True
        self.min_expected_tts_duration_sec = 3.0
        self.natural_delay_target_sec = 1.5
        self.avoid_last_n = 5
        self.chars_per_sec = 25.0

class InterjectionManager:
    def __init__(self, config_path=None):
        self.config = InterjectionConfig()
        self.recent_ids = []
        # Usar caminho absoluto direto
        self.base_path = Path(r"G:\vrpg\vrpg-client\assets-and-models\voices\interjections")
        
        # Carregar clipes disponíveis
        self.clips = {}
        if self.base_path.exists():
            for wav_file in self.base_path.glob("*.wav"):
                clip_id = wav_file.stem
                if clip_id.startswith("dm_"):
                    try:
                        info = sf.info(str(wav_file))
                        self.clips[clip_id] = {
                            'id': clip_id,
                            'file': str(wav_file),
                            'duration_sec': info.duration
                        }
                    except:
                        pass
    
    def should_use_interjection(self, text_length_chars, profile="cinematic"):
        if not self.config.enabled:
            return False
        
        threshold = self.config.min_expected_tts_duration_sec
        if profile == "fast":
            threshold *= 1.33
        
        expected_duration_sec = text_length_chars / self.config.chars_per_sec
        return expected_duration_sec >= threshold
    
    def calculate_delay_to_interjection(self, elapsed_since_user_end):
        target = self.config.natural_delay_target_sec
        return max(0.0, target - elapsed_since_user_end)
    
    def select_interjection(self):
        if not self.clips:
            return None
        
        available = [c for c in self.clips.keys() if c not in self.recent_ids[-self.config.avoid_last_n:]]
        if not available:
            available = list(self.clips.keys())
        
        import random
        selected = random.choice(available)
        self.recent_ids.append(selected)
        if len(self.recent_ids) > self.config.avoid_last_n:
            self.recent_ids.pop(0)
        
        return self.clips.get(selected)
    
    def load_interjection_audio(self, clip):
        if not clip:
            return None, None
        try:
            audio, sr = sf.read(clip['file'])
            if len(audio.shape) > 1:
                audio = np.mean(audio, axis=0)
            return audio, sr
        except:
            return None, None

# Textos de teste
TEXT_SHORT = "The door creaks open."  # ~20 chars, não deve usar interjeição
TEXT_LONG = """In the depths of the forgotten library, ancient tomes whispered secrets to those who dared to listen. The air itself seemed to carry the weight of centuries, each breath tasting of dust and forgotten knowledge. As the scholar approached the central chamber, the very stones beneath their feet began to glow with an otherworldly light, revealing runes that had been hidden for millennia."""  # ~300 chars, deve usar interjeição

def test_interjections_pipeline():
    """Testa o pipeline completo com interjeições"""
    print("\n" + "="*70)
    print("  TESTE: PIPELINE COMPLETO COM INTERJEIÇÕES")
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
    
    # Carregar gerenciador de interjeições
    print("\n📥 Carregando interjeições...")
    interjection_mgr = InterjectionManager()
    print(f"✅ {len(interjection_mgr.clips)} interjeições carregadas")
    
    # Criar diretório de saída
    output_dir = script_dir / "test_interjections_pipeline"
    output_dir.mkdir(exist_ok=True)
    
    # Teste 1: Texto curto (sem interjeição)
    print("\n" + "="*70)
    print("  TESTE 1: TEXTO CURTO (sem interjeição)")
    print("="*70)
    
    user_speech_end_time = time.time()
    text = TEXT_SHORT
    text_length = len(text)
    
    print(f"\n📝 Texto: {text}")
    print(f"   Tamanho: {text_length} chars")
    
    should_use = interjection_mgr.should_use_interjection(text_length, "cinematic")
    print(f"   Usar interjeição: {should_use}")
    
    if not should_use:
        print("   ✅ Correto: texto curto não deve usar interjeição")
        
        # Gerar TTS diretamente
        tts_start = time.time()
        with torch.inference_mode():
            audio = tts.tts(text, speaker="Ana Florence", language="en")
        tts_time = time.time() - tts_start
        
        # Converter para numpy se necessário
        if isinstance(audio, list):
            audio = np.array(audio)
        if len(audio.shape) > 1:
            audio = np.mean(audio, axis=0)
        
        sr = tts.synthesizer.output_sample_rate
        
        elapsed = time.time() - user_speech_end_time
        print(f"\n⏱️  Métricas:")
        print(f"   Tempo desde fim da fala: {elapsed:.3f}s")
        print(f"   Tempo de geração TTS: {tts_time:.3f}s")
        print(f"   Duração do áudio: {len(audio)/sr:.2f}s")
        
        # Salvar
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_file = output_dir / f"test_short_no_interjection_{timestamp}.wav"
        sf.write(str(output_file), audio.astype(np.float32), sr, subtype='FLOAT')
        print(f"   💾 Salvo: {output_file}")
    
    # Teste 2: Texto longo (com interjeição)
    print("\n" + "="*70)
    print("  TESTE 2: TEXTO LONGO (com interjeição)")
    print("="*70)
    
    user_speech_end_time = time.time()
    text = TEXT_LONG
    text_length = len(text)
    
    print(f"\n📝 Texto: {text[:100]}...")
    print(f"   Tamanho: {text_length} chars")
    
    should_use = interjection_mgr.should_use_interjection(text_length, "cinematic")
    print(f"   Usar interjeição: {should_use}")
    
    if should_use:
        # Simular processamento ASR + LLM
        asr_llm_time = 0.3
        time.sleep(asr_llm_time)
        
        elapsed_since_user_end = time.time() - user_speech_end_time
        delay_to_interjection = interjection_mgr.calculate_delay_to_interjection(elapsed_since_user_end)
        
        print(f"\n⏱️  Cálculo de delay:")
        print(f"   Tempo desde fim da fala: {elapsed_since_user_end:.3f}s")
        print(f"   Delay até interjeição: {delay_to_interjection:.3f}s")
        
        # Selecionar interjeição
        interjection_clip = interjection_mgr.select_interjection()
        
        if interjection_clip:
            print(f"\n🎵 Interjeição selecionada: {interjection_clip['id']}")
            
            # Aguardar delay
            if delay_to_interjection > 0:
                print(f"   Aguardando {delay_to_interjection:.3f}s...")
                time.sleep(delay_to_interjection)
            
            # Tocar interjeição
            interjection_start = time.time()
            interjection_audio, interjection_sr = interjection_mgr.load_interjection_audio(interjection_clip)
            
            if interjection_audio is not None:
                interjection_duration = len(interjection_audio) / interjection_sr
                print(f"   ✅ Interjeição tocada ({interjection_duration:.2f}s)")
            else:
                print(f"   ⚠️  Interjeição não encontrada")
                interjection_duration = 0
            
            interjection_end = time.time()
            time_to_interjection = interjection_start - user_speech_end_time
            
            # Gerar TTS em paralelo (simulado)
            tts_start = time.time()
            with torch.inference_mode():
                tts_audio = tts.tts(text, speaker="Ana Florence", language="en")
            tts_time = time.time() - tts_start
            
            # Converter para numpy se necessário
            if isinstance(tts_audio, list):
                tts_audio = np.array(tts_audio)
            if len(tts_audio.shape) > 1:
                tts_audio = np.mean(tts_audio, axis=0)
            
            tts_sr = tts.synthesizer.output_sample_rate
            
            # Aguardar interjeição terminar (se ainda não terminou)
            if time.time() < interjection_end:
                remaining = interjection_end - time.time()
                if remaining > 0:
                    time.sleep(remaining)
            
            # Pequeno gap antes do TTS principal
            gap_ms = 50
            time.sleep(gap_ms / 1000.0)
            
            tts_play_start = time.time()
            tts_duration = len(tts_audio) / tts_sr
            print(f"\n🎵 TTS principal:")
            print(f"   Duração: {tts_duration:.2f}s")
            print(f"   Tempo de geração: {tts_time:.3f}s")
            
            # Concatenar interjeição + TTS
            if interjection_audio is not None:
                # Resample se necessário
                if interjection_sr != tts_sr:
                    num_samples = int(len(interjection_audio) * tts_sr / interjection_sr)
                    interjection_audio = signal.resample(interjection_audio, num_samples)
                
                # Concatenar com gap
                gap_samples = int(tts_sr * gap_ms / 1000.0)
                gap = np.zeros(gap_samples)
                full_audio = np.concatenate([interjection_audio, gap, tts_audio])
                
                timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
                full_file = output_dir / f"test_long_with_interjection_{timestamp}.wav"
                sf.write(str(full_file), full_audio, tts_sr, subtype='FLOAT')
                print(f"   💾 Áudio completo: {full_file}")
            
            # Métricas finais
            time_user_end_to_interjection = time_to_interjection
            time_user_end_to_tts_start = tts_play_start - user_speech_end_time
            
            print(f"\n📊 Métricas Finais:")
            print(f"   Tempo fim fala → interjeição: {time_user_end_to_interjection:.3f}s")
            print(f"   Tempo fim fala → TTS início: {time_user_end_to_tts_start:.3f}s")
            print(f"   Target delay: {interjection_mgr.config.natural_delay_target_sec}s")
            print(f"   ✅ Delay próximo do target: {abs(time_user_end_to_interjection - interjection_mgr.config.natural_delay_target_sec) < 0.3}")
    
    print("\n" + "="*70)
    print("  TESTE CONCLUÍDO")
    print("="*70 + "\n")

if __name__ == "__main__":
    test_interjections_pipeline()

