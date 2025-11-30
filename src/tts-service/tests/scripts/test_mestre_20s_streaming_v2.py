#!/usr/bin/env python3
"""
Teste de geração de áudio de 20s da voz do Mestre (Ana Florence)
com perfis FAST e CINEMATIC e medição de latência inicial para streaming

Versão 2: Implementa perfis TTS e streaming real com FIFO
"""

import sys
import os
import time
from pathlib import Path
from datetime import datetime
import queue
import threading

# Configurar encoding para UTF-8
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

script_dir = Path(__file__).parent
base_dir = script_dir.parent.parent.parent.parent

try:
    import soundfile as sf
    import numpy as np
    import torch
    import torchaudio
    from TTS.api import TTS
    from scipy import signal
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("   Instale: pip install TTS soundfile torch torchaudio scipy", file=sys.stderr)
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

# Texto de ~20 segundos de áudio
TEXT_20S = """In the depths of the forgotten library, ancient tomes whispered secrets to those who dared to listen. The air itself seemed to carry the weight of centuries, each breath tasting of dust and forgotten knowledge. As the scholar approached the central chamber, the very stones beneath their feet began to glow with an otherworldly light, revealing runes that had been hidden for millennia. The walls themselves seemed to pulse with a rhythm that matched the beating of a heart long since stilled, and shadows danced along the edges of vision, promising both revelation and ruin."""

# Perfis TTS
class TtsProfile:
    def __init__(self, name, first_chunk_max_chars, next_chunk_max_chars, sample_rate, audio_block_ms, initial_prebuffer_ms):
        self.name = name
        self.first_chunk_max_chars = first_chunk_max_chars
        self.next_chunk_max_chars = next_chunk_max_chars
        self.sample_rate = sample_rate
        self.audio_block_ms = audio_block_ms
        self.initial_prebuffer_ms = initial_prebuffer_ms
    
    @staticmethod
    def fast():
        return TtsProfile("FAST", 30, 90, 16000, 50, 240)
    
    @staticmethod
    def cinematic():
        return TtsProfile("CINEMATIC", 100, 150, 24000, 60, 500)
    
    def audio_block_samples(self):
        return int(self.sample_rate * self.audio_block_ms / 1000)
    
    def initial_prebuffer_samples(self):
        return int(self.sample_rate * self.initial_prebuffer_ms / 1000)

def split_text_for_tts(text, profile):
    """Split text into chunks based on profile"""
    words = text.split()
    chunks = []
    current = ""
    current_limit = profile.first_chunk_max_chars
    
    for word in words:
        word_with_space = f"{word} " if current else word
        if len(current) + len(word_with_space) > current_limit and current:
            chunks.append(current.strip())
            current = word + " "
            current_limit = profile.next_chunk_max_chars
        else:
            current += word_with_space
    
    if current.strip():
        chunks.append(current.strip())
    
    return chunks

