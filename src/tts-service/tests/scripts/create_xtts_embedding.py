#!/usr/bin/env python3
"""
Script para criar arquivo de referência de voz personalizado do XTTS usando dataset do dungeon master
O XTTS v2 aceita um arquivo WAV de referência (speaker_wav) para clonar a voz.
Este script cria um arquivo WAV consolidado com os melhores exemplos do dataset.
"""

import sys
import os
import time
from pathlib import Path
import numpy as np
import torch

# Aceitar termos de serviço do Coqui TTS
os.environ["COQUI_TOS_AGREED"] = "1"

# Adicionar o diretório do SoVITS ao path
script_dir = Path(__file__).parent  # tests/scripts/
tests_dir = script_dir.parent  # tests/
tts_service_dir = tests_dir.parent  # tts-service/
vrpg_client_dir = tts_service_dir.parent.parent  # vrpg-client/
sovits_dir = vrpg_client_dir / "assets-and-models" / "models" / "tts" / "sovits"
dataset_dir = sovits_dir / "dataset" / "44k" / "dungeon_master_en"
sys.path.insert(0, str(sovits_dir))

try:
    from TTS.api import TTS
    import soundfile as sf
except ImportError as e:
    print(f"❌ ERRO: Dependências não encontradas: {e}", file=sys.stderr)
    print("Certifique-se de que está no ambiente virtual do SoVITS", file=sys.stderr)
    print("E que o Coqui TTS está instalado: pip install TTS", file=sys.stderr)
    sys.exit(1)

# Fix para PyTorch 2.6+
original_load = torch.load
def patched_load(*args, **kwargs):
    if 'weights_only' not in kwargs:
        kwargs['weights_only'] = False
    return original_load(*args, **kwargs)
torch.load = patched_load


def find_wav_files(dataset_path: Path) -> list[Path]:
    """Encontra todos os arquivos WAV no dataset"""
    wav_files = list(dataset_path.glob("*.wav"))
    # Filtrar arquivos que não são processados pelo SoVITS (sem .soft.pt)
    # Mas para XTTS, queremos todos os WAVs
    return sorted(wav_files)


def load_and_preprocess_audio_from_array(audio: np.ndarray, sr: int, target_sr: int = 24000) -> tuple[np.ndarray, int]:
    """Pré-processa áudio já carregado"""
    # Converter para mono se necessário
    if len(audio.shape) > 1:
        audio = np.mean(audio, axis=1)
    
    # Resample se necessário
    if sr != target_sr:
        try:
            from scipy import signal
            num_samples = int(len(audio) * target_sr / sr)
            audio = signal.resample(audio, num_samples)
            sr = target_sr
        except ImportError:
            try:
                import librosa
                audio = librosa.resample(audio, orig_sr=sr, target_sr=target_sr)
                sr = target_sr
            except ImportError:
                pass  # Continuar com sample rate original
    
    # Normalizar
    if audio.max() > 1.0 or audio.min() < -1.0:
        audio = audio / np.max(np.abs(audio))
    
    return audio, sr


def load_and_preprocess_audio(wav_path: Path, target_sr: int = 24000) -> tuple[np.ndarray, int]:
    """
    Carrega e pré-processa áudio para XTTS
    XTTS espera 24kHz, mono
    """
    try:
        audio, sr = sf.read(str(wav_path))
        
        # Converter para mono se necessário
        if len(audio.shape) > 1:
            audio = np.mean(audio, axis=1)
        
        # Resample se necessário (usando scipy se disponível)
        if sr != target_sr:
            try:
                from scipy import signal
                num_samples = int(len(audio) * target_sr / sr)
                audio = signal.resample(audio, num_samples)
                sr = target_sr
            except ImportError:
                # Se scipy não estiver disponível, tentar com librosa
                try:
                    import librosa
                    audio = librosa.resample(audio, orig_sr=sr, target_sr=target_sr)
                    sr = target_sr
                except ImportError:
                    print(f"⚠️  Aviso: scipy/librosa não disponível, pulando resample de {wav_path.name}")
                    print(f"   Sample rate: {sr} Hz (esperado: {target_sr} Hz)")
                    # Continuar mesmo assim - XTTS pode aceitar outros sample rates
        
        # Normalizar para [-1, 1]
        if audio.max() > 1.0 or audio.min() < -1.0:
            audio = audio / np.max(np.abs(audio))
        
        return audio, sr
    except Exception as e:
        print(f"⚠️  Erro ao carregar {wav_path.name}: {e}")
        return None, None


