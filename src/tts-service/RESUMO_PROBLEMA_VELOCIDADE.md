# Resumo: Problema de Velocidade e Inteligibilidade

**Data:** 2025-11-25

## Problemas Identificados

### 1. Áudio Muito Curto
- **"Hello World"**: 325ms (esperado ~1-2s)
- **Texto longo (329 chars)**: ~7-10s (esperado ~50s)
- **Causa provável**: Poucos fonemas sendo mapeados ou muitos sendo pulados

### 2. Áudio Ininteligível
- Palavras não são formadas corretamente
- Som de "mumbling" ou "aglomerado de fonemas"
- **Causa provável**: 
  - Phonemização incorreta
  - Mapeamento de fonemas para IDs incorreto
  - Muitos fonemas desconhecidos sendo pulados

### 3. `length_scale` Não Funciona Como Esperado
- Valores > 1.0 tornam o áudio **mais rápido** (comportamento inesperado)
- Valores < 1.0 também não desaceleram significativamente
- **Causa provável**: 
  - Ordem dos parâmetros pode estar errada
  - O modelo ONNX pode interpretar os parâmetros de forma diferente
  - O parâmetro pode não ser o primeiro elemento do array `scales`

### 4. Time-Stretching Muda o Pitch
- Time-stretching simples (interpolação linear) muda o pitch
- Voz fica grave como "modulador de voz"
- **Solução**: Removido time-stretching, precisa de técnica que preserve pitch (PSOLA/WSOLA)

## Próximas Ações

### Prioridade 1: Corrigir Inteligibilidade
1. **Verificar logs do servidor** para ver:
   - Quantos fonemas IPA foram gerados
   - Quantos fonemas foram mapeados vs pulados
   - Quais fonemas estão sendo pulados (unknown phonemes)

2. **Verificar phonemização**:
   - Testar `espeak-ng` diretamente com "Hello World"
   - Verificar se os fonemas IPA gerados estão corretos
   - Comparar com a saída esperada do Piper Python

3. **Verificar mapeamento de fonemas**:
   - Verificar se o `phoneme_id_map` do modelo está correto
   - Verificar se estamos mapeando os fonemas corretamente
   - Adicionar fallback para fonemas desconhecidos (em vez de pular)

### Prioridade 2: Corrigir Velocidade
1. **Investigar ordem dos parâmetros `scales`**:
   - Verificar documentação do Piper ONNX
   - Testar diferentes ordens: `[length_scale, noise_scale, noise_w]` vs outras combinações
   - Verificar se o modelo espera os parâmetros em ordem diferente

2. **Testar valores extremos**:
   - Testar `length_scale = 0.01` (muito baixo)
   - Testar `length_scale = 20.0` (muito alto)
   - Ver se há algum threshold ou comportamento diferente

3. **Implementar time-stretching que preserve pitch**:
   - Usar técnica PSOLA (Pitch Synchronous Overlap and Add)
   - Ou usar biblioteca Rust para time-stretching com preservação de pitch
   - Como última opção, se `length_scale` não funcionar

## Testes Realizados

1. ✅ `length_scale = 1.5` → 7.72s (rápido)
2. ✅ `length_scale = 2.5` → 7.06s (mais rápido - inesperado!)
3. ✅ `length_scale = 0.3` → 7.10s (similar)
4. ✅ `length_scale = 5.0` → 9.15s (um pouco melhor, mas ainda rápido)
5. ✅ `length_scale = 10.0` → Áudio acelerado (pior)
6. ✅ Time-stretching (speed_factor = 0.5) → 15.35s mas pitch alterado (voz grave)
7. ✅ Removido time-stretching, voltado para `length_scale = 1.0` → 325ms para "Hello World" (muito curto)

## Conclusão

O problema principal **não é apenas a velocidade**, mas sim a **inteligibilidade e duração muito curta**. Precisamos primeiro corrigir a phonemização e o mapeamento de fonemas antes de ajustar a velocidade.



