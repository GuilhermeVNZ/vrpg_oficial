# Investigação - Áudio Ininteligível Persistente

## Data: 2025-11-25

## Status Atual

Mesmo após corrigir a ordem dos parâmetros `scales`, o áudio ainda está ininteligível.

### Correções Aplicadas
1. ✅ Ordem dos parâmetros corrigida: `[noise_scale, length_scale, noise_w]`
2. ✅ Duração aumentou de 301ms para 557ms
3. ✅ Amostras aumentaram de 6656 para 12288
4. ❌ **Áudio ainda ininteligível**

## Possíveis Causas Adicionais

### 1. Problema com Formato do Tensor de Saída
- Shape do output: `[1, 1, 1, 12288]`
- Pode haver necessidade de reshape ou flatten
- Valores podem estar fora do range esperado [-1.0, 1.0]

### 2. Problema com Normalização
- Valores podem estar em range diferente (ex: [0, 1] ou [-32768, 32768])
- Pode precisar de normalização ou conversão

### 3. Problema com Mapeamento de Fonemas
- Mesmo com 100% de mapeamento, pode haver IDs errados
- Pode haver problema com a ordem dos fonemas

### 4. Problema com o Modelo
- Modelo pode estar corrompido
- Modelo pode ser incompatível com nossa implementação

### 5. Problema com Phonemização
- Phonemes IPA podem estar incorretos
- Pode haver problema com espeak-ng vs espeak

## Diagnósticos Adicionados

Adicionados logs para verificar:
- Primeiros e últimos valores de áudio
- Presença de NaN ou Inf
- Range de valores
- Normalização automática se necessário

## Próximos Passos

1. ✅ Verificar valores de áudio (NaN, Inf, range)
2. ⏳ Comparar com implementação Python oficial do Piper
3. ⏳ Testar com modelo diferente
4. ⏳ Verificar se há problema com phonemização
5. ⏳ Verificar se há problema com formato de saída

## Referências

- [Piper TTS GitHub](https://github.com/rhasspy/piper)
- [Piper Python Implementation](https://github.com/rhasspy/piper-python)


