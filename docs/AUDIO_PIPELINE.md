# VRPG — Arquitetura de Voz Local (Pipeline de 3 Agentes: Qwen-1.5B → Qwen-14B → XTTS)

O objetivo do pipeline sonoro do VRPG é permitir que o jogador experimente uma mesa real, narrada por um Mestre IA com vozes de NPCs, acompanhado por jogadores IA com personalidade, música dinâmica, e efeitos sonoros emergentes, sem rupturas.

---

## 🔥 Princípio Central

**O LLM não é a voz. A voz não é o LLM. A emoção não é o texto.**

Cada camada tem uma responsabilidade única:

- **Qwen-1.5B cria reação humana imediata (prelúdio).**
- **Qwen-14B cria a intenção narrativa completa e a fala.**
- **XTTS (Coqui XTTS v2) sintetiza o texto diretamente com a voz do personagem usando embeddings personalizados.**

Isso elimina latência, evita instabilidade emocional do TTS end-to-end e garante consistência vocal entre sessões.

**Regra de Ouro**: O 1.5B sempre responde antes do 14B para evitar silêncio cognitivo.

---

## 1. Arquitetura Geral de Áudio

O áudio do VRPG é dividido em 4 camadas independentes:

1. **Voz mestre** (Narrador + NPCs)
2. **Voz jogadores IA**
3. **Música procedural**
4. **Sound FX** (ambiente / ações / combate)

Cada camada possui:

- modelo local dedicado
- buffer PCM
- mixagem dinâmica
- prioridade temporal

**A voz sempre vence a música.**  
Efeitos sonoros não interrompem fala.

---

## 2. Objetivo Principal: Zero API

❌ **Nada de ElevenLabs ou TTS web.**

Mesmo se a qualidade "pareça melhor", a latência destrói a imersão:

```
whisper → texto → API → resposta → download → playback
= 1500–2500ms + jitter
→ ruim em RPG narrativo.
```

✔️ **IA local com arquitetura em 3 camadas:**

1. **Qwen 2.5 1.5B (q4_K_M)** → reação humana imediata, prelúdio emocional (< 1.2s) → **Perfil FAST TTS** (≤ 0.8s latência)
2. **Qwen 2.5 14B (q4_K_M)** → raciocínio, contexto, intenção narrativa completa, direção emocional (< 6s) → **Perfil CINEMATIC TTS** (1.5-3s latência)
3. **XTTS v2 (Coqui)** → síntese direta com voz do personagem usando embeddings personalizados (velocidade, inteligibilidade, multi-idioma, identidade vocal) → **Streaming real com FIFO** (não batch)

---

## 3. QWEN 1.5B — "O reflexo humano"

### Função:

- gerar **reação emocional imediata** (1-2 frases, 15-45 palavras)
- preencher silêncio cognitivo enquanto o 14B prepara resposta completa
- simular a "respiração" de um mestre humano experiente
- **NUNCA** narrar resultados, aplicar regras, ou resolver ações

**Ver especificação completa em [QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md)**

---

## 4. QWEN 2.5 14B — "O cérebro"

### Função:

- gerar **fala dramática** (não só respostas)
- interpretar o estado da cena e do turno
- decidir **quem está falando**
- gerar **direção de atuação** para XTTS (via Voice INTENTs)
- modelar as intenções internas:
  - medo, arrogância, calma, sarcasmo, fúria
- contextualizar com lore, regras, passado e decisões anteriores
- **TTS**: Usa **Perfil CINEMATIC** (1.5-3s latência, primeiro chunk de 100 chars)

### Exemplo de saída ideal de Qwen:

```xml
<VOICE actor="NPC_Cultist" emotion="rage" style="crackled">
"TRAIDORES! O Deus das Cinzas os consumirá!!"
</VOICE>
```

**Qwen nunca gera áudio.**  
**Qwen gera texto + metadados emocionais.**

