# Modelos TTS (Text-to-Speech)

Este diretório contém os modelos de síntese de voz para o pipeline **Qwen → XTTS**.

## Arquitetura

O VRPG usa uma pipeline de 2 camadas:
1. **Qwen 2.5 14B** → Gera texto com tags VOICE e direção emocional
2. **XTTS v2 (Coqui)** → Síntese direta com voz do personagem usando embeddings personalizados

## Modelos Necessários

### 1. XTTS v2 (Coqui) - Obrigatório

**Modelo PyTorch do Coqui TTS:**

- **XTTS v2**: Baixado automaticamente via Coqui TTS (cache local)
- **Tamanho**: ~1.5GB (baixado automaticamente na primeira execução)
- **Fonte**: https://github.com/coqui-ai/TTS

**Embeddings XTTS (Reference WAVs):**

Cada personagem precisa de um arquivo WAV de referência (embedding):

- `xtts_embeddings/narrator_default_xtts_reference_clean.wav` - Voz do mestre/narrador
- `xtts_embeddings/npc_guard_xtts_reference_clean.wav` - Voz do guarda
- `xtts_embeddings/npc_barkeep_xtts_reference_clean.wav` - Voz do taverneiro
- ... (um embedding por personagem importante)

**Tamanho**: ~5-50MB cada (depende da duração do áudio de referência)
**Criação**: Use `create_clean_xtts_embedding.py` para criar embeddings

## Estrutura Esperada

```
assets-and-models/
└── models/
    └── tts/
        └── xtts_embeddings/
            ├── narrator_default_xtts_reference_clean.wav  # Embedding narrador
            ├── npc_guard_xtts_reference_clean.wav         # Embedding guarda
            └── npc_barkeep_xtts_reference_clean.wav       # Embedding taverneiro
```

## Como Criar Embeddings XTTS

### Passo 1: Coletar Áudio do Personagem

- Colete 5-10 minutos de áudio limpo do personagem
- Formato: WAV, 24kHz ou 44.1kHz, mono ou estéreo
- Qualidade: Áudio limpo, sem ruído de fundo excessivo
- Conteúdo: Fala variada (narrativa, diálogo, emoções diferentes)

### Passo 2: Processar e Criar Embedding

Use o script `create_clean_xtts_embedding.py`:

```bash
cd vrpg-client/src/tts-service/tests/scripts
python create_clean_xtts_embedding.py
```

O script:
- Processa e normaliza os arquivos de áudio
- Remove ruído de fundo
- Normaliza volumes
- Cria um arquivo consolidado `{character_id}_xtts_reference_clean.wav`

### Passo 3: Salvar Embedding

- Copie o arquivo gerado para: `assets-and-models/models/tts/xtts_embeddings/`
- Renomeie para: `{character_id}_xtts_reference_clean.wav`

### Exemplo Completo

```bash
# 1. Coletar áudio do personagem (ex: dungeon_master_en)
#    Arquivos em: dataset/44k/dungeon_master_en/

# 2. Criar embedding
cd vrpg-client/src/tts-service/tests/scripts
python create_clean_xtts_embedding.py

# 3. Copiar para local correto
cp dungeon_master_en_xtts_reference_clean.wav \
   ../../../../assets-and-models/models/tts/xtts_embeddings/narrator_default_xtts_reference_clean.wav
```

## Qualidade do Áudio

**IMPORTANTE: Use áudio RAW (sem processamento)**

O XTTS já gera áudio de alta qualidade. Processamento adicional (filtros, normalização excessiva, etc.) **degradam a qualidade**.

### Processo Recomendado:

1. **Criar embedding limpo** (usando `create_clean_xtts_embedding.py`)
   - Processa e normaliza o dataset
   - Remove ruído de fundo
   - Padroniza volumes

2. **Síntese XTTS** (usar embedding)
   - XTTS gera áudio diretamente com voz do personagem
   - **Salvar em Float32 WAV (sem processamento adicional)**
   - **Não aplicar filtros, DC offset removal, fade, etc.**

3. **Resultado**: Áudio perfeito, natural, sem artefatos

## Personagens Prioritários para Criar Embeddings

Se você vai criar embeddings, comece por estes personagens mais importantes:

1. **Narrador/Mestre** (`narrator_default`)
2. **Vilão Principal** (`villain_primary`)
3. **Companheiros Fixos** (`companion_1`, `companion_2`, etc.)
4. **NPCs Recorrentes** (`npc_guard`, `npc_barkeep`, etc.)

NPCs menores podem compartilhar embeddings similares ou usar um embedding genérico.

## Configuração

O caminho dos embeddings é configurado em:
- `config/vrpg.json` → `services.tts_service.xtts_embeddings_path`

## Fallback

Se um embedding não estiver disponível para um personagem:
- O sistema usa o embedding padrão (`narrator_default`)
- Ou usa voz padrão do XTTS (sem embedding)

## Recursos

- **Documentação Completa**: `docs/AUDIO_PIPELINE.md`
- **Script de Criação**: `src/tts-service/tests/scripts/create_clean_xtts_embedding.py`
- **Teste de Embedding**: `src/tts-service/tests/scripts/test_xtts_book_paragraph.py`
- **Descoberta RAW**: `src/tts-service/tests/scripts/DESCOBERTA_RAW.md`

## Notas Técnicas

### Por que XTTS ao invés de SoVITS?

- **Qualidade superior**: Áudio RAW do XTTS é infinitamente melhor
- **Latência menor**: Síntese direta sem camadas adicionais
- **Mais simples**: Apenas um arquivo WAV por personagem (vs modelo treinado)
- **Escalável**: Fácil criar novos embeddings
- **Sem artefatos**: Processamento adicional degrada qualidade

### Formato dos Embeddings

- **Formato**: WAV, Float32 ou Int16
- **Sample Rate**: 24kHz (recomendado) ou 44.1kHz
- **Canais**: Mono ou estéreo (XTTS converte automaticamente)
- **Duração**: 30-60 segundos ideal (pode ser mais longo)

### Performance

- **Síntese XTTS**: 50-200ms (dependendo do hardware e GPU)
- **Latência total**: 350-800ms (prelúdio), 1.8-4.7s (narrativa completa)
- **GPU recomendada**: Para latência < 1s
