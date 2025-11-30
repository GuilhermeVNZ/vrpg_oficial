# Resumo da Investigação - Áudio Ininteligível

## Data: 2025-11-25

## Status Atual

O áudio continua ininteligível mesmo após todas as correções aplicadas.

## Correções Aplicadas

1. ✅ Ordem dos parâmetros `scales`: `[noise_scale, length_scale, noise_w]`
2. ✅ Espaço mapeado para ID 3 (não ID 0)
3. ✅ Tokens BOS/EOS adicionados (^ -> ID 1, $ -> ID 2)
4. ✅ Diagnósticos detalhados adicionados

## Verificações Realizadas

### Phonemes IPA
- ✅ Esperado: `həloʊ wɜld`
- ✅ Recebido: `h ə l oʊ   w ɜ l d` (correto, com espaços)
- ✅ Mapeamento: 100% dos phonemes mapeados corretamente
- ✅ Sequência final: `[1, 20, 59, 24, 27, 100, 3, 35, 62, 24, 17, 2]`
  - ID 1 = BOS (^)
  - IDs 20, 59, 24, 27, 100 = "Hello" (h, ə, l, o, ʊ)
  - ID 3 = espaço
  - IDs 35, 62, 24, 17 = "World" (w, ɜ, l, d)
  - ID 2 = EOS ($)

### Áudio Gerado
- ✅ Duração: 640ms (adequado)
- ✅ Amostras: 14,080
- ✅ Range: [-0.756407, 0.633562] (dentro de [-1.0, 1.0])
- ✅ Sem NaN ou Inf
- ✅ Normalização aplicada quando necessário

## Possíveis Causas Restantes

### 1. Problema com o Modelo
- Modelo pode estar corrompido
- Modelo pode ser incompatível com nossa implementação
- **Ação**: Testar com modelo diferente

### 2. Problema com Phonemização
- Phonemes IPA podem estar corretos, mas a ordem pode estar errada
- Pode haver problema com digraphs (oʊ sendo dividido em o + ʊ)
- **Ação**: Verificar se o modelo espera digraphs como um único token

### 3. Problema com Normalização
- Normalização pode estar distorcendo o áudio
- **Ação**: Testar sem normalização

### 4. Problema com Formato de Saída
- Conversão para WAV pode estar introduzindo problemas
- **Ação**: Testar saída raw

### 5. Problema com Implementação ONNX
- Pode haver diferença na forma como estamos chamando o modelo
- **Ação**: Comparar com implementação Python oficial

## Próximos Passos

1. ⏳ Testar com modelo diferente
2. ⏳ Verificar se digraphs devem ser mantidos juntos
3. ⏳ Testar sem normalização
4. ⏳ Comparar com implementação Python oficial
5. ⏳ Verificar se há problema com o formato do tensor de entrada