---

## 5. XTTS — "A voz do personagem"

O XTTS (Coqui XTTS v2) é o sintetizador TTS ultra-rápido, local, de baixa latência que gera áudio diretamente com a voz do personagem.

**O XTTS é a voz completa do personagem, não apenas síntese neutra.**

### Funções do XTTS:

- converter a fala do Qwen → áudio com voz do personagem
- usar embeddings personalizados (reference WAV) para cada personagem
- output consistente e natural
- 100% offline
- 50–200ms/inferência (dependendo do hardware e GPU)
- Multi-idioma nativo
- Suporte GPU para latência reduzida
- **Áudio RAW (sem processamento) = melhor qualidade**

### Streaming Real-Time Cinematográfico

O XTTS agora suporta **streaming real-time cinematográfico** com perfis de performance adaptativos:

#### Perfis de Performance TTS

O sistema implementa dois perfis distintos para otimizar latência:

**Perfil FAST (Qwen 1.5B)**:
- **Primeiro chunk**: 30 caracteres máximo (~0.7-1.0s de fala)
- **Próximos chunks**: 90 caracteres máximo (~2-3s de fala)
- **Sample rate**: 16 kHz (mono)
- **Precisão**: FP16 (half precision)
- **Audio blocks**: 50ms (800 samples @ 16 kHz)
- **Pre-buffer inicial**: 240ms
- **Target latência**: ≤ 0.8s (ideal 0.5-0.7s)
- **Uso**: Respostas rápidas do Qwen 1.5B (prelúdio emocional)

**Perfil CINEMATIC (Qwen 14B)**:
- **Primeiro chunk**: 100 caracteres máximo (~3s de fala)
- **Próximos chunks**: 150 caracteres máximo (~4-5s de fala)
- **Sample rate**: 24 kHz (mono)
- **Precisão**: FP16 (half precision)
- **Audio blocks**: 60-80ms (1440-1920 samples @ 24 kHz)
- **Pre-buffer inicial**: 500ms
- **Target latência**: 1.5-3s
- **Uso**: Narrativas completas do Qwen 14B

#### Streaming Real-Time

- **Chunking adaptativo**: Primeiro chunk minúsculo (FAST) ou moderado (CINEMATIC)
- **FIFO streaming**: Blocos de 50-80ms empurrados imediatamente para fila
- **Pre-buffering adaptativo**: 240ms (FAST) ou 500ms (CINEMATIC) antes de iniciar playback
- **Paralelização adaptativa**: 2-3 CUDA streams (High-End) ou sequencial (Modest)
- **FIFO buffer**: Thread-safe, zero-gap playback
- **Thread dedicada**: I/O de áudio isolada (não compartilha com UI/modelo)
- **Latência inicial**: ≤ 0.8s (FAST) ou 1.5-3s (CINEMATIC)
- **Continuidade**: Zero gaps entre chunks após início

### Controle Adaptativo de GPU

O sistema detecta automaticamente o hardware e adapta configuração:

- **High-End** (RTX 5090): 2-3 streams paralelos, 2.5s pre-buffer, 80-95% GPU
- **Mid-Range** (RTX 3070): 1-2 streams, 1.75s pre-buffer, 60-80% GPU
- **Modest** (RTX 3050): 1 stream sequencial, 1.25s pre-buffer, 40-60% GPU
- **Low-End** (< 4GB): 0-1 stream, 0.75s pre-buffer, 30-50% GPU

**Performance mantida em todos os tiers**: < 5s latência inicial, zero gaps, sistema responsivo.

### Otimizações de Áudio

- **Sample rate**: 16-24 kHz (suficiente para voz, NÃO 48 kHz)
- **Channels**: Mono (1 canal, NÃO estéreo - 50% menos banda)
- **Buffer size**: 256-512 frames (baixa latência, NÃO 2048/4096)
- **Formato I/O**: int16 PCM (eficiente, compatível Opus, NÃO float64)
- **Formato interno**: Float32 (inferência XTTS, preserva qualidade)

