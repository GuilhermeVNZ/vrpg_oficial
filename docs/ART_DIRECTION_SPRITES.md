# VRPG – Direção Artística para Sprites de Animação

**Template de Prompt e Direção Artística para Geração de Assets Animados**

---

## 🎨 Direção Artística

**Temática:** **Dungeons & Dragons (D&D)** - Todos os assets devem refletir o universo e estética de D&D, incluindo raças, classes, criaturas e elementos mágicos característicos do jogo.

**Estilo:** Dark Fantasy Anime Illustration (inspirado em *Solo Leveling* e *The Legend of Vox Machina*)  

**Formato:** **Miniaturas físicas de RPG de mesa** - As sprites devem parecer miniaturas físicas pintadas, com base circular ou hexagonal, como se fossem peças de tabuleiro de D&D.

**Perspectiva:** **Top-down estrito** (câmera olhando para baixo em ~80–90°; corpo com foreshortening mas legível)  

**Vibe:** Dark fantasy sombria e dramática, com forte presença mágica. Silhuetas fortes, leitura clara à primeira vista. **Aparência de jogo de RPG de mesa físico, não videogame digital.**

**Quantidade por Personagem/Criatura:** **9 sprites** - Cada personagem ou criatura deve ter 9 variações de sprites para representar diferentes ângulos, poses ou estados.

### Palavras-chave Visuais

Use combinações destes termos em todos os prompts:

- `Dungeons and Dragons` ou `D&D`
- `tabletop RPG miniature`
- `painted miniature`
- `physical game piece`
- `circular base` ou `hexagonal base`
- `dark fantasy`
- `anime style`
- `top-down view`
- `high contrast lighting`
- `dramatic shadows`
- `game asset`
- `solid color background` (NUNCA "transparent background" ou "checkered")
- `idle pose`

### Regras de Forma e Proporção (Personagens / Criaturas)

- **Base:** Toda miniatura deve ter uma base circular ou hexagonal visível, como peças físicas de RPG de mesa.
- **Proporções:** Ligeiramente exageradas, heroicas / anime (armas grandes, formas de armadura claras).
- **Silhueta:** Cada personagem deve ser reconhecível em silhueta pura preta vista de cima.
- **Densidade de Detalhes:** Maioria dos detalhes na cabeça, ombros, arma e tronco superior (mais próximos da câmera).
- **Iluminação:** Luz principal forte de cima ou levemente diagonal, com bordas mais escuras para "ancorar" a miniatura no tabuleiro.
- **Aparência:** Deve parecer uma miniatura física pintada à mão, não um sprite digital de videogame.

---

## 🌀 Direção de Animação

**Regra padrão: se o prompt não especificar o contrário, todo personagem ou criatura deve ser gerado em animação `IDLE`.**

- **Tipo de Animação:** `idle loop`
- **Movimento:** Respiração sutil, movimento de capa/tecido, movimento de cabelo e barba, runas/olhos brilhantes, pequeno tremor da arma.
- **O que NÃO fazer por padrão:** Sem ataques completos, sem ciclos de caminhada/corrida, sem grandes movimentos de câmera.
- **Pose:** Postura de prontidão – preparado para combate mas não atacando ativamente. Pés firmes, centro de massa estável.

Se o modelo não puder gerar animação (apenas frames estáticos), interprete `IDLE` como:

> "Personagem em postura de combate relaxada, claramente legível de cima, com movimento implícito em cabelo/tecido/luz."

---

## ⚙️ Especificações Técnicas

### Canvas e Grid

- **Proporção:** `1:1` (quadrado) **para todos os assets VRPG**.
- **Grid D&D:** Cada tile representa **5 pés** (padrão D&D 5e), garantindo compatibilidade com as regras de movimento e alcance do jogo.
- **Ajuste ao Grid:** O personagem, criatura ou ícone inteiro deve caber confortavelmente dentro de **um único tile 1×1**.
  - Nenhuma parte importante cortada pelas bordas.
  - Se algo quebrar o frame (ponta de capa, aura), não deve tornar o sprite ilegível quando reduzido.

### Resoluções Recomendadas

Você pode ajustar, mas mantenha a proporção **1:1**:

