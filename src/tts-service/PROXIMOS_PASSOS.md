# Próximos Passos - Investigação Áudio Ininteligível

## Data: 2025-11-25

## Status Atual
- ✅ Fonemização corrigida (əʊ dividido corretamente)
- ✅ Mapeamento de fonemas correto
- ✅ Ordem dos parâmetros `scales` corrigida
- ✅ Mapeamento de espaço corrigido (ID 3)
- ✅ BOS/EOS testado (com e sem)
- ❌ Áudio ainda ininteligível

## Diferenças Críticas Identificadas

### Python (Funciona)
- Inputs nomeados: `{"input": ..., "input_lengths": ..., "scales": ...}`
- Amostras: 11,776 (0.53s)
- Valores iniciais: `[0.0079008, -0.00091667, ...]`
- **Áudio**: ✅ Inteligível

### Rust (Não Funciona)
- Inputs posicionais: `ort::inputs![input_tensor, input_lengths_tensor, scales_tensor]`
- Amostras: 11,520 (0.52s) - similar
- Valores iniciais: `[0.042473674, 0.029806431, ...]` - DIFERENTE!
- **Áudio**: ❌ Ininteligível

## Possíveis Causas Restantes

### 1. 🔴 Ordem dos Inputs (MAIS PROVÁVEL)
- `ort-rs` pode estar passando inputs na ordem errada
- Python usa inputs nomeados, garantindo ordem correta
- Rust usa inputs posicionais, pode estar incorreto

### 2. 🟡 Formato do Tensor
- Python: `np.array([phoneme_ids], dtype=np.int64)` → shape `[1, N]`
- Rust: `Tensor::from_array(([1, N], Vec<i64>))` → pode estar incorreto
- Verificar se `ort-rs` está criando o tensor corretamente

### 3. 🟡 Precisão dos Valores
- Valores de áudio completamente diferentes mesmo com mesmos fonemas
- Pode haver diferença na forma como os valores são interpretados

### 4. 🟡 Versão do ort-rs
- Pode haver bug na versão atual do `ort-rs`
- Verificar se há atualizações ou issues conhecidos

## Próximos Passos Prioritários

1. 🔴 **CRÍTICO**: Verificar se `ort-rs` suporta inputs nomeados
2. 🔴 **CRÍTICO**: Comparar byte-a-byte os tensores de entrada
3. 🟡 Verificar se há diferença na forma como `Tensor::from_array` cria o tensor
4. 🟡 Testar com versão diferente do `ort-rs` ou modelo ONNX
5. 🟡 Verificar se há problema com a versão do ONNX Runtime

## Referências
- Python test: `test_piper_python.py`
- Documentação: `RESUMO_INVESTIGACAO_FINAL.md`
- Causas: `CAUSAS_POSSIVEIS.md`


