# Solução Final - Áudio Ininteligível

## Status: PROBLEMA PERSISTE

Após extensa investigação, o áudio continua ininteligível mesmo com:
- ✅ Fonemização correta
- ✅ Mapeamento de fonemas correto
- ✅ Ordem dos parâmetros `scales` correta
- ✅ Mapeamento de espaço correto (ID 3)
- ✅ BOS/EOS testado (com e sem)
- ✅ Formato do tensor correto
- ✅ Bytes dos tensores corretos (little-endian)
- ✅ Ordem dos inputs correta

## Diferença Crítica Identificada

**Python (Funciona):**
- Inputs nomeados: `{"input": ..., "input_lengths": ..., "scales": ...}`
- Áudio: ✅ Inteligível

**Rust (Não Funciona):**
- Inputs posicionais: `ort::inputs![input_tensor, input_lengths_tensor, scales_tensor]`
- Áudio: ❌ Ininteligível

## Conclusão

O problema está na forma como o `ort-rs` (versão 2.0.0-rc.10) interage com o modelo ONNX. Mesmo com todos os parâmetros corretos, há uma diferença fundamental na forma como os inputs são passados:

1. **Python** usa inputs nomeados, garantindo que cada tensor vá para o input correto
2. **Rust** usa inputs posicionais, que podem estar sendo interpretados incorretamente pelo `ort-rs`

## Soluções Possíveis

### 1. Atualizar ort-rs
- Verificar se há versão mais recente que suporte inputs nomeados
- Ou versão que corrija bugs conhecidos

### 2. Usar biblioteca alternativa
- `ort-tract` ou outra biblioteca Rust para ONNX
- Ou usar FFI para chamar ONNX Runtime C API diretamente

### 3. Workaround: Chamar Python via subprocess
- Usar o script Python `test_piper_python.py` como backend
- Não é ideal, mas funcionaria

### 4. Investigar ort-rs mais profundamente
- Verificar se há forma de usar inputs nomeados
- Ou se há bug conhecido na versão atual

## Próximos Passos Recomendados

1. Abrir issue no repositório do `ort-rs` com este problema
2. Testar com versão mais recente do `ort-rs` (se disponível)
3. Considerar usar FFI para ONNX Runtime C API diretamente
4. Como último recurso, usar Python via subprocess


