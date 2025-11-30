# VRPG Client - Estratégia de Performance

## Visão Geral

O VRPG Client prioriza **baixa latência** e **experiência fluida** em todas as operações críticas. Este documento descreve estratégias de otimização, métricas de performance e técnicas para garantir que o sistema atenda aos requisitos de latência e throughput.

**Princípios Fundamentais**:
1. **Hot Path da Sessão nunca bloqueia**: Voz → Texto → Mestre IA → Regras → Resposta → Voz precisa ser rápido, independente da geração de imagem
2. **Geração pesada = modo preparação**: LoRA, battlemaps complexos, cenas cinematográficas
3. **Cache agressivo**: Qualquer asset visual gerado é armazenado e reutilizado
4. **Assíncrono sempre que possível**: Gerar imagens/cenas em segundo plano, nunca travar o loop de input do jogador

## Modos de Operação

### Sessão de Jogo (Tempo Real)

**Duração sugerida**: ~3h

**Prioridades**:
- Baixa latência (voz → resposta)
- Decisões táticas rápidas
- Imagens leves/on-the-fly (retratos, close-ups, variações)
- Animações de combate e rolagens de dados

**Orçamento de Latência (Sessão)**:
- ASR (voz → texto): ≤ 300 ms
- Mestre IA (texto → decisão/narração): 1–3 s (idealmente 1–2 s)
- Regras 5e (ataque/dano): ≤ 100 ms
- TTS (texto → voz): ≤ 300 ms
- Geração visual leve:
  - Retrato/emotion: 0.5–2 s, mas **não pode bloquear o restante**

### Modo de Preparação (Pós-Sessão)

**Duração sugerida**: ~1h

**O Mestre IA (e serviços auxiliares) preparam**:
- Battlemaps complexos
- Retratos completos de NPCs importantes
- Datasets de imagens para LoRA/embeddings
- Cenas chave (keyframes narrativos)
- Atualizações na memória da campanha (Vectorizer + Nexus + Lexum)

**Ciclo de Melhoria**: A cada ciclo de 3h de jogo + 1h de preparação, a campanha ganha **coerência e assets mais ricos**, sem penalizar a performance da sessão.

## Requisitos de Performance

### Latências Críticas (Sessão)

| Operação | Target | Máximo Aceitável |
|----------|--------|------------------|
| **Pipeline Voz→Voz** | < 250ms | < 300ms |
| **ASR (Transcrição)** | < 80ms | < 100ms |
| **LLM Inference** | < 150ms | < 200ms |
| **TTS (Síntese)** | < 40ms | < 60ms |
| **Game State Update** | < 10ms | < 20ms |
| **Rules Calculation** | < 5ms | < 10ms |
| **Memory Lookup** | < 50ms | < 100ms |
| **Image Generation (Leve)** | 0.5–2s | < 3s (não bloqueia) |
| **Image Generation (Pesada)** | N/A | Modo Preparação |

### Throughput

- **Tokens/s LLM**: > 30 tokens/s (Qwen 2.5 14B Q4_K_M)
- **FPS Frontend**: 60 FPS constante
- **Audio Latency**: < 20ms buffer
- **Memory Queries**: > 100 queries/s

## Otimização GPU para Latência < 1.5s

### Objetivo

Atingir **máximo de 1.5 segundos** de latência total do momento que o jogador para de falar até a resposta do mestre começar a tocar.

### Pipeline Completo

```
Jogador para de falar
    ↓
Whisper (ASR) → Texto
    ↓
Qwen (LLM) → Resposta
    ↓
XTTS (TTS) → Áudio neutro
    ↓
SoVITS → Áudio do personagem
    ↓
Reprodução começa
```

**Target**: < 1.5s total

### Distribuição de Tempo (Target)

| Componente | Tempo Target | Tempo Máximo |
|------------|--------------|--------------|
| **Whisper (ASR)** | 50ms | 100ms |
| **Qwen (LLM)** | 300ms | 500ms |
| **XTTS (TTS)** | 500ms | 800ms |
| **SoVITS** | 300ms | 500ms |
| **Overhead** | 50ms | 100ms |
| **TOTAL** | **1200ms** | **2000ms** |

### Configuração GPU

#### Variáveis de Ambiente

