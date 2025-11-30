# Causas Possíveis - Áudio Ininteligível

## Data: 2025-11-25

## Problema
Áudio soa como "eoud" - todos os fonemas parecem ser falados simultaneamente.

## Testes Realizados

### Python (Referência)
- Fonemas: `[20, 59, 24, 27, 100, 3, 35, 62, 24, 17]`
- Primeiros 20 valores: `[0.0079008, -0.00091667, 0.00364618, ...]`
- Range: `[-0.555933, 0.456614]`
- Amostras: 12,032 (0.56s)
- **Áudio**: ✅ Inteligível

### Rust (Nossa Implementação)
- Fonemas: `[20, 59, 24, 27, 100, 3, 35, 62, 24, 17]` (mesmos)
- Primeiros 10 valores: `[0.025528392, 0.010824707, 0.020378623, ...]`
- Range: `[-0.594067, 0.414122]`
- Amostras: 11,264 (0.51s)
- **Áudio**: ❌ Ininteligível

## Diferenças Críticas

1. **Valores de áudio diferentes** mesmo com mesmos fonemas
2. **Número de amostras diferente** (12,032 vs 11,264)
3. **Valores iniciais diferentes** (Python: 0.0079, Rust: 0.0255)

## Possíveis Causas

### 1. ⚠️ Ordem dos Inputs (MAIS PROVÁVEL)
- Python usa inputs nomeados: `{"input": ..., "input_lengths": ..., "scales": ...}`
- Rust usa inputs posicionais: `ort::inputs![input_tensor, input_lengths_tensor, scales_tensor]`
- **Ação**: Testar com inputs nomeados se `ort-rs` suportar

### 2. ⚠️ Formato do Tensor de Entrada
- Python: `np.array([phoneme_ids], dtype=np.int64)` → shape `[1, N]`
- Rust: `Tensor::from_array(([1, N], Vec<i64>))` → pode estar incorreto
- **Ação**: Verificar se `ort-rs` está criando o tensor corretamente

### 3. ⚠️ BOS/EOS Tokens
- Atualmente desabilitados (`use_bos_eos = false`)
- O modelo pode **REQUERER** BOS/EOS para funcionar corretamente
- **Ação**: Testar com BOS/EOS habilitados

### 4. ⚠️ Ordem dos Parâmetros `scales`
- Atualmente: `[noise_scale, length_scale, noise_w]` = `[0.667, 2.0, 0.8]`
- Pode estar na ordem errada apesar do JSON indicar esta ordem
- **Ação**: Testar diferentes ordens

### 5. ⚠️ Problema com `input_lengths`
- Python: `np.array([len(phoneme_ids)], dtype=np.int64)` → shape `[1]`
- Rust: `Tensor::from_array(([1], Vec<i64>))` → pode estar incorreto
- **Ação**: Verificar se está sendo passado corretamente

## Próximos Passos Prioritários

1. 🔴 **CRÍTICO**: Testar com BOS/EOS tokens habilitados
2. 🔴 **CRÍTICO**: Verificar se `ort-rs` suporta inputs nomeados e usar isso
3. 🟡 Verificar se há diferença na forma como `Tensor::from_array` cria o tensor
4. 🟡 Testar diferentes ordens de `scales`
5. 🟡 Comparar byte-a-byte os tensores de entrada entre Python e Rust


