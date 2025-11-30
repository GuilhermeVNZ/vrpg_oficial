# VRPG - Arquitetura de Pipeline com 3 Agentes

## 📘 Visão Geral

O objetivo do orquestrador é garantir **tempo de resposta real-time** sem sacrificar **qualidade narrativa**.

Isso é feito dividindo a pipeline em **3 agentes cognitivos + camadas determinísticas**:

```
Orquestrador → lógica pura, determinística
Qwen-1.5B → reação humana inicial ("prelúdio")
Qwen-14B → narrativa, consequência, resolução
```

### Regra Central

**Nenhuma resposta final é emitida pelo 14B até que o 1.5B já tenha iniciado a resposta.**

---

## 🧠 Papéis (Sem Ambiguidade)

### 1. ORQUESTRADOR (Autoridade Absoluta)

**Conecta os componentes**

**Controla quem responde e quando**

**Não "raciocina"**

**Não inventa regras**

**Responde perguntas objetivas sozinho**

**Garante que o 1.5B sempre responde antes do 14B**

#### NUNCA chamar LLM quando:

- Pergunta factual
- Estado de jogo
- Logs diretos
- Regras simples e unívocas

**Exemplo**:

```
"Quantos slots nível 2 eu tenho?"

Orquestrador:
  return player.spell_slots.level2

Zero Qwen. Zero vectorizer. Zero drama.
```

---

### 2. QWEN 1.5B — "MESTRE REFLEXO"

Serve para **preencher silêncio** e **simular reação humana imediata**.

#### Funções Permitidas:

- ✅ Reação emocional curta
- ✅ Ack da intenção
- ✅ Mini-narração inconclusiva
- ✅ Perguntas simples de follow-up
- ✅ Clarificação ("Você disse goblin da esquerda?")

#### Funções PROIBIDAS:

- ❌ Resultado final
- ❌ Análise de sistemas
- ❌ Consequências
- ❌ Aplicação de regras
- ❌ Qualquer julgamento do tipo "acertou/errou"
- ❌ Narrativa de 2º ato

#### Estilo de Resposta:

- **1 ou 2 frases**
- **Nunca repetitivas**
- **Nunca formulaicas**
- **Não "enche linguiça"**
- **Deve abrir espaço narrativo**

#### Exemplos:

**BOM**:
```
"Interessante. Você segura firme a lâmina, sentindo o calor da batalha."
```

**RUIM**:
```
"Interessante… você corre… ok… certo… certo… tá…"
```

O 1.5B cria **GRAVIDADE** — não "PREENCHIMENTO".

---

### 3. QWEN 14B — "MESTRE REAL"

É o **autor**. O **diretor**. O que **resolve a cena**.

#### Funções Permitidas:

- ✅ Descrever a cena com riqueza
- ✅ Consequências de ações
- ✅ Falhas críticas / sucessos críticos
- ✅ Reações de NPCs
- ✅ Pedidos de teste ("role ataque")
- ✅ Aplicação de regras dentro do contexto
- ✅ Integração de lore / memória
- ✅ Avanço da história

#### PROIBIDO:

- ❌ Repetir texto do 1.5B
- ❌ Contradizer 1.5B
- ❌ Resetar o contexto da ação
- ❌ Explicar regra como manual (a não ser que seja a pedido)

---

## 🧬 Estado e Cache (Núcleo do Sistema)

### O que armazenamos no cache:

#### 1. Contexto de Combate:

- Round atual
- Iniciativa
- HP por entidade
- Status: "poisoned", "stealth", "prone", etc
- Localização (grid 2D/3D)
- Buffs / debuffs
- Recursos (rage, slots, smites, ki)

#### 2. Histórico Curto (últimas 3–8 ações)

Por turno do jogador:

- Ação
- Resultado
- Teste executado
- Interação com NPC

**🛑 Não armazene histórico gigantesco no contexto do prompt.**

Use vector search de memória episódica.

#### 3. Memória de Lore (vectorizer)