- **Personagens / Criaturas (Padrão):** `1024×1024` px  
- **Chefes / Criaturas Grandes:** `1536×1536` ou `2048×2048` px (ainda tratado como tile 1×1 conceitualmente; mais detalhes)  
- **Armas / Itens / Grimórios / Ícones:** `512×512` px  
- **Símbolos de UI / Ícones Pequenos:** `256×256` px

### Regras Técnicas Gerais

- **Formato:** PNG com **fundo transparente**.
- **Padding:** Use o canvas eficientemente; mantenha margem vazia de ~5–10% para evitar clipping na engine.
- **Centralização:** Centro de massa do personagem aproximadamente no centro da imagem.  
- **Orientação:** Personagem voltado **para o topo da tela** (para cima) a menos que especificado o contrário.

---

## 📝 Template de Prompt Mestre – Personagens e Criaturas

Use esta estrutura ao pedir a outro modelo para gerar um novo personagem ou monstro VRPG:

> **Prompt:**  
> `Tabletop RPG miniature, Dungeons and Dragons dark fantasy anime style, [raça/classe/papel], [descrição física curta], [estilo de armadura/roupa], [arma(s)], [efeitos ou runas notáveis]. Viewed from top-down angle, standing on circular/hexagonal base, painted miniature appearance, high-contrast lighting, detailed but readable from above, strong silhouette, 1:1 aspect ratio, fits entirely inside a single grid tile, solid dark background NO CHECKERED PATTERN, idle pose with subtle movement implied, physical tabletop game piece aesthetic.`

### Exemplo – Bárbaro Anão (Dark Fantasy D&D)

> `Tabletop RPG miniature, Dungeons and Dragons dark fantasy anime style dwarf barbarian, short and massively built, braided beard, scarred face, heavy fur cloak over spiked leather armor, wielding a huge double-headed battleaxe with runes glowing faint crimson. Viewed from top-down angle, standing on circular stone base, painted miniature appearance, dramatic high-contrast lighting, detailed but readable from above, strong bulky silhouette, 1:1 aspect ratio, fits entirely inside a single grid tile, solid dark grey background NO CHECKERED PATTERN, idle pose with subtle breathing implied, physical tabletop game piece aesthetic.`

### Geração de 9 Sprites por Personagem

Para cada personagem ou criatura, gere **9 variações** seguindo estas diretrizes:

1. **Sprite Principal (Idle)** - Pose padrão de prontidão
2. **Variação de Ângulo 1** - Mesmo personagem, ângulo ligeiramente rotacionado (45°)
3. **Variação de Ângulo 2** - Rotação adicional (90°)
4. **Variação de Ângulo 3** - Rotação adicional (135°)
5. **Pose de Ataque 1** - Postura ofensiva
6. **Pose de Defesa** - Escudo erguido ou postura defensiva
7. **Pose de Movimento** - Caminhando ou correndo
8. **Pose de Magia** - Conjurando (se aplicável) ou gesto especial
9. **Pose Alternativa** - Ferido, cansado ou estado especial

---

## 🧱 Template de Prompt Mestre – Armas, Robes e Grimórios (Ícones)

Todos os ícones seguem a mesma regra de **grid 1×1** e devem ocupar os **2/3 superiores da imagem**, deixando espaço visual na parte inferior para overlays ou frames de raridade.

> **Prompt:**  
> `Dungeons and Dragons dark fantasy anime style VRPG item icon, [descrição do objeto], centered in frame, viewed slightly from above (top-down readability), occupying the upper two-thirds of a 1:1 canvas. Sharp silhouette, detailed texture, high-contrast lighting, subtle glow if magical, PNG with transparent background, game icon.`

### Exemplo – Quarterstaff (Ícone)

> `Dungeons and Dragons dark fantasy anime style VRPG item icon, long enchanted quarterstaff made of dark wood with silver inlays and a faint icy blue crystal at the top, oriented vertically in the center, occupying the upper two-thirds of a 1:1 canvas. Sharp silhouette, high-contrast lighting, soft cold glow from the crystal, PNG with transparent background, game icon.`

### Exemplo – Robe de Mago com Capuz (Ícone)

> `Dungeons and Dragons dark fantasy anime style VRPG item icon, ornate hooded mage robe hanging as if on an invisible figure, deep navy fabric with silver embroidered runes and leather straps, centered and occupying the upper two-thirds of a 1:1 canvas. Dramatic lighting from above, cloak folds clear and readable, subtle arcane glow around the hood, PNG with transparent background, game icon.`

### Exemplo – Grimório (Ícone)

