# Implementação do Sistema de Interjeições

**Data**: 2025-11-29  
**Status**: ✅ Estrutura Base Implementada

---

## 📋 Componentes Criados

### 1. Módulo Rust (`src/interjections.rs`)

**Estruturas principais**:
- `InterjectionConfig`: Configuração do sistema
- `InterjectionClip`: Metadados de clipe de áudio
- `InterjectionState`: Estado de uso (evitar repetição)
- `InterjectionManager`: Gerenciador principal

**Funcionalidades**:
- ✅ Carregar configuração de YAML
- ✅ Validar e carregar clipes de áudio
- ✅ Calcular duração de clipes WAV
- ✅ Detectar se deve usar interjeição (heurístico)
- ✅ Calcular delay "humano" (1.5s desde fim da fala)
- ✅ Selecionar interjeição evitando repetição
- ✅ Registrar uso de interjeições

### 2. Configuração YAML (`config/interjections.yaml`)

**Parâmetros configuráveis**:
- `enabled`: Habilitar/desabilitar sistema
- `min_expected_tts_duration_sec`: Threshold para usar interjeição (3.0s)
- `natural_delay_target_sec`: Delay desejado (1.5s)
- `avoid_last_n`: Evitar últimas N usadas (5)
- `chars_per_sec`: Estimativa de duração (25.0)
- `clips`: Lista de ~40 clipes de interjeição

### 3. Teste Python (`tests/scripts/test_interjections_system.py`)

**Cenários de teste**:
- ✅ Texto curto (sem interjeição)
- ✅ Texto longo (com interjeição)
- ✅ Cálculo de delay
- ✅ Seleção de interjeição
- ✅ Reprodução sequencial (interjeição + TTS)

### 4. Documentação (`docs/INTERJECTIONS_SYSTEM.md`)

Documentação completa do sistema, incluindo:
- Comportamento desejado
- Configuração
- Algoritmos
- Exemplos de uso

---

## 🔧 Funcionalidades Implementadas

### ✅ Detecção de Respostas Longas

```rust
expected_duration_sec = text_length_chars / chars_per_sec
should_use = expected_duration_sec >= min_expected_tts_duration_sec
```

**Perfis específicos**:
- FAST: `threshold * 1.33` (mais conservador)
- CINEMATIC: `threshold` (padrão)

### ✅ Cálculo de Delay "Humano"

```rust
elapsed_since_user_end = now() - last_user_speech_end_ts
delay_to_interjection = max(0.0, natural_delay_target_sec - elapsed_since_user_end)
```

**Comportamento**:
- Se `elapsed < 1.5s`: Aguarda até completar 1.5s
- Se `elapsed >= 1.5s`: Toca imediatamente

### ✅ Evitar Repetição

- Filtra últimas N interjeições usadas
- Seleciona aleatoriamente entre candidatos restantes
- Se todos usados, relaxa restrição
- Mantém estado por sessão (FIFO queue)

### ✅ Integração com Perfis

- FAST: Threshold mais alto (4.0s vs 3.0s)
- CINEMATIC: Threshold padrão (3.0s)
- Configurável por perfil

---

## ⚠️ Pendências

### 1. Integração com Pipeline Rust

**Arquivos a modificar**:
- `src/pipeline.rs`: Adicionar verificação de interjeição
- `src/streaming.rs`: Integrar interjeição no fluxo de streaming
- `src/server.rs`: Passar timestamp de fim da fala

**Ações necessárias**:
- Adicionar `InterjectionManager` ao `TtsPipeline`
- Rastrear `last_user_speech_end_ts` no orquestrador
- Agendar interjeição com timer async
- Enfileirar interjeição antes do TTS principal

### 2. Clipes de Áudio

**Necessário**:
- Criar ~40 clipes WAV de interjeições
- Colocar em `assets/audio/interjections/`
- Exemplos: "hmm...", "deixe-me ver...", "interessante...", etc.

**Formato**:
- WAV, 16kHz ou 24kHz mono
- Duração: 0.5s a 2.0s (ideal ~1.0s)
- Qualidade: Alta (voz da DM)

### 3. Logging e Telemetria

**Métricas a adicionar**:
- `used_interjection: bool`
- `interjection_id: Option<String>`
- `expected_duration_sec: f64`
- `time_user_end_to_interjection_start_sec: f64`
- `time_user_end_to_tts_start_sec: f64`
- `profile: String`
- `llm_model: String`

---

## 📊 Pontos de Entrada Principais

### 1. Pipeline de TTS (`src/pipeline.rs`)

```rust
impl TtsPipeline {
    pub async fn synthesize(&self, request: PipelineRequest) -> Result<PipelineResponse> {
        // 1. Verificar se deve usar interjeição
        let should_use = self.interjection_manager
            .should_use_interjection(text.len(), profile);
        
        // 2. Se sim, calcular delay e agendar
        if should_use {
            let delay = self.interjection_manager
                .calculate_delay_to_interjection(elapsed_since_user_end);
            // Agendar interjeição...
        }
        
        // 3. Iniciar TTS em paralelo
        // ...
    }
}
```

### 2. Streaming Pipeline (`src/streaming.rs`)

```rust
impl StreamingPipeline {
    pub async fn stream(&self, request: StreamingRequest) -> Result<...> {
        // 1. Verificar interjeição
        // 2. Enfileirar interjeição antes do primeiro chunk TTS
        // 3. Continuar com streaming normal
    }
}
```

### 3. Orquestrador

```rust
// Rastrear timestamp de fim da fala
conversation_state.last_user_speech_end_ts = now();

// Passar para pipeline de TTS
pipeline.synthesize(request, last_user_speech_end_ts).await
```

---

## 🧪 Exemplos de Logs

### Log 1: Com Interjeição

```
INFO: Interjection triggered
  interjection_id: "dm_hmm_02"
  expected_duration: 4.2s
  time_user_end_to_interjection_start: 1.48s
  time_user_end_to_tts_start: 3.92s
  profile: "cinematic"
  llm_model: "qwen_14b"
```

### Log 2: Sem Interjeição

```
INFO: Interjection not used
  expected_duration: 0.8s
  threshold: 3.0s
  profile: "fast"
  llm_model: "qwen_1_5b"
```

---

## 🎯 Próximos Passos

1. **Criar clipes de áudio** (~40 interjeições WAV)
2. **Integrar com pipeline Rust** (adicionar ao `TtsPipeline`)
3. **Adicionar rastreamento de timestamp** (no orquestrador)
4. **Implementar timer async** (para agendar interjeição)
5. **Testar sistema completo** (com clipes reais)
6. **Adicionar logging completo** (todas as métricas)

---

## 📚 Arquivos Criados/Modificados

### Criados
- ✅ `src/interjections.rs` - Módulo principal
- ✅ `config/interjections.yaml` - Configuração
- ✅ `tests/scripts/test_interjections_system.py` - Teste Python
- ✅ `docs/INTERJECTIONS_SYSTEM.md` - Documentação
- ✅ `IMPLEMENTACAO_INTERJECOES.md` - Este arquivo

### Modificados
- ✅ `Cargo.toml` - Adicionado `rand` e `serde_yaml`
- ✅ `src/lib.rs` - Exportado módulo `interjections`

### A Modificar (Pendente)
- ⏳ `src/pipeline.rs` - Integrar interjeições
- ⏳ `src/streaming.rs` - Integrar no streaming
- ⏳ `src/server.rs` - Passar timestamps
- ⏳ Orquestrador - Rastrear `last_user_speech_end_ts`

---

**Última atualização**: 2025-11-29



