# Sistema de Interjeições - Pré-roll de Áudio

**Objetivo**: Mascarar latência do TTS em respostas longas com interjeições pré-gravadas que criam uma sensação de "pensamento humano" antes da resposta principal.

---

## 🎯 Comportamento Desejado

### Fluxo Normal (Resposta Curta)
```
Jogador termina de falar
    ↓
ASR + LLM processam (0.1-0.5s)
    ↓
TTS gera áudio (2.4s)
    ↓
Reprodução começa
```
**Sem interjeição** - resposta rápida, não precisa mascarar latência.

### Fluxo com Interjeição (Resposta Longa)
```
Jogador termina de falar
    ↓
ASR + LLM processam (0.1-0.5s)
    ↓
Sistema detecta resposta longa (> 3s)
    ↓
Aguarda delay "humano" (1.5s desde fim da fala)
    ↓
Toca interjeição pré-gravada ("hmm...", "deixe-me ver...")
    ↓
TTS gera áudio principal em paralelo
    ↓
Após interjeição, toca TTS principal
```
**Com interjeição** - mascarar latência com som natural de "pensamento".

---

## 📋 Configuração

### Arquivo: `config/interjections.yaml`

```yaml
enabled: true
min_expected_tts_duration_sec: 3.0   # Só usa se resposta > 3s
natural_delay_target_sec: 1.5        # Delay total desejado
avoid_last_n: 5                      # Evitar últimas 5 usadas
max_uses_per_session: 999           # Limite por sessão
chars_per_sec: 25.0                  # Estimativa de duração
clips:
  - id: "dm_hmm_01"
    file: "assets/audio/interjections/dm_hmm_01.wav"
  # ... ~40 clipes
```

### Parâmetros por Perfil

- **FAST (Qwen 1.5B)**: `min_expected_tts_duration_sec * 1.33` (mais agressivo)
- **CINEMATIC (Qwen 14B)**: `min_expected_tts_duration_sec` (padrão)

---

## 🔍 Detecção de Respostas Longas

### Heurístico Baseado em Caracteres

```rust
expected_duration_sec = text_length_chars / chars_per_sec

if expected_duration_sec >= min_expected_tts_duration_sec:
    usar_interjeicao = true
```

**Exemplo**:
- Texto: 100 chars
- Estimativa: 100 / 25 = 4.0s
- Threshold: 3.0s
- **Resultado**: ✅ Usar interjeição

---

## ⏱️ Cálculo de Delay "Humano"

### Fórmula

```rust
elapsed_since_user_end = now() - last_user_speech_end_ts
delay_to_interjection = max(0.0, natural_delay_target_sec - elapsed_since_user_end)
```

**Comportamento**:
- Se `elapsed_since_user_end < 1.5s`: Aguarda até completar 1.5s
- Se `elapsed_since_user_end >= 1.5s`: Toca imediatamente (sem delay extra)

**Exemplo**:
- Fim da fala: 0.0s
- ASR + LLM: 0.3s
- Elapsed: 0.3s
- Delay necessário: 1.5 - 0.3 = **1.2s** (aguarda)
- Interjeição toca em: 1.5s ✅

---

## 🎵 Seleção de Interjeições

### Algoritmo de Evitar Repetição

1. **Filtrar candidatos**: Remove últimas N interjeições usadas
2. **Selecionar aleatoriamente**: Entre candidatos restantes
3. **Se todos usados**: Relaxa restrição (permite repetir)
4. **Registrar uso**: Adiciona à lista recente (FIFO)

### Estado por Sessão

```rust
struct InterjectionState {
    recent_ids: VecDeque<String>,  // Últimas N usadas
    use_counts: HashMap<String, usize>,  // Contador por ID
    total_uses: usize,  // Total na sessão
}
```

---

## 🔄 Pipeline de Execução

### Quando Interjeição é Necessária

1. **Detecção**: Texto longo detectado
2. **Cálculo de delay**: `delay_to_interjection = max(0, 1.5 - elapsed)`
3. **Seleção**: Escolher interjeição (evitando últimas N)
4. **Agendamento**: Timer para tocar após `delay_to_interjection`
5. **TTS paralelo**: Iniciar geração do TTS principal (não esperar)
6. **Reprodução**:
   - Tocar interjeição quando timer disparar
   - Após interjeição terminar, tocar TTS principal
   - Se TTS terminar antes, aguardar fim da interjeição + gap (50-100ms)

### Cancelamento

