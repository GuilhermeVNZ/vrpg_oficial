#!/usr/bin/env python3
"""
Teste de Pipeline Completo - Simulação do Orquestrador
Simula Qwen 1.5B (FAST) + Qwen 14B (CINEMATIC) com streaming real

Objetivo: Medir latência desde recebimento do texto até primeira reprodução
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

# Texto completo (mesmo parágrafo do teste anterior)
TEXT_FULL = """In the depths of the forgotten library, ancient tomes whispered secrets to those who dared to listen. The air itself seemed to carry the weight of centuries, each breath tasting of dust and forgotten knowledge. As the scholar approached the central chamber, the very stones beneath their feet began to glow with an otherworldly light, revealing runes that had been hidden for millennia. The walls themselves seemed to pulse with a rhythm that matched the beating of a heart long since stilled, and shadows danced along the edges of vision, promising both revelation and ruin."""

# Simulação: Qwen 1.5B gera início (primeiras 2-3 frases)
# Qwen 14B gera o restante
TEXT_1_5B = "In the depths of the forgotten library, ancient tomes whispered secrets to those who dared to listen."
TEXT_14B = "The air itself seemed to carry the weight of centuries, each breath tasting of dust and forgotten knowledge. As the scholar approached the central chamber, the very stones beneath their feet began to glow with an otherworldly light, revealing runes that had been hidden for millennia. The walls themselves seemed to pulse with a rhythm that matched the beating of a heart long since stilled, and shadows danced along the edges of vision, promising both revelation and ruin."

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
        return TtsProfile("FAST", 20, 90, 16000, 25, 100)  # Primeiro chunk menor (20 chars), blocos menores (25ms), pre-buffer mínimo (100ms)
    
    @staticmethod
    def cinematic():
        return TtsProfile("CINEMATIC", 100, 150, 24000, 60, 500)
    
    def audio_block_samples(self):
        return int(self.sample_rate * self.audio_block_ms / 1000)
    
    def initial_prebuffer_samples(self):
        return int(self.sample_rate * self.initial_prebuffer_ms / 1000)

def split_text_for_tts(text, profile, accumulated_audio_duration=0.0):
    """
    Split text into chunks based on profile.
    
    Strategy:
    - First chunk: Goes until first comma or period (natural pause) - respects punctuation
    - After 5s of audio: Prefer sentence boundaries (natural pauses)
    - Otherwise: Word boundaries respecting limits
    """
    chunks = []
    current_limit = profile.first_chunk_max_chars
    is_first_chunk = True
    current_audio_duration = accumulated_audio_duration
    
    # Estimate: ~150 words/min = 2.5 words/sec = ~5 chars/word average
    # So: chars / 5 / 2.5 = chars / 12.5 seconds
    chars_per_second = 12.5
    
    # For first chunk: find first comma or period
    if is_first_chunk:
        # Find first natural pause (comma, period, exclamation, question mark)
        first_comma = text.find(',')
        first_period = text.find('.')
        first_excl = text.find('!')
        first_quest = text.find('?')
        
        # Find the earliest punctuation mark
        first_pause = -1
        for punct_pos in [first_comma, first_period, first_excl, first_quest]:
            if punct_pos != -1 and (first_pause == -1 or punct_pos < first_pause):
                first_pause = punct_pos
        
        if first_pause != -1:
            # First chunk goes until first punctuation (inclusive)
            first_chunk = text[:first_pause + 1].strip()
            remaining_text = text[first_pause + 1:].strip()
            
            # Only use this if it's not too long (allow some flexibility)
            if len(first_chunk) <= current_limit * 2:  # Allow up to 2x limit for first chunk with punctuation
                chunks.append(first_chunk)
                estimated_duration = len(first_chunk) / chars_per_second
                current_audio_duration += estimated_duration
                is_first_chunk = False
                current_limit = profile.next_chunk_max_chars
                text = remaining_text  # Continue with remaining text
            else:
                # If too long, fall back to word-based splitting
                first_chunk = text[:current_limit].strip()
                # Try to find comma or period within limit
                for punct in [',', '.', '!', '?']:
                    punct_pos = first_chunk.rfind(punct)
                    if punct_pos != -1:
                        first_chunk = first_chunk[:punct_pos + 1].strip()
                        break
                
                chunks.append(first_chunk)
                estimated_duration = len(first_chunk) / chars_per_second
                current_audio_duration += estimated_duration
                is_first_chunk = False
                current_limit = profile.next_chunk_max_chars
                text = text[len(first_chunk):].strip()
        else:
            # No punctuation found, use word-based splitting
            pass  # Will fall through to word-based logic below
    
    # Continue with remaining text using word-based splitting
    words = text.split()
    current = ""
    
    for word in words:
        word_with_space = f"{word} " if current else word
        
        # Check if adding this word would exceed limit
        if len(current) + len(word_with_space) > current_limit and current:
            # CRITICAL: Always find punctuation before cutting - never cut mid-sentence
            # Build remaining text to search for punctuation
            word_idx = words.index(word) if word in words else 0
            remaining_words = words[word_idx:]
            remaining_text = current + " " + " ".join(remaining_words)
            
            # Find nearest punctuation mark (comma, period, exclamation, question)
            punct_pos = -1
            search_start = len(current)
            for punct in [', ', '. ', '! ', '? ']:
                idx = remaining_text.find(punct, search_start)
                if idx != -1 and (punct_pos == -1 or idx < punct_pos):
                    punct_pos = idx + len(punct)
            
            # If we found punctuation nearby (within reasonable distance), extend to it
            if punct_pos != -1 and punct_pos <= len(current) + current_limit * 1.5:
                # Extend current chunk to punctuation
                extended = remaining_text[:punct_pos].strip()
                if len(extended) <= current_limit * 2.0:  # Allow more flexibility to reach punctuation
                    current = extended
                    # Calculate how many words we consumed
                    extended_word_count = len(extended.split())
                    current_word_count = len(current.split())
                    words_consumed = extended_word_count - current_word_count if extended_word_count > current_word_count else extended_word_count
                    
                    # Skip consumed words
                    if word_idx + words_consumed < len(words):
                        words = words[word_idx + words_consumed:]
                    else:
                        words = []
                    
                    if not words:
                        break
                    # Finalize this chunk (guaranteed to end with punctuation)
                    chunks.append(current.strip())
                    estimated_duration = len(current.strip()) / chars_per_second
                    current_audio_duration += estimated_duration
                    current = ""
                    current_limit = profile.next_chunk_max_chars
                    is_first_chunk = False
                    continue
            
            # No nearby punctuation found, or too far
            # CRITICAL: Never finalize without punctuation - extend search further
            # Search up to 2x the limit to find punctuation
            extended_search_limit = current_limit * 2
            if punct_pos == -1 or punct_pos > len(current) + extended_search_limit:
                # Still no punctuation found - this is a problem
                # Try to find ANY punctuation in the entire remaining text
                full_remaining = " ".join([current] + words[words.index(word):] if word in words else words)
                for punct in [', ', '. ', '! ', '? ']:
                    idx = full_remaining.find(punct, len(current))
                    if idx != -1:
                        punct_pos = idx + len(punct)
                        break
                
                if punct_pos != -1 and punct_pos <= len(current) + extended_search_limit * 1.5:
                    # Found punctuation, extend to it
                    extended = full_remaining[:punct_pos].strip()
                    if len(extended) <= current_limit * 2.5:  # Allow more flexibility
                        current = extended
                        # Skip processed words
                        extended_words = extended.split()
                        if word in words:
                            word_idx = words.index(word)
                            words_processed = len(extended_words) - len(current.split())
                            words = words[word_idx + words_processed:] if words_processed > 0 else words[word_idx + 1:]
                        if not words:
                            break
                        chunks.append(current.strip())
                        estimated_duration = len(current.strip()) / chars_per_second
                        current_audio_duration += estimated_duration
                        current = ""
                        current_limit = profile.next_chunk_max_chars
                        is_first_chunk = False
                        continue
            
            # If still no punctuation, we have no choice but to cut
            # But log a warning
            if not current.rstrip().endswith(('.', '!', '?', ',')):
                print(f"⚠️  WARNING: Chunk finalized without punctuation: '{current[:50]}...'")
            
            chunks.append(current.strip())
            estimated_duration = len(current.strip()) / chars_per_second
            current_audio_duration += estimated_duration
            current = word + " "
            current_limit = profile.next_chunk_max_chars
            is_first_chunk = False
        else:
            current += word_with_space
    
    # Final chunk - ensure it ends with punctuation if possible
    if current.strip():
        # Check if final chunk ends with punctuation
        if not current.rstrip().endswith(('.', '!', '?', ',')):
            # Try to find punctuation in the remaining text (shouldn't happen, but safety check)
            pass
        chunks.append(current.strip())
    
    # VALIDATION: Check for actual text duplication (not word repetition, which can be intentional)
    # Only warn if we're transcribing the exact same text segment twice
    for i in range(1, len(chunks)):
        prev_chunk = chunks[i-1].strip()
        curr_chunk = chunks[i].strip()
        
        # Check if end of previous chunk is identical to start of current (actual duplication)
        # Compare last 20 chars of previous with first 20 chars of current
        prev_end = prev_chunk[-20:].strip() if len(prev_chunk) >= 20 else prev_chunk.strip()
        curr_start = curr_chunk[:20].strip() if len(curr_chunk) >= 20 else curr_chunk.strip()
        
        # Only warn if there's significant identical text overlap (actual duplication)
        if len(prev_end) >= 10 and len(curr_start) >= 10:
            # Check if there's a substantial identical substring
            if prev_end in curr_start or curr_start in prev_end:
                # Check if it's more than just a few words (actual duplication)
                if len(prev_end) >= 15 or len(curr_start) >= 15:
                    print(f"⚠️  WARNING: Possível duplicação de trecho entre chunks {i-1} e {i}")
                    print(f"   Chunk {i-1} termina: '...{prev_chunk[-40:]}'")
                    print(f"   Chunk {i} começa: '{curr_chunk[:40]}...'")
    
    return chunks

def synthesize_with_profile(tts, text, profile, speaker="Ana Florence", language="en", use_fp16=True):
    """Synthesize audio with profile-specific settings and FP16 optimization"""
    import torch
    
    # Generate audio - use inference_mode for best performance
    # If model is in FP16 (set during initialization), it will use FP16 automatically
    # No need for autocast overhead if model is already in FP16
    if use_fp16 and torch.cuda.is_available():
        # Simple: use inference_mode - if model is FP16, it uses FP16; if not, uses FP32
        # This avoids autocast overhead when model is already in FP16
        with torch.inference_mode():
            audio = tts.tts(
                text=text,
                speaker=speaker,
                language=language
            )
    else:
        # CPU or FP16 disabled
        audio = tts.tts(
            text=text,
            speaker=speaker,
            language=language
        )
    
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
    
    return audio, sample_rate

def test_orchestrator_pipeline(tts, text_1_5b, text_14b):
    """Test complete orchestrator pipeline with FAST + CINEMATIC profiles"""
    print("\n" + "="*70)
    print("  TESTE: PIPELINE COMPLETO - ORQUESTRADOR SIMULADO")
    print("="*70)
    
    fast_profile = TtsProfile.fast()
    cinematic_profile = TtsProfile.cinematic()
    
    print(f"\n📋 Simulação do Orquestrador:")
    print(f"   Qwen 1.5B (FAST): {len(text_1_5b)} chars - \"{text_1_5b[:60]}...\"")
    print(f"   Qwen 14B (CINEMATIC): {len(text_14b)} chars - \"{text_14b[:60]}...\"")
    
    # FIFO queue for audio blocks
    audio_fifo = queue.Queue()
    first_audio_time = None
    playback_started = threading.Event()
    all_audio_blocks = []  # Para salvar o áudio completo
    total_samples = 0
    tts_lock = threading.Lock()  # Lock para evitar chamadas concorrentes ao TTS
    accumulated_duration = 0.0  # Track accumulated audio duration for chunking strategy
    
    # Timestamps
    orchestrator_receive_time = time.time()
    qwen_1_5b_ready_time = None
    qwen_14b_ready_time = None
    first_chunk_generated_time = None
    playback_start_time = None
    
    def tts_producer_fast():
        """Producer: FAST profile (Qwen 1.5B)"""
        nonlocal qwen_1_5b_ready_time, first_chunk_generated_time, first_audio_time, accumulated_duration
        
        # Simular tempo de geração do Qwen 1.5B (muito rápido)
        time.sleep(0.1)  # Simula < 1.2s do Qwen 1.5B
        qwen_1_5b_ready_time = time.time()
        
        print(f"\n✅ Qwen 1.5B pronto em {qwen_1_5b_ready_time - orchestrator_receive_time:.3f}s")
        
        # Split text (first chunk, no accumulated audio yet)
        text_chunks = split_text_for_tts(text_1_5b, fast_profile, accumulated_audio_duration=0.0)
        print(f"   Chunks FAST: {len(text_chunks)} (primeiro: {len(text_chunks[0])} chars)")
        
        accumulated_duration = 0.0
        
        # Generate and push chunks (sequentially to avoid CUDA conflicts)
        for i, chunk_text in enumerate(text_chunks):
            chunk_start = time.time()
            
            try:
                # Use lock to prevent concurrent TTS calls
                with tts_lock:
                    audio, sr = synthesize_with_profile(tts, chunk_text, fast_profile, use_fp16=True)
                
                chunk_time = time.time() - chunk_start
                chunk_duration = len(audio) / sr
                accumulated_duration += chunk_duration
                
                if first_chunk_generated_time is None:
                    first_chunk_generated_time = time.time()
                    print(f"✅ Primeiro chunk FAST gerado em {chunk_time:.3f}s ({chunk_duration:.2f}s de áudio)")
                
                # OPTIMIZATION: Clear CUDA cache between chunks to prevent memory buildup
                if torch.cuda.is_available():
                    torch.cuda.empty_cache()
                
                # Split into smaller blocks for real-time streaming (25ms for FAST)
                block_samples = fast_profile.audio_block_samples()
                for block_start in range(0, len(audio), block_samples):
                    block = audio[block_start:block_start + block_samples]
                    # Ensure no overlap - exact block boundaries
                    if block_start + block_samples > len(audio):
                        block = audio[block_start:]  # Last block may be smaller
                    audio_fifo.put(block)
                    all_audio_blocks.append((block, fast_profile.sample_rate))
                    
                    if first_audio_time is None:
                        first_audio_time = time.time()
                
                print(f"   Chunk {i+1}/{len(text_chunks)} FAST: {chunk_time:.3f}s (RTF: {chunk_time / chunk_duration:.2f}x, {chunk_duration:.2f}s áudio, acumulado: {accumulated_duration:.2f}s)")
                
            except Exception as e:
                print(f"❌ Erro ao gerar chunk FAST {i+1}: {e}")
                import traceback
                traceback.print_exc()
                break
    
    def tts_producer_cinematic():
        """Producer: CINEMATIC profile (Qwen 14B)"""
        nonlocal qwen_14b_ready_time, accumulated_duration
        
        # Simular tempo de geração do Qwen 14B (mais lento)
        time.sleep(0.5)  # Simula < 6s do Qwen 14B
        qwen_14b_ready_time = time.time()
        
        print(f"\n✅ Qwen 14B pronto em {qwen_14b_ready_time - orchestrator_receive_time:.3f}s")
        
        # Split text (use accumulated duration from FAST chunks)
        # Wait a bit for FAST thread to start generating
        time.sleep(0.2)
        # Use current accumulated_duration (will be updated by FAST thread)
        text_chunks = split_text_for_tts(text_14b, cinematic_profile, accumulated_audio_duration=accumulated_duration)
        print(f"   Chunks CINEMATIC: {len(text_chunks)} (primeiro: {len(text_chunks[0])} chars, acumulado: {accumulated_duration:.2f}s)")
        
        # Generate and push chunks (sequentially to avoid CUDA conflicts)
        for i, chunk_text in enumerate(text_chunks):
            chunk_start = time.time()
            
            try:
                # Use lock to prevent concurrent TTS calls
                with tts_lock:
                    audio, sr = synthesize_with_profile(tts, chunk_text, cinematic_profile, use_fp16=True)
                
                chunk_time = time.time() - chunk_start
                chunk_duration = len(audio) / sr
                accumulated_duration += chunk_duration
                
                # OPTIMIZATION: Clear CUDA cache between chunks
                if torch.cuda.is_available():
                    torch.cuda.empty_cache()
                
                # Split into blocks (ensure no overlap)
                block_samples = cinematic_profile.audio_block_samples()
                for block_start in range(0, len(audio), block_samples):
                    block = audio[block_start:block_start + block_samples]
                    # Ensure no overlap - exact block boundaries
                    if block_start + block_samples > len(audio):
                        block = audio[block_start:]  # Last block may be smaller
                    audio_fifo.put(block)
                    all_audio_blocks.append((block, cinematic_profile.sample_rate))
                
                print(f"   Chunk {i+1}/{len(text_chunks)} CINEMATIC: {chunk_time:.3f}s (RTF: {chunk_time / chunk_duration:.2f}x, {chunk_duration:.2f}s áudio)")
                
            except Exception as e:
                print(f"❌ Erro ao gerar chunk CINEMATIC {i+1}: {e}")
                import traceback
                traceback.print_exc()
                break
        
        # Signal end of generation
        audio_fifo.put(None)
    
    def audio_consumer():
        """Consumer: Wait for pre-buffer and start playback"""
        nonlocal playback_start_time, total_samples
        
        # Wait for pre-buffer (FAST profile threshold - reduced to 200ms)
        prebuffer_samples = fast_profile.initial_prebuffer_samples()
        prebuffer_sample_rate = fast_profile.sample_rate
        
        print(f"\n⏳ Aguardando pre-buffer ({fast_profile.initial_prebuffer_ms} ms)...")
        
        while total_samples < prebuffer_samples:
            try:
                block = audio_fifo.get(timeout=0.1)
                if block is None:
                    return
                # Calculate samples based on block's sample rate
                # For simplicity, assume first blocks are from FAST profile
                total_samples += len(block)
            except queue.Empty:
                continue
        
        # Start playback
        playback_start_time = time.time()
        playback_started.set()
        print(f"🎵 Playback iniciado! ({total_samples/prebuffer_sample_rate:.3f}s de buffer)")
        
        # Consume remaining blocks (simulate playback)
        blocks_played = 0
        while True:
            try:
                block = audio_fifo.get(timeout=1.0)
                if block is None:
                    break
                blocks_played += 1
                # Simulate playback time
                time.sleep(len(block) / prebuffer_sample_rate)
            except queue.Empty:
                break
        
        playback_duration = time.time() - playback_start_time
        print(f"✅ Playback concluído: {blocks_played} blocos, {playback_duration:.2f}s")
    
    # Start all threads
    fast_thread = threading.Thread(target=tts_producer_fast, daemon=True)
    cinematic_thread = threading.Thread(target=tts_producer_cinematic, daemon=True)
    consumer_thread = threading.Thread(target=audio_consumer, daemon=True)
    
    fast_thread.start()
    cinematic_thread.start()
    consumer_thread.start()
    
    # Wait for playback to start
    playback_started.wait(timeout=30.0)
    
    # Wait for all threads to complete
    fast_thread.join(timeout=60.0)
    cinematic_thread.join(timeout=60.0)
    consumer_thread.join(timeout=60.0)
    
    # Calculate metrics
    if playback_start_time and orchestrator_receive_time:
        time_to_first_audio = playback_start_time - orchestrator_receive_time
        time_to_first_chunk = (first_chunk_generated_time - orchestrator_receive_time) if first_chunk_generated_time else None
        
        print("\n" + "="*70)
        print("  MÉTRICAS DE LATÊNCIA")
        print("="*70)
        print(f"\n⏱️  Tempo desde recebimento do texto até primeira reprodução:")
        print(f"   {time_to_first_audio:.3f}s")
        
        if time_to_first_chunk:
            print(f"\n⏱️  Tempo até primeiro chunk gerado:")
            print(f"   {time_to_first_chunk:.3f}s")
        
        if qwen_1_5b_ready_time:
            print(f"\n⏱️  Qwen 1.5B pronto:")
            print(f"   {qwen_1_5b_ready_time - orchestrator_receive_time:.3f}s")
        
        if qwen_14b_ready_time:
            print(f"\n⏱️  Qwen 14B pronto:")
            print(f"   {qwen_14b_ready_time - orchestrator_receive_time:.3f}s")
        
        print(f"\n📊 Targets:")
        print(f"   FAST Profile: ≤ 0.8s (atual: {time_to_first_audio:.3f}s)")
        print(f"   CINEMATIC Profile: 1.5-3s")
        print("="*70)
    
    # Concatenate all audio blocks
    if all_audio_blocks:
        # Group by sample rate
        fast_audio = []
        cinematic_audio = []
        
        for block, sr in all_audio_blocks:
            if sr == fast_profile.sample_rate:
                fast_audio.append(block)
            else:
                cinematic_audio.append(block)
        
        # Concatenate and resample if needed
        if fast_audio and cinematic_audio:
            fast_concatenated = np.concatenate(fast_audio)
            cinematic_concatenated = np.concatenate(cinematic_audio)
            
            # Resample cinematic to match fast (or vice versa)
            if fast_profile.sample_rate != cinematic_profile.sample_rate:
                # Resample cinematic to fast sample rate
                num_samples = int(len(cinematic_concatenated) * fast_profile.sample_rate / cinematic_profile.sample_rate)
                cinematic_resampled = signal.resample(cinematic_concatenated, num_samples)
                final_audio = np.concatenate([fast_concatenated, cinematic_resampled])
                final_sample_rate = fast_profile.sample_rate
            else:
                final_audio = np.concatenate([fast_concatenated, cinematic_concatenated])
                final_sample_rate = fast_profile.sample_rate
        elif fast_audio:
            final_audio = np.concatenate(fast_audio)
            final_sample_rate = fast_profile.sample_rate
        elif cinematic_audio:
            final_audio = np.concatenate(cinematic_audio)
            final_sample_rate = cinematic_profile.sample_rate
        else:
            final_audio = None
            final_sample_rate = None
        
        if final_audio is not None:
            # Save audio file
            output_dir = script_dir / "test_orchestrator_pipeline"
            output_dir.mkdir(exist_ok=True)
            
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            output_file = output_dir / f"orchestrator_pipeline_{timestamp}.wav"
            
            # Save as Float32 WAV (24 kHz mono)
            sf.write(
                str(output_file),
                final_audio.astype(np.float32),
                final_sample_rate,
                subtype='FLOAT'
            )
            
            duration = len(final_audio) / final_sample_rate
            print(f"\n💾 Áudio salvo: {output_file}")
            print(f"   Duração: {duration:.2f}s")
            print(f"   Sample rate: {final_sample_rate} Hz")
            print(f"   Formato: Float32 mono")
            
            return {
                "time_to_first_audio": time_to_first_audio if playback_start_time else None,
                "time_to_first_chunk": time_to_first_chunk,
                "audio_file": str(output_file),
                "duration": duration,
                "sample_rate": final_sample_rate
            }
    
    return None

def main():
    print("\n" + "="*70)
    print("  TESTE: PIPELINE COMPLETO - ORQUESTRADOR")
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
        
        # OPTIMIZATION 1: Set model to FP16 and GPU immediately after loading
        fp16_active = False
        if use_gpu and torch.cuda.is_available():
            print("🔧 Configurando modelo para FP16...")
            try:
                # Move model to GPU and convert to half precision
                if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
                    # Check current dtype
                    current_dtype = next(tts.synthesizer.model.parameters()).dtype
                    print(f"   Dtype antes: {current_dtype}")
                    
                    # Convert to half precision and move to GPU
                    tts.synthesizer.model = tts.synthesizer.model.half().cuda()
                    
                    # Verify FP16 is active - check multiple parameters to be sure
                    model_dtype = next(tts.synthesizer.model.parameters()).dtype
                    
                    # Additional verification: check if all parameters are FP16
                    all_fp16 = all(p.dtype == torch.float16 for p in tts.synthesizer.model.parameters() if p.requires_grad)
                    
                    if model_dtype == torch.float16 and all_fp16:
                        fp16_active = True
                        print(f"✅ Modelo configurado para FP16 (half precision)")
                        print(f"   Dtype após conversão: {model_dtype}")
                        print(f"   Todos os parâmetros em FP16: {all_fp16}")
                    else:
                        print(f"⚠️  Modelo não está totalmente em FP16")
                        print(f"   Dtype: {model_dtype}")
                        print(f"   Todos FP16: {all_fp16}")
                        
                        # Try alternative method: convert each module
                        print("   Tentando conversão alternativa...")
                        try:
                            tts.synthesizer.model = tts.synthesizer.model.to(torch.float16).cuda()
                            model_dtype = next(tts.synthesizer.model.parameters()).dtype
                            if model_dtype == torch.float16:
                                fp16_active = True
                                print(f"✅ Conversão alternativa bem-sucedida - dtype: {model_dtype}")
                        except Exception as alt_error:
                            print(f"⚠️  Conversão alternativa falhou: {alt_error}")
            except Exception as e:
                print(f"⚠️  Não foi possível configurar FP16: {e}")
                import traceback
                traceback.print_exc()
        
        # OPTIMIZATION 2: Pre-load speaker embedding (reduces first inference time)
        print("🔧 Pré-carregando speaker embedding...")
        try:
            # XTTS handles speaker embedding internally, but we can trigger it with warm-up
            # The first call will cache the embedding
            pass  # Will be done during warm-up
        except Exception as e:
            print(f"⚠️  Não foi possível pré-carregar embedding: {e}")
        
        # Warm-up (critical for first inference speed)
        print("🔥 Executando warm-up...")
        warmup_start = time.time()
        # Simple: use inference_mode - if model is FP16, it uses FP16 automatically
        with torch.inference_mode():
            _ = tts.tts("Warmup line for TTS", speaker="Ana Florence", language="en")
        warmup_time = time.time() - warmup_start
        print(f"✅ Warm-up concluído em {warmup_time:.3f}s")
        
        # OPTIMIZATION: Clear CUDA cache after warm-up to ensure clean state
        if use_gpu:
            torch.cuda.empty_cache()
            torch.cuda.synchronize()  # Ensure all operations complete
            print("🧹 Cache CUDA limpo e sincronizado")
        
        # Quick FP16 status check (minimal overhead)
        if use_gpu and torch.cuda.is_available():
            try:
                if hasattr(tts, 'synthesizer') and hasattr(tts.synthesizer, 'model'):
                    model_dtype = next(tts.synthesizer.model.parameters()).dtype
                    if model_dtype == torch.float16:
                        print("✅ FP16 ativo")
                    else:
                        print(f"⚠️  FP16 não ativo - dtype: {model_dtype}")
            except:
                pass
        
    except Exception as e:
        print(f"❌ ERRO ao carregar modelo: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Executar teste
    result = test_orchestrator_pipeline(tts, TEXT_1_5B, TEXT_14B)
    
    if result:
        print(f"\n✅ Teste concluído!")
        print(f"   Áudio disponível em: {result['audio_file']}")
        if result['time_to_first_audio']:
            print(f"   Latência: {result['time_to_first_audio']:.3f}s")
    else:
        print("\n❌ Teste falhou")
    
    print("\n" + "="*70 + "\n")

if __name__ == "__main__":
    main()

