# Estratégia de Perfis TTS para Redução de Latência

**Data**: 2025-11-29  
**Status**: ✅ Estrutura implementada, otimizações em andamento

---

## 📋 Contexto e Problema

### Diagnóstico do Problema Atual

Baseado em testes de benchmark, identificamos que:

1. **Latência inicial medida**: 10.16s
2. **Primeiro chunk**: ~13.17s de áudio (~200 caracteres)
3. **RTF completo**: 0.67x (XTTS é mais rápido que tempo real)
4. **Problema real**: Streaming funciona como **batch processing** - gera chunk enorme antes de tocar

### Conclusão

O problema **NÃO é a GPU ou drivers**. O problema é:
- **Chunk muito grande** (13s de áudio no primeiro chunk)
- **Ausência de streaming real** (espera chunk inteiro antes de tocar)
- **Falta de otimizações** (FP16, sample rate, warm-up)

---

## 🎯 Estratégia: Dois Perfis de Performance

### Perfil 1: FAST (Qwen 1.5B)

**Cenário**: Respostas guiadas pelo modelo Qwen 1.5B (prelúdio emocional)

**Requisito**: Latência entre "texto pronto" e primeiro som **< 1s**

**Configuração**:
- `first_chunk_max_chars`: 30 caracteres
  - Alvo: ~0.7-1.0s de fala
  - Pode cortar no meio de frase (prioridade: velocidade)
- `next_chunk_max_chars`: 90 caracteres
  - Alvo: ~2-3s de fala por chunk
- `sample_rate`: 16 kHz (mono)
  - Reduz custo computacional vs 24 kHz
- `dtype`: FP16 (half precision)
  - Modelo: `.half().to("cuda")`
  - Inferência: `torch.cuda.amp.autocast(device_type="cuda")`
  - Reduz tempo de inferência em 30-40%
- `audio_block_ms`: 50ms
  - Tamanho dos blocos que vão para FIFO
  - 800 samples @ 16 kHz
- `initial_prebuffer_ms`: 240ms
  - Começar playback com ~¼ de segundo de áudio no buffer
  - 3840 samples @ 16 kHz

**Target**: `time_to_first_audio` ≤ 0.8s (ideal 0.5-0.7s)

### Perfil 2: CINEMATIC (Qwen 14B)

**Cenário**: Respostas guiadas pelo modelo Qwen 14B (narrativa completa)

**Requisito**: Primeiro som em **1.5-3s** (bem abaixo dos 10s atuais)

**Configuração**:
- `first_chunk_max_chars`: 100 caracteres
  - Alvo: ~3s de fala no primeiro chunk
  - Tenta respeitar pontuação quando possível
- `next_chunk_max_chars`: 150 caracteres
  - Alvo: ~4-5s de fala
- `sample_rate`: 24 kHz (mono)
  - Melhor fidelidade para narrativas
- `dtype`: FP16 (half precision)
  - Mesmas otimizações do FAST
- `audio_block_ms`: 60-80ms
  - Blocos um pouco maiores para eficiência
  - 1440-1920 samples @ 24 kHz
- `initial_prebuffer_ms`: 500ms
  - Pre-buffer maior para garantir continuidade
  - 12000 samples @ 24 kHz

**Target**: `time_to_first_audio` na faixa de 1.5-3s

---

## 🔧 Implementação Técnica

### 1. Estrutura de Perfis

```rust
pub struct TtsProfile {
    pub profile_type: TtsProfileType,  // FAST or CINEMATIC
    pub first_chunk_max_chars: usize,
    pub next_chunk_max_chars: usize,
    pub sample_rate: u32,
    pub use_fp16: bool,
    pub audio_block_ms: u32,
    pub initial_prebuffer_ms: u32,
}

impl TtsProfile {
    pub fn from_llm_model(llm_model_name: &str) -> Self {
        // Auto-seleção baseada no nome do modelo
        if model_name.contains("1.5") || model_name.contains("1_5") {
            Self::fast()
        } else if model_name.contains("14") || model_name.contains("14b") {
            Self::cinematic()
        } else {
            Self::cinematic() // Default
        }
    }
}
```

### 2. Chunker Configurável

```rust
fn split_text_for_tts(text: &str, profile: &TtsProfile) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut current_limit = profile.first_chunk_max_chars;
    
    for word in words {
        let word_with_space = if current.is_empty() {
            word.to_string()
        } else {
            format!(" {}", word)
        };
        
        // Para FAST: pode cortar no meio de frase
        // Para CINEMATIC: tenta respeitar limites mas pode cortar se necessário
        if current.len() + word_with_space.len() > current_limit && !current.is_empty() {
            chunks.push(current.trim().to_string());
            current.clear();
            current_limit = profile.next_chunk_max_chars; // Switch após primeiro chunk
        }
        
        current.push_str(&word_with_space);
    }
    
    if !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }
    
    chunks
}
```

### 3. Otimização XTTS

#### FP16 + Autocast