Adicione ao `.env`:
```bash
# GPU Configuration - ESSENCIAL para < 1.5s
VRPG_GPU_ENABLED=true
VRPG_TTS_USE_GPU=true
VRPG_ASR_USE_GPU=true
VRPG_LLM_USE_GPU=true
VRPG_SOVITS_USE_GPU=true
VRPG_GPU_MEMORY_MB=4096
VRPG_GPU_LAYERS=35
CUDA_VISIBLE_DEVICES=0
```

#### Verificar GPU Disponível

```bash
# Windows (PowerShell)
nvidia-smi

# Verificar PyTorch com CUDA
python -c "import torch; print(torch.cuda.is_available()); print(torch.cuda.get_device_name(0))"
```

#### Instalar Dependências GPU

**Para XTTS (Coqui TTS)**:
```bash
# ⚠️ IMPORTANTE: Se PyTorch já está instalado mas sem CUDA, desinstale primeiro:
pip uninstall torch torchvision torchaudio -y

# Instalar PyTorch com CUDA 12.1
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121

# Instalar Coqui TTS
pip install TTS
```

**Para SoVITS**:
```bash
# No venv do SoVITS
cd assets-and-models/models/tts/sovits
.\venv310\Scripts\activate
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121
```

**Para Whisper**:
```bash
pip install faster-whisper
```

### Análise de Latência do Pipeline

#### 1. Whisper (ASR - Transcrição)
**Target**: ≤ 80ms (ideal) | **Máximo aceitável**: < 100ms  
**Realidade esperada**: 50-200ms (depende do tamanho do áudio)

**Fatores que afetam**:
- Tamanho do chunk de áudio (320ms recomendado)
- Modelo usado (whisper-large-v3)
- GPU vs CPU (GPU pode reduzir para 30-100ms)
- VAD (Voice Activity Detection) pré-filtragem

**Otimizações aplicadas**:
- Streaming incremental (processa enquanto recebe)
- Chunk overlap de 80ms para continuidade
- Modelo quantizado (INT8)
- VAD para filtrar silêncio

#### 2. Qwen 2.5 1.5B (LLM - Reação Rápida / Prelúdio)
**Target**: < 1.2s total (ideal) | **Máximo aceitável**: < 1.5s  
**Realidade esperada**: 600-1200ms (incluindo TTS)

**Fatores que afetam**:
- Parse intent: 30-80ms
- Geração: 200-450ms (max 40 tokens)
- XTTS: 150-300ms
- SoVITS: 200-600ms
- Throughput: > 50 tokens/s (Qwen 2.5 1.5B Q4_K_M)

**Cenários**:
- **Prelúdio curto** (1 frase, 15-25 palavras): 600-800ms
- **Prelúdio médio** (2 frases, 30-45 palavras): 800-1200ms

**Ver especificação completa em [QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md)**

#### 3. Qwen 2.5 14B (LLM - Narrativa Completa)
**Target**: < 6s total (ideal) | **Máximo aceitável**: < 8s  
**Realidade esperada**: 2.5-6s (resposta média), 8-15s (resposta longa)

**Fatores que afetam**:
- Ingest contexto: 200-500ms
- Geração narrativa: 1.5-4s (resposta média), 8-15s (longa)
- TTS: 300-700ms
- Tamanho do contexto (8192 tokens configurado)
- Número de tokens gerados (resposta curta vs longa)
- GPU layers (35+ recomendado)
- Throughput: > 30 tokens/s (Qwen 2.5 14B Q4_K_M)

**Cenários**:
- **Resposta curta** (50-100 tokens): 2.5-4s
- **Resposta média** (150-300 tokens): 4-6s
- **Resposta longa** (500+ tokens): 8-15s

**Regra de Ouro**: O 14B nunca responde antes do 1.5B iniciar.

**Ver especificação completa em [QWEN_14B_SPEC.md](QWEN_14B_SPEC.md)**

#### 4. XTTS (TTS - Síntese de Áudio)
**Target**: ≤ 40ms (ideal) | **Máximo aceitável**: < 60ms  
**Realidade esperada**: 500-5000ms (depende do tamanho do texto)

**⚠️ PROBLEMA IDENTIFICADO**: XTTS é o maior gargalo do pipeline!

**Cenários**:
- **Texto curto** ("Hello World"): 2-5 segundos (primeira vez), 1-3s (subsequentes)
- **Texto médio** (50-100 palavras): 5-15 segundos
- **Texto longo** (200+ palavras): 15-60 segundos