> `Dungeons and Dragons dark fantasy anime style VRPG item icon, ancient spell grimoire slightly open, dark leather cover with a central metal sigil and glowing violet runes, faint smoke or arcane mist rising, centered and occupying the upper two-thirds of a 1:1 canvas. High-contrast lighting, crisp silhouette, PNG with transparent background, game icon.`

---

## 🐉 Exemplos de Prompts de Personagens (para Variedade)

### 1. Assassino Élfico das Sombras

> `Tabletop RPG miniature, Dungeons and Dragons dark fantasy anime style shadow elven assassin, slender build, pale grey skin, long white hair tied back, black leather armor with purple accents, dual curved daggers glowing faintly toxic green. Viewed from top-down angle, standing on circular dark base, painted miniature appearance, strong silhouette, 1:1 aspect ratio, fits inside one grid tile, dramatic rim lighting, solid dark background NO CHECKERED PATTERN, idle pose with subtle movement implied, physical tabletop game piece aesthetic.`

### 2. Bruxo Humano

> `Tabletop RPG miniature, Dungeons and Dragons dark fantasy anime style human warlock, tall and thin, crimson eyes, long dark coat with golden arcane patterns, wielding a staff crowned with a burning red crystal. Dark aura swirling around feet. Viewed from top-down angle, standing on hexagonal mystical base, painted miniature appearance, strong silhouette, detailed upper body, 1:1 aspect ratio, single grid tile, solid dark background NO CHECKERED PATTERN, idle pose with crystal pulsing implied, physical tabletop game piece aesthetic.`

### 3. Cavaleiro Abissal (Inimigo Elite)

> `Tabletop RPG miniature, Dungeons and Dragons dark fantasy anime style abyssal knight enemy, towering black armor with jagged plates, glowing orange cracks between the armor segments like cooled lava, massive cursed greatsword dragged slightly behind. Viewed from top-down angle, standing on circular molten base, painted miniature appearance, strong bulky silhouette readable from above, 1:1 aspect ratio, fits inside one grid tile, high-contrast fiery highlights, solid dark background NO CHECKERED PATTERN, idle pose with subtle glow on armor, physical tabletop game piece aesthetic.`

### 4. Guardião da Peste (Inimigo Caster)

> `Tabletop RPG miniature, Dungeons and Dragons dark fantasy anime style plague warden enemy, hunched figure in tattered dark green robes, plague doctor mask, staff topped with a sickly green lantern leaking ghostly fumes. Viewed from top-down angle, standing on circular corrupted base, painted miniature appearance, narrow but clear silhouette, 1:1 aspect ratio, single grid tile, solid dark background NO CHECKERED PATTERN, idle pose with lantern glow implied, physical tabletop game piece aesthetic.`

---

## 🚀 Uso Rápido para Novas Ideias

Sempre que precisar de um novo asset para VRPG, você pode dizer algo como:

> **"Gere um [personagem/inimigo/item] Dungeons and Dragons dark fantasy anime style VRPG top-down usando o estilo artístico padrão VRPG e animação idle."**

Ou cole este contexto em outro modelo:

> **Contexto:**  
> `Preciso de um asset de jogo VRPG em estilo Dungeons and Dragons dark fantasy anime, inspirado em Solo Leveling e The Legend of Vox Machina. Use uma visão top-down estrita, proporção 1:1, e certifique-se de que todo o design caiba dentro de um único tile de grid. Personagens e criaturas devem estar em loop de animação idle (respiração, movimento de tecido, efeitos sutis), prontos para combate mas não atacando. Use iluminação de alto contraste, silhuetas fortes, e PNG com fundo transparente. O asset deve refletir a temática e estética de Dungeons & Dragons (raças, classes, criaturas e elementos mágicos característicos). Agora gere um prompt detalhado para um [descrever personagem/criatura/item].`

Isso mantém **todos os assets** (heróis, monstros, armas, ícones) visualmente coerentes para VRPG e prontos para serem colocados diretamente em um jogo baseado em **grid 1×1** top-down.

---

## 🎬 Especificações para Animações no Battlemap

### Tipos de Animações Suportadas

#### 1. Animação Idle (Padrão)

**Uso:** Estado padrão de personagens e criaturas no battlemap quando não estão executando ações.

**Características:**
- Loop contínuo e suave
- Duração recomendada: 1–2 segundos por ciclo
- Movimentos sutis e naturais
- Sem transições bruscas