### Por que XTTS com Embeddings?

Porque:

- **Qualidade superior**: Áudio RAW do XTTS é infinitamente melhor que qualquer processamento
- **Latência baixa**: Síntese direta sem camadas adicionais
- **Escalável**: Um embedding por personagem (fácil de criar e gerenciar)
- **Natural**: Voz preserva características naturais do personagem
- **Sem artefatos**: Processamento adicional degrada qualidade

**Em VRPG você quer:**

Texto com emoção → XTTS com embedding do personagem → Áudio final perfeito.

### Embeddings XTTS

Cada personagem tem seu próprio **reference WAV** (embedding) que define:
- Timbre único
- Sotaque
- Características vocais
- Identidade do personagem

**Criar embedding:**
- Colete 5-10 minutos de áudio limpo do personagem
- Use `create_clean_xtts_embedding.py` para processar e normalizar
- Salve como `{character_id}_xtts_reference_clean.wav`
- Use no XTTS via `speaker_wav` parameter

**VRPG = teatro. Você precisa de embeddings bem feitos.**

---

## 7. Fluxo Completo (turnos e narrativa)

### 🎭 Entrada de Voz do Jogador

```
[Whisper local] → Texto (asr_partial / asr_final)
```

### 🧠 Interpretação (Pipeline de 2 Modelos)

```
Texto parcial (6-8s) → Qwen 1.5B → Prelúdio emocional (< 1.2s)
    ↓
Texto final → Qwen 14B → Intenção + Fala + Emoção completa (< 6s)
```

### 🎵 Sistema de Interjeições

**Objetivo**: Mascarar latência do TTS em respostas longas com interjeições pré-gravadas.

**Funcionamento**:
- **Detecção**: Heurístico `expected_duration = text_length_chars / 25.0`
- **Threshold**: 3.0s (CINEMATIC) ou 4.0s (FAST - mais conservador)
- **Delay**: 1.5s desde fim da fala do jogador até início da interjeição
- **Seleção**: Evita repetir últimas 5 interjeições usadas
- **Reprodução**: Interjeição → Gap (50ms) → TTS Principal

**Assets**:
- **Localização**: `assets-and-models/voices/interjections/`
- **Total**: 53 interjeições e frases curtas
- **Formato**: WAV, Float32, 24kHz mono
- **Duração média**: ~1.9s

**Integração**:
- TTS gera em paralelo enquanto interjeição toca
- Elimina "silêncio cognitivo" em respostas longas
- Experiência natural: DM "pensa" antes de responder

**Ver documentação completa**: [INTERJECTIONS_SYSTEM_COMPLETE.md](../src/tts-service/docs/INTERJECTIONS_SYSTEM_COMPLETE.md)

### 🔊 Conversão Vocal (Streaming Real-Time com Perfis)

**Qwen 1.5B → Perfil FAST** (≤ 0.8s latência):
```
1.5B_output → Chunker FAST (30 chars primeiro) → XTTS (16 kHz, FP16) → FIFO (50ms blocks)
    ↓
AudioBuffer FIFO (Float32 → int16)
    ↓
Audio Output Thread (dedicada, WASAPI/ASIO/CoreAudio)
    ↓
Playback contínuo (zero gaps, pre-buffer 240ms)
```

**Qwen 14B → Perfil CINEMATIC** (1.5-3s latência):
```
14B_output → Chunker CINEMATIC (100 chars primeiro) → XTTS (24 kHz, FP16) → FIFO (60-80ms blocks)
    ↓
AudioBuffer FIFO (Float32 → int16)
    ↓
Audio Output Thread (dedicada, WASAPI/ASIO/CoreAudio)
    ↓
Playback contínuo (zero gaps, pre-buffer 500ms)
```

