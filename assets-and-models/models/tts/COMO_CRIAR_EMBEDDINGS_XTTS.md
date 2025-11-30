# Como Criar Embeddings XTTS

## 📋 Visão Geral

Embeddings XTTS são arquivos WAV de referência que definem a voz de cada personagem. Cada personagem precisa de um embedding único para ter sua voz característica.

## 🎯 Por que Embeddings XTTS?

- **Qualidade superior**: Áudio RAW do XTTS é infinitamente melhor que processamento adicional
- **Latência menor**: Síntese direta sem camadas adicionais
- **Mais simples**: Apenas um arquivo WAV por personagem (vs modelo treinado complexo)
- **Escalável**: Fácil criar novos embeddings
- **Sem artefatos**: Processamento adicional degrada qualidade

## 📝 Pré-requisitos

1. **Áudio do personagem** (5-10 minutos de fala limpa)
2. **Python 3.10+** com dependências instaladas
3. **Scripts de criação** (`create_clean_xtts_embedding.py`)

## 🚀 Passo a Passo

### Passo 1: Coletar Áudio do Personagem

**Requisitos do áudio:**
- **Duração**: 5-10 minutos total (pode ser múltiplos arquivos)
- **Formato**: WAV (recomendado), MP3 também funciona
- **Sample Rate**: 24kHz ou 44.1kHz (será re-amostrado se necessário)
- **Canais**: Mono ou estéreo (será convertido se necessário)
- **Qualidade**: Áudio limpo, sem ruído de fundo excessivo
- **Conteúdo**: Fala variada (narrativa, diálogo, emoções diferentes)

**Onde colocar:**
- Coloque os arquivos WAV em uma pasta (ex: `dataset/44k/dungeon_master_en/`)

### Passo 2: Selecionar Arquivos para o Embedding

Escolha os melhores arquivos que representam a voz do personagem:

**Prioridade:**
1. **Rainbow Passage** (se disponível) - texto padrão para TTS
2. **Narrativa clara** - fala natural e expressiva
3. **Diálogo variado** - diferentes emoções e estilos
4. **Evite**: Sons não-fala (respiração, tosse, etc.) a menos que sejam importantes

**Exemplo de seleção:**
```
✅ Prompt-01.wav até Prompt-20.wav (narrativa)
✅ NewsP - Rainbow Passage.wav (texto padrão)
✅ Episode 1.wav, Episode 2.wav (narrativa longa)
❌ Evitar sons muito curtos ou não-fala
```

### Passo 3: Criar Embedding

Use o script `create_clean_xtts_embedding.py`:

```bash
cd vrpg-client/src/tts-service/tests/scripts
python create_clean_xtts_embedding.py
```

**O que o script faz:**
1. Processa cada arquivo individualmente:
   - Remove DC offset
   - Aplica filtros high-pass (80Hz) e low-pass (15kHz)
   - Reduz ruído de fundo
   - Normaliza RMS e peak
   - Re-amostra para 24kHz

2. Corta arquivos longos (máximo 30s por segmento)

3. Consolida todos os segmentos:
   - Aplica crossfade entre segmentos (100ms)
   - Salva como `{character_id}_xtts_reference_clean.wav`

### Passo 4: Salvar Embedding

Copie o arquivo gerado para o local correto:

```bash
# Exemplo: Narrador
cp dungeon_master_en_xtts_reference_clean.wav \
   ../../../../assets-and-models/models/tts/xtts_embeddings/narrator_default_xtts_reference_clean.wav

# Exemplo: Guarda
cp npc_guard_xtts_reference_clean.wav \
   ../../../../assets-and-models/models/tts/xtts_embeddings/npc_guard_xtts_reference_clean.wav
```

**Estrutura final:**
```
assets-and-models/models/tts/xtts_embeddings/
├── narrator_default_xtts_reference_clean.wav
├── npc_guard_xtts_reference_clean.wav
└── npc_barkeep_xtts_reference_clean.wav
```

### Passo 5: Testar Embedding

Use o script de teste:

```bash
cd vrpg-client/src/tts-service/tests/scripts
python test_xtts_book_paragraph.py
```

Ou teste com embedding específico:

```python
# No código
tts.tts(
    text="Teste de voz do personagem",
    speaker_wav="path/to/embedding.wav",
    language="en"
)
```

## 🎨 Dicas para Melhor Qualidade

### 1. Seleção de Arquivos

- **Prefira**: Fala natural, clara, expressiva
- **Evite**: Áudio com muito ruído, clipping, ou distorção
- **Variedade**: Inclua diferentes emoções e estilos de fala

### 2. Processamento

- **Use o script**: `create_clean_xtts_embedding.py` já faz o processamento correto
- **Não processe manualmente**: O script já normaliza e limpa o áudio

### 3. Duração

- **Mínimo**: 30 segundos (funciona, mas qualidade menor)
- **Ideal**: 2-5 minutos (melhor qualidade)
- **Máximo**: 10 minutos (não há ganho significativo além disso)

### 4. Qualidade do Áudio Original

- **Sample Rate**: 24kHz ou 44.1kHz (será re-amostrado se necessário)
- **Bit Depth**: 16-bit ou 24-bit (Float32 também funciona)
- **Canais**: Mono ou estéreo (será convertido se necessário)

## 🔧 Troubleshooting

### Embedding não funciona

- Verifique se o arquivo existe no caminho correto
- Verifique se o formato é WAV válido
- Tente regenerar o embedding com menos arquivos

### Qualidade ruim

- Use mais arquivos de melhor qualidade
- Verifique se os arquivos originais não têm ruído excessivo
- Tente diferentes combinações de arquivos

### Áudio muito longo

- O script corta automaticamente para 30s por segmento
- Se o embedding final for muito longo, edite o script para reduzir

## 📚 Recursos

- **Script de Criação**: `src/tts-service/tests/scripts/create_clean_xtts_embedding.py`
- **Script de Teste**: `src/tts-service/tests/scripts/test_xtts_book_paragraph.py`
- **Documentação**: `docs/AUDIO_PIPELINE.md`
- **Descoberta RAW**: `src/tts-service/tests/scripts/DESCOBERTA_RAW.md`

## ✅ Checklist

- [ ] Áudio coletado (5-10 minutos)
- [ ] Arquivos selecionados (melhor qualidade)
- [ ] Embedding criado (`create_clean_xtts_embedding.py`)
- [ ] Embedding salvo em `xtts_embeddings/`
- [ ] Embedding testado (`test_xtts_book_paragraph.py`)
- [ ] Qualidade verificada (áudio natural, sem artefatos)

---

**Última atualização**: 2025-11-28