def select_best_reference_audio(wav_files: list[Path], max_duration: float = 30.0) -> list[Path]:
    """
    Seleciona os melhores arquivos de áudio para usar como referência
    Prioriza arquivos com fala clara e duração adequada
    """
    selected = []
    total_duration = 0.0
    
    # Priorizar arquivos com nomes que indicam fala clara
    # Ordem de prioridade: textos padrão > conversação > episódios > outros
    priority_keywords = [
        "Rainbow Passage",  # Texto padrão para TTS (melhor para referência)
        "Core Interaction",  # Frases principais
        "ChatB",  # Conversação
        "Conv",  # Conversação
        "Doc",  # Documentação/narração
        "Episode",  # Episódios completos
    ]
    
    # Excluir arquivos que não são fala normal
    exclude_keywords = [
        "yell", "yelling", "cry", "crying", "breath", "sigh", "yawn",
        "cough", "laugh", "whistle", "hum", "sneeze", "snore", "sniff",
        "swallow", "vocalize", "scales", "instruments", "hold", "prompt"
    ]
    
    # Filtrar e ordenar por prioridade
    def should_exclude(wav_path: Path) -> bool:
        name_lower = wav_path.name.lower()
        # Verificar se qualquer palavra de exclusão está no nome (substring match)
        for exclude in exclude_keywords:
            if exclude.lower() in name_lower:
                return True
        return False
    
    def should_include(wav_path: Path) -> bool:
        """Verifica se o arquivo deve ser incluído (tem palavras-chave de prioridade)"""
        name_lower = wav_path.name.lower()
        for keyword in priority_keywords:
            if keyword.lower() in name_lower:
                return True
        return False
    
    def get_priority(wav_path: Path) -> tuple[int, float]:
        """Retorna (prioridade, duração) para ordenação"""
        name_lower = wav_path.name.lower()
        priority = len(priority_keywords)
        for i, keyword in enumerate(priority_keywords):
            if keyword.lower() in name_lower:
                priority = i
                break
        
        # Tentar obter duração para ordenação
        try:
            audio, sr = sf.read(str(wav_path))
            duration = len(audio) / sr
        except:
            duration = 0.0
        
        return (priority, -duration)  # Negativo para ordenar do maior para menor
    
    # Filtrar arquivos: priorizar os com palavras-chave, mas aceitar outros se necessário
    priority_files = [f for f in wav_files if should_include(f) and not should_exclude(f)]
    other_files = [f for f in wav_files if not should_include(f) and not should_exclude(f)]
    
    # Combinar: primeiro os com prioridade, depois os outros
    filtered_files = priority_files + other_files
    
    sorted_files = sorted(filtered_files, key=get_priority)
    
    for wav_path in sorted_files:
        if total_duration >= max_duration:
            break
        
        try:
            audio, sr = sf.read(str(wav_path))
            duration = len(audio) / sr
            
            # Filtrar: duração entre 5 e 30 segundos (ideal para XTTS)
            if 5.0 <= duration <= 30.0:
                selected.append(wav_path)
                total_duration += duration
        except:
            continue
    
    return selected