**Thread Architecture:**
- **Thread A**: Qwen 1.5B → Prelude
- **Thread B**: Qwen 14B → Narrative
- **Thread C**: XTTS Worker (adaptive parallel/sequential)
- **Thread D**: Audio Consumer (dedicated I/O, não bloqueia geração)

### 📢 Reprodução

```
AudioEngine → WebRTC → Cliente
```

**Regra**: O 1.5B sempre toca antes do 14B para evitar silêncio.

---

## 8. Exemplo de ciclo na prática

**Jogador:**

"Eu tento persuadir o guarda dizendo que sou emissário."

**Pipeline:**

1. **Whisper → texto parcial (6-8s)**

```
"Eu tento persuadir o guarda..."
```

2. **Qwen 1.5B → prelúdio emocional (< 1.2s)**

```
"O guarda observa você com desconfiança."
```

3. **Whisper → texto final**

```
"Eu tento persuadir o guarda dizendo que sou emissário."
```

4. **Qwen 14B → narrativa completa (< 6s)**

```xml
<VOICE actor="NPC_Guard" emotion="skeptic" style="dry">
"Emissário? De qual reino? Mostre sua insígnia!"
</VOICE>
```

5. **XTTS (para ambos)**

- 1.5B output → XTTS (com embedding do personagem) → Voz Final (prelúdio toca primeiro)
- 14B output → XTTS (com embedding do personagem) → Voz Final (narrativa completa depois)

🎧 **Resultado final → NPC real com resposta imediata + narrativa completa**

---

## 9. Direção emocional via tags

Tags obrigatórias que o Orquestrador envia ao XTTS:

- `actor` → Define qual embedding usar (qual personagem)
- `emotion` → Contexto emocional (pode influenciar pitch/velocidade)
- `style` → Estilo de fala
- `volume` → Volume relativo
- `pace` → Velocidade de fala
- `context` → Contexto narrativo

**Exemplo:**

```xml
<VOICE actor="Wizard_Elder"
 emotion="pain"
 style="ancient_whisper"
 pace="slow"
 volume="low">
"Você tocou o fogo que jamais foi para os vivos…"
</VOICE>
```

**O XTTS usa o embedding do personagem para gerar voz natural e consistente.**

---

## 10. Embeddings XTTS por personagem

**Regra de ouro:**

1 embedding (reference WAV) por personagem importante.

- Protagonista
- Vilão
- Companheiros fixos
- NPCs recorrentes

**NPCs "menores"** podem usar embedding genérico OU compartilhar embeddings similares.

**Criar embedding:**
1. Colete 5-10 minutos de áudio limpo do personagem
2. Use `create_clean_xtts_embedding.py` para processar e normalizar
3. Salve como `{character_id}_xtts_reference_clean.wav`
4. Coloque em `assets-and-models/models/tts/xtts_embeddings/`

---

## 11. Onde o 1.5B e 14B brilham

### Qwen-1.5B:
- Reação humana imediata (< 1.2s)
- Previne silêncio cognitivo
- Cria expectativa e gravidade emocional
- Não resolve, apenas antecipa

### Qwen-14B:
- Conexões entre sessões
- Memória narrativa via Vectorizer
- Coerência de papel social
- Motivação real
- Loucura controlada
- Lore oculto
- Reações / Consequências completas

**A combinação 1.5B + 14B entrega latência humana com qualidade narrativa completa.**

---

## 12. Por que essa arquitetura é imbatível

❌ **Apenas TTS:**
- robótico
- pouco estável
- sem acting
- mata a fantasia

❌ **Apenas RVC:**
- voz "achatada"
- pobre para monólogos
- bom para vtuber, NÃO VRPG

✔️ **Qwen + XTTS (com embeddings):**
- autonomia narrativa
- acting real
- emoção cinematográfica
- baixa latência
- escalável para 50 NPCs
- qualidade superior (áudio RAW)

---