```python
# Carregar modelo em FP16
model = TTS("tts_models/multilingual/multi-dataset/xtts_v2")
model.to("cuda").half()  # FP16

# Inferência com autocast
with torch.cuda.amp.autocast(device_type="cuda"):
    with torch.inference_mode():
        audio = model.tts(text, speaker_wav=..., language="en")
```

#### Sample Rate por Perfil

```python
# FAST: 16 kHz
audio = tts.tts(text, speaker_wav=..., language="en", sample_rate=16000)

# CINEMATIC: 24 kHz
audio = tts.tts(text, speaker_wav=..., language="en", sample_rate=24000)
```

#### Warm-up na Inicialização

```python
# Ao iniciar serviço
tts.warmup("Warmup line for TTS", profile=fast_profile)
```

### 4. Streaming Real com FIFO

#### Producer (TTS Generation)

```python
def tts_producer(text, profile):
    chunks = split_text_for_tts(text, profile)
    
    for chunk in chunks:
        # Gerar áudio
        audio = synth(chunk, profile)  # numpy array
        
        # Dividir em blocos
        block_samples = profile.audio_block_samples()
        for block_start in range(0, len(audio), block_samples):
            block = audio[block_start:block_start + block_samples]
            audio_fifo.put(block)  # Empurrar imediatamente
```

#### Consumer (Audio Playback)

```python
def audio_consumer(profile):
    # Esperar pre-buffer
    while fifo_duration_ms() < profile.initial_prebuffer_ms:
        time.sleep(0.01)
    
    # Iniciar playback
    start_output_stream()
    
    # Consumir blocos continuamente
    while True:
        block = audio_fifo.get()
        play_block(block)  # Enviar para dispositivo de áudio
```

---

## 📊 Métricas de Latência

### Métricas Obrigatórias

Para cada perfil, o sistema DEVE medir:

1. **tts_text_length_chars**: Comprimento total do texto
2. **first_chunk_text_length_chars**: Comprimento do primeiro chunk apenas
3. **first_chunk_audio_duration_sec**: Duração do primeiro chunk (baseado em samples gerados)
4. **xtts_first_chunk_infer_time_sec**: Tempo de inferência do chunk 1
5. **time_to_first_audio_sec**: Tempo entre `start_tts_request(text)` e momento em que o player efetivamente escreve o primeiro bloco de áudio no device

### Targets

**FAST / Qwen 1.5B**:
- `time_to_first_audio_sec` ≤ 0.8s (ideal 0.5-0.7s)

**CINEMATIC / Qwen 14B**:
- `time_to_first_audio_sec` na faixa de 1.5-3s

---

## 🔗 Integração com Orquestrador

### Auto-Seleção de Perfil

No orquestrador (onde já decide se usa Qwen 1.5B ou 14B):

```rust
// Se llm_model_name contém "1.5" → usa tts_profiles.fast
// Se llm_model_name contém "14b" → usa tts_profiles.cinematic

let profile = TtsProfile::from_llm_model(&llm_model_name);

let streaming_request = StreamingRequest {
    text: qwen_output,
    character_id: character_id,
    language: "en",
    profile: Some(profile),
    llm_model_name: Some(llm_model_name),
};
```

### Fluxo Completo

```
Qwen 1.5B → Texto → TTS Service (Perfil FAST) → Streaming FIFO → Áudio (< 0.8s)
Qwen 14B → Texto → TTS Service (Perfil CINEMATIC) → Streaming FIFO → Áudio (1.5-3s)
```

---

## 📈 Resultados Esperados

### Antes (Problema Atual)
- Latência inicial: 10.16s
- Primeiro chunk: 13.17s de áudio
- Streaming: Batch (espera chunk inteiro)

### Depois (Com Perfis)

**FAST Profile**:
- Latência inicial: ≤ 0.8s (ideal 0.5-0.7s)
- Primeiro chunk: 30 chars (~0.7-1.0s de áudio)
- Streaming: Real (blocos de 50ms)

**CINEMATIC Profile**:
- Latência inicial: 1.5-3s
- Primeiro chunk: 100 chars (~3s de áudio)
- Streaming: Real (blocos de 60-80ms)

---

## 🔍 Diagnóstico e Validação

### Como Validar

1. **Medir time_to_first_audio** para cada perfil
2. **Verificar primeiro chunk** está dentro dos limites (30 chars FAST, 100 chars CINEMATIC)
3. **Verificar RTF** < 1.0x para primeiro chunk
4. **Verificar streaming contínuo** após início (zero gaps)

### Problemas Comuns

1. **Primeiro chunk ainda grande**: Verificar chunker está usando limites do perfil
2. **Latência ainda alta**: Verificar FP16 está ativo, warm-up executado, sample rate correto
3. **Gaps no áudio**: Verificar FIFO está funcionando, pre-buffer suficiente

---

## 📚 Referências

- Especificação técnica: `rulebook/tasks/implement-tts-service/specs/tts-service/TTS_PROFILES_SPEC.md`
- Arquitetura: `docs/ARCHITECTURE.md`
- Pipeline de áudio: `docs/AUDIO_PIPELINE.md`
- Status de implementação: `src/tts-service/REFATORACAO_STREAMING.md`

---

**Última atualização**: 2025-11-29



