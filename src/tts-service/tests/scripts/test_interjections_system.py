#!/usr/bin/env python3
"""
Teste do Sistema de Interjeições - Pré-roll de áudio para mascarar latência TTS
"""

import sys
import os
import time
from pathlib import Path
from datetime import datetime
import queue
import threading
import numpy as np
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
    from scipy import signal
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

# Configuração de Interjeições
class InterjectionConfig:
    def __init__(self, config_path=None):
        if config_path and Path(config_path).exists():
            with open(config_path, 'r', encoding='utf-8') as f:
                config = yaml.safe_load(f)
        else:
            # Configuração padrão
            config = {
                'enabled': True,
                'min_expected_tts_duration_sec': 3.0,
                'natural_delay_target_sec': 1.5,
                'avoid_last_n': 5,
                'max_uses_per_session': 999,
                'chars_per_sec': 25.0,
                'clips': []
            }
        
        self.enabled = config.get('enabled', True)
        self.min_expected_tts_duration_sec = config.get('min_expected_tts_duration_sec', 3.0)
        self.natural_delay_target_sec = config.get('natural_delay_target_sec', 1.5)
        self.avoid_last_n = config.get('avoid_last_n', 5)
        self.max_uses_per_session = config.get('max_uses_per_session', 999)
        self.chars_per_sec = config.get('chars_per_sec', 25.0)
        self.clips = config.get('clips', [])
        
        # Load clip durations
        self._load_clip_durations()
    
    def _load_clip_durations(self):
        """Carrega durações dos clipes de áudio"""
        for clip in self.clips:
            file_path = Path(clip['file'])
            if not file_path.is_absolute():
                # Assume relative to base directory
                file_path = base_dir / file_path
            
            if file_path.exists():
                try:
                    info = sf.info(str(file_path))
                    clip['duration_sec'] = info.duration
                except Exception as e:
                    print(f"⚠️  Não foi possível carregar duração de {file_path}: {e}")
                    clip['duration_sec'] = None
            else:
                clip['duration_sec'] = None

class InterjectionState:
    def __init__(self, avoid_last_n=5):
        self.recent_ids = []
        self.avoid_last_n = avoid_last_n
        self.use_counts = {}
        self.total_uses = 0
    
    def select_interjection(self, available_ids):
        """Seleciona uma interjeição evitando as últimas N usadas"""
        if not available_ids:
            return None
        
        # Filtrar IDs recentes
        candidates = [id for id in available_ids if id not in self.recent_ids]
        
        # Se todos estão na lista recente, resetar ou relaxar
        if not candidates:
            candidates = available_ids
        
        # Selecionar aleatoriamente
        import random
        selected = random.choice(candidates)
        
        # Registrar uso
        self.record_use(selected)
        
        return selected
    
    def record_use(self, clip_id):
        """Registra uso de uma interjeição"""
        # Adicionar à lista recente
        self.recent_ids.append(clip_id)
        if len(self.recent_ids) > self.avoid_last_n:
            self.recent_ids.pop(0)
        
        # Atualizar contador
        self.use_counts[clip_id] = self.use_counts.get(clip_id, 0) + 1
        self.total_uses += 1