**Otimizações aplicadas**:
- Modelo pré-carregado (evita latência de carregamento)
- Streaming synthesis (gerar em chunks de 100ms)
- Voice embedding cache
- GPU acceleration (quando disponível) - **ESSENCIAL**

#### 5. SoVITS (Conversão de Voz)
**Target**: ≤ 100ms (ideal) | **Máximo aceitável**: < 200ms  
**Realidade esperada**: 500-3000ms (depende do tamanho do áudio)

**Cenários**:
- **Áudio curto** (1-2 segundos): 500-1000ms
- **Áudio médio** (5-10 segundos): 1000-2000ms
- **Áudio longo** (20+ segundos): 2000-5000ms

**Otimizações aplicadas**:
- Modelo pré-carregado
- Python bridge otimizado
- GPU acceleration (auto-detecta)

### Resumo de Latência

**Cenário Ideal (GPU, resposta curta)**:
- Whisper: 50ms
- Qwen 1.5B (prelúdio): 600ms
- Qwen 14B (narrativa): 3000ms
- XTTS (1.5B): 200ms
- XTTS (14B): 500ms
- SoVITS (1.5B): 300ms
- SoVITS (14B): 500ms
- **Total prelúdio**: ~1150ms ✅ (< 1.2s target)
- **Total narrativa**: ~4800ms ✅ (< 6s target)

**Cenário Pessimista (CPU, resposta longa)**:
- Whisper: 200ms
- Qwen 1.5B: 1200ms
- Qwen 14B: 15000ms
- XTTS (1.5B): 500ms
- XTTS (14B): 5000ms
- SoVITS (1.5B): 1000ms
- SoVITS (14B): 5000ms
- **Total prelúdio**: ~2900ms ⚠️ (aceitável)
- **Total narrativa**: ~25200ms ❌ (muito lento, mas prelúdio já tocou)

**Conclusão**: GPU é **ESSENCIAL** para atingir o target de < 1.5s.

## Otimizações por Módulo

### 1. LLM Core

#### Modelo e Quantização

- **Modelo**: Qwen 2.5 14B Q4_K_M (balance ideal qualidade/velocidade)
- **GPU Layers**: Máximo possível (35+ para 14B)
- **Context Size**: 8192 tokens (balance memória/performance)
- **Batch Size**: 1 (inferência sequencial otimizada)

#### Otimizações de Inferência

```rust
// Configuração otimizada
struct LLMConfig {
    model_path: String,
    context_size: usize,      // 8192
    gpu_layers: usize,        // auto (máximo)
    threads: usize,           // 8 (CPU cores)
    batch_size: usize,        // 1
    use_mmap: bool,           // true (memory mapping)
    use_mlock: bool,          // true (lock memory)
    numa: bool,               // true (NUMA optimization)
}
```

**Técnicas**:
- **Memory Mapping (mmap)**: Carregar modelo via mmap para reduzir uso de RAM
- **Memory Locking (mlock)**: Evitar swap do modelo
- **NUMA Awareness**: Otimizar acesso à memória em sistemas multi-socket
- **KV Cache**: Manter cache de key-value para contexto
- **Streaming**: Stream tokens enquanto gera (reduz latência percebida)

#### Prompt Optimization

- **LessTokens Integration**: Comprimir prompts longos antes de enviar
- **Context Pruning**: Remover contexto irrelevante antigo
- **Template Caching**: Cache de templates de prompt frequentes

### 2. ASR Service

#### Modelo e Configuração

- **Modelo**: Whisper-large-v3-turbo quantizado
- **Chunk Size**: 320ms (balance latência/qualidade)
- **VAD (Voice Activity Detection)**: Detectar início/fim de fala
- **Streaming**: Processar chunks incrementalmente

#### Otimizações

```rust
struct ASRConfig {
    model_path: String,
    chunk_size_ms: u32,       // 320ms
    overlap_ms: u32,          // 80ms
    vad_enabled: bool,        // true
    beam_size: usize,         // 5 (balance qualidade/velocidade)
    temperature: f32,         // 0.0 (determinístico)
    language: Option<String>, // auto-detect
}
```

