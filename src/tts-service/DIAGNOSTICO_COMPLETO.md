# Diagnóstico Completo - Áudio Ininteligível

## Data: 2025-11-25

## Problema Identificado

O áudio gerado está muito curto e ininteligível. Para "Hello World":
- **Duração obtida**: 301ms
- **Duração esperada**: 500-800ms (mínimo)
- **Amostras geradas**: 6656 (sample_rate: 22050 Hz)

## Análise dos Logs

### 1. Phonemização ✅ CORRETO
- **Fonemas IPA gerados**: 9
- **Fonemas mapeados**: 9 (100%)
- **Fonemas pulados**: 0 (0%)
- **Phoneme IDs gerados**: 10 (o diphthong "oʊ" foi corretamente dividido)

### 2. Mapeamento de Fonemas ✅ CORRETO
```
[0] 'h' -> ID 20 (exact)
[1] 'ə' -> ID 59 (exact)
[2] 'l' -> ID 24 (exact)
[3] 'oʊ' -> split to 'o' -> ID 27
[3] 'oʊ' -> split to 'ʊ' -> ID 100
[4] ' ' -> ID 0 (pause)
[5] 'w' -> ID 35 (exact)
[6] 'ɜ' -> ID 62 (exact)
[7] 'l' -> ID 24 (exact)
[8] 'd' -> ID 17 (exact)
```

### 3. Problema Identificado ❌

**O modelo está gerando muito poucas amostras por fonema:**
- 10 phoneme IDs → 6656 amostras
- **Média**: ~666 amostras por phoneme ID
- **Duração por fonema**: ~30ms (muito curto!)

**Para comparação, um fonema normal deveria durar:**
- Vogais: 100-200ms
- Consoantes: 50-100ms
- Pausas: 50-200ms

### 4. Parâmetros ONNX Atuais

```
scales: [1.0, 1.0, 1.0]
- length_scale: 1.0 (padrão)
- noise_scale: 1.0
- noise_w: 1.0
```

## Hipóteses

### Hipótese 1: `length_scale` não está funcionando
- O parâmetro `length_scale` deveria controlar a duração
- Valores > 1.0 deveriam tornar o áudio mais lento
- **Teste necessário**: Aumentar `length_scale` para 2.0, 3.0, 5.0

### Hipótese 2: Ordem incorreta dos parâmetros `scales`
- O modelo pode esperar os parâmetros em ordem diferente
- **Teste necessário**: Testar diferentes ordens: `[noise_scale, length_scale, noise_w]`

### Hipótese 3: Modelo ONNX incompatível ou corrompido
- O modelo pode estar gerando amostras muito curtas por padrão
- **Teste necessário**: Verificar se o modelo funciona corretamente com outros valores

### Hipótese 4: Falta de tokens BOS/EOS
- Alguns modelos Piper requerem tokens de início/fim
- **Status**: Verificado - não há BOS/EOS no `phoneme_id_map`

## Próximos Passos

1. **Testar `length_scale` aumentado**:
   - Testar com `length_scale = 2.0` (duplicar duração)
   - Testar com `length_scale = 3.0` (triplicar duração)
   - Testar com `length_scale = 5.0` (quintuplicar duração)

2. **Testar ordem dos parâmetros**:
   - Testar `[noise_scale, length_scale, noise_w]`
   - Testar `[noise_w, length_scale, noise_scale]`

3. **Verificar documentação do modelo**:
   - Verificar se há documentação sobre os parâmetros `scales`
   - Verificar se há exemplos de uso do modelo

4. **Testar com modelo diferente**:
   - Se possível, testar com outro modelo Piper para comparar

## Conclusão

O problema **NÃO** está no mapeamento de fonemas (100% de sucesso), mas sim na **geração de amostras pelo modelo ONNX**. O modelo está gerando muito poucas amostras por fonema, resultando em áudio muito curto e ininteligível.

**Ação imediata**: Testar valores mais altos de `length_scale` para aumentar a duração do áudio.


