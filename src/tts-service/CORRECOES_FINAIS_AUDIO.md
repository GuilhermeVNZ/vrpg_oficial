# Correções Finais - Áudio Ininteligível

## Data: 2025-11-25

## Problemas Identificados e Corrigidos

### 1. ✅ Ordem Incorreta dos Parâmetros `scales`
**Problema**: Estávamos passando `[length_scale, noise_scale, noise_w]`  
**Correção**: Ordem correta é `[noise_scale, length_scale, noise_w]` (do JSON do modelo)

### 2. ✅ Mapeamento Incorreto de Espaços
**Problema**: Espaços `" "` estavam sendo mapeados para ID 0 (que é `"_"`)  
**Correção**: Espaços agora mapeiam para ID 3 (correto, conforme JSON do modelo)

### 3. ✅ Tokens BOS/EOS Faltando
**Problema**: Não estávamos adicionando tokens de início/fim da sequência  
**Correção**: Agora adicionamos automaticamente:
- BOS token `"^"` -> ID 1 (no início)
- EOS token `"$"` -> ID 2 (no final)

## Mapeamento Correto do Modelo

Do arquivo `piper-en-us.onnx.json`:
- `"_"` -> ID 0 (underscore, não espaço!)
- `"^"` -> ID 1 (BOS - Beginning of Sequence)
- `"$"` -> ID 2 (EOS - End of Sequence)
- `" "` -> ID 3 (espaço/pausa)

## Resultados

### Antes das Correções:
- Duração: ~300ms (muito curto)
- Áudio: Ininteligível
- Problemas: Espaços errados, sem BOS/EOS, parâmetros na ordem errada

### Depois das Correções:
- Duração: ~570ms (adequado para "Hello World")
- Amostras: 12,544 (com BOS/EOS tokens)
- Sequência: `[1, 20, 59, 24, 27, 100, 3, 35, 62, 24, 17, 2]`
  - ID 1 = BOS
  - IDs 20, 59, 24, 27, 100 = "Hello"
  - ID 3 = espaço
  - IDs 35, 62, 24, 17 = "World"
  - ID 2 = EOS

## Status

✅ **TODAS AS CORREÇÕES APLICADAS**

O áudio agora deve estar inteligível. Se ainda houver problemas, pode ser necessário:
1. Verificar a qualidade do modelo
2. Ajustar parâmetros `noise_scale` e `noise_w`
3. Testar com diferentes textos


