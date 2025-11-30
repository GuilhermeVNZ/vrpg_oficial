# VRPG — Pipeline de Treinamento (Estilo, Acting e Worldbuilding)

> ⚠️ **Aviso importante**  
> Este documento descreve **como estruturar** um pipeline de treino/fine-tuning/LoRA em termos técnicos.  
> Antes de usar qualquer obra (Critical Role, Dimension20, livros, etc.), você é responsável por garantir **direitos legais / licença / autorização** para uso desses materiais em treinamento de modelos.

---

## 1. Objetivo Geral do Treino

O pipeline de treino para o VRPG tem três focos principais:

1. **Estilo de narração e acting**  
   - Ensinar o modelo a mestrar / interpretar **como mesa de RPG**, não como chatbot.
   - Inspirar-se em exemplos reais de jogo (streamings, áudios, campanhas).

2. **Coerência de personagens e worldbuilding**  
   - Ensinar o modelo a construir e sustentar:
     - arcos dramáticos,
     - consistência de NPCs,
     - tom de mundo,
     - estilo de fantasia desejado (dark / high fantasy / anime-esque VRPG).

3. **Melhorar a habilidade de "estruturar intenção"**  
   - Não treinar regras, mas treinar a habilidade de:
     - reconhecer quando uma ação é mecânica,
     - falar como Mestre,
     - em momentos certos, produzir INTENTs na DSL do VRPG.

---

## 2. Componentes do Pipeline

O pipeline de treino se divide em:

1. **Ingestão de conteúdo bruto**
   - Áudio/vídeo (campanhas gravadas).
   - Textos (romances, livros de lore, módulos de aventura, conteúdo próprio).

2. **Transcrição e normalização**
   - ASR → texto com marcação de falantes.
   - Limpeza de ruído (risos, fala sobre stream, OOC).

3. **Anotação e segmentação**
   - Identificação de:
     - momentos de narração ("Mestre");
     - falas de personagens (jogadores, NPCs);
     - momentos mecânicos (pedir teste, rolagens, regras).

4. **Construção de datasets distintos**
   - Dataset de "Mestre narrando".
   - Dataset de "Jogador interpretando".
   - Dataset de "Estrutura de INTENT" (supervisionado).
   - Dataset de worldbuilding (lore + tom literário).

5. **Treino / Fine-tuning / LoRA**
   - **Runtime usa pipeline de 2 modelos**: Qwen 1.5B (reação rápida) + Qwen 14B (narrativa completa)
   - LoRA leve sobre Qwen 2.5 14B (para o modelo principal):
     - 1 cabeça "Mestre" (acting + narrativa completa).
   - LoRA leve sobre Qwen 2.5 1.5B (para reação rápida):
     - 1 cabeça "Mestre Reflexo" (reação emocional imediata).
   - LoRA para "Jogador IA" (acting companion) - pode usar 1.5B ou 14B dependendo do contexto.
   - (Opcional futuro) Modelos menores especializados (distill).

6. **Validação e scoring**
   - Avaliação manual / semi-automática:
     - qualidade de narração,
     - respeito à DSL de INTENT,
     - coerência de estilo.

---

## 3. Ingestão de Conteúdo

### 3.1 Áudio / Vídeo (Sessões de RPG)

Fontes possíveis (desde que legalmente permitidas):

- Sessões próprias gravadas (você + players).
- Conteúdo de terceiros com **autorização formal**.
- Materiais licenciados livremente para esse uso.

**Pipeline ideal:**

1. Download/ingest de vídeo/áudio.
2. Divisão em chunks (ex: 5–10 min).
3. Transcrição com WhisperX (timestamps + diarização básica).
4. Salvar em um formato padronizado (ex: JSONL).

```json
{
  "source": "minha_campanha_s01e01",
  "start": 120.0,
  "end": 180.0,
  "speaker": "DM",
  "text": "Vocês cruzam o portão de pedra, o vento frio corta o rosto..."
}
```

### 3.2 Textos (Livros, Lore e Romances de Fantasia)

Aqui entram:

- textos de mundo próprios que você criar;
- livros SRD/livres;
- modules de aventura que você licenciou;
- contos e romances com permissão.

**Uso:**

- alimentar o estilo literário (tom do mundo);
- extrair trechos de ambientação e descrição;
- rotular "tom X" (dark, high fantasy, grounded, anime-fantasia VRPG).

---

## 4. Transcrição e Normalização

### 4.1 Objetivo

Transformar áudio/vídeo caóticos em um dataset regular:

- quem está falando;
- quando fala;
- o que está sendo dito;
- em que contexto (narração, diálogo, meta, mecânica).

### 4.2 Pipeline sugerido

**WhisperX:**

- transcrição + diarização inicial.

**Classify (Hive):**

- classifica cada trecho em:
  - DM_NARRATION (descrição do mundo),
  - DM_RULE_TALK (fala de regra/metajogo),
  - PLAYER_IN_CHARACTER (fala diegética),
  - PLAYER_OUT_OF_CHARACTER (meta, piada),
  - TABLE_NOISE (off-topic).

**Transmutation:**

