# Resumo da Investigação - Áudio Ininteligível

## Data: 2025-11-25

## Problema
Áudio soa como "eoud" - todos os fonemas parecem ser falados simultaneamente.

## Testes Realizados

### Python (Referência - SEM BOS/EOS)
- Fonemas: `[20, 59, 24, 27, 100, 3, 35, 62, 24, 17]`
- Inputs: `{"input": input_ids, "input_lengths": input_lengths, "scales": scales}` (nomeados)
- Amostras: 11,776 (0.53s)
- **Áudio**: ✅ Inteligível

### Rust (Nossa Implementação - COM BOS/EOS)
- Fonemas: `[1, 20, 59, 24, 27, 100, 3, 35, 62, 24, 17, 2]` (com BOS/EOS)
- Inputs: `ort::inputs![input_tensor, input_lengths_tensor, scales_tensor]` (posicionais)
- Amostras: 10,752 (0.49s)
- **Áudio**: ❌ Ininteligível

## Diferenças Críticas

1. **Python usa inputs nomeados**, Rust usa inputs posicionais
2. **Valores de áudio completamente diferentes** mesmo com mesmos fonemas
3. **Número de amostras diferente** (11,776 vs 10,752)

## Causa Mais Provável

### 🔴 ORDEM DOS INPUTS (CRÍTICO)
O `ort-rs` pode estar passando os inputs na ordem errada quando usamos `ort::inputs![]` posicional. O Python usa inputs nomeados, garantindo a ordem correta.

**Solução**: Verificar se `ort-rs` suporta inputs nomeados ou garantir que a ordem posicional está correta.

## Próximos Passos

1. 🔴 **CRÍTICO**: Verificar se `ort-rs` suporta inputs nomeados (HashMap/BTreeMap)
2. 🔴 **CRÍTICO**: Testar sem BOS/EOS novamente (Python não usa)
3. 🟡 Verificar se há diferença na forma como `Tensor::from_array` cria o tensor
4. 🟡 Comparar byte-a-byte os tensores de entrada entre Python e Rust
5. 🟡 Verificar se há problema com a versão do `ort-rs` ou do modelo ONNX