- Descrição de raças
- Cidade / regiões / dungeons
- NPCs recorrentes
- História da campanha
- Áreas, facções, crenças
- Estilo narrativo desejado

**LLM consulta vectorizer, não inventa.**

#### 4. Memória do Jogador

- Classe
- Inventário
- Magias preparadas
- Habilidades
- Perícias
- Defeitos / motivação / background

#### Nada de calcular via LLM:

Dano, HP, Armor Class, iniciativa, duração de efeitos → **estado matemático puro**.

---

## 🧭 Fluxo Técnico: Urgente + Cristalino

### SITUAÇÃO: JOGADOR DECLARA AÇÃO

**Exemplo**:
```
"Corro pela lateral da mesa e corto a garganta do goblin."
```

### ETAPA 1 — STT STREAMING

- Whisper processa em chunks (300–600ms)
- Orquestrador recebe `asr_partial`

### ETAPA 2 — INTENT ROUTER

**Simples**:

```
INTENT = ACTION
ENTITY = goblin
VERB = attack
WEAPON = sword
MOVEMENT = lateral
```

### ETAPA 3 — DISPARO AUTOMÁTICO DO 1.5B

**Condição**:

- Fala passou 6–8 segundos
- **OU**
- Pausa detectada
- **OU**
- Ação clara identificada

**Prompt do 1.5B**:

- Máx. 25–40 tokens
- Estilo emocional
- Zero consequência
- Zero regra
- Zero resultado

**BOM**:
```
"Interessante. Você segura firme a lâmina, sentindo o calor da batalha."
```

Vai direto para **XTTS Streaming Pipeline** (não mais SoVITS).

### ETAPA 3.5 — XTTS STREAMING REAL-TIME

**Pipeline de Streaming**:

1. **Semantic Chunker**: Divide texto por pausas narrativas (3-7s, 180-320 chars)
2. **XTTS Worker** (Thread C): Gera chunks em paralelo (High-End) ou sequencial (Modest)
3. **AudioBuffer FIFO**: Thread-safe, Float32 interno, int16 I/O
4. **Pre-Buffer Manager**: Mantém 1-2 chunks à frente (tier-dependent)
5. **Audio Output** (Thread D): Thread dedicada, WASAPI/ASIO/CoreAudio, 256-512 frames

**Controle Adaptativo de GPU**:
- Detecta hardware automaticamente (High-End/Mid-Range/Modest/Low-End)
- Aplica configuração apropriada (paralelização, VRAM limit, pre-buffer)
- Mantém performance em todos os tiers (< 5s latência inicial)

**Otimizações de Áudio**:
- Sample rate: 16-24 kHz (não 48 kHz)
- Channels: Mono (1 canal, não estéreo)
- Buffer: 256-512 frames (não 2048/4096)
- Formato: Float32 interno, int16 I/O

### ETAPA 4 — ESPERAR O JOGADOR TERMINAR

Quando Whisper fecha `asr_final`, orquestrador prepara prompt do 14B:

**NELE**:

- `fast_prelude` (texto 1.5B)
- `asr_final`
- `game_state`
- `context_slice` (últimos 3–6 eventos)
- `vectorizer results` (se relevante)
- ligação com a cena

**E o 14B produz**:

```
"...com um impulso súbito você avança pela lateral.
O goblin tenta erguer o punhal, mas tarde demais—
Faça uma rolagem de ataque."
```

### 🚫 NÃO PODE JAMAIS ACONTECER

**Qwen 1.5B**:
```
"Você corta a garganta dele."
```

**Isso é o 14B.**

---

## 📐 Consulta de Regras

### A) Pergunta Objetiva (Orquestrador)

```
"Quantos slots de magia de nível 3 eu tenho?"
```

**Responde direto dos dados**:

```rust
return player.slots.level3
```

### B) Pergunta de Regra Simples (vectorizer + 1.5B)

```
"Stealth usa Destreza?"
```

**Vectorizer busca definição exata**:

```
Stealth ― habilidade baseada em Destreza.
```

**1.5B converte em resposta humana**:

```
"Stealth usa Destreza. Investigation é Inteligência."
```

