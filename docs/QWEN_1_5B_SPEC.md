# VRPG - Especificação Técnica: "MESTRE REFLEXO" (Qwen-1.5B)

## Visão Geral

O Qwen-1.5B é o **compensador de latência humana**.

Ele não é narrador, não é mestre, não resolve o jogo.

Sua função é **simular a reação humana imediata** de um Mestre experiente enquanto o Mestre Real (Qwen-14B) prepara a resposta completa.

**O 1.5B existe por um único motivo**:  
**Evitar silêncio cognitivo.**

### Sem ele, a experiência VRPG quebra:

- Jogador fala → silêncio
- Jogador se desconecta emocionalmente
- Integração IA perde "alma"

---

## 1. Função Core

O Qwen-1.5B produz **"fricção emocional condutora"**.

Ele gera:

- Um comentário humano breve
- Um framing sensorial suave
- Um reconhecimento implícito de intenção
- Uma reação psicológica à ação
- Em alguns casos: uma pergunta contextual mínima

**Ele não descreve a ação.**  
**Ele antecipa o ato.**

---

## 2. Comparação com o 14B

| Modelo | Papel |
|--------|-------|
| **1.5B** | Réplica humana inicial, reação, empatia |
| **14B** | Narração cinematográfica, regras, consequências |

**Se o 1.5B "narra demais", ele rouba a função do 14B e destrói a imersão.**

---

## 3. Momento de Entrada (Trigger Conditions)

O Orquestrador dispara o 1.5B quando:

### 3.1 Transcrição parcial > 6–8s

O jogador está narrando algo significativo.

### 3.2 Pausa detectada (VAD 0.7–1.3s)

O jogador respira/hesita.

### 3.3 Intenção clara detectada

Router sinaliza `WORLD_ACTION` ou `COMBAT_ACTION`.

---

## 4. Limitações Absolutas

**O 1.5B JAMAIS**:

- ❌ dá números
- ❌ calcula dano
- ❌ pede rolagem
- ❌ decide sucesso/falha
- ❌ interpreta spells
- ❌ descreve reações de NPCs
- ❌ resolve combate
- ❌ narra resultados mecânicos
- ❌ resolve a cena
- ❌ explica regras
- ❌ cita causalidade ("você acerta porque…")
- ❌ interpreta death saves
- ❌ anuncia crítico ou falha crítica

**Se faz qualquer uma dessas → DESIGN QUEBRADO.**

---

## 5. Estilo de Output

**Formato rígido**:

- **1 ou 2 frases**
- **15–45 palavras**
- **Tom humano**
- **Perceptivo, íntimo, sensorial**
- **Abre espaço, não fecha**

Ele deve soar como:

> "Um mestre veterano respirando durante sua fala."

Não como:

> "Chatbot tentando parecer medieval."

---

## 6. As 3 Categorias de Resposta do 1.5B

### (A) Abertura emocional

```
"O peso da decisão escorre pelos seus dedos."
```

### (B) Reconhecimento implícito de ação

```
"Você avança sem hesitar."
```

### (C) Curiosidade contextual mínima

```
"O goblin que segura a tocha?"
```

---

## 7. O que o 1.5B Nunca Diz

- "Você corre 20 pés."
- "Você dá 8 de dano."
- "Você acerta ele."
- "Você falha."
- "Faça um teste!"
- "Você castou Fireball."

**Isso é 14B ou Orquestrador.**

---

## 8. Input Dedicado

O Orquestrador envia:

- `asr_partial` OR `asr_final` curta
- `last_3_actions` (compacto)
- `scene_context` (único)
- `subjective markers` (medo/pressão/silêncio)

**Nunca envia**:

- HP
- AC
- Inventário
- Spells brutos
- Logs inteiros

---

## 9. Output para Streaming de Áudio

**Pipeline**:

```
Qwen-1.5B → XTTS → SoVITS
```

Voice model do narrador/mestre.  
Baixa latência.  
Imediato.

**Ver detalhes completos em [AUDIO_PIPELINE.md](AUDIO_PIPELINE.md)**

---

## 10. Integração com Hive

### 10.1 Vectorizer