## 13. Lógica de orquestração (simples e clara)

```yaml
if SPEAKER == PLAYER:
    → Whisper → Texto para Qwen

if SPEAKER == NPC or MASTER:
    Qwen → Fala + Emoção
    XTTS(embedding_personagem) → Áudio Final
```

---

## 14. Performance Real (cenário PC)

### Qwen-1.5B (prelúdio):
- **Geração**: 200–500ms
- **XTTS**: 150–300ms
- **Total**: 350–800ms (< 1.2s target)

### Qwen-14B (narrativa completa):
- **Geração**: 1.5–4s (resposta média), 8–15s (resposta longa)
- **XTTS Streaming**: 1.2–2.8s por chunk (RTF 0.4x)
- **Total**: 2.5–4.0s (inicial) + streaming contínuo

**Latência percebida pelo jogador:**

≈ 0.6–1.2s (prelúdio) → ≈ 2.5–4.0s (narrativa completa inicia) → streaming contínuo sem gaps

**Conversação fluida estilo Discord com resposta imediata + streaming cinematográfico**

### Performance por GPU Tier:

| Tier | Latência Inicial | RTF | GPU Usage | Pre-Buffer |
|------|------------------|-----|-----------|------------|
| **High-End** (RTX 5090) | 2.5-3.8s | < 0.5x | 80-95% | 2.5s |
| **Mid-Range** (RTX 3070) | 2.5-4.0s | < 0.6x | 60-80% | 1.75s |
| **Modest** (RTX 3050) | 3.0-4.5s | < 0.8x | 40-60% | 1.25s |
| **Low-End** (< 4GB) | 3.5-5.0s | < 1.0x | 30-50% | 0.75s |

**Todos os tiers mantêm zero-gap playback e sistema responsivo.**

---

## 15. Filosofia

**Texto é roteiro.**  
**XTTS com embedding é a voz completa do personagem.**

Você não está programando um chatbot.  
Você está construindo um diretor de RPG com atores reais.

---

## 16. Identificador de fala (UI)

Você pediu:

> "Indicador mostra quem está falando (jogador / mestre / npc)"

**UI acoplada ao mixer:**

- Cada player agent / mestre / npc tem ID de canal
- Playback registra "speaker"
- UI abre highlight no card correspondente
- A animação força foco do jogador sem atrapalhar input

**Formato:**

- radial glow na portrait
- onda minimalista (não onda de waveform de whatsapp real)

---

## 17. Música Procedural

**Zero trilha estática codificada.**

Você quer:

- motivos por ambiente
- intensidade por fase:
  - explorativa
  - social
  - tensão
  - combate
  - resultado

**Modelos recomendados:**

- Suno local OFF? (não existe oficialmente)
- Riffusion / Harmonai / AudioLDM locais
- Música modular — loops de 30s–60s em camadas

**Valor real: camadas.**

**Exploração:**

- base pad
- percussão suave
- cordas mornas

**Combate:**

- ativa layer de ritmo
- ativa brass
- subgrave

**Vitória:**

- corta ritmo
- mantém cordas
- sobe arpeggio pequeno

**Morte / derrota:**

- remove paleta alta
- reverb longo
- sub caindo

---

## 18. Sound FX Dinâmico

**Categoria A — ambiente:**

- vento
- chuva
- taverna (copos, murmurinho)
- floresta
- dungeon dripping

**Categoria B — ações:**

- abrir porta
- pegar item
- passos diferentes (madeira/pedra/água)

**Categoria C — combate:**

- espada
- flecha
- magia
- impacto crítico

**Sistema:**

```
evento → envelope → mix → prioridade
```

**Não use wav "cru".**  
Use assets com curva ADSR:

- attack (rápido)
- sustain (curto)
- release (programado)

Misturar som seco com ambientes → imersão.

---

## 19. Callback Narrativo Épico

Você pediu:

> "Quando uma condição acaba, o mestre deve ser avisado para narrar."

**Exemplo:**

- Buff dura 5 turnos
- Turno 6 → engine manda callback
- Mestre IA responde

> "A centelha rubra deixa seus músculos… você sente o peso de volta."

Isso é 100% áudio + narrativa.

O jogador não vê "BUFF EXPIROU".  
Ele ouve.

---

## 20. Integração com Turn-based Engine

**Turno ≠ mensagem de texto.**  
**Turno = momento dramático auditivo.**

**Fluxo:**

1. Engine manda: `EVENT: initiative_rolled`
2. Música sobe layer "ritmo"
3. SFX toca "switch to combat"
4. Mestre narra
5. Jogadores IA reagem com fala (não com números)

**Quando turno conclui:**

- callback: `END_TURN`
- se ninguém falar → SFX "soft pass"

---

## 21. Perfis Vocais Internos (XTTS Embeddings)

Crie uma estrutura:

```
xtts_embeddings/
    narrator_default_xtts_reference_clean.wav
    npc_guard_xtts_reference_clean.wav
    npc_barkeep_xtts_reference_clean.wav
    npc_mysterious_woman_xtts_reference_clean.wav
    race_drow_xtts_reference_clean.wav
    monster_undead_xtts_reference_clean.wav
```

**Cada embedding XTTS:**

- reference WAV processado e normalizado
- 5-10 minutos de áudio limpo do personagem
- características vocais preservadas
- qualidade RAW (sem processamento adicional)

**1 embedding por personagem importante.**  
**NPCs menores podem compartilhar embeddings similares.**

**Criar embedding:**
- Use `create_clean_xtts_embedding.py` para processar dataset
- Salve como `{character_id}_xtts_reference_clean.wav`
- Use no XTTS via `speaker_wav` parameter

---

## 22. Whisper Local

**Whisper tiny / small GPU:**

- ideal para inglês
- responde <80ms
- latência zero de rede
- integração direta no client

**Jogador fala → evento RAW → IA reage.**

**Sem PTT (push to talk) se possível:**

- detecte início/fim de fala por amplitude RMS
- evita mecânica de "rádio Discord" dentro do RPG

---

## 23. Emulando Mesa REAL (dica psicológica)

**A voz nunca deve começar abrupta:**

- fade-in de 30–50ms

**A música nunca para instantânea:**

- crossfade 400–900ms

Você não está "tocando áudio".  
Está controlando emoção.

---

## 24. Mobile vs PC

**PC (GPU disponível)**

- Qwen 1.5B q4_K_M (reação rápida)
- Qwen 14B q4_K_M (narrativa completa)
- XTTS v2 (síntese com embeddings por personagem)
- FX + música dinâmicos

**Mobile**

- Client não renderiza áudio complexo.
- Ele streama PCM do servidor host da sessão (mestre).
- EVC local (leve)
- mix parcial
- cache de samples
- nunca gerar TTS local mobile

---

## 25. Docker & Deploy

**Ideal:**

- Contêiner de voz
- Contêiner de FX
- Contêiner de música procedural

**Cada um expõe API interna:**

- `generate_voice(text, character_embedding_path)`
- `play_sfx(event_id)`
- `music_state(phase)`

**Sem API externa web.**

---

## 26. Falhas / fallback

**Se TTS travar:**

- avatar UI pisca
- narração é substituída por texto
- engine "preenche silêncio"

**Sem "bugs audíveis".**

---

## 27. Benefício REAL do sistema

VRPG não é Foundry com VTTS.

Você tem:

- mestre que respira
- NPCs que soam vivos
- jogadores IA que discutem
- música que reage
- combate que soa pesado

Isso entrega imersão de Critical Role para 1 pessoa — em local.

---

## 28. Voice INTENTS (Design Técnico e Funcional)

Este sistema define **a API de alto nível** que o Mestre IA utiliza para produzir VOZ em runtime, sem API externa, com baixa latência e coerência dramática.