### C) Pergunta que Impacta Narrativa

```
"Se eu pular do balcão e tentar acertar pelas costas, ganho vantagem?"
```

**14B entra porque**:

- posição
- movimento
- surpresa
- reação do inimigo
- tensão

---

## 💣 O Ponto que Você Não Quer Errar

**O 1.5B NÃO ENCHE LINGUIÇA.**

Ele cria **"GRAVIDADE"** — não **"PREENCHIMENTO"**.

**Exemplo errado (IA típica)**:
```
"Interessante… você corre… ok… certo… certo… tá…"
```

**Horrível. Mecânico. Artificial.**

**Exemplo correto (humano experiente)**:
```
"Você inspira fundo. Essa decisão diz muito sobre você."
```

**Frase pequena, densa, humana.**

---

## ⚙️ Design Anti-Loop

### 1. Banco Local com 50–300 Frases de "Ponte Humana"

Divididas por emoção

Aleatorizadas

Nunca repetitivas

**Exemplos**:

- "Hmmm… ousado."
- "Você escolhe a via difícil."
- "Isso vai ser interessante."
- "Vamos ver até onde isso vai."

**O 1.5B escolhe, não inventa.**

---

## 🌌 Onde a Arquitetura Quebra (Casos Reais)

### 🔥 Erro 1 — 1.5B narrar demais

Ele vira mini-mestre.

1.5B vira lixo.

14B vira pós-produtor.

### 🔥 Erro 2 — 14B entrar frio

Sem `fast_prelude`, ele gasta tokens:

- recap
- framing
- setup
- emoção

**Latência 2–5s → horrível.**

### 🔥 Erro 3 — Falta de Cache

14B precisa reprocessar contexto → 3–9s

### 🔥 Erro 4 — 1.5B virar manual de regras

Jogador sente "chatbot wiki".

### 🔥 Erro 5 — Orquestrador Fraco

LLM decide a própria função.

**Resultado: caos.**

---

## ⚡ Latência Real

### 1.5B (Prelude)

- parse intent: 30–80ms
- geração: 200–450ms
- XTTS streaming: 150–300ms (primeiro chunk)
- **Total**: 380–830ms

**👉 Resposta inicial < 1.2s**

### 14B (Narrative - Streaming)

- ingest contexto: 200–500ms
- geração narrativa: 1.5–4s (texto completo)
- Semantic chunking: 10–50ms
- XTTS streaming: 1.2–2.8s por chunk (RTF 0.4x)
- Pre-buffer: 1.0–2.5s (tier-dependent)

**👉 Latência inicial: 2.5–4.0s (todos os tiers)**
**👉 Streaming contínuo: Zero gaps, playback fluido**

### Performance por GPU Tier

| Tier | Latência Inicial | RTF | GPU Usage | Pre-Buffer |
|------|------------------|-----|-----------|------------|
| **High-End** | 2.5-3.8s | < 0.5x | 80-95% | 2.5s |
| **Mid-Range** | 2.5-4.0s | < 0.6x | 60-80% | 1.75s |
| **Modest** | 3.0-4.5s | < 0.8x | 40-60% | 1.25s |
| **Low-End** | 3.5-5.0s | < 1.0x | 30-50% | 0.75s |

---

## 💾 Como Salvar o Cache (Baixo Custo)

### game_state (RAM)

- HP
- AC
- recursos
- status
- cooldowns
- posição
- iniciativa

### scene_context (RAM + Vector)

- últimas ações (3–6)
- resultado de rolagens
- NPCs ativos
- quem interagiu com quem

### lore_context (Vectorizer)

- queries curtas
- textos originais
- passagens relevantes

---

## 🧊 Última Regra

**Se a pergunta puder ser respondida sem imaginação,**

**LLM NÃO DEVE SER CHAMADO.**

---

## Implementação Técnica

### Estrutura de Dados

