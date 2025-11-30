#!/usr/bin/env python3
"""
Teste do XTTS com voz personalizada usando parágrafo de livro
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
    from scipy import signal
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("   Instale scipy: pip install scipy", file=sys.stderr)
    sys.exit(1)

# Fix para PyTorch 2.6+
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load

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


def test_xtts_with_book_paragraph(use_original_embedding=False):
    """Testa XTTS com voz personalizada usando parágrafo de livro"""
    print("\n" + "="*70)
    print("  TESTE: XTTS com Voz Personalizada - Parágrafo de Livro")
    print("="*70 + "\n")
    
    # Escolher qual embedding usar
    if use_original_embedding:
        # Usar embedding original (sem limpeza)
        reference_wav = script_dir / "dungeon_master_en_xtts_reference.wav"
        embedding_type = "ORIGINAL (sem limpeza)"
    else:
        # Usar embedding limpo (padrão)
        reference_wav = script_dir / "dungeon_master_en_xtts_reference_clean.wav"
        embedding_type = "LIMPO (processado)"
        # Fallback para versão antiga se a limpa não existir
        if not reference_wav.exists():
            reference_wav = script_dir / "dungeon_master_en_xtts_reference.wav"
            embedding_type = "ORIGINAL (fallback)"
    
    if not reference_wav.exists():
        print(f"❌ ERRO: Arquivo de referência não encontrado: {reference_wav}")
        print("   Execute primeiro: create_xtts_embedding.py")
        sys.exit(1)
    
    print(f"✅ Arquivo de referência encontrado: {reference_wav.name}")
    print(f"   Tipo: {embedding_type}")
    
    # Parágrafo de exemplo de um livro (fantasia/RPG)
    book_paragraph = """In the depths of the ancient dungeon, shadows danced along the stone walls as torchlight flickered. The air was thick with the scent of damp earth and something else—something that made the hairs on the back of your neck stand on end. You could hear the distant echo of water dripping, each drop a reminder that you were far from the safety of the surface. The corridor stretched before you, disappearing into darkness, and you knew that whatever lay ahead would test not just your strength, but your very resolve."""
    
    print(f"\n📖 Parágrafo de teste ({len(book_paragraph)} caracteres):")
    print("-" * 70)
    print(book_paragraph)
    print("-" * 70)
    
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
    print(f"🎙️  Sintetizando parágrafo com voz personalizada do dungeon master...")
    print(f"   Usando: {reference_wav.name}\n")
    
    try:
        import time
        import re
        
        # Pré-processar texto para melhor segmentação
        # Adicionar espaços após pontuação para melhor divisão
        processed_text = re.sub(r'([.!?])([A-Z])', r'\1 \2', book_paragraph)
        processed_text = re.sub(r'([.!?])\s+', r'\1 ', processed_text)  # Normalizar espaços
        
        print("📝 Texto pré-processado para melhor segmentação")
        print(f"   Original: {len(book_paragraph)} caracteres")
        print(f"   Processado: {len(processed_text)} caracteres\n")
        
        start_time = time.time()
        
        # Usar síntese com melhor controle de segmentação
        # O XTTS internamente divide em sentenças, então vamos garantir que o texto está bem formatado
        audio = tts.tts(
            text=processed_text,
            speaker_wav=str(reference_wav),  # Usar arquivo de referência personalizado
            language="en",
            # Adicionar pequena pausa entre sentenças para evitar sobreposição
            # (parâmetros podem variar dependendo da versão do TTS)
        )
        
        synthesis_time = time.time() - start_time
        
        print(f"✅ Áudio gerado com sucesso!")
        print(f"   - Amostras: {len(audio)}")
        print(f"   - Sample rate: {tts.synthesizer.output_sample_rate} Hz")
        print(f"   - Duração: {len(audio) / tts.synthesizer.output_sample_rate:.2f}s")
        print(f"   - Tempo de síntese: {synthesis_time:.2f}s")
        print(f"   - Real-time factor: {synthesis_time / (len(audio) / tts.synthesizer.output_sample_rate):.2f}x")
        
        # Converter para numpy (evitar múltiplas conversões)
        print("\n💾 Salvando áudio RAW (sem processamento)...")
        print("   ✅ Descoberta: RAW é infinitamente melhor que qualquer processamento!")
        
        # Converter de forma eficiente (evitar múltiplas conversões)
        if isinstance(audio, torch.Tensor):
            audio_np = audio.cpu().numpy().astype(np.float32)  # Converter direto para float32
        elif isinstance(audio, np.ndarray):
            audio_np = audio.astype(np.float32)  # Garantir float32
        else:
            audio_np = np.array(audio, dtype=np.float32)  # Converter para float32
        
        # Garantir que é 1D (sem cópia se já for 1D)
        if len(audio_np.shape) > 1:
            audio_np = audio_np.flatten()
        
        sr = tts.synthesizer.output_sample_rate
        
        # SALVAR APENAS RAW (sem processamento nenhum - é o melhor resultado!)
        from datetime import datetime
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        embedding_suffix = "original" if use_original_embedding else "clean"
        
        # Salvar versão com timestamp
        output_path_timestamped = script_dir / f"test_book_paragraph_xtts_{embedding_suffix}_{timestamp}.wav"
        # Salvar RAW em Float32 (sem quantização, sem processamento)
        sf.write(str(output_path_timestamped), audio_np, sr, subtype='FLOAT')
        
        # Também salvar versão "latest" para referência
        output_path_latest = script_dir / f"test_book_paragraph_xtts_{embedding_suffix}_latest.wav"
        sf.write(str(output_path_latest), audio_np, sr, subtype='FLOAT')
        
        print(f"   ✅ Áudio RAW salvo (Float32, sem processamento)")
        print(f"\n💾 Arquivos salvos:")
        print(f"   - Timestamped: {output_path_timestamped.name}")
        print(f"   - Latest: {output_path_latest.name}")
        print(f"\n📊 Processo aplicado (RAW - MELHOR RESULTADO):")
        print(f"   ✅ SEM processamento (direto do XTTS)")
        print(f"   ✅ SEM filtros (evita delay/robótico)")
        print(f"   ✅ SEM DC offset removal (evita artefatos)")
        print(f"   ✅ SEM fade (evita artefatos)")
        print(f"   ✅ SEM normalização (preserva original)")
        print(f"   ✅ Float32 WAV (sem quantização Int16/Int24)")
        print(f"   ✅ Conversão mínima (Tensor → NumPy float32 direto)")
        print(f"   ✅ Embedding: {embedding_type}")
        print(f"\n🎯 CONCLUSÃO: O XTTS já gera áudio perfeito - processamento só degrada!")
        print(f"\n🎧 Compare com versões anteriores para verificar a melhoria!")
        print("="*70 + "\n")
        
    except Exception as e:
        print(f"❌ ERRO ao sintetizar: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    # Aceitar argumento para usar embedding original
    use_original = False
    if len(sys.argv) > 1:
        if sys.argv[1] in ["--original", "-o", "original"]:
            use_original = True
            print("📌 Usando embedding ORIGINAL (sem limpeza)")
        elif sys.argv[1] in ["--help", "-h", "help"]:
            print("Uso: test_xtts_book_paragraph.py [--original|-o]")
            print("  --original, -o: Usa o embedding original (sem limpeza)")
            print("  (sem argumentos): Usa o embedding limpo (padrão)")
            sys.exit(0)
    
    test_xtts_with_book_paragraph(use_original_embedding=use_original)