Se contexto mudar (jogador interromper, cancelar):
- Cancelar timer da interjeição (se ainda não começou)
- Continuar com TTS normalmente

---

## 📊 Logging e Telemetria

### Métricas Registradas

```rust
struct InterjectionMetrics {
    used_interjection: bool,
    interjection_id: Option<String>,
    expected_duration_sec: f64,
    time_user_end_to_interjection_start_sec: f64,
    time_user_end_to_tts_start_sec: f64,
    profile: String,  // "fast" ou "cinematic"
    llm_model: String,  // "qwen_1_5b" ou "qwen_14b"
}
```

### Exemplo de Log

```
INFO: Interjection triggered
  interjection_id: "dm_hmm_02"
  expected_duration: 4.2s
  time_to_interjection: 1.48s
  time_to_tts: 3.92s
  profile: "cinematic"
  llm_model: "qwen_14b"
```

---

## 🎮 Integração com Pipeline

### Pontos de Entrada

1. **Pipeline de TTS** (`src/pipeline.rs`):
   - Verificar se deve usar interjeição antes de sintetizar
   - Calcular delay e agendar interjeição
   - Iniciar TTS em paralelo

2. **Streaming Pipeline** (`src/streaming.rs`):
   - Integrar interjeição no fluxo de streaming
   - Enfileirar interjeição antes do primeiro chunk TTS

3. **Orquestrador** (`src/orchestrator/`):
   - Rastrear `last_user_speech_end_ts`
   - Passar timestamp para pipeline de TTS

---

## 📁 Estrutura de Arquivos

```
vrpg-client/src/tts-service/
├── src/
│   ├── interjections.rs          # Módulo principal
│   ├── pipeline.rs                # Integração com pipeline
│   └── streaming.rs               # Integração com streaming
├── config/
│   └── interjections.yaml         # Configuração
├── assets/
│   └── audio/
│       └── interjections/         # ~40 clipes WAV
│           ├── dm_hmm_01.wav
│           ├── dm_hmm_02.wav
│           └── ...
└── tests/
    └── scripts/
        └── test_interjections_system.py  # Teste completo
```

---

## 🧪 Testes

### Teste Python: `test_interjections_system.py`

**Cenários**:
1. **Texto curto** (< 3s): Não deve usar interjeição
2. **Texto longo** (> 3s): Deve usar interjeição com delay ~1.5s
3. **Múltiplas interjeições**: Verificar que não repete imediatamente
4. **Cancelamento**: Verificar que cancela se contexto mudar

**Métricas validadas**:
- `time_user_end_to_interjection_start_sec` ≈ 1.5s
- `time_user_end_to_tts_start_sec` > `time_user_end_to_interjection_start_sec`
- Interjeições não se repetem nas últimas N

---

## 🎯 Exemplos de Uso

### Exemplo 1: Resposta Curta (Sem Interjeição)

```
Texto: "The door creaks open." (20 chars)
Expected: 20 / 25 = 0.8s
Threshold: 3.0s
Result: ❌ Não usar interjeição
```

### Exemplo 2: Resposta Longa (Com Interjeição)

```
Texto: "In the depths of the forgotten library..." (300 chars)
Expected: 300 / 25 = 12.0s
Threshold: 3.0s
Result: ✅ Usar interjeição

Timeline:
  0.0s: Jogador para de falar
  0.3s: ASR + LLM prontos
  1.5s: Interjeição toca ("hmm...")
  2.0s: Interjeição termina
  2.1s: TTS principal começa (gap de 100ms)
```

---

## 🔧 Configuração Avançada

### Perfis Específicos

```yaml
profiles:
  fast:
    min_expected_tts_duration_sec: 4.0  # Mais conservador
  cinematic:
    min_expected_tts_duration_sec: 3.0  # Padrão
```

### Ajuste de Delay

```yaml
natural_delay_target_sec: 1.5  # Padrão (humano)
# Valores menores: mais responsivo, menos natural
# Valores maiores: mais natural, mas pode parecer lento
```

---

## 📚 Referências

- [TTS_PROFILES_STRATEGY.md](../docs/TTS_PROFILES_STRATEGY.md) - Estratégia de perfis
- [ANALISE_OTIMIZACAO_LATENCIA.md](../ANALISE_OTIMIZACAO_LATENCIA.md) - Análise de latência
- [ARCHITECTURE.md](../../docs/ARCHITECTURE.md) - Arquitetura do sistema

---

**Última atualização**: 2025-11-29