class InterjectionManager:
    def __init__(self, config: InterjectionConfig):
        self.config = config
        self.state = InterjectionState(config.avoid_last_n)
    
    def should_use_interjection(self, text_length_chars, profile="cinematic"):
        """Verifica se deve usar interjeição baseado no tamanho do texto"""
        if not self.config.enabled:
            return False
        
        # Threshold específico por perfil
        threshold = self.config.min_expected_tts_duration_sec
        if profile == "fast":
            threshold *= 1.33  # Mais agressivo para FAST
        
        # Estimar duração
        expected_duration_sec = text_length_chars / self.config.chars_per_sec
        
        return expected_duration_sec >= threshold
    
    def calculate_delay_to_interjection(self, elapsed_since_user_end):
        """Calcula delay até início da interjeição"""
        target = self.config.natural_delay_target_sec
        return max(0.0, target - elapsed_since_user_end)
    
    def select_interjection(self):
        """Seleciona próxima interjeição"""
        if self.config.max_uses_per_session > 0:
            if self.state.total_uses >= self.config.max_uses_per_session:
                return None
        
        available_ids = [clip['id'] for clip in self.config.clips if clip.get('duration_sec') is not None]
        selected_id = self.state.select_interjection(available_ids)
        
        if selected_id:
            return next((clip for clip in self.config.clips if clip['id'] == selected_id), None)
        return None
    
    def load_interjection_audio(self, clip):
        """Carrega áudio da interjeição"""
        file_path = Path(clip['file'])
        if not file_path.is_absolute():
            file_path = base_dir / file_path
        
        if not file_path.exists():
            return None, None
        
        try:
            audio, sr = sf.read(str(file_path))
            # Converter para mono se necessário
            if len(audio.shape) > 1:
                audio = np.mean(audio, axis=0)
            return audio, sr
        except Exception as e:
            print(f"⚠️  Erro ao carregar interjeição {clip['id']}: {e}")
            return None, None

# Textos de teste
TEXT_SHORT = "The door creaks open."  # ~20 chars, < 1s, não deve usar interjeição
TEXT_LONG = """In the depths of the forgotten library, ancient tomes whispered secrets to those who dared to listen. The air itself seemed to carry the weight of centuries, each breath tasting of dust and forgotten knowledge. As the scholar approached the central chamber, the very stones beneath their feet began to glow with an otherworldly light, revealing runes that had been hidden for millennia."""  # ~300 chars, ~12s, deve usar interjeição

