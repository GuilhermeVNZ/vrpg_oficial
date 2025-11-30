# VRPG Client - Geração de Assets

## Visão Geral

O VRPG Client utiliza geração de assets (imagens, vozes, cenas) para criar conteúdo dinâmico durante as sessões de jogo. Este documento descreve os pipelines de geração de assets, incluindo imagens, LoRAs, embeddings, battlemaps e integração com o sistema de memória.

**Princípios Fundamentais**:
- **Modo Sessão (Tempo Real)**: Geração leve e rápida (retratos, close-ups, variações)
- **Modo Preparação (Pós-Sessão)**: Geração pesada e de alta qualidade (battlemaps complexos, LoRAs, datasets)
- **Consistência Visual**: Garantir que personagens mantenham aparência consistente ao longo da campanha
- **Cache Agressivo**: Qualquer asset visual gerado é armazenado e reutilizado

## Modos de Operação

### Sessão de Jogo (Tempo Real)

**Duração sugerida**: ~3h

**Prioridades**:
- Baixa latência (voz → resposta)
- Decisões táticas rápidas
- Imagens leves/on-the-fly (retratos, close-ups, variações)
- Animações de combate e rolagens de dados

**Geração de Imagens em Sessão**:
- Retratos e emoções on-the-fly (0.5–2s, não bloqueia)
- Cenas rápidas e close-ups
- Efeitos visuais de combate (shaders/sprites, não IA)

### Modo de Preparação (Pós-Sessão)

**Duração sugerida**: ~1h

**O Mestre IA e serviços auxiliares preparam**:
- Battlemaps complexos
- Retratos completos de NPCs importantes
- Datasets de imagens para LoRA/embeddings
- Cenas chave (keyframes narrativos)
- Atualizações na memória da campanha (Vectorizer + Nexus + Lexum)

**Ciclo de Melhoria**: A cada ciclo de 3h de jogo + 1h de preparação, a campanha ganha **coerência e assets mais ricos**, sem penalizar a performance da sessão.

## Pipeline de Geração de Imagens

### Tipos de Imagens

#### 1. Retratos de Personagem (Portraits)

**Uso**:
- Interface principal (top cards, talking heads)
- Fichas de personagem
- Tela de carregamento/contexto

**Requisitos**:
- Consistência de fisionomia (mesmo personagem ao longo da campanha)
- Variedade de expressões: neutro, feliz, triste, furioso, ferido, concentrado, assustado, determinado
- Resolução alvo: 768×768 ou 1024×1024, recortados para UI

#### 2. Cenas Narrativas (Keyframes / Cutscenes Estáticas)

**Uso**:
- Eventos importantes (revelações, encontros com chefes, flashbacks)
- Títulos de aventura
- Momentos cinematográficos

**Requisitos**:
- Alta qualidade, mais tempo de geração aceitável (preparação)
- Composição complexa (múltiplos personagens, cenário detalhado)
- Consistência de personagens (mesmos rostos, trajes, armas)

#### 3. Battlemaps (Mapas de Combate com Grid)

**Uso**:
- Combate tático estilo BG3/Solasta
- Visualização top-down ou isométrica da cena

**Requisitos**:
- Grid em quadrados (ver seção Battlemaps)
- Claridade de áreas caminháveis vs obstáculos
- Iluminação coerente
- Possibilidade de reutilizar o mapa com pequenas variações (clima, horário)