### Filosofia Central

Voz não é "speech synthesis". **Voz é contexto dramático.**

O VRPG usa INTENTS de voz que funcionam como **ordens direcionais**, nunca como texto bruto do LLM.

O Mestre não produz um áudio "quando quer". Ele produz áudio **quando uma INTENT de voz é acionada**.

Isso permite:
- menor latência
- ritmo natural
- sincronização com música e efeitos
- UI consistente (quem está falando)

### Estrutura de Intent

**Formato (sempre)**:
```
[VOICE_INTENT:<tipo>]
payload {
speaker: enum,
style: enum,
emotion: enum,
text: string,
meta: {...}
}
```

> O **LLM NÃO gera áudio**. Ele **gera a INTENT**, e o módulo de voz executa o áudio.

### Categorias de VOICE_INTENT

#### VOZ_MESTRE
Narra o mundo, descreve ambientes, resolve transformações.

```
[VOICE_INTENT:NARRATE]
{
speaker: "mestre",
style: "neutral",
emotion: "calm",
text: "O corredor é estreito, iluminado por tochas antigas."
}
```

**Uso**: Introduções de cena, descrições de ambiente, resolução de ações fora de combate, escalonamento narrativo.

#### VOZ_NPC
O Mestre IA interpreta um personagem específico.

```
[VOICE_INTENT:NPC_DIALOGUE]
{
speaker: "npc_guard",
style: "gravel_low",
emotion: "mild_irritation",
text: "Não tenho tempo pra vocês. Sigam andando."
}
```

**Notas**: `speaker` deve apontar para **perfil vocal** carregado. `emotion` ajusta pitch/ritmo. NPC não fala sobre mecânica.

#### VOZ_PLAYER_IA
Jogadores IA interpretam seus personagens de forma diegética.

```
[VOICE_INTENT:PLAYER_DIALOGUE]
{
speaker: "player_rogue",
style: "casual",
emotion: "amused",
text: "Relaxa... eu abro a porta. Só preparou a magia, né?"
}
```

**Regras**: Nunca explicar status mecânico. Reagir emocionalmente a eventos. Interagir como humano real numa mesa.

#### VOZ_EVENT (combat / drama)
Trilhas de áudio narrativas rápidas para **impacto psicológico**.

```
[VOICE_INTENT:EVENT]
{
speaker: "mestre",
style: "intense",
emotion: "danger",
text: "O ogro avança e a sala inteira treme com o impacto."
}
```

**Uso**: Entrada de boss, desastre ambiental, traição / revelação.

#### VOZ_CONDIÇÃO
Condições temporárias **terminando** ou **iniciando**.

```
[VOICE_INTENT:CONDITION_EXPIRE]
{
speaker: "mestre",
style: "neutral",
emotion: "solemn",
text: "A energia rubra abandona seus músculos. A dor retorna."
}
```

> Não "+2 acabou". Só narração diegética.

#### VOZ_SISTEMA
Mensagens de segurança **sem quebrar a ficção**.

```
[VOICE_INTENT:SYSTEM]
{
speaker: "mestre",
style: "low",
emotion: "neutral",
text: "Preciso de alguns segundos para organizar a cena."
}
```

**Contextos**: carga de assets, latência momentânea, delays de GPU.

### Modelo de Diálogo Dinâmico

A IA **não narra monólogos de 40 segundos**. Ela cria **turnos emocionais curtos**.

Exemplo:
```
[VOICE_INTENT:NPC_DIALOGUE] — 2s
[VOICE_INTENT:PLAYER_DIALOGUE] — 1.5s
[VOICE_INTENT:NARRATE] — 3s
```

> É ritmo teatral. RPG é micro-jazz conversacional.

### Multiplicidade de Vozes

**Perfis vocais = skin de áudio.**