```rust
pub struct PipelineState {
    // Estado do jogo (RAM)
    pub game_state: GameState,
    
    // Contexto da cena (RAM + Vector)
    pub scene_context: SceneContext,
    
    // Cache de memória (Vectorizer)
    pub lore_cache: LoreCache,
    
    // Estado do pipeline
    pub pipeline_status: PipelineStatus,
}

pub enum PipelineStatus {
    WaitingForInput,
    Processing1_5B,      // 1.5B está gerando prelúdio
    WaitingForFinalASR,  // Aguardando asr_final
    Processing14B,       // 14B está gerando narrativa completa
    ReadyForTTS,
}
```

### Fluxo de Execução (Streaming Real-Time)

```rust
impl Orchestrator {
    async fn handle_player_input(&mut self, asr_partial: &str) -> Result<()> {
        // 1. Parse intent
        let intent = self.parse_intent(asr_partial)?;
        
        // 2. Verificar se deve disparar 1.5B
        if self.should_trigger_1_5b() {
            // Disparar 1.5B em paralelo (Thread A)
            let prelude = self.trigger_1_5b(intent.clone()).await?;
            
            // Enviar para XTTS Streaming Pipeline imediatamente
            // Thread C: XTTS Worker gera chunks
            // Thread D: Audio Consumer toca em streaming
            self.send_to_tts_streaming(prelude).await?;
        }
        
        // 3. Aguardar asr_final
        let asr_final = self.wait_for_final_asr().await?;
        
        // 4. Preparar contexto para 14B
        let context = self.prepare_14b_context(asr_final, intent).await?;
        
        // 5. Gerar narrativa completa com 14B (Thread B)
        let narration = self.trigger_14b(context).await?;
        
        // 6. Enviar para XTTS Streaming Pipeline
        // Semantic Chunker → XTTS Worker (Thread C) → AudioBuffer FIFO → Audio Output (Thread D)
        self.send_to_tts_streaming(narration).await?;
        
        Ok(())
    }
    
    async fn send_to_tts_streaming(&mut self, text: String) -> Result<()> {
        // Thread C: XTTS Worker (adaptive parallel/sequential)
        // - Semantic chunking (3-7s chunks)
        // - GPU adaptive control (tier-based)
        // - Pre-buffering (1-2 chunks ahead)
        // - Push to AudioBuffer FIFO
        
        // Thread D: Audio Consumer (dedicated I/O)
        // - Pop from AudioBuffer FIFO
        // - Convert Float32 to int16
        // - Native audio output (WASAPI/ASIO/CoreAudio)
        // - Zero-gap playback
        
        Ok(())
    }
}
```

### Configuração de Modelos

```json
{
  "models": {
    "qwen_1_5b": {
      "path": "assets-and-models/models/llm/qwen2.5-1.5b-instruct-q4_k_m.gguf",
      "max_tokens": 40,
      "temperature": 0.8,
      "top_p": 0.9,
      "role": "prelude"
    },
    "qwen_14b": {
      "path": "assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf",
      "max_tokens": 2048,
      "temperature": 0.7,
      "top_p": 0.9,
      "role": "narration"
    }
  }
}
```

---

## Testes

### Teste 1: 1.5B não deve narrar resultado

**Input**: "Eu ataco o goblin"

**1.5B esperado**: "Você avança com determinação."

**1.5B não deve**: "Você acerta o goblin e causa 8 de dano."

### Teste 2: 14B recebe prelúdio

**Verificar**: Contexto do 14B contém `fast_prelude` do 1.5B

### Teste 3: Orquestrador responde perguntas objetivas

**Input**: "Quantos HP eu tenho?"

**Esperado**: Resposta direta do estado, sem chamar LLM

### Teste 4: Latência do pipeline

**Target**: 
- 1.5B resposta < 1.2s
- 14B resposta < 6s total

---

## Referências

- [ORCHESTRATOR.md](ORCHESTRATOR.md) - Especificação técnica do orquestrador
- [LLM_CORE_SPEC.md](specs/LLM_CORE_SPEC.md) - Especificação dos modelos LLM
- [PERFORMANCE.md](PERFORMANCE.md) - Métricas de performance e latência

