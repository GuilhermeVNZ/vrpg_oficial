# Pontos de Entrada - Sistema de Interjeições

**Arquivos principais a modificar para integração completa**

---

## 📁 Arquivos Criados

### 1. `src/interjections.rs`
**Módulo principal do sistema de interjeições**

**Estruturas principais**:
- `InterjectionConfig`: Configuração (YAML)
- `InterjectionClip`: Metadados de clipe
- `InterjectionState`: Estado de uso (evitar repetição)
- `InterjectionManager`: Gerenciador principal

**Funções principais**:
- `InterjectionManager::from_config_file()`: Carregar de YAML
- `should_use_interjection()`: Verificar se deve usar
- `calculate_delay_to_interjection()`: Calcular delay
- `select_interjection()`: Selecionar clipe

---

## 🔧 Arquivos a Modificar

### 1. `src/pipeline.rs`

**Adicionar ao `TtsPipeline`**:
```rust
pub struct TtsPipeline {
    // ... campos existentes
    interjection_manager: Option<Arc<InterjectionManager>>,
    last_user_speech_end_ts: Arc<RwLock<Option<Instant>>>,
}
```

**Modificar `synthesize()`**:
```rust
pub async fn synthesize(
    &self,
    request: PipelineRequest,
    user_speech_end_ts: Option<Instant>,  // NOVO parâmetro
) -> Result<PipelineResponse> {
    // 1. Verificar se deve usar interjeição
    if let Some(manager) = &self.interjection_manager {
        let text_length = request.text.len();
        let profile = self.get_profile_for_request(&request);
        
        if manager.should_use_interjection(text_length, profile) {
            // 2. Calcular delay
            let elapsed = user_speech_end_ts
                .map(|ts| ts.elapsed().as_secs_f64())
                .unwrap_or(0.0);
            let delay = manager.calculate_delay_to_interjection(elapsed);
            
            // 3. Selecionar interjeição
            if let Some(clip) = manager.select_interjection() {
                // 4. Agendar interjeição (async timer)
                // 5. Iniciar TTS em paralelo
            }
        }
    }
    
    // ... resto do código
}
```

---

### 2. `src/streaming.rs`

**Modificar `stream()`**:
```rust
pub async fn stream(&self, request: StreamingRequest) -> Result<...> {
    // 1. Verificar interjeição antes de chunking
    let use_interjection = if let Some(manager) = &self.interjection_manager {
        manager.should_use_interjection(request.text.len(), profile_name)
    } else {
        false
    };
    
    // 2. Se usar interjeição:
    if use_interjection {
        // - Calcular delay
        // - Agendar interjeição
        // - Enfileirar interjeição antes do primeiro chunk TTS
    }
    
    // 3. Continuar com streaming normal
    // ...
}
```

---

### 3. `src/server.rs` ou Orquestrador

**Rastrear timestamp de fim da fala**:
```rust
// Quando ASR detectar fim da fala
let user_speech_end_ts = Instant::now();

// Passar para pipeline
pipeline.synthesize(request, Some(user_speech_end_ts)).await
```

---

## 📊 Exemplos de Logs

### Exemplo 1: Com Interjeição

```
[INFO] Interjection triggered
  interjection_id: "dm_hmm_02"
  expected_duration_sec: 4.2
  time_user_end_to_interjection_start_sec: 1.48
  time_user_end_to_tts_start_sec: 3.92
  profile: "cinematic"
  llm_model: "qwen_14b"
```

### Exemplo 2: Sem Interjeição

```
[INFO] Interjection not used
  expected_duration_sec: 0.8
  threshold: 3.0
  profile: "fast"
  llm_model: "qwen_1_5b"
```

---

## 🎯 Fluxo Completo

```
1. Jogador para de falar
   → Orquestrador: last_user_speech_end_ts = now()

2. ASR + LLM processam
   → Tempo: ~0.1-0.5s

3. Texto pronto → Pipeline TTS
   → Pipeline: Verificar interjeição
   → expected_duration = text.len() / 25.0
   → if expected_duration >= 3.0s: usar interjeição

4. Se usar interjeição:
   → Calcular delay: max(0, 1.5 - elapsed)
   → Selecionar interjeição (evitando últimas 5)
   → Agendar timer async para tocar após delay
   → Iniciar TTS em paralelo (não esperar)

5. Timer dispara → Tocar interjeição
   → Carregar áudio WAV
   → Enfileirar no output de áudio

6. TTS termina → Aguardar interjeição terminar
   → Gap de 50-100ms
   → Enfileirar TTS principal

7. Reprodução sequencial:
   → Interjeição → Gap → TTS principal
```

---

## 🔍 Validações

### Teste 1: Texto Curto
```
Input: "The door creaks open." (20 chars)
Expected: 20/25 = 0.8s
Threshold: 3.0s
Result: ❌ Não usar interjeição
```

### Teste 2: Texto Longo
```
Input: "In the depths..." (300 chars)
Expected: 300/25 = 12.0s
Threshold: 3.0s
Result: ✅ Usar interjeição

Timeline:
  0.0s: Fim da fala
  0.3s: ASR + LLM prontos
  1.5s: Interjeição toca
  2.0s: Interjeição termina
  2.1s: TTS principal começa
```

---

**Última atualização**: 2025-11-29