def test_profile_streaming(tts, text, profile, use_gpu):
    """Test streaming with profile-specific configuration"""
    print(f"\n{'='*70}")
    print(f"  TESTE: PERFIL {profile.name}")
    print(f"{'='*70}")
    
    print(f"\n📋 Configuração do perfil:")
    print(f"   Primeiro chunk: {profile.first_chunk_max_chars} chars")
    print(f"   Próximos chunks: {profile.next_chunk_max_chars} chars")
    print(f"   Sample rate: {profile.sample_rate} Hz")
    print(f"   Audio block: {profile.audio_block_ms} ms ({profile.audio_block_samples()} samples)")
    print(f"   Pre-buffer: {profile.initial_prebuffer_ms} ms ({profile.initial_prebuffer_samples()} samples)")
    
    # Split text
    text_chunks = split_text_for_tts(text, profile)
    print(f"\n📝 Texto dividido em {len(text_chunks)} chunks:")
    for i, chunk in enumerate(text_chunks[:3], 1):
        print(f"   Chunk {i}: {len(chunk)} chars - \"{chunk[:50]}...\"")
    if len(text_chunks) > 3:
        print(f"   ... ({len(text_chunks) - 3} chunks restantes)")
    
    # FIFO queue for audio blocks
    audio_fifo = queue.Queue()
    first_audio_time = None
    playback_started = threading.Event()
    
    def tts_producer():
        """Producer: Generate audio chunks and push to FIFO"""
        nonlocal first_audio_time
        
        for i, chunk_text in enumerate(text_chunks):
            chunk_start = time.time()
            
            # Generate audio
            try:
                audio = tts.tts(
                    text=chunk_text,
                    speaker="Ana Florence",
                    language="en"
                )
                
                chunk_time = time.time() - chunk_start
                sample_rate = tts.synthesizer.output_sample_rate
                
                # Convert to mono if needed
                if isinstance(audio, np.ndarray) and len(audio.shape) > 1:
                    if audio.shape[0] > audio.shape[1]:
                        audio = np.mean(audio, axis=0)
                    else:
                        audio = np.mean(audio, axis=1)
                
                # Resample to profile sample rate if needed
                if sample_rate != profile.sample_rate:
                    num_samples = int(len(audio) * profile.sample_rate / sample_rate)
                    audio = signal.resample(audio, num_samples)
                    sample_rate = profile.sample_rate
                
                # Split into blocks
                block_samples = profile.audio_block_samples()
                for block_start in range(0, len(audio), block_samples):
                    block = audio[block_start:block_start + block_samples]
                    audio_fifo.put(block)
                    
                    # Record time of first audio block
                    if first_audio_time is None:
                        first_audio_time = time.time()
                        print(f"\n✅ Primeiro bloco de áudio gerado em {chunk_time:.3f}s")
                        print(f"   Chunk {i+1}: {len(chunk_text)} chars → {len(audio)/sample_rate:.2f}s de áudio")
                
                print(f"   Chunk {i+1}/{len(text_chunks)}: {chunk_time:.3f}s (RTF: {chunk_time / (len(audio)/sample_rate):.2f}x)")
                
            except Exception as e:
                print(f"❌ Erro ao gerar chunk {i+1}: {e}")
                break
        
        # Signal end of generation
        audio_fifo.put(None)
    
    def audio_consumer():
        """Consumer: Wait for pre-buffer and start playback"""
        # Wait for pre-buffer
        total_samples = 0
        prebuffer_samples = profile.initial_prebuffer_samples()
        
        print(f"\n⏳ Aguardando pre-buffer ({profile.initial_prebuffer_ms} ms)...")
        while total_samples < prebuffer_samples:
            try:
                block = audio_fifo.get(timeout=0.1)
                if block is None:
                    return
                total_samples += len(block)
            except queue.Empty:
                continue
        
        # Start playback
        playback_start_time = time.time()
        playback_started.set()
        print(f"🎵 Playback iniciado! ({total_samples/profile.sample_rate:.3f}s de buffer)")
        
        # Consume remaining blocks
        blocks_played = 0
        while True:
            try:
                block = audio_fifo.get(timeout=1.0)
                if block is None:
                    break
                blocks_played += 1
                # Simulate playback (in real implementation, send to audio device)
                time.sleep(len(block) / profile.sample_rate)
            except queue.Empty:
                break
        
        playback_duration = time.time() - playback_start_time
        print(f"✅ Playback concluído: {blocks_played} blocos, {playback_duration:.2f}s")
    
    # Start producer and consumer
    request_start_time = time.time()
    
    producer_thread = threading.Thread(target=tts_producer, daemon=True)
    consumer_thread = threading.Thread(target=audio_consumer, daemon=True)
    
    producer_thread.start()
    consumer_thread.start()
    
    # Wait for playback to start
    playback_started.wait(timeout=30.0)
    
    if playback_started.is_set():
        time_to_first_audio = first_audio_time - request_start_time if first_audio_time else None
        time_to_playback = time.time() - request_start_time
        
        return {
            "profile": profile.name,
            "time_to_first_audio": time_to_first_audio,
            "time_to_playback": time_to_playback,
            "first_chunk_chars": len(text_chunks[0]) if text_chunks else 0,
            "total_chunks": len(text_chunks),
        }
    else:
        print("❌ Timeout aguardando início do playback")
        return None

def main():
    print("\n" + "="*70)
    print("  TESTE: ÁUDIO DE 20s - PERFIS FAST E CINEMATIC")
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
        
        # Warm-up
        print("🔥 Executando warm-up...")
        warmup_start = time.time()
        _ = tts.tts("Warmup line for TTS", speaker="Ana Florence", language="en")
        warmup_time = time.time() - warmup_start
        print(f"✅ Warm-up concluído em {warmup_time:.3f}s")
        
    except Exception as e:
        print(f"❌ ERRO ao carregar modelo: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Testar ambos os perfis
    results = []
    
    # Perfil FAST
    fast_profile = TtsProfile.fast()
    fast_result = test_profile_streaming(tts, TEXT_20S, fast_profile, use_gpu)
    if fast_result:
        results.append(fast_result)
    
    # Limpar cache
    if use_gpu:
        torch.cuda.empty_cache()
        time.sleep(1)
    
    # Perfil CINEMATIC
    cinematic_profile = TtsProfile.cinematic()
    cinematic_result = test_profile_streaming(tts, TEXT_20S, cinematic_profile, use_gpu)
    if cinematic_result:
        results.append(cinematic_result)
    
    # Resumo
    print("\n" + "="*70)
    print("  RESUMO - LATÊNCIA INICIAL")
    print("="*70)
    
    print(f"\n{'Perfil':<15}{'Primeiro Chunk':<18}{'Time to First Audio':<25}{'Time to Playback':<20}")
    print("-" * 70)
    
    for result in results:
        first_chunk = f"{result['first_chunk_chars']} chars"
        time_first = f"{result['time_to_first_audio']:.3f}s" if result['time_to_first_audio'] else "N/A"
        time_playback = f"{result['time_to_playback']:.3f}s" if result['time_to_playback'] else "N/A"
        print(f"{result['profile']:<15}{first_chunk:<18}{time_first:<25}{time_playback:<20}")
    
    print("\n📊 Targets:")
    print("   FAST: time_to_first_audio ≤ 0.8s (ideal 0.5-0.7s)")
    print("   CINEMATIC: time_to_first_audio na faixa de 1.5-3s")
    print("="*70 + "\n")

if __name__ == "__main__":
    main()



