# Estratégias de Otimização de Latência - XTTS Streaming

**Data**: 2025-11-29  
**Status**: ✅ Implementadas / ⚠️ Em otimização

---

## 📊 Problema Atual

- **Latência inicial**: 2.5-3.0s (target: ≤ 0.8s para FAST)
- **Primeiro chunk**: 39 chars → 2.5-3.0s de áudio (RTF ~1.2-1.3x)
- **Repetição entre chunks**: "otherworldly light" repetido (problema de overlap)

---

## ✅ Estratégias Implementadas

### 1. Chunker Inteligente com Pontuação

**Status**: ✅ Implementado

- **Primeiro chunk**: Vai até primeira vírgula/ponto (nunca corta no meio)
- **Chunks subsequentes**: Sempre procura pontuação antes de cortar
- **Após 5s de áudio**: Respeita limites de frase (natural pauses)
- **Busca estendida**: Procura pontuação até 2x o limite se necessário
- **Warning**: Alerta se chunk for finalizado sem pontuação

**Código**: `split_text_for_tts()` com lógica de busca de pontuação

### 2. FP16 (Half Precision)

**Status**: ✅ Implementado (parcialmente)

- **Autocast**: `torch.cuda.amp.autocast()` durante inferência
- **Model conversion**: Tentativa de converter modelo para `.half().cuda()`
- **Problema**: Pode não estar totalmente ativo (verificar)

**Código**: `synthesize_with_profile()` com `use_fp16=True`

### 3. Warm-up na Inicialização

**Status**: ✅ Implementado

- **Execução**: Uma inferência curta ao carregar modelo
- **Objetivo**: "Compilar" kernels CUDA antes do primeiro uso real
- **Tempo**: ~2-3s (aceitável, só acontece uma vez)

**Código**: `main()` - warm-up após carregar modelo

### 4. Pre-buffer Reduzido

**Status**: ✅ Implementado

- **FAST profile**: 200ms (reduzido de 240ms)
- **CINEMATIC profile**: 500ms
- **Objetivo**: Começar playback mais rápido

**Código**: `TtsProfile.fast()` - `initial_prebuffer_ms: 200`

### 5. Blocos de Áudio Menores

**Status**: ✅ Implementado

- **FAST profile**: 25ms (reduzido de 50ms)
- **CINEMATIC profile**: 60ms
- **Objetivo**: Streaming mais fino, menor latência percebida

**Código**: `TtsProfile.fast()` - `audio_block_ms: 25`

### 6. Primeiro Chunk Otimizado

**Status**: ✅ Implementado

- **Tamanho**: 20 chars (reduzido de 30)
- **Estratégia**: Vai até primeira vírgula (39 chars no exemplo)
- **Objetivo**: Gerar menos áudio no primeiro chunk

**Código**: `TtsProfile.fast()` - `first_chunk_max_chars: 20`

### 7. Limpeza de Cache CUDA

**Status**: ✅ Implementado

- **Entre chunks**: `torch.cuda.empty_cache()` após cada chunk
- **Após warm-up**: Limpeza e sincronização
- **Objetivo**: Evitar acúmulo de memória que pode causar lentidão

**Código**: Após cada `synthesize_with_profile()`

---

## ⚠️ Problemas Identificados

### 1. Repetição entre Chunks

**Sintoma**: "otherworldly light" repetido no final/início de chunks

**Causa possível**:
- Overlap na concatenação de blocos
- Chunks sendo duplicados
- Problema na lógica de split de blocos

**Solução proposta**:
- Verificar se há overlap na concatenação
- Garantir que blocos não se sobreponham
- Validar que cada chunk é único

### 2. FP16 Não Totalmente Ativo

**Sintoma**: RTF ainda acima de 1.0x (1.2-1.3x)

**Causa possível**:
- Modelo não está em half precision
- Autocast não está sendo aplicado corretamente
- XTTS pode não suportar FP16 completamente

**Solução proposta**:
- Verificar se modelo está realmente em FP16
- Usar `torch.compile()` para otimizar
- Verificar se XTTS suporta FP16 nativamente

### 3. Primeiro Chunk Ainda Gera Muito Áudio

**Sintoma**: 39 chars geram 2.5-3.0s de áudio (deveria ser ~1s)

**Causa possível**:
- XTTS adiciona pausas/padding
- Modelo não está otimizado
- Sample rate ou configuração incorreta

**Solução proposta**:
- Verificar configuração de sample rate
- Reduzir ainda mais primeiro chunk (10-15 chars)
- Usar texto mais curto para primeiro chunk

---

## 🚀 Estratégias Adicionais Propostas

### 1. Torch Compile (JIT Compilation)

**Objetivo**: Compilar modelo para acelerar primeira inferência

```python
# Compilar modelo após warm-up
if hasattr(tts.synthesizer, 'model'):
    tts.synthesizer.model = torch.compile(tts.synthesizer.model, mode="reduce-overhead")
```

**Benefício esperado**: 20-30% redução na primeira inferência

### 2. Pre-load Speaker Embedding

**Objetivo**: Cachear embedding do speaker antes do primeiro uso

```python
# Pre-load speaker WAV embedding
speaker_wav_path = "path/to/speaker.wav"
tts.synthesizer.speaker_manager.compute_speaker_embedding(speaker_wav_path)
```

**Benefício esperado**: 100-200ms redução na primeira inferência

### 3. CUDA Streams Paralelos

**Objetivo**: Usar múltiplos CUDA streams para paralelizar

```python
# Criar streams separados para diferentes operações
stream1 = torch.cuda.Stream()
stream2 = torch.cuda.Stream()
```