**Detalhes completos na seção [Battlemaps](#battlemaps)**

#### 4. Imagens de Apoio (Thumbnails, Itens, Magias, Efeitos)

**Uso**:
- Ícones de magias e habilidades
- Representação visual de itens importantes
- Ilustrações de monstros
- Efeitos visuais (área de efeito, rajadas, etc.)

**Requisitos**:
- Rápida geração (podem ser feitas em lote no modo preparação)
- Arte coerente com o estilo VRPG

### Modelos e Configuração

#### Modelo Base: Flux.1

- **Modo rápido** (Schnell ou similar) para uso em sessão
- **Modo de alta qualidade** para preparação offline

#### LoRA Global de Estilo

- Um único **LoRA de estilo VRPG anime** é sempre carregado
- Define:
  - Paleta de cores
  - Estilo de linhas
  - Nível de detalhamento
  - Linguagem visual geral

#### LoRAs e Embeddings de Personagens

- Cada personagem-chave pode ter:
  - Um **embedding** (Textual Inversion / ID) para variações rápidas
  - Uma **LoRA leve de identidade** para máxima consistência (treinada no modo preparação)

**Detalhes completos na seção [LoRA e Embeddings](#lora-e-embeddings)**

### Pipeline em Sessão (Tempo Real)

**Foco**: Baixa latência

#### Retratos e Emoções On-the-Fly

Quando necessário (por exemplo, Mestre descrevendo reação forte):

1. O sistema verifica se já existe retrato com aquela emoção no cache
2. Se existir → carrega imediatamente
3. Se não existir:
   - Chama Flux.1 (modo rápido) com:
     - LoRA de estilo VRPG
     - Embedding/LoRA do personagem (se houver)
     - Prompt de emoção (ex: "furious, shouting, intense eyes, VRPG style")
   - Gera imagem 768×768
   - Salva no cache associando: `character_id + emotion + outfit`

#### Cenas Rápidas e Close-ups

Para cenas rápidas (ex.: "close-up do rosto do vilão enquanto ele ameaça o grupo"):

- Flux.1 rápido, usando:
  - Retratos já existentes como referência
  - Prompts curtos e diretos
- Toleramos 0.5–2s de geração (isso é aceitável enquanto o Mestre IA narra)

#### Efeitos Visuais de Combate

- Sempre que possível, **não usar IA** para FX em tempo real:
  - Usar shaders, sprites e partículas (engine 2D/2.5D)
- Imagens IA de efeitos são pré-geradas como spritesheets no modo preparação

### Pipeline no Modo de Preparação

**Foco**: Qualidade máxima e consistência

#### Geração de Dataset Visual

Com base no que aconteceu na sessão:

1. O Mestre IA lista:
   - NPCs novos que serão importantes
   - Locais novos que serão revisitados
   - Situações futuras prováveis (com base na campanha e nas ações do jogador)

2. Para cada NPC importante:
   - Gera de 20 a 60 imagens:
     - Frontal, 3/4, meio-perfil
     - Várias emoções (neutro, raiva, dor, riso, choro, foco, medo)
     - Variações leves de iluminação, cenário neutro

3. Para batalhas futuras:
   - Gera battlemaps base (ver seção Battlemaps):
     - 1 mapa por ambiente-chave (taverna, floresta, caverna, etc.)
     - Variações de clima/luminosidade, se relevante

4. Para cenas de história:
   - Cria keyframes narrativos de alta qualidade

#### Organização e Indexação

- **Classify** organiza as imagens por:
  - Personagem
  - Emoção
  - Tipo de cena
  - Localização
- **Vectorizer** indexa descrições de cada imagem para busca semântica
- **Nexus** relaciona:
  - Imagens → eventos → NPCs → locais

Isso permite que o Mestre IA recupere e reutilize facilmente artes já existentes.

### Problemas Clássicos e Soluções

#### Problema: Consistência de rosto/corpo entre imagens

**Solução**:
- Usar **LoRA + embedding** para personagens fixos
- Manter prompts estáveis (mesmos descritores) para cada personagem
- Congelar seeds para alguns ângulos "canônicos" (ex.: o retrato oficial)

#### Problema: IA mudando a roupa/armadura do personagem sem pedido

**Solução**:
- Incluir no prompt:
  - Descrição detalhada da armadura/traje
  - "same outfit as reference" quando usar imagem de referência
- No dataset de LoRA, garantir:
  - Uma "roupa base" dominante

#### Problema: Battlemaps pouco legíveis (confusão visual)

**Solução**:
- Definir regras de clareza:
  - Contraste forte entre caminho e obstáculo
  - Grid claramente visível
  - Evitar poluição visual desnecessária

#### Problema: Geração pesada travando sessão

**Solução**:
- Qualquer geração que demore > 3s deve ser marcada como tarefa de **preparação**
- O jogo mostra placeholders e substitui pela arte final depois
- O Mestre IA é instruído a **não depender da arte** para continuar narrando

## LoRA e Embeddings

### Objetivos

- Garantir que o personagem principal e NPCs importantes **tenham o mesmo rosto e corpo** ao longo da campanha
- Permitir variações de emoção, ângulo, iluminação, roupas e contexto
- Não travar a gameplay com treino pesado em tempo real:
  - Treino acontece no **modo preparação**
  - Uso de embeddings e referências visuais em sessão

### Diferença entre LoRA e Embeddings

#### Embeddings (Textual Inversion / ID)

- Representam o personagem como **um token especial** no prompt:
  - Ex.: `<char_shura_vrpg>`
- Treino rápido:
  - ~10 a 30 imagens
  - Minutos, não horas
- Leve em VRAM
- Ideais para:
  - Retratos
  - Variações de emoção
  - Leve mudança de ângulo
  - Uso frequente em sessão

#### LoRA (Low-Rank Adaptation)

- Ajuste fino dos pesos do modelo
- Pode capturar:
  - Identidade
  - Proporções corporais
  - Estilo de roupa
  - Arte corporal, cicatrizes, etc.
- Requer mais dados:
  - 30–80 imagens por personagem (bem limpas)
- Treino mais pesado:
  - Dezenas de minutos em GPU
- Ideais para:
  - Personagens principais
  - NPCs extremamente recorrentes
  - Cenas complexas onde a fidelidade é crítica

### Quando usar Embedding vs LoRA

#### Embeddings – Regra

- Todo personagem que o Mestre IA considera **recorrente** ganha um embedding
- Treinados assim que houver:
  - Pelo menos 10 boas imagens do personagem
  - Descrição textual clara (lore e aparência)

**Usos**:
- Sessão (tempo real)
- Retratos rápidos
- Close-ups e pequenas cenas

#### LoRA – Regra

- Somente para:
  - Personagem do jogador
  - Antagonistas principais (arcos importantes)
  - Aliados centrais (companions recorrentes)
  - NPCs que aparecem em várias aventuras/capítulos

**Critérios para disparar treino de LoRA**:
- Personagem apareceu em **3+ sessões**
- Existe um embedding estável que produz bons resultados
- Já temos ao menos:
  - 30 imagens variáveis do personagem (ângulos/emotions)

### Dataset Ideal para LoRA de Corpo + Fisionomia

#### Quantidade de Imagens por Personagem

- Mínimo: 30 imagens
- Ideal: 50–60 (evitar ir muito acima pra não overfit)

#### Tipos de Poses

**Rosto / Bustos**:
- Neutro frontal
- 3/4 virado à esquerda
- 3/4 virado à direita
- Olhar para cima/baixo
- Olhos fechados

**Corpo**:
- Meio-corpo (torso + cabeça)
- Corpo inteiro estático
- Corpo inteiro em pose de ação (ataque / cast / defesa)

**Emoções**:
- Neutro
- Raiva
- Medo
- Tristeza
- Riso
- Dor
- Surpresa
- Foco/concentração
- Desesperado

**Contextos**:
- Fundo neutro (mais importante)
- Fundo leve de cenário (2–3 só)

#### Cuidados

- Mesma roupa base na maior parte das imagens
- Variações de roupa em poucas imagens (ou em dataset separado)
- Iluminação variada, mas não extrema (evitar distorcer cores demais)
- Nunca misturar estilos artísticos diferentes para o mesmo personagem

### Pipeline de Treino (Modo Preparação)

1. Coletar imagens geradas na sessão (retratos, cenas)
2. O Mestre IA/engine marca as melhores para dataset
3. Ferramenta automatizada:
   - Recorta
   - Remove fundo se necessário
   - Normaliza resolução (ex.: 768×768)
4. Gera metadados:
   - Nome do personagem
   - Emoção
   - Ângulo
   - Contexto
5. Treina:
   - Primeiro um embedding (rápido)
   - Depois, se critérios forem atendidos, uma LoRA leve

### Uso em Geração de Imagem

#### Em Sessão

Ao gerar imagem:

- Sempre incluir:
  - LoRA global de estilo VRPG
- Se existir:
  - Embedding do personagem (token especial)
  - Ou LoRA de identidade

**Prompt exemplo**:
> `masterpiece, VRPG style, <char_shura_vrpg>, furious, shouting, golden armor, green magical eyes, close up portrait, dramatic lighting`

#### Em Preparação

- Usar embeddings + LoRA de identidade
- Gerar:
  - Conjunto de imagens adicionais
  - Cobrir emoções/poses faltantes
  - Atualizar dataset

### Personagens que NÃO recebem LoRA

- NPCs de cena única
- Figurantes
- Encontros aleatórios de estrada
- Inimigos genéricos (bandidos quaisquer, goblins 1-of)

**Para esses**:
- Usar somente descrição + estilo global
- Ou um conjunto pequeno de prompts prontos

## Battlemaps

### Objetivo

- Oferecer mapas claros e utilizáveis para combate tático D&D 5e
- Garantir que a IA Mestre e a UI tenham entendimento consistente da **grade (grid)**
- Integrar battlemaps gerados por IA (Flux) com uma camada lógica de grid que o jogo entende

### Representação Lógica do Grid

Independente da imagem de fundo, o jogo mantém um **grid lógico**:

- **Tipo**: Quadrados (não hex)
- **Dimensão típica**: 24×24, 32×32, 40×40 células (configurável)
- **Cada célula representa**: 5 ft padrão D&D 5e

O grid é representado em uma matriz de células, cada uma com:

- `walkable` (bool)
- `cover` (none/half/full)
- `elevation` (nível relativo)
- `tags` (escadas, portas, obstáculos especiais, terreno difícil etc.)

### Integração Imagem ↔ Grid

#### Geração do Battlemaps com IA

A imagem IA (Flux) é gerada com as seguintes características:

- Perspectiva isométrica leve ou top-down estilizado
- Linhas ou padrões suaves sugerindo grid visual (opcional)
- Elementos visuais coerentes com o layout lógico do grid

#### Mapeamento

Existem duas estratégias:

1. **Primeiro o grid, depois a imagem** (Recomendado)
   - O jogo define o layout do grid (células walkable/blocked)
   - Gera um "layout mask" (imagem simples ou mapa de calor)
   - Essa máscara é passada como input/guia (ControlNet ou similar) para Flux
   - Flux gera a imagem respeitando esse layout

2. **Primeiro a imagem, depois anotação** (Menos ideal)
   - IA gera imagem de forma mais livre
   - Ferramenta de anotação (manual ou semiautomática) marca:
     - Células passáveis
     - Obstáculos
     - Paredes
   - Produz-se o grid lógico a partir disso

**Para reprodução consistente, o recomendado é estratégia (1)**.

### Resolução e Escala

- **Resolução típica de exportação**:
  - 2048×2048 (para 32×32 células → 64px/célula)
  - 4096×4096 para mapas maiores
- **A engine de UI faz**: zoom, pan, recortes

### Camadas

#### Background

- Arte IA (Flux):
  - Piso
  - Paredes
  - Ambiente
  - Decoração

#### Grid Overlay

- Linhas finas semi-transparentes, desenhadas pela engine, não pela IA
- Cores configuráveis (ex.: branco 30% opacidade, contorno discretamente luminoso)

#### Tokens

- Personagens
- NPCs
- Criaturas

Aplicados acima da grid, com sombras e halos.

#### Efeitos Especiais

- Áreas de efeito (cones, círculos)
- Magias desenhadas por shader/sprites

### Pipeline de Geração de Battlemaps

#### No Modo Preparação

1. Mestre IA decide:
   - Tipo de ambiente (taverna, floresta, dungeon)
   - Dimensões (ex.: 32×32)
   - Características específicas (elevado, ponte, água, lava, etc.)

2. Gera-se um layout lógico:
   - Matriz grid marcando:
     - Piso
     - Paredes
     - Obstáculos
     - Entradas/saídas

3. A partir deste layout:
   - Cria-se um input para Flux (imagem guia simples ou mapa de profundidade/esboço)
   - Flux gera a arte final da cena

4. Ferramentas internas (ou o próprio Mestre IA) validam:
   - Se a imagem corresponde ao layout (ex.: portas onde grid marca saída)
   - Se a legibilidade está boa (contraste, clareza)

5. O mapa é salvo com:
   - `battlemap_image_path`
   - `grid_definition.json`

#### Em Sessão

- O jogo carrega:
  - A imagem
  - O grid lógico
- O jogador enxerga os quadrados e o Mestre IA:
  - Usa a grid para movimento
  - Calcula alcances
  - Aplica regras de cobertura e terreno difícil

### Estilo Visual de Battlemaps

Mesmo sendo IA, os mapas devem:

- Ser coerentes com o estilo VRPG anime (linhas fortes, cores vivas, luz dramática)
- Evitar poluição visual excessiva:
  - Chão não pode ser tão detalhado a ponto de sumir o grid
- Usar:
  - Luzes de ambiente
  - Sombras suaves
  - Destaques em áreas importantes (altar, centro da sala)

### Reutilização e Variação

O mesmo mapa pode ter múltiplas versões:

- Dia/noite
- Sem destruição / com destruição
- Vazio / com corpos / com rituais ativos

Essas variações:
- Podem ser geradas no modo preparação
- Reaproveitam o mesmo grid lógico

## Estrutura de Aventuras

### Tipos de Aventuras

#### One-Shot

- **Duração**: 1 sessão
- **Foco**:
  - 1–2 locais principais
  - Poucos NPCs relevantes
  - 1 batalha chave

**Assets mínimos**:
- 2–3 battlemaps
- 3–5 NPC portraits importantes
- 1 imagem de capa (key art)
- 3–5 ilustrações de eventos chave

#### Mini-Campanha

- **Duração**: 3–6 sessões
- **Foco**:
  - Arco narrativo curto
  - Vilão central definido
  - Grupo fixo de NPCs aliados/inimigos

**Assets típicos**:
- 5–10 battlemaps
- 8–15 NPC portraits
- 2–3 imagens de capa (capítulo/ato)
- 10–20 ilustrações internas

#### Campanha Longa

- **Duração**: 10+ sessões
- Vários arcos narrativos
- Vários vilões
- NPCs recorrentes

**Assets**:
- 10–30 battlemaps
- 20–50 NPC portraits (alguns com LoRA)
- 5–10 artes de capa
- 30–80 ilustrações diversas

### Macro Estrutura de Cada Sessão

Cada sessão de aventura pode ser pensada em blocos:

1. **Abertura / Recap**
2. **Exploração**
3. **Interação Social (Roleplay)**
4. **Combate**
5. **Resolução / Gancho para próxima sessão**

Para cada bloco, há necessidades visuais específicas.

### Bloco por Bloco – Necessidades Visuais

#### Abertura / Recap

- 1 imagem de capa/recap (opcional)
- Retratos dos personagens principais em estado "neutro"

#### Exploração

Dependendo do tipo:

- **Exploração urbana**:
  - 1–2 vistas da cidade/bairro
  - Retratos dos comerciantes importantes
- **Exploração selvagem**:
  - 1–2 vistas do ambiente (floresta, deserto, montanha)
  - Ilustrações de pontos de interesse (ruínas, altares, entradas de dungeon)

#### Interação Social

- Retratos dos NPCs envolvidos
- 1 imagem de "momento-chave" se for cena dramática
- Avatar dinâmico (emoções) dos NPCs principais

#### Combate

- 1 battlemap principal com grid
- 1–3 ilustrações do monstro/inimigos
- Pequenas imagens representando habilidades especiais (opcional)

#### Resolução / Gancho

- 1 ilustração do resultado (festa, destruição parcial, segredo revelado)
- Retratos dos NPCs aliados se forem ganhar relevância futura

### Planejamento no Modo Preparação

No modo preparação, o Mestre IA:

1. Analisa o que foi feito na sessão anterior
2. Estima:
   - Quais locais serão revisitados
   - Quais NPCs têm probabilidade alta de aparecer
   - Qual o "caminho mais provável" da aventura seguinte

3. A partir disso, gera:
   - Battlemaps necessários
   - Retratos de NPCs prováveis
   - Ilustrações de eventos esperados

**Se o jogador "sair do script"**, o sistema ainda consegue improvisar com:
- Retratos rápidos
- Cenas genéricas
- Reuso inteligente de mapas

### Organização Interna da Aventura

Cada aventura tem:

- `adventure_id`
- `chapters[]`
- Cada `chapter` contém:
  - `scenes[]`
  - `assets` associados

**Exemplo simplificado**:

```json
{
  "adventure_id": "curse_of_the_tavern",
  "chapters": [
    {
      "id": "ch1",
      "title": "Whispers Behind the Door",
      "scenes": [
        {
          "id": "scene_tavern_main",
          "type": "social",
          "assets": {
            "battlemaps": ["tavern_interior_grid"],
            "portraits": ["npc_marrow", "npc_guard"],
            "illustrations": ["tavern_exterior_night"]
          }
        },
        {
          "id": "scene_basement_fight",
          "type": "combat",
          "assets": {
            "battlemaps": ["tavern_basement_grid"],
            "portraits": ["cultist_leader"],
            "illustrations": ["ritual_circle_glow"]
          }
        }
      ]
    }
  ]
}
```

### Reutilização de Assets

- Battlemaps detalhados são reutilizados em várias sessões
- NPCs recorrentes ganham mais imagens e eventualmente LoRA/embeddings
- Arte "genérica" (como inimigos e cidade) pode ser reciclada com pequenas variações

## Caching de Imagens

### Estrutura

Cada imagem gerada é salva com uma chave:

- `type` (portrait/scene/battlemap/icon)
- `character_id` (se aplicável)
- `emotion` (se aplicável)
- `pose` (se aplicável)
- `scene_id` (se aplicável)
- `seed`
- `style_version`

O caminho é armazenado em um índice no banco local.

### Política de Reutilização

Antes de gerar qualquer imagem:

1. Busca no cache por uma imagem que cubra:
   - Mesmo personagem
   - Mesma emoção
   - Mesmo tipo (retratos, inteiro, etc.)
2. Se existir, usa diretamente
3. Se não, gera:
   - Salva no cache
   - E registra metadados

### Invalidando Cache

Quando:

- LoRA global de estilo é atualizado
- O personagem muda visual drasticamente (novo arco de evolução)

Pode-se:

- Versionar as imagens antigas (para manter histórico)
- Gerar novas versões com o estilo novo

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

## Degradação Elegante

Se a máquina do usuário for fraca:

- Reduzir resolução de imagens geradas
- Limitar uso de LoRA complexos
- Geração pesada pode ser opcional ou reduzir qualidade
- Priorizar:
  - ASR/TTS
  - Mestre IA
  - Engine de jogo

## Configuração

### Configuração de Geração de Imagens

```json
{
  "image_generation": {
    "enabled": true,
    "model": {
      "type": "flux.1",
      "fast_mode_path": "models/image/flux.1-schnell.safetensors",
      "quality_mode_path": "models/image/flux.1-dev.safetensors",
      "device": "cuda",
      "precision": "fp16"
    },
    "session_mode": {
      "resolution": [768, 768],
      "steps": 15,
      "guidance_scale": 7.5,
      "sampler": "euler_a"
    },
    "preparation_mode": {
      "resolution": [2048, 2048],
      "steps": 30,
      "guidance_scale": 8.5,
      "sampler": "dpm++_2m_karras"
    },
    "cache": {
      "enabled": true,
      "max_size_mb": 2048,
      "ttl_days": 30
    },
    "post_processing": {
      "upscale": false,
      "face_enhancement": true,
      "color_correction": true
    }
  }
}
```

### Configuração de LoRA e Embeddings

```json
{
  "lora": {
    "enabled": true,
    "global_style_lora": "assets/loras/vrpg_style_v1.safetensors",
    "training": {
      "base_model": "flux.1-dev",
      "rank": 16,
      "alpha": 16,
      "steps": 1500,
      "learning_rate": 0.0001,
      "batch_size": 2
    },
    "runtime": {
      "cache_enabled": true,
      "max_cached_loras": 10,
      "lazy_loading": true
    }
  },
  "embeddings": {
    "enabled": true,
    "auto_train_threshold": 10,
    "training_steps": 500
  }
}
```

### Configuração de Battlemaps

```json
{
  "battlemaps": {
    "default_grid_size": [32, 32],
    "cell_size_ft": 5,
    "resolution": [2048, 2048],
    "grid_overlay": {
      "enabled": true,
      "color": "rgba(255, 255, 255, 0.3)",
      "line_width": 1
    },
    "generation_strategy": "grid_first"
  }
}
```

## Integração com Memory Service

Aventuras e seus assets são automaticamente indexados:

```typescript
// Indexação automática ao carregar aventura
await memoryService.indexAdventure({
  adventure_id: 'curse_of_the_tavern',
  scenes: [...],
  npcs: [...],
  battlemaps: [...],
  lore: [...],
  metadata: {...}
});
```

## Boas Práticas

1. **Consistência Visual**: Manter estilo consistente entre imagens da mesma campanha
2. **Cache Inteligente**: Reutilizar imagens quando possível
3. **Qualidade vs Performance**: Balancear qualidade com tempo de geração
4. **Versionamento**: Manter histórico de versões de LoRAs e imagens
5. **Metadata Rica**: Adicionar metadata detalhada para busca e organização
6. **Indexação Automática**: Indexar automaticamente no sistema de memória
7. **Separação de Modos**: Nunca fazer geração pesada durante sessão
8. **Preparação Proativa**: Gerar assets antecipadamente quando possível

---

## Pipeline Visual (Flux + LoRA + Battlemaps)

O pipeline visual do VRPG entrega 4 tipos de assets:
1. **Retratos consistentes de personagens** (NPC + jogadores IA)
2. **Cenas sociais** (taverna, ruínas, cavernas)
3. **Battlemaps com grid e perspectiva** (Baldur's Gate 3 / Solasta)
4. **Eventos visuais de combate** (on-the-fly ou sprites)

**Regras de ouro**:
- **Zero API** no runtime (máxima performance)
- **Flux + LoRA** para manter **consistência visual** dos personagens
- **Geração pesada ocorre no Downtime**, nunca no meio da sessão
- **Runtime usa cache** → 0,1–0,4s de resposta visual

### Modelo Principal — Flux (SDXL successor)

Flux é excepcional em **consistência estética** e **qualidade cinematográfica**. Mas: inferência pura demanda **VRAM/tempo de GPU**.

Portanto:
- **Sem LoRA pesada durante gameplay**
- **Treino LoRA → no Downtime** (fase de preparação)
- **Geração on-the-fly → apenas com LoRA leve já pronta**

### Categorias de Geração

#### Retratos (Portraits)
- Tipo: busto/waist-up
- Fundo neutro
- Estilo fixo (visual guia do projeto)
- Objetivo: Identidade visual persistente, personagem reconhecível ao longo de 50+ sessões

#### Scenes (Social / Exploration)
- Imagem estática panorâmica
- Profundidade, atmosfera
- Zero grid
- Usadas para: Entrada de ambientes, Comunicação narrativa, Atmosfera

#### Battlemaps
- Perspectiva isométrica / top-down 3D fake
- **Grid 5 ft** (resolvido na arte)
- **Hit boxes** previsíveis (tamanho token 1x1 / 2x2 / 3x3)
- Essas imagens **precisam de "claridade tática"**, não só estética: áreas navegáveis, obstáculos, terreno difícil, zonas elevadas

#### Eventos
- sprites/frames (ex: slash, fire, smoke)
- nunca são gerados em real-time
- **pré-baked** com small models

### Pipeline Técnico (Backend)

**Backend = ComfyUI (Headless)**

Razões:
- **nodes reutilizáveis**
- **LoRA injection profissional**
- **control net plug-and-play**
- **export checkpoint**

Flux baseado em um graph:
```
PROMPT INPUT
↓
LoRA (face)
↓
Scheduler
↓
ControlNet (only for battlemap)
↓
FluxSampler
↓
Upscale (x2, optional)
↓
Output
```

Você gera um `.json graph`, salva no repositório e nunca altera no runtime.

### Estilo Visual: Vox Machina Ocidental

**Referência Visual**: The Legend of Vox Machina (Anime Ocidental)

**O que define o estilo**:
- Contornos marcados, grossura variável – cartoon não infantil
- Paleta saturada, detalhes fortes de cor, sombras duras
- Volumes simplificados (nariz/queixo, cabelo) mas lighting cinematográfico
- Texturas pintadas, sem ultra render 3D realista
- Expressões exageradas, linguagem facial clara (como anime, mas em inglês/ocidental)
- Designs fantasy hero: roupas com layers, couro, metal, magia e assinatura visual

**NÃO usar**:
- Hiperrealismo
- Rosto Blender/Unity low quality
- LoRA anime japonês puro (vai puxar olhos gigantes / rosto moe)
- Shading cel simples tipo mobile gacha
- "Render 3D realista" → mata a proposta

**Dimensões Ideais**:
- **Retratos**: 768×1024 (vertical hero) ou 1024×1024 (ícone UI/avatar)
- **Scenes**: 1920×1080 ou 2560×1440 (desktop cinematic, 16:9 obrigatório → UI encaixa melhor)
- **Battlemaps**: 2048×2048 ou 4096×4096 (isométrico, grid 5ft)

**Prompt Oficial – Retrato de Personagem**:
```
Vox Machina / western anime style fantasy character portrait,
medium close-up, symmetrical composition,
thick lineart, expressive anime western face,
heroic lighting, painterly shading,
vibrant fantasy palette, soft dramatic rim light,
emotion: <calm|anger|determined|fear|joy>,
focus on eyes and silhouette, detailed hair,
background blurred neutral bokeh,
no logo, no watermark, no text,
studio-quality high concept illustration
```

**Prompt Oficial – Scene Social**:
```
fantasy environment in Vox Machina western anime style,
deep perspective, cinematic composition,
painted textures, thick strokes, bold shapes,
warm lighting, volumetric ambience,
wooden structures, medieval fantasy architecture,
dramatic color grading, anime western atmospheric effects,
no characters (unless requested), no UI, no grid
```

**Prompt Oficial — Battlemap**:
```
isometric fantasy battlemap, Vox Machina western anime style,
clean materials, sculpted geometry, readable terrain,
thin subtle 5ft grid integrated into ground,
high contrast between walkable floor and obstacles,
painted lighting and deep shadows,
props: barrels, torches, pillars, bridges,
avoid characters, avoid text, avoid HUD
```

**Regras Táticas para Battlemaps**:
- Grid sutil (5ft)
- Tokens não inclusos
- Iluminação clara → caminhos óbvios
- Perspectiva leve → depth sem virar diorama
- Contraste alto entre terreno e props
- Edges fortes
- Sombras coerentes
- Highlight lateral
- **Evitar**: backgrounds ultra render, partículas exageradas, volumetria pesada

**Prompt Oficial — Eventos de Combate / Ação**:
```
fantasy combat effect anime western style,
energetic brush strokes, cinematic movement,
magic slash motion, painterly vfx,
bold contrasts, dramatic rimlight,
single effect, transparent background
```

---

## LoRA Guidelines

### Princípios Sagrados

#### 1. LoRA NÃO é estilo + rosto + pose
Uma LoRA = UMA ideia. Não tente ensinar 10 conceitos num único treino.

❌ **errado**: "lora estilo anime + personagem + armadura + magia + corpo + skin"

✔️ **correto**:
- LoRA estilo (global do jogo)
- LoRA personagem (face + silhueta + cabelo)

Combina 2 LoRAs no runtime, no máximo 3 em casos excepcionais. Mais que isso = arte Frankenstein.

#### 2. Dataset pequeno e preciso vence dataset gigante
Não existe "mais = melhor". Existe "mais foco = melhor".

- **Personagem recorrente (Party, Vilão Principal)**: Dataset ideal 20–45 imagens
- **NPC secundário recorrente**: Dataset 12–18 imagens
- **NPC de cena única**: Não ganha LoRA

#### 3. Treine LoRAs no downtime, nunca durante gameplay
Você implementou VRPG com 3h sessão / 1h preparação. Use essa 1h como janela de render + treino.

Fluxo:
1. Mestre finalizou sessão
2. IA separa faces nos momentos dramáticos
3. você faz dataset + LoRA
4. pré-renderiza assets
5. próxima sessão = 0 latência visual

#### 4. A estética não é "anime japonês"
Você não quer: olhos gigantes, moe, proporções infantis, shading celular simplista

Você quer: contorno forte, expressão facial, proporção heroica, iluminação cinematográfica, atmosfera painterly

### Tipos de LoRA

#### 1. LoRA de Estilo (GLOBAL)
É a personalidade visual do VRPG. É o "DNA" da sua arte.

Treine com 40–80 imagens de: personagens em close, cenas internas, cenas externas, battlemaps isométricos, vfx simples (fogo, magia suave)

**Objetivo**: Quando Flux recebe "fantasy tavern scene" → a paleta, strokes e line weight já puxam para Vox Machina.

🔐 **Nunca misturar no dataset**: anime japonês, pixel art, cartoon infantil, realismo fotográfico

#### 2. LoRA de Personagem
Serve pra "reconhecer o rosto".

Ela **não deve ensinar**: uniformes complexos, magia, monstros, poses

**Ensina**: cabelo, traços faciais, cicatrizes, olhos, expressão base

**Resultado**: Retratos + close-ups serão coerentes a sessão inteira.

#### 3. LoRA de corpo (opcional – perigosa)
Só use quando: personagem tem armadura icônica, silhueta é assinatura, visual precisa persistir

**Dataset deve ter poses completas, mas**: 20–30 imagens, 2–3 ângulos, 2–3 emoções, 2–3 backgrounds simples

**Nunca treine LoRA corpo + rosto + estilo no mesmo pacote.**

### Curadoria — Como Escolher as Fotos Certas

**O que é BOM**:
✔️ rosto nítido  
✔️ iluminação clara  
✔️ expressão clara (raiva, calma, ironia)  
✔️ cabelo visível  
✔️ design consistente (mesmo brinco, cicatriz, mana)

**O que MATA LoRA**:
❌ prints comprimidos (WhatsApp facebook)  
❌ filtros instagram  
❌ resoluções < 512px  
❌ 15 variações com mesma pose  
❌ diferença estética brutal (cel shading japonês + arcane + realista)

Você está ensinando a IA "quem ele é", não "o que o mundo é".

### Resolução (Profissional)
- **1024×1024** – ideal
- **768×1152** – bom
- **512×512** – aceitável apenas se dataset forte

**Nunca upscale via Photoshop/Espaço/waifu**: → IA detecta como fake / perda de "edge consistency".

**Se precisar upscale**: SDXL refiner, Real-ESRGAN GPFGAN

### Treino (ComfyUI / Locally)

**Configuração ideal** — Local GPU (RTX 5090 / 4080 / 3090):
- Base Model: Flux / SDXL
- Mixed Precision: FP16
- Rank (dim): 8–16
- Alpha: 8–16
- LR: 1e-4 → 4e-4
- Epochs: 4–8
- Batch Size: 2–8

**Quanto menor rank →** Mais fiel ao personagem (rostos)

**Quanto maior rank →** Mais estilização (roupa / corpo)

### Combinação no Runtime

**Retrato**:
```
Flux + LoRA_style + LoRA_character
```

**Cena narrativa (taverna/cidade)**:
```
Flux + LoRA_style
```

**Battlemap**:
```
Flux + LoRA_style + ControlNet (openpose / lineart)
```

### Workflow ComfyUI (Headless) - Recomendação

**Graph estável**:
```
PROMPT → BASE(Flux)
     → LoRA_Style(VoxMachina)
     → [optional] LoRA_Character_<name>
     → Sampler (DPM++ 2M Karras, 20–40 steps)
     → ControlNet (apenas battlemap pose/path)
     → Output
```

**Nunca empilhar >3 LoRAs**:
- 1 style (global)
- 1 face (personagem)
- 1 corpo/roupa (OPCIONAL)

**Batch 4–6 → cache local**

### Integração no VRPG - Arquitetura

**Fluxo**:
1. Mestre cria NPC → sistema marca personagem como "recorrente"
2. Durante downtime de 1h:
   - coleta 15–25 retratos
   - treina LoRA face (rank 8–16)
   - gera pack: portrait neutral + angry + happy
3. Runtime só faz:
   - lookup
   - resize
   - load texture
   - **Zero geração pesada on-session**

### Exemplo de Geração Real

**Personagem**: Clériga sombria
- Olhos azuis frios
- Pele cinza clara
- Marcas da ordem religiosa
- Humor seco / sarcasmo

**Prompt**:
```
Vox Machina western anime fantasy portrait,
female cleric, pale stone-like skin tone,
cold blue eyes, braided dark hair, leather pauldrons,
arcane sigil glowing softly on forehead,
thick expressive lines, painterly shading,
dramatic rim lighting, tavern candle atmosphere,
emotion: stoic, neutral
```

### Personagens Gerados In-Game (Improviso do Mestre)

**Durante sessão**:
- NPC aparece rapidamente
- IA descreve
- **Não gerar LoRA corporativa**
- Gerar 1–3 retratos rápidos só para UI
- Se NPC virar recorrente → LoRA criada no downtime

### O Truque da Consistência Mundial

**Regra**:
- Tudo que não for personagem → estilo global LoRA
- NPCs = LoRA personagem específica
- Ambientes, cidades, mapas = style LoRA somente

---

## Biblioteca Oficial de Prompts

Este documento contém prompts de alta qualidade para uso com Flux + LoRA_Style(VoxMachina) e LoRA_Character_X (quando existir). Todos os prompts listados foram otimizados para coesão visual do projeto e clareza de assets para gameplay.

**Nunca use prompts 1000 palavras longos.** Flux + anime ocidental funciona com direção artística clara, concisa e sem ruído.

### RETRATOS — CHARACTERS (UI / CENAS SOCIAIS)

**Composição**: busto / rosto / 3/4  
**Estilo**: western anime (Vox Machina / Arcane feel)  
**Resultado**: identidade visual consistente

#### Retrato Base — Player ou NPC
```
Vox Machina western anime character portrait,
medium close-up, 3/4 view, strong lineart,
painterly shading, vibrant fantasy palette,
cinematic rimlight, expressive eyes,
soft blurred neutral background,
no text, no watermark, no UI
```

#### Retrato — Emotivo (para uso no chat de cena)
```
Vox Machina western anime portrait,
close-up, strong emotional expression: <anger|fear|sadness|joy|determined>,
dramatic lighting, painterly shadows,
sharp line weight, deep contrast,
no background details, soft bokeh
```

📌 **Use com LoRA de personagem para consistência**: `LoRA_Style(VoxMachina)` + `LoRA_Character_<nome>`

### NPCs — PERSONAS COMUNS (SEM LoRA, RAPIDEZ)

#### Taverneiro
```
Vox Machina western anime style,
fantasy innkeeper middle-aged, rugged face,
leather apron, sturdy build,
warm tavern candlelight, chest-level portrait,
painterly lineart, soft rim light
```

#### Guarda de cidade
```
Vox Machina anime western fantasy guard,
steel pauldron, blue surcoat with emblem,
stern expression, torch-lit environment,
painterly shading, clean brush strokes
```

#### Ladina misteriosa
```
Vox Machina anime western rogue portrait,
hooded figure, confident smirk,
dagger reflection, teal rimlight,
sharp lineart, dramatic shadows
```

### SCENES — SOCIAIS + EXPLORAÇÃO

Essas imagens entram como "ambiente narrativo". **Não use grid**.

#### Taverna
```
fantasy tavern interior, Vox Machina western anime style,
wooden beams, candle lanterns, patrons blurred,
painterly textures, warm color grading,
cinematic depth, dramatic chiaroscuro
```

#### Selva / Floresta
```
fantasy forest clearing, anime western Vox Machina style,
sunbeams, fog rays, lush foliage,
painted shadows, mystical atmosphere,
stone ruins hint at past civilization
```

#### Ruínas
```
ancient ruins under twilight, Vox Machina western anime style,
collapsed stone arches, moss, broken statues,
volumetric fog, painterly shading,
cinematic mood, no characters
```

#### Cidade viva
```
fantasy medieval city plaza at dusk,
anime western Vox Machina style,
stone buildings, merchant stalls, banners,
warm glow torchlight, volumetric air,
crowd blurred silhouettes
```

### BATTLEMAP — ISOMÉTRICO TÁTICO

🔥 **OBJETIVO**: jogabilidade. Jamais gerar concept art difusa. **Grid 5ft sutil.**

#### Dungeon / Interior
```
isometric fantasy dungeon battlemap,
Vox Machina western anime style,
stone floor with subtle 5ft grid lines,
pillars, broken walls, torch sconces,
high readability, painted shadows,
no characters, no text
```

#### Floresta
```
isometric forest battlemap, Vox Machina anime western style,
soft green floor with subtle 5ft grid,
rocks, fallen trees, moss patches,
readable paths, painterly light,
no characters, no UI
```

#### Fortaleza / Assalto
```
isometric castle courtyard battlemap,
Vox Machina anime western style,
flagstones, ramparts, crates, banners,
subtle 5ft grid, warm torch lighting,
clean tactical layout, no text
```

### EVENTOS — SPRITES / ANIMAÇÕES

#### Slash
```
anime western magical slash effect,
neon arc energy, painterly vfx strokes,
transparent background, cinematic motion
```

#### Fire
```
anime western magical fireburst,
bold orange core, blue rim sparks,
painterly flame shapes, transparent background
```

#### Ice
```
anime western ice spike burst,
sharp crystalline geometry, cyan glow,
transparent background, clean silhouette
```

### NEGATIVE UNIVERSAL

Use **sempre**, independente do caso:
```
photorealistic, 3d realistic, game render,
uncanny valley, skin pores, blender render,
moe, kawaii, loli, hentai, gacha,
flat cel shading, voxel, cartoon childish,
overexposed, washed out, busy noisy background
```

### Dicas Profissionais

- Prompts curtos → arte consistente
- Grid só em battlemap
- NPCs temporários → evitar LoRA personagem
- Antagonista recorrente → LoRA
- Não gerar LoRA durante combate
- Nunca >3 LoRAs por prompt
- **Vox Machina ≠ anime japonês**
- Expressões faciais > detalhes minuciosos

---

Este sistema de geração de assets permite criar conteúdo visual dinâmico e consistente durante as sessões de jogo, melhorando a imersão e a experiência do jogador, enquanto mantém performance fluida através da separação clara entre modo sessão e modo preparação.