def get_selected_files() -> list[Path]:
    """
    Retorna lista específica de arquivos selecionados pelo usuário
    Ordenados por prioridade: fala normal > variações vocais
    """
    selected = []
    
    # Prioridade 1: Arquivos de fala normal (Rainbow Passage, Core Phrases, etc.)
    priority_files = [
        "NewsP - Rainbow Passage.wav",
        "ChatB - Core Interaction Phrases.wav",
        "ChatB - Exclamations.wav",
        "NewsP - Exclamations.wav",
        "NewsP - Hesitations.wav",
        "Episode 1.wav",
        "Episode 2.wav",
        "Victor - Whispering - pitched - T2.wav",
    ]
    
    # Prioridade 2: Todos os Prompts (01-20)
    prompt_files = [f"Prompt-{i:02d}.wav" for i in range(1, 21)]
    
    # Prioridade 3: Todos os arquivos Conv
    conv_files = [
        "Conv - Core Phrases.wav",
        "Conv - Digits and special Symbols - Other Numbers.wav",
        "Conv - Digits and special Symbols - Item numbers.wav",
        "Conv - Digits and special Symbols - Large Numbers.wav",
        "Conv - Digits and special Symbols - Phone Numbers.wav",
        "Conv - Digits and special Symbols - confirmation code.wav",
        "Conv - Digits and special Symbols - Symbols.wav",
        "Conv - Exclamations.wav",
        "Conv - Hesitation.wav",
        "Conv - Questions.wav",
        "Conv - Rainbow Passage.wav",
    ]
    
    # Prioridade 4: Variações vocais (respiração, risos, gritos, etc.)
    vocal_variations = [
        "1 breath mouth.wav",
        "1. Triumphant, celebratory yelling.wav",
        "2. Painful Yell.wav",
        "3 sighs.wav",
        "3. Warning Yell.wav",
        "4 yawns.wav",
        "4. Battle cries & Commands.wav",
        "5 laughter.wav",
        "5. Angry yell.wav",
        "6 coughing.wav",
        "7 crying.wav",
        "9 humming mouth closed.wav",
        "10 vocalize .wav",
        "11 sneezing.wav",
        "12 snoring.wav",
        "13 sniffing.wav",
        "14 swallowing lip smacking.wav",
    ]
    
    # Combinar todas as listas na ordem de prioridade
    all_files = priority_files + prompt_files + conv_files + vocal_variations
    
    # Adicionar arquivos na ordem de prioridade
    for filename in all_files:
        file_path = dataset_dir / filename
        if file_path.exists():
            selected.append(file_path)
        else:
            print(f"⚠️  Arquivo não encontrado: {filename}")
    
    return selected