**Benefício esperado**: Redução de latência em GPUs high-end

### 4. Model Quantization (INT8)

**Objetivo**: Quantizar modelo para INT8 (mais agressivo que FP16)

```python
# Quantizar modelo para INT8
model = torch.quantization.quantize_dynamic(model, {torch.nn.Linear}, dtype=torch.qint8)
```

**Benefício esperado**: 40-50% redução de latência (com possível perda de qualidade)

### 5. Text Pre-processing Otimizado

**Objetivo**: Reduzir tamanho do texto antes de enviar para XTTS

- Remover espaços extras
- Normalizar pontuação
- Pré-processar para reduzir tokens

**Benefício esperado**: 10-20% redução de latência

### 6. Streaming com Primeiro Bloco Mínimo

**Objetivo**: Começar playback com apenas 1-2 blocos (50-100ms)

```python
# Reduzir pre-buffer para mínimo absoluto
initial_prebuffer_ms = 100  # Apenas 2 blocos de 50ms
```

**Benefício esperado**: Redução de 100-150ms na latência inicial

### 7. Model Caching e Pre-loading

**Objetivo**: Manter modelo sempre em memória GPU

```python
# Manter modelo em GPU após carregamento
model = model.cuda()
torch.cuda.empty_cache()  # Limpar apenas cache, não modelo
```

**Benefício esperado**: Eliminar latência de carregamento

### 8. Batch Processing Otimizado

**Objetivo**: Processar múltiplos chunks pequenos em batch

```python
# Agrupar chunks pequenos para processar juntos
small_chunks = [chunk1, chunk2, chunk3]
batch_audio = tts.batch_synthesize(small_chunks)
```

**Benefício esperado**: Melhor utilização de GPU

---

## 📋 Checklist de Verificação

### Chunker
- [x] Primeiro chunk vai até primeira vírgula
- [x] Chunks subsequentes procuram pontuação
- [x] Busca estendida se pontuação não encontrada próxima
- [ ] Validação de não-overlap entre chunks
- [ ] Log de chunks para debug

### FP16
- [x] Autocast implementado
- [ ] Modelo realmente em half precision (verificar)
- [ ] Verificar se XTTS suporta FP16
- [ ] Medir ganho real de FP16

### Warm-up
- [x] Warm-up executado na inicialização
- [x] Warm-up usa FP16
- [ ] Warm-up com texto similar ao primeiro chunk real
- [ ] Verificar se warm-up está realmente compilando kernels

### Pre-buffer
- [x] Pre-buffer reduzido para 200ms (FAST)
- [ ] Testar com pre-buffer ainda menor (100ms)
- [ ] Validar que não causa underrun

### Blocos de Áudio
- [x] Blocos reduzidos para 25ms (FAST)
- [ ] Verificar se não causa overhead de processamento
- [ ] Validar continuidade entre blocos

### Otimizações Adicionais
- [ ] Torch compile implementado
- [ ] Speaker embedding pré-carregado
- [ ] CUDA streams paralelos
- [ ] Model quantization (se necessário)
- [ ] Text pre-processing
- [ ] Model caching otimizado

---

## 🎯 Targets de Performance

### FAST Profile (Qwen 1.5B)
- **Target atual**: ≤ 0.8s `time_to_first_audio`
- **Atual**: 2.5-3.0s
- **Gap**: ~2.2s a reduzir

### Estratégias para Reduzir 2.2s

1. **FP16 totalmente ativo**: -0.5s (estimado)
2. **Torch compile**: -0.3s (estimado)
3. **Pre-buffer mínimo (100ms)**: -0.1s
4. **Primeiro chunk menor (10-15 chars)**: -0.5s
5. **Speaker embedding pré-carregado**: -0.2s
6. **Otimizações de modelo**: -0.6s

**Total estimado**: -2.2s → **Target atingível**

---

## 🔍 Debugging

### Verificar Overlap/Repetição

```python
# Adicionar validação de chunks
for i, chunk in enumerate(chunks):
    if i > 0:
        # Verificar se há overlap com chunk anterior
        prev_chunk_end = chunks[i-1][-20:]  # Últimas 20 chars
        current_chunk_start = chunk[:20]  # Primeiras 20 chars
        if prev_chunk_end in current_chunk_start or current_chunk_start in prev_chunk_end:
            print(f"⚠️  OVERLAP detectado entre chunks {i-1} e {i}")
```

### Verificar FP16

```python
# Verificar se modelo está em FP16
if hasattr(tts.synthesizer, 'model'):
    model_dtype = next(tts.synthesizer.model.parameters()).dtype
    print(f"Model dtype: {model_dtype}")  # Deve ser torch.float16
```

### Medir Latência por Componente

```python
# Medir cada etapa separadamente
text_prep_start = time.time()
# ... preparação de texto ...
text_prep_time = time.time() - text_prep_start

tts_start = time.time()
# ... síntese XTTS ...
tts_time = time.time() - tts_start

block_split_start = time.time()
# ... split em blocos ...
block_split_time = time.time() - block_split_start
```

---

## 📚 Referências

- [TTS_PROFILES_STRATEGY.md](../docs/TTS_PROFILES_STRATEGY.md) - Estratégia de perfis
- [TTS_PROFILES_SPEC.md](../../rulebook/tasks/implement-tts-service/specs/tts-service/TTS_PROFILES_SPEC.md) - Especificação técnica
- [ARCHITECTURE.md](../../docs/ARCHITECTURE.md) - Arquitetura do sistema

---

**Última atualização**: 2025-11-29