**Técnicas**:
- **VAD Pre-filtering**: Filtrar silêncio antes de processar
- **Chunk Overlap**: Overlap de 80ms entre chunks para continuidade
- **Incremental Decoding**: Decodificar enquanto recebe áudio
- **Model Quantization**: Usar modelo quantizado (INT8)

### 3. TTS Service

#### Modelo e Configuração

- **Arquitetura**: Qwen 2.5 14B q4_K_M → XTTS v2 → SoVITS (otimizado para inferência local com GPU)
- **Streaming**: Síntese incremental
- **Voice Caching**: Cache de vozes frequentes
- **Batch Processing**: Processar múltiplas falas quando possível

#### Otimizações

```rust
struct TTSConfig {
    model_path: String,
    format: AudioFormat,      // PCM 16kHz
    streaming: bool,          // true
    voice_cache_size: usize,  // 10
    chunk_size_samples: usize, // 1600 (100ms)
}
```

**Técnicas**:
- **Streaming Synthesis**: Gerar áudio em chunks de 100ms
- **Voice Embedding Cache**: Cache de embeddings de vozes
- **ONNX Optimization**: Usar ONNX Runtime com otimizações
- **Audio Buffer Pool**: Pool de buffers reutilizáveis

### 4. Game Engine

#### Otimizações de Estado

- **Immutable State**: Usar estruturas imutáveis para facilitar comparação
- **State Diffing**: Apenas atualizar mudanças (React-like)
- **Event Batching**: Agrupar múltiplos eventos em um batch
- **Lazy Evaluation**: Calcular valores apenas quando necessário

```rust
// State update otimizado
impl GameEngine {
    fn apply_updates(&mut self, updates: Vec<StateUpdate>) -> StateDiff {
        // Batch processing
        let mut diff = StateDiff::new();
        for update in updates {
            diff.merge(self.apply_update(update));
        }
        diff
    }
}
```

#### Cache de Cálculos

- **Dice Roll Cache**: Cache de rolagens comuns
- **Combat Calculation Cache**: Cache de cálculos de combate
- **Pathfinding Cache**: Cache de caminhos calculados

### 5. Memory Service

#### Otimizações de Busca

- **Index Optimization**: Índices otimizados no Vectorizer
- **Query Caching**: Cache de queries frequentes
- **Batch Queries**: Agrupar múltiplas queries
- **Lazy Loading**: Carregar resultados sob demanda

```rust
// Busca otimizada
async fn search_memory(
    &self,
    query: &str,
    scope: &[String],
    limit: usize,
) -> Result<Vec<MemoryResult>> {
    // 1. Check cache
    if let Some(cached) = self.cache.get(query) {
        return Ok(cached);
    }
    
    // 2. Parallel search (Nexus + Lexum + Vectorizer)
    let (graph_results, text_results, semantic_results) = tokio::join!(
        self.nexus.search(query, scope),
        self.lexum.search(query, scope),
        self.vectorizer.search(query, scope, limit)
    );
    
    // 3. Combine and rank
    let results = self.combine_results(
        graph_results?,
        text_results?,
        semantic_results?
    );
    
    // 4. Cache results
    self.cache.set(query, results.clone());
    
    Ok(results)
}
```

### 6. Frontend (Electron)

#### Renderização

- **Virtual Scrolling**: Para listas longas (histórico, inventário)
- **Canvas Optimization**: Otimizar renderização do BattleMap
- **Request Animation Frame**: Usar RAF para animações suaves
- **Web Workers**: Processar cálculos pesados em workers

```typescript
// Otimização de renderização
class BattleMapRenderer {
  private renderLoop() {
    requestAnimationFrame(() => {
      // Apenas renderizar se houver mudanças
      if (this.needsUpdate) {
        this.render();
        this.needsUpdate = false;
      }
      this.renderLoop();
    });
  }
  
  private render() {
    // Batch updates
    this.pixiApp.stage.children.forEach(child => {
      if (child.visible && child.dirty) {
        child.update();
        child.dirty = false;
      }
    });
  }
}
```

#### Estado e Memória

- **State Management**: Zustand com seletores otimizados
- **Memoization**: React.memo e useMemo para componentes pesados
- **Code Splitting**: Lazy loading de rotas e componentes
- **Memory Management**: Limpar recursos não utilizados

### 7. IPC e Comunicação

#### Otimizações