def create_xtts_reference_audio():
    """Cria arquivo WAV de referência consolidado para XTTS usando arquivos específicos do dataset"""
    print("\n" + "="*70)
    print("  CRIAÇÃO DE ARQUIVO DE REFERÊNCIA XTTS")
    print("  Dataset: dungeon_master_en (Arquivos Selecionados)")
    print("="*70 + "\n")
    
    # Verificar se o dataset existe
    if not dataset_dir.exists():
        print(f"❌ ERRO: Dataset não encontrado: {dataset_dir}")
        sys.exit(1)
    
    # Obter lista de arquivos selecionados
    print("🔍 Carregando lista de arquivos selecionados...")
    selected_files = get_selected_files()
    
    if not selected_files:
        print(f"❌ ERRO: Nenhum arquivo selecionado encontrado!")
        sys.exit(1)
    
    print(f"✅ Encontrados {len(selected_files)} arquivos selecionados")
    print(f"   Diretório: {dataset_dir}\n")
    
    if not selected_files:
        print("❌ ERRO: Nenhum arquivo adequado encontrado!")
        sys.exit(1)
    
    print(f"✅ Selecionados {len(selected_files)} arquivos para referência\n")
    
    # Carregar e consolidar áudios
    print("🔄 Carregando e consolidando áudios...\n")
    audio_segments = []
    target_sr = 24000  # XTTS espera 24kHz
    successful = 0
    failed = 0
    total_duration = 0.0
    
    # Limite máximo de duração total (15 minutos = 900 segundos)
    # XTTS funciona melhor com referências de 1-15 minutos
    max_total_duration = 900.0  # 15 minutos
    max_segment_duration = 20.0  # Limitar segmentos individuais a 20s (melhor distribuição)
    
    for i, wav_path in enumerate(selected_files, 1):
        if total_duration >= max_total_duration:
            print(f"\n⚠️  Limite de duração atingido ({max_total_duration:.0f}s). Parando...")
            break
        
        print(f"[{i}/{len(selected_files)}] Processando: {wav_path.name}...", end=" ", flush=True)
        
        try:
            audio, sr = load_and_preprocess_audio(wav_path, target_sr)
            
            if audio is None:
                print("❌ (erro ao carregar)")
                failed += 1
                continue
            
            duration = len(audio) / sr
            
            # Se o segmento for muito longo, usar apenas os primeiros N segundos
            if duration > max_segment_duration:
                max_samples = int(max_segment_duration * target_sr)
                audio = audio[:max_samples]
                duration = len(audio) / target_sr
                print(f"✂️  Cortado para {duration:.1f}s", end=" ")
            
            # Verificar se ainda cabe no limite total
            if total_duration + duration > max_total_duration:
                remaining = max_total_duration - total_duration
                if remaining > 10.0:  # Só adicionar se sobrar pelo menos 10s
                    max_samples = int(remaining * target_sr)
                    audio = audio[:max_samples]
                    duration = len(audio) / target_sr
                    print(f"✂️  Ajustado para {duration:.1f}s (limite total)", end=" ")
                else:
                    print(f"⚠️  Pulado (limite atingido)")
                    break
            
            audio_segments.append(audio)
            total_duration += duration
            successful += 1
            print(f"✅ ({duration:.1f}s, total: {total_duration/60:.1f}min)")
            
        except Exception as e:
            print(f"❌ (erro: {str(e)[:50]})")
            failed += 1
            continue
    
    print(f"\n📊 Resultado do carregamento:")
    print(f"   ✅ Sucesso: {successful}/{len(selected_files)}")
    print(f"   ❌ Falhas: {failed}/{len(selected_files)}")
    
    if not audio_segments:
        print("\n❌ ERRO: Nenhum áudio foi carregado com sucesso!")
        sys.exit(1)
    
    # Concatenar áudios com crossfade suave para evitar cliques/pops
    print(f"\n🔄 Concatenando {len(audio_segments)} segmentos de áudio com crossfade...")
    
    def apply_crossfade(audio1, audio2, fade_duration=0.1, sr=target_sr):
        """
        Aplica crossfade suave entre dois segmentos de áudio para evitar cliques
        """
        fade_samples = int(sr * fade_duration)
        
        # Garantir que temos espaço suficiente para o crossfade
        if len(audio1) < fade_samples or len(audio2) < fade_samples:
            # Se algum segmento for muito curto, usar apenas concatenação simples
            return np.concatenate([audio1, audio2])
        
        # Criar curvas de fade (linear)
        fade_out = np.linspace(1.0, 0.0, fade_samples)
        fade_in = np.linspace(0.0, 1.0, fade_samples)
        
        # Aplicar fade out no final do primeiro áudio
        audio1_faded = audio1.copy()
        audio1_faded[-fade_samples:] *= fade_out
        
        # Aplicar fade in no início do segundo áudio
        audio2_faded = audio2.copy()
        audio2_faded[:fade_samples] *= fade_in
        
        # Combinar: parte sem fade do audio1 + parte com fade sobreposta + parte sem fade do audio2
        result = np.concatenate([
            audio1_faded[:-fade_samples],  # Parte sem fade do audio1
            audio1_faded[-fade_samples:] + audio2_faded[:fade_samples],  # Crossfade sobreposto
            audio2_faded[fade_samples:]  # Parte sem fade do audio2
        ])
        
        return result
    
    # Normalizar cada segmento antes de concatenar (evita cliques por diferença de amplitude)
    print("   🔧 Normalizando segmentos individuais...")
    normalized_segments = []
    for i, segment in enumerate(audio_segments):
        # Normalizar para evitar clipping
        max_val = np.max(np.abs(segment))
        if max_val > 0:
            # Normalizar para 0.95 (deixa margem para evitar clipping)
            segment = segment * (0.95 / max_val)
        normalized_segments.append(segment)
    
    # Concatenar com crossfade
    consolidated_audio = normalized_segments[0]
    for i, segment in enumerate(normalized_segments[1:], 1):
        consolidated_audio = apply_crossfade(consolidated_audio, segment, fade_duration=0.1, sr=target_sr)
        if (i + 1) % 10 == 0:
            print(f"   ✅ Processados {i + 1}/{len(normalized_segments)} segmentos...")
    
    # Normalização final e remoção de DC offset
    print("\n🔧 Aplicando processamento final...")
    
    # Remover DC offset (componente DC pode causar cliques)
    consolidated_audio = consolidated_audio - np.mean(consolidated_audio)
    
    # Não aplicar filtros agressivos - manter áudio natural
    # Os filtros estavam deixando o áudio muito abafado
    
    # Normalização final (evita clipping)
    max_val = np.max(np.abs(consolidated_audio))
    if max_val > 0:
        consolidated_audio = consolidated_audio * (0.95 / max_val)
    
    # Aplicar fade in/out suave nas extremidades (evita cliques no início/fim)
    fade_samples = int(target_sr * 0.05)  # 50ms de fade
    if len(consolidated_audio) > fade_samples * 2:
        fade_curve = np.linspace(0.0, 1.0, fade_samples)
        consolidated_audio[:fade_samples] *= fade_curve
        consolidated_audio[-fade_samples:] *= np.flip(fade_curve)
    
    final_duration = len(consolidated_audio) / target_sr
    print(f"✅ Áudio consolidado criado e processado!")
    print(f"   Duração total: {final_duration:.2f}s ({final_duration/60:.1f} minutos)")
    print(f"   Sample rate: {target_sr} Hz")
    print(f"   Amostras: {len(consolidated_audio)}")
    print(f"   Normalizado: Sim (0.95 peak)")
    print(f"   Crossfade: Sim (100ms entre segmentos)")
    print(f"   DC offset removido: Sim")
    
    # Salvar arquivo WAV de referência
    output_dir = script_dir
    output_path = output_dir / "dungeon_master_en_xtts_reference.wav"
    
    print(f"\n💾 Salvando arquivo de referência em: {output_path}")
    sf.write(str(output_path), consolidated_audio, target_sr)
    
    # Estatísticas finais
    print("\n" + "="*70)
    print("  RESULTADO FINAL")
    print("="*70)
    print(f"✅ Arquivo de referência criado com sucesso!")
    print(f"✅ Arquivo salvo: {output_path.name}")
    print(f"✅ Duração: {total_duration:.2f}s")
    print(f"✅ Sample rate: {target_sr} Hz")
    print(f"✅ Baseado em {len(audio_segments)} segmentos de áudio")
    print(f"\n📝 Para usar este arquivo com XTTS:")
    print(f"   tts = TTS('tts_models/multilingual/multi-dataset/xtts_v2')")
    print(f"   audio = tts.tts(")
    print(f"       text='Seu texto aqui',")
    print(f"       speaker_wav='{output_path.name}',  # Use este arquivo")
    print(f"       language='en'")
    print(f"   )")
    print(f"\n📋 Arquivos incluídos no embedding:")
    print(f"   - {successful} arquivos de áudio consolidados")
    print(f"   - Inclui: Victor Whispering, Prompts (20), NewsP, Episodes, Conv, ChatB,")
    print(f"     e variações vocais (respiração, risos, gritos, etc.)")
    print("="*70 + "\n")


if __name__ == "__main__":
    create_xtts_reference_audio()