def test_interjection_system():
    """Testa o sistema de interjeições"""
    print("\n" + "="*70)
    print("  TESTE: SISTEMA DE INTERJEIÇÕES")
    print("="*70)
    
    # Carregar configuração
    config_path = base_dir / "vrpg-client" / "src" / "tts-service" / "config" / "interjections.yaml"
    config = InterjectionConfig(str(config_path) if config_path.exists() else None)
    manager = InterjectionManager(config)
    
    print(f"\n📋 Configuração:")
    print(f"   Habilitado: {config.enabled}")
    print(f"   Min duração TTS: {config.min_expected_tts_duration_sec}s")
    print(f"   Delay natural: {config.natural_delay_target_sec}s")
    print(f"   Clipes disponíveis: {len([c for c in config.clips if c.get('duration_sec')])}")
    
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
            _ = tts.tts("Warmup line for TTS", speaker="Ana Florence", language="en")
        print("✅ Warm-up concluído")
        
    except Exception as e:
        print(f"❌ ERRO ao carregar modelo: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Teste 1: Texto curto (não deve usar interjeição)
    print("\n" + "="*70)
    print("  TESTE 1: TEXTO CURTO (sem interjeição)")
    print("="*70)
    
    user_speech_end_time = time.time()
    text = TEXT_SHORT
    text_length = len(text)
    
    print(f"\n📝 Texto: {text}")
    print(f"   Tamanho: {text_length} chars")
    
    # Verificar se deve usar interjeição
    should_use = manager.should_use_interjection(text_length, "cinematic")
    print(f"   Usar interjeição: {should_use}")
    
    if not should_use:
        print("   ✅ Correto: texto curto não deve usar interjeição")
        
        # Gerar TTS diretamente
        tts_start = time.time()
        with torch.inference_mode():
            audio, sr = tts.tts(text, speaker="Ana Florence", language="en")
        tts_time = time.time() - tts_start
        
        elapsed = time.time() - user_speech_end_time
        print(f"\n⏱️  Métricas:")
        print(f"   Tempo desde fim da fala: {elapsed:.3f}s")
        print(f"   Tempo de geração TTS: {tts_time:.3f}s")
        print(f"   Duração do áudio: {len(audio)/sr:.2f}s")
    
    # Teste 2: Texto longo (deve usar interjeição)
    print("\n" + "="*70)
    print("  TESTE 2: TEXTO LONGO (com interjeição)")
    print("="*70)
    
    user_speech_end_time = time.time()
    text = TEXT_LONG
    text_length = len(text)
    
    print(f"\n📝 Texto: {text[:100]}...")
    print(f"   Tamanho: {text_length} chars")
    
    # Verificar se deve usar interjeição
    should_use = manager.should_use_interjection(text_length, "cinematic")
    print(f"   Usar interjeição: {should_use}")
    
    if should_use:
        # Simular processamento ASR + LLM
        asr_llm_time = 0.3  # Simula tempo de ASR + LLM
        time.sleep(asr_llm_time)
        
        elapsed_since_user_end = time.time() - user_speech_end_time
        delay_to_interjection = manager.calculate_delay_to_interjection(elapsed_since_user_end)
        
        print(f"\n⏱️  Cálculo de delay:")
        print(f"   Tempo desde fim da fala: {elapsed_since_user_end:.3f}s")
        print(f"   Delay até interjeição: {delay_to_interjection:.3f}s")
        
        # Selecionar interjeição
        interjection_clip = manager.select_interjection()
        
        if interjection_clip:
            print(f"\n🎵 Interjeição selecionada: {interjection_clip['id']}")
            
            # Aguardar delay
            if delay_to_interjection > 0:
                print(f"   Aguardando {delay_to_interjection:.3f}s...")
                time.sleep(delay_to_interjection)
            
            # Tocar interjeição
            interjection_start = time.time()
            interjection_audio, interjection_sr = manager.load_interjection_audio(interjection_clip)
            
            if interjection_audio is not None:
                interjection_duration = len(interjection_audio) / interjection_sr
                print(f"   ✅ Interjeição tocada ({interjection_duration:.2f}s)")
                
                # Salvar interjeição
                output_dir = script_dir / "test_interjections"
                output_dir.mkdir(exist_ok=True)
                timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
                interjection_file = output_dir / f"interjection_{interjection_clip['id']}_{timestamp}.wav"
                sf.write(str(interjection_file), interjection_audio, interjection_sr)
                print(f"   💾 Salvo: {interjection_file}")
            else:
                print(f"   ⚠️  Interjeição não encontrada, simulando...")
                interjection_duration = 1.0  # Simular
                time.sleep(interjection_duration)
            
            interjection_end = time.time()
            time_to_interjection = interjection_start - user_speech_end_time
            
            # Gerar TTS em paralelo (simulado)
            tts_start = time.time()
            with torch.inference_mode():
                tts_audio, tts_sr = tts.tts(text, speaker="Ana Florence", language="en")
            tts_time = time.time() - tts_start
            
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
            
            # Salvar TTS
            tts_file = output_dir / f"tts_main_{timestamp}.wav"
            sf.write(str(tts_file), tts_audio, tts_sr)
            print(f"   💾 Salvo: {tts_file}")
            
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
                
                full_file = output_dir / f"full_interjection_tts_{timestamp}.wav"
                sf.write(str(full_file), full_audio, tts_sr)
                print(f"   💾 Áudio completo: {full_file}")
            
            # Métricas finais
            time_user_end_to_interjection = time_to_interjection
            time_user_end_to_tts_start = tts_play_start - user_speech_end_time
            
            print(f"\n📊 Métricas Finais:")
            print(f"   Tempo fim fala → interjeição: {time_user_end_to_interjection:.3f}s")
            print(f"   Tempo fim fala → TTS início: {time_user_end_to_tts_start:.3f}s")
            print(f"   Target delay: {config.natural_delay_target_sec}s")
            print(f"   ✅ Delay próximo do target: {abs(time_user_end_to_interjection - config.natural_delay_target_sec) < 0.3}")
        else:
            print("   ⚠️  Nenhuma interjeição disponível")
    else:
        print("   ⚠️  Interjeição não deveria ser usada para este texto")
    
    print("\n" + "="*70)
    print("  TESTE CONCLUÍDO")
    print("="*70 + "\n")

if __name__ == "__main__":
    test_interjection_system()