- **Message Batching**: Agrupar múltiplas mensagens IPC
- **Binary Protocol**: Usar formato binário para dados grandes
- **Connection Pooling**: Pool de conexões HTTP
- **Keep-Alive**: Manter conexões HTTP abertas

```typescript
// IPC otimizado
class IPCManager {
  private messageQueue: IPCMessage[] = [];
  private batchTimeout: NodeJS.Timeout | null = null;
  
  send(message: IPCMessage) {
    this.messageQueue.push(message);
    
    if (!this.batchTimeout) {
      this.batchTimeout = setTimeout(() => {
        this.flush();
      }, 10); // Batch de 10ms
    }
  }
  
  private flush() {
    if (this.messageQueue.length > 0) {
      ipcRenderer.invoke('batch', this.messageQueue);
      this.messageQueue = [];
    }
    this.batchTimeout = null;
  }
}
```

## Monitoramento e Métricas

### Métricas Principais

```typescript
interface PerformanceMetrics {
  // Latências
  voice_to_voice_latency: number;    // ms
  asr_latency: number;               // ms
  llm_latency: number;               // ms
  tts_latency: number;                // ms
  memory_lookup_latency: number;     // ms
  
  // Throughput
  llm_tokens_per_second: number;
  fps: number;                        // Frontend FPS
  memory_queries_per_second: number;
  
  // Recursos
  cpu_usage: number;                  // %
  gpu_usage: number;                  // %
  memory_usage: number;               // MB
  disk_io: number;                     // MB/s
}
```

### Coleta de Métricas

```rust
// Exemplo de coleta
struct PerformanceMonitor {
    metrics: Arc<Mutex<PerformanceMetrics>>,
}

impl PerformanceMonitor {
    fn record_latency(&self, operation: &str, duration: Duration) {
        let mut metrics = self.metrics.lock().unwrap();
        match operation {
            "asr" => metrics.asr_latency = duration.as_millis() as u64,
            "llm" => metrics.llm_latency = duration.as_millis() as u64,
            "tts" => metrics.tts_latency = duration.as_millis() as u64,
            _ => {}
        }
    }
}
```

### Endpoint de Métricas

```json
GET /metrics
{
  "latencies": {
    "voice_to_voice": 245,
    "asr": 78,
    "llm": 142,
    "tts": 38
  },
  "throughput": {
    "llm_tokens_per_second": 32,
    "fps": 60,
    "memory_queries_per_second": 125
  },
  "resources": {
    "cpu_usage": 45.2,
    "gpu_usage": 78.5,
    "memory_usage_mb": 2048,
    "disk_io_mb_s": 12.3
  }
}
```

## Estratégias de Cache

### Cache de Modelos

- **Model Loading**: Carregar modelos uma vez e manter em memória
- **KV Cache**: Manter cache de key-value para LLM
- **Voice Embeddings**: Cache de embeddings de vozes TTS

### Cache de Dados

- **Memory Queries**: Cache de queries frequentes (TTL: 5 minutos)
- **Game State**: Cache de snapshots de estado
- **Image Assets**: Cache de imagens geradas (LRU)

### Caching de Imagens

#### Estrutura

Cada imagem gerada é salva com uma chave:

- `type` (portrait/scene/battlemap/icon)
- `character_id` (se aplicável)
- `emotion` (se aplicável)
- `pose` (se aplicável)
- `scene_id` (se aplicável)
- `seed`
- `style_version`

O caminho é armazenado em um índice no banco local.

#### Política de Reutilização

Antes de gerar qualquer imagem:

1. Busca no cache por uma imagem que cubra:
   - Mesmo personagem
   - Mesma emoção
   - Mesmo tipo (retratos, inteiro, etc.)
2. Se existir, usa diretamente
3. Se não, gera:
   - Salva no cache
   - E registra metadados

#### Invalidando Cache

Quando:

- LoRA global de estilo é atualizado
- O personagem muda visual drasticamente (novo arco de evolução)

Pode-se:

- Versionar as imagens antigas (para manter histórico)
- Gerar novas versões com o estilo novo

### Cache de Cálculos

- **Dice Rolls**: Cache de rolagens comuns
- **Combat Calculations**: Cache de cálculos de combate
- **Pathfinding**: Cache de caminhos calculados

## Planejamento do Modo Preparação

Ao fim da sessão:

- O sistema gera uma **lista de jobs** para o modo preparação:
  - `TrainEmbedding(char_id)` se atingido critério
  - `TrainLoRA(char_id)` se personagem se tornou central
  - `GenerateBattlemap(scene_id)` para cenas prováveis
  - `GeneratePortraitSet(char_id)` para emoções faltantes

O modo preparação executa esses jobs com priorização:

1. Assets usados com certeza na próxima sessão
2. Assets prováveis (80%+)
3. Assets futuros/bonus se sobrar tempo

## Utilização de GPU

### Durante Sessão

- **Prioridade**:
  - Mestre IA
  - ASR/TTS
  - Regras
  - Engine gráfica
- **Geração de imagem rápida**:
  - Limitada em concorrência
  - Fila pequena
  - Se GPU está muito carregada, a geração é adiada

### Durante Preparação

- GPU pode operar no limite:
  - LoRA training
  - Geração de lotes de imagens
  - Atualização de embeddings

## Otimizações de Hardware

### GPU

- **CUDA/ROCm**: Usar GPU para inferência LLM e geração de imagens
- **Mixed Precision**: FP16 para reduzir uso de memória
- **Tensor Cores**: Aproveitar Tensor Cores quando disponível

### CPU

- **Thread Affinity**: Fixar threads em cores específicos
- **NUMA**: Otimizar acesso à memória em sistemas multi-socket
- **SIMD**: Usar instruções SIMD quando possível

### Memória

- **Memory Mapping**: Usar mmap para modelos grandes
- **Memory Pooling**: Pool de buffers reutilizáveis
- **Garbage Collection**: Otimizar GC (Rust não tem, mas Node.js sim)

## Profiling e Debugging

### Ferramentas

- **perf** (Linux): Profiling de sistema
- **Instruments** (macOS): Profiling de aplicação
- **Chrome DevTools**: Profiling de frontend
- **cargo flamegraph**: Profiling de Rust

### Análise de Performance

```bash
# Profiling LLM
cargo flamegraph --bin llm-core -- --profile

# Profiling Frontend
npm run dev:profile

# Análise de memória
valgrind --tool=massif ./target/release/llm-core
```

## Configuração de Performance

```json
{
  "performance": {
    "target_latencies": {
      "voice_to_voice_ms": 250,
      "asr_ms": 80,
      "llm_ms": 150,
      "tts_ms": 40
    },
    "cache": {
      "enabled": true,
      "memory_queries_ttl_seconds": 300,
      "image_cache_size_mb": 1024,
      "model_cache_enabled": true
    },
    "optimization": {
      "gpu_layers": "auto",
      "batch_size": 1,
      "streaming": true,
      "lazy_loading": true
    },
    "monitoring": {
      "enabled": true,
      "metrics_endpoint": "/metrics",
      "sampling_rate": 0.1
    }
  }
}
```

## Degradação Elegante

Se a máquina do usuário for fraca:

- Reduzir resolução de imagens geradas
- Limitar uso de LoRA complexos
- Geração pesada pode ser opcional ou reduzir qualidade
- Priorizar:
  - ASR/TTS
  - Mestre IA
  - Engine de jogo

## Boas Práticas

1. **Measure First**: Sempre medir antes de otimizar
2. **Profile Regularly**: Profiling regular para identificar gargalos
3. **Cache Strategically**: Cache apenas dados que realmente beneficiam
4. **Lazy Loading**: Carregar recursos sob demanda
5. **Batch Operations**: Agrupar operações quando possível
6. **Stream When Possible**: Usar streaming para reduzir latência percebida
7. **Monitor Continuously**: Monitorar métricas em produção
8. **Separate Hot and Heavy Paths**: Nunca fazer geração pesada durante sessão
9. **Preparation Mode**: Usar modo preparação para assets pesados
10. **Cache Aggressively**: Reutilizar assets sempre que possível

## Resumo

- **Performance em sessão** é garantida pela separação clara entre:
  - Hot path (voz, raciocínio, regras)
  - Heavy path (imagens, LoRAs)
- **Caching e preparação offline** garantem que a IA pareça "instantânea" na maior parte dos casos
- **Modo preparação** permite gerar assets de alta qualidade sem impactar a experiência do jogador

---

Esta estratégia de performance garante que o VRPG Client atenda aos requisitos de latência e throughput, proporcionando uma experiência fluida e responsiva através da separação inteligente entre modo sessão (tempo real) e modo preparação (offline).