- converte em markdown estruturado:
  - blocos de cena,
  - diálogos formatados.

```markdown
## Cena 3 – Entrada na taverna

**DM (NARRATION):**  
As portas rangem, revelando um salão abarrotado...

**Jogador 1 (IN CHARACTER):**  
Me aproximo do balcão, segurando a capa para esconder minha armadura.

**Jogador 2 (IN CHARACTER):**  
Eu fico na porta, observando quem entra e quem sai.
```

---

## 5. Anotação e Segmentação

### 5.1 Rotulagem de papéis

Cada trecho é anotado com:

- role: DM | PLAYER
- mode: narration | dialogue | rules_talk | meta

Assim, o dataset pode separar:

- fala de Mestre (para treinar agent_mindset),
- fala de Jogador (para treinar character_agents).

### 5.2 Segmentos de treino

Segmentos ideais:

- 1–4 turnos de fala por exemplo.

Contendo:

- input (contexto curto + fala anterior),
- output esperado (próxima fala do Mestre/Jogador).

**Formato:**

```json
{
  "role": "DM",
  "mode": "narration",
  "context": [
    {"speaker": "PLAYER_1", "text": "Eu abro a porta com cuidado."},
    {"speaker": "PLAYER_2", "text": "Eu fico atento a qualquer som."}
  ],
  "target": "A porta range, revelando o interior escuro..."
}
```

---

## 6. Dataset para o Mestre IA

### 6.1 O que treinar

Treinar o modelo a:

**Responder como Mestre:**

- Diante de falas de jogadores,
- Narrar o mundo,
- Interpretar NPCs,
- Sugerir pedidos de rolagem (em linguagem natural).

**Estruturar intenção (mas não mecânica):**

- Em versões futuras, você pode treinar com exemplos de DSL:
  - "quando o DM diz X, a INTENT deve ser Y".

### 6.2 Formato de exemplos

**Sem DSL (acting puro):**

```json
{
  "input": "<contexto de cena + falas anteriores>",
  "output": "<fala do DM conforme agent_mindset.md>"
}
```

**Com DSL embutida em bloco (fase avançada):**

```
[CONTEXT]
Jogador: "Quero convencer o guarda a nos deixar entrar."
[/CONTEXT]

[EXPECTED_DM_OUTPUT]
"O guarda te encara por um instante, desconfiado. Sua mão aperta o cabo da lança."
"Faça um teste de Persuasão."
[/EXPECTED_DM_OUTPUT]

[EXPECTED_INTENTS]
INTENT: SKILL_CHECK
ACTOR: player_1
SKILL: persuasion
TARGET: npc_guard_01
CONTEXT: "convencer o guarda a liberar a passagem"
SUGGEST_DC: YES
END_INTENT
[/EXPECTED_INTENTS]
```

Para treino, você pode congelar a parte DM e usar a parte INTENT como alvo para fine-tune de "estruturação de intenção", não de regra.

---

## 7. Dataset para Jogadores IA

### 7.1 Objetivo

Ensinar o modelo a:

- falar como um personagem de party (ver character_agents.md);
- reagir à narração do Mestre;
- ter opinião, humor, medo, convicção.

### 7.2 Exemplo

```json
{
  "input": [
    {"speaker": "DM", "text": "O guarda bloqueia a porta com a lança."},
    {"speaker": "PLAYER_1", "text": "Eu ergo as mãos, mostrando que estou desarmado."}
  ],
  "output": "Respira, vamos tentar conversar primeiro. Deixe que eu falo com ele."
}
```

Você pode criar personagens arquetípicos:

- paladino idealista,
- ladino desconfiado,
- mago arrogante,
- clérigo pragmático.

E treinar cada "persona" separadamente (tagging por personagem).

---

## 8. Worldbuilding (Lore + Tom Literário)

### 8.1 Construção da "Biblioteca de Mundo"

Você vai alimentar o Hive (Vectorizer + Nexus + Lexum) com:

**Lore Hard (fatos do mundo):**

- mapas,
- cronologias,
- facções,
- deuses,
- cidades,
- conflitos históricos.

**Lore Soft (pedaços de texto com flavor):**

- descrições de locais;
- textos de ambientação;
- contos curtos;
- diálogos memoráveis.

**Tom de Fantasia VRPG:**

- aqui entram seus próprios textos,
- mais passagens de autores/livros com licença, para inspirar a "voz" do mundo.

### 8.2 Uso no treino

Em vez de "ensinar tudo de worldbuilding" dentro do modelo (o que é rígido e arriscado), você faz:

1. O modelo aprende a pedir lore via INTENT (LORE_QUERY).
2. O modelo aprende a usar lore recebido para narrar.

Você pode ter dataset de exemplos do tipo:

```
[CONTEXT]
LORE_SNIPPETS:
- "A cidade de Myrvalen foi erguida sobre ruínas antigas..."
- "Os Magos Rubros de Thay são temidos por seus pactos..."

JOGADOR: "O que eu sei sobre esses magos?"
[/CONTEXT]

[EXPECTED_DM_OUTPUT]
"Você se lembra de histórias sussurradas em tavernas: magos com mantos escarlates, conhecidos por vender favores em troca de partes da alma."
```