**Elementos Animados:**
- Respiração sutil do torso
- Movimento leve de capa/vestes
- Cabelo/barba balançando suavemente
- Efeitos mágicos pulsantes (runas, olhos brilhantes)
- Pequeno tremor de armas (se aplicável)

#### 2. Animação de Ataque

**Uso:** Executada quando o personagem realiza um ataque.

**Características:**
- Animação única (não loop)
- Duração: 0.3–0.8 segundos
- Movimento claro e legível mesmo em escala reduzida
- Retorna para idle após conclusão

**Elementos Animados:**
- Movimento do braço/arma
- Deslocamento do corpo (se aplicável)
- Efeitos de impacto (opcional, como overlay)

#### 3. Animação de Movimento

**Uso:** Quando o personagem se move entre tiles.

**Características:**
- Loop durante o movimento
- Duração por frame: 0.1–0.2 segundos
- Transição suave entre tiles
- Pode ser substituída por movimento interpolado pela engine

#### 4. Animação de Magia/Abilidade

**Uso:** Durante o cast de magias ou habilidades especiais.

**Características:**
- Animação única ou loop curto
- Duração: 0.5–1.5 segundos
- Efeitos visuais claros e reconhecíveis
- Retorna para idle após conclusão

**Elementos Animados:**
- Gestos de mão/braço
- Efeitos mágicos ao redor do personagem
- Mudança de postura (se aplicável)

#### 5. Animação de Dano/Morte

**Uso:** Quando o personagem recebe dano ou é derrotado.

**Características:**
- Animação única
- Duração: 0.5–1 segundo
- Movimento claro mas não excessivo
- Pode ter estado final (morto/caído)

### Formato de Spritesheet

**Estrutura Recomendada:**

```
sprite_<character_id>_<animation_type>.png
```

**Layout de Spritesheet:**
- Frames organizados horizontalmente (da esquerda para direita)
- Todos os frames na mesma linha
- Tamanho de frame consistente (ex: 1024×1024 por frame)
- Espaçamento mínimo entre frames (2–4px)

**Exemplo de Spritesheet:**
- `sprite_dwarf_barbarian_idle.png` → 8 frames, 8192×1024 (8×1024)
- `sprite_dwarf_barbarian_attack.png` → 6 frames, 6144×1024 (6×1024)

### Metadados de Animação

Cada spritesheet deve ter um arquivo JSON associado:

```json
{
  "character_id": "dwarf_barbarian",
  "animation_type": "idle",
  "frame_count": 8,
  "frame_width": 1024,
  "frame_height": 1024,
  "fps": 12,
  "loop": true,
  "duration_seconds": 0.67
}
```

### Integração com Battlemap

**Regras de Renderização:**

1. **Escala:** Sprites são renderizados em escala fixa baseada no tamanho do tile (ex: 64×64px por tile)
2. **Orientação:** Personagens sempre voltados para o topo da tela por padrão
3. **Camadas:** Sprites de personagens renderizados acima do battlemap, abaixo de efeitos visuais
4. **Sincronização:** Animações sincronizadas com o sistema de turnos do jogo

**Performance:**

- Cache de spritesheets carregados em memória
- Animações pausadas quando fora da viewport
- LOD (Level of Detail) para sprites distantes (menos frames, menor resolução)

---

## 📋 Checklist de Qualidade

Antes de considerar um sprite pronto para uso no battlemap:

- [ ] Proporção 1:1 mantida
- [ ] Fundo transparente (PNG)
- [ ] Silhueta legível em escala reduzida (64×64px)
- [ ] Detalhes principais visíveis (cabeça, arma, torso superior)
- [ ] Iluminação de alto contraste
- [ ] Animação suave (se aplicável)
- [ ] Spritesheet formatado corretamente
- [ ] Metadados JSON incluídos
- [ ] Testado no battlemap em escala real

---

## 🔗 Referências

- [ASSETS_GENERATION.md](./ASSETS_GENERATION.md) - Pipeline geral de geração de assets
- [COMBAT_FLOW.md](./COMBAT_FLOW.md) - Fluxo de combate e integração de animações
- [DESIGN_SYSTEM.md](./DESIGN_SYSTEM.md) - Sistema de design visual do VRPG

---

**Última atualização:** 2025-01-XX  
**Versão:** 1.0.0