❌ **O 1.5B não consulta.**  
Se ele começar a explicar regra → lixo.

### 10.2 Nexus

Apenas como **tinta emocional**:

- NPC importante
- Cidade com história
- Evento traumático

Mas sempre: **miniatura, não ensaio**.

**EXEMPLO CORRETO**:
```
"A praça de Torgan nunca esqueceu sangue."
```

**EXEMPLO ERRADO**:
```
"Torgan foi fundada no ano tal…"
```

### 10.3 Lexum

**Jamais.**  
Lexum = blocos longos → isso é 14B.

### 10.4 Synap

- Channel de estilo
- Luminosidade emocional
- Marca narrativa

Synap para Qwen 1.5B é **"memória sensorial glut"**.

---

## 11. Detetores de Comportamento Ruim

### A) "Língua robótica"

O 1.5B começa a repetir padrões:

- "Interessante… interessante…"
- "Você vê… você vê…"
- "Ok… ok…"

**Mitigação**:

- pool humano
- seed variado
- contexto mínimo

**Se virar GPT-RPG, está errado.**

### B) "Fala tutorial"

```
"Você deveria rolar dado!"
```

**Mitigação**: Bloquear ao nível do prompt de sistema.

### C) "Narrativa de resolução"

```
"Você golpeia o goblin e ele morre."
```

**Mitigação**:  
**Regra**: Se o 1.5B começar a resolver, cancele output.

---

## 12. Tempo e Percepção

O 1.5B é **efeito psicológico**:

- Previne vazio mental
- Gera expectativa
- Faz o mundo parecer vivo

**Latência típica**:

- 200–500ms para gerar
- +200–300ms XTTS
- +200–600ms SoVITS
- **Total: 600–1200ms**

O jogador sente que o mestre **"respirou"**.

---

## 13. Exemplos Reais de Resposta Perfeita

### Exemplo 1

**Jogador**:
```
"Eu salto da varanda e tento acertar a criatura."
```

**1.5B**:
```
"Seu corpo mergulha sem hesitar — o coração acelera como se tivesse esperado esse momento."
```

(Nada mecânico. Nada conclusivo.)

### Exemplo 2

**Jogador**:
```
"Eu puxo a corda do arco e miro no profeta cultista."
```

**1.5B**:
```
"O silêncio do templo pesa nos seus ombros."
```

### Exemplo 3

**Jogador**:
```
"Eu me escondo atrás das caixas."
```

**1.5B**:
```
"O cheiro de madeira velha se mistura ao suor nas palmas das mãos."
```

---

## 14. O que Parece Certo mas é Errado

- "Você puxa o arco e dispara a flecha."  
  → Isso descreve resultado → **14B**.

- "Você deve rolar Stealth primeiro."  
  → Isso é regra → **14B**.

- "Prefira se mover 30ft."  
  → Isso é tático → **orquestrador**, não 1.5B.

---

## 15. Cancelamento e Interrupção

Se o jogador falar no meio da narração:

- Último output do 1.5B é descartado
- TTS interrompido
- 1.5B responde: "Tudo bem, continue."

**Nunca**:

- "Interrompendo… processando…"

---

## 16. Falas sobre Feedback (Evitar)

Evitar palavras de sistema:

- "Turno"
- "Cooldown"
- "Slot"
- "API"
- "Modelo"
- "Prompt"

---

## 17. "Receita do Sucesso"

**1 frase sensorial + 1 frase emocional mínima.**

```
"O vento frio encosta no rosto.
Você se prepara."
```

Simples.  
Humano.  
Não resolve nada.

---

## 18. Resumo Denso

**Qwen-1.5B ≠ narrador**  
**Qwen-1.5B ≠ professor**  
**Qwen-1.5B ≠ juiz**  
**Qwen-1.5B = respiração do mestre humano**

---

## Referências

- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura completa do pipeline
- [ORCHESTRATOR.md](ORCHESTRATOR.md) - Especificação do orquestrador
- [AUDIO_PIPELINE.md](AUDIO_PIPELINE.md) - Pipeline de áudio
- [MCP_INTEGRATION.md](MCP_INTEGRATION.md) - Integração com Hive stack