Assim você treina estilo de uso da memória, não "memória hardcoded".

---

## 9. Estrutura Técnica do Pipeline

### 9.1 Stages

1. **ingest_raw**
   - baixa / importa material.

2. **transcribe**
   - WhisperX → JSONL com diarização.

3. **classify_modes**
   - Classify (Hive) → DM/Player + mode.

4. **normalize_markdown**
   - Transmutation → MD padronizado.

5. **segment_dialogues**
   - corta em janelas contextuais.

6. **label_roles**
   - DM vs Player vs NPC.

7. **build_datasets**
   - mestre acting,
   - jogadores acting,
   - exemplos INTENT (opcional avançado),
   - worldbuilding usage.

8. **train_lora**
   - LoRA para Qwen 2.5 14B (narrativa completa, acting de mestre).
   - LoRA para Qwen 2.5 1.5B (reação rápida, prelúdio emocional).

9. **eval**
   - scripts de avaliação + revisão manual.

### 9.2 Saídas esperadas

- `lora_dm_actor.safetensors`
- `lora_player_rogue.safetensors`
- `lora_player_paladin.safetensors`
- eventuais variações de estilo (dark, heroico, grim, anime-fantasy VRPG).

---

## 10. Tamanho de Dataset (ordem de grandeza)

### 10.1 Acting de Mestre

Para boa qualidade:

- **mínimo aceitável:**  
  ~50–100h de boa narração de Mestre.

- **ideal:**  
  200–400h (ou mais), desde que limpas e bem rotuladas.

Você não precisa usar tudo de uma stream gigantesca —  
qualidade > quantidade.

### 10.2 Acting de Jogador

20–100h de interações boas por arquétipo.

Não precisa ser equilibrado em todas as combinações de classe.

### 10.3 Worldbuilding

Texto: milhares de segmentos de 1–3 parágrafos.

O importante aqui é consistência de tom.

---

## 11. Integração com o Runtime do VRPG

### 11.1 Como o modelo treinado entra no jogo

No runtime, você terá:

- **Pipeline de 2 modelos**:
  - BaseModel 1.5B: Qwen 2.5 1.5B Q4_K_M + LoRA_dm_prelude (reação rápida)
  - BaseModel 14B: Qwen 2.5 14B Q4_K_M + LoRA_dm_actor (narrativa completa)
- LoRA_player_X (para jogadores IA)

**Ativação:**

- **Em modo Mestre (1.5B - Prelúdio):**
  - carrega o LoRA do Mestre Reflexo (reação emocional imediata).
  - Resposta < 1.2s, 1-2 frases, 15-45 palavras.

- **Em modo Mestre (14B - Narrativa):**
  - carrega o LoRA do Mestre Real (agent_mindset + acting completo).
  - Resposta < 6s, narrativa completa, consequências, resolução.
- **Em modo Jogador IA:**
  - carrega LoRA correspondente ao arquétipo/personagem.

### 11.2 Interaction com Hive

O modelo treinado vai:

- reconhecer quando precisa de lore/regra,
- gerar INTENT LORE_QUERY / RULE_QUERY,
- incorporar o resultado no acting.

---

## 12. Iteração e Refinamento

O VRPG gera, a cada sessão:

- logs do orquestrador,
- INTENTs enviadas,
- outcomes mecânicos,
- falas do Mestre e dos Jogadores IA,
- feedback (implícito, ex: interrupções, reações do humano).

Você pode:

- exportar sessões boas como novo dataset,
- usar para refinar LoRA:
  - corrigir vícios,
  - reforçar boas práticas.

---

## 13. O que NÃO treinar

Você não deve:

- Treinar regras cruas (PHB, DMG) como "resposta natural".
- Treinar o modelo a dizer números (gostamos que ele não saiba).
- Treinar a IA a fazer toolcall direta:
  - isso é trabalho do Orquestrador.

O modelo deve aprender:

- quando sugerir teste,
- quando sugerir consulta de lore,
- como narrar outcomes.

---

## 14. Worldbuilding Dinâmico

Você pode acoplar isso a um conceito de:

**"World Seeds":**

- configurações de campanha,
- eixos de tom (grim x heroico, baixo x alto nível de magia, grounded x anime).

Durante treino, marque datasets com tags:

- `style: dark_fantasy`
- `style: noble_bright`
- `style: vrpg_anime`

No runtime, você seleciona a mistura de estilos que quer ativar.

---

## 15. Resumo

O pipeline de treino do VRPG serve para:

- Ensinar o modelo a agir como um Mestre humano (sem regras).
- Ensinar o modelo a agir como companheiros de party (Jogadores IA).
- Ensinar o modelo a usar memória externa (Hive) em vez de "decorar o mundo".
- Criar uma experiência de mesa consistente, viva e escalável, sem depender de API.

A arquitetura resultante é:

- Treino = ensinar comportamento dramático e estrutural.
- Runtime = combinar isso com Engine D&D 5e + Hive + Orquestrador Rust.