Em runtime, não re-treine modelo TTS. Você troca perfis:
- `npc_barkeep`
- `npc_royal_guard`
- `npc_old_sage`
- `villain_primary`
- `monster_shadow`

Cada perfil possui: pitch base, tempo base, instabilidade, "grain".

### Perfis de Voz e Emocionais

Um perfil pode falar com várias emoções.

Ex:
```
speaker: "npc_barkeep",
style: "warm_low",
emotion: "fear"
```

Evitar:
```
speaker: "npc_barkeep",
emotion: "screaming rage"
```
se ele é tímido/quieto.

**Emocionalidade sempre coerente com personagem.**

### Integração com Música

O áudio controla a música, não o contrário.

Exemplo:
- **Iniciativa rolada**: `[VOICE_INTENT:EVENT] → música sobe layer_rhythm` + `[SFX:MILITIA_GONG]`
- **Combate encerrado**: `[VOICE_INTENT:NARRATE] → música cai → layer_relief`

### Integração com FX

Efeito **não interrompe fala**. Fala sempre PRIORIDADE.

Quando evento climático é narrado, a engine sonoriza:
```
[VOICE_INTENT:NARRATE]
"A chuva escorre pelas pedras..."

→ [SFX:RAIN_LIGHT_LOOP]
```

### Whisper → INTENT

Pipeline social:
```
jogador humano fala →
whisper local → texto →
Mestre IA pensa →
gera INTENT →
motor TTS reproduz.
```

Ele não retorna textão infinito: fala no tom correto com emoção.

### Priorização

| Prioridade | Tipo |
|---|---|
| 1 | VOICE_INTENT:NARRATE |
| 2 | PLAYER_DIALOGUE |
| 3 | NPC_DIALOGUE |
| 4 | EVENT |
| 5 | FX |
| 6 | MUSIC |

### Tempos de pausa (importante)

- 200–600ms entre falas
- 1000–2000ms após revelações
- 500–800ms antes de decisão tática

Isso gera **dramaturgia**.

### Modo Falha de Voz

Se TTS ou perfil falhar:
- UI exibe texto do mestre
- música baixa
- indicador visual mostra "voz indisponível"
- nunca som robótico glitchado

### Anti-spam

O Mestre IA nunca envia 10 falas seguidas. Ele passa a vez à party IA ou ao jogador humano.

### Linguagem proibida

- "+5 de CA"
- "Use DEX"
- "Você tem vantagem / desvantagem"

**Sempre metafórico.**

### Linguagem ideal

- fisiológica (respiração, fadiga)
- espacial (pressão, eco)
- emocional (raiva, medo)
- estética (luz, textura, som)

### API Interna (para engine)

Audio Engine expõe:
```
queue_voice(intent)
stop_voice()
play_sfx(event_id)
set_music_state(state)
```

UI consome:
```
on_voice_speaker(id:string)
```

### Output Final

> A voz do jogo não é "fala do modelo". É um **teatro auditivo** controlado por INTENTS.

Narrador cria mundo. NPCs respiram nele. Jogadores IA conversam com você. Música acompanha. FX reforça.

**Tudo local. Sem API. Sem streaming externo.**

---

## 29. Resumo Final em 1 frase

> **Som = emoção.**  
> Voz é o narrador invisível que cola a ficção na psique do jogador.  
> Por isso o áudio deve ser local, vivo e instantâneo.  
> O pipeline 1.5B → 14B → XTTS garante resposta imediata sem sacrificar qualidade narrativa.  
> **Áudio RAW do XTTS = qualidade perfeita, sem processamento que degrada.**

---

## Referências

- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura completa do pipeline
- [QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md) - Especificação do Qwen-1.5B
- [QWEN_14B_SPEC.md](QWEN_14B_SPEC.md) - Especificação do Qwen-14B
- [ORCHESTRATOR.md](ORCHESTRATOR.md) - Especificação do orquestrador

