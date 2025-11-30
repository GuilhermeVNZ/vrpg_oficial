# Investigação Profunda - Áudio Ininteligível

## Data: 2025-11-25

## Problema
O áudio gerado soa como "eoud" e parece que todos os fonemas estão sendo falados ao mesmo tempo.

## Testes Realizados

### 1. Teste Python Direto
- **Fonemas**: `[20, 59, 24, 27, 100, 3, 35, 62, 24, 17]`
- **Resultado**: 12,032 amostras (0.56s)
- **Áudio**: Gerado corretamente pelo Python

### 2. Teste Rust
- **Fonemas**: Mesmos IDs
- **Resultado**: 11,264-13,056 amostras (similar)
- **Áudio**: Ininteligível

## Possíveis Causas

### 1. Formato do Tensor de Entrada
- Python usa: `np.array([phoneme_ids], dtype=np.int64)` → shape `[1, N]`
- Rust usa: `Tensor::from_array(([1, N], Vec<i64>))` → deve ser equivalente
- **Status**: ✅ Parece correto

### 2. Ordem dos Inputs
- Python: `{"input": input_ids, "input_lengths": input_lengths, "scales": scales}`
- Rust: `ort::inputs![input_tensor, input_lengths_tensor, scales_tensor]`
- **Status**: ✅ Parece correto (ordem posicional)

### 3. Formato do Tensor de Saída
- Python: `outputs[0]` → shape `[1, 1, 1, N]` → `flatten()` → `[N]`
- Rust: `outputs[0].try_extract_tensor::<f32>()` → `(shape, data)` → `data` já é flat?
- **Status**: ⚠️ PRECISA VERIFICAR - pode estar extraindo incorretamente

### 4. Fonemização
- `espeak-ng` retorna: `həlˈəʊ wˈɜːld`
- Parser divide: `h`, `ə`, `l`, `əʊ` → `o`, `ʊ`, ` `, `w`, `ɜ`, `l`, `d`
- **Status**: ✅ Corrigido (əʊ agora é dividido corretamente)

### 5. BOS/EOS Tokens
- Atualmente desabilitados (`use_bos_eos = false`)
- **Status**: ⚠️ Pode ser necessário

## Próximos Passos

1. ⏳ Verificar se `try_extract_tensor` está extraindo o tensor corretamente (pode estar mantendo shape 4D)
2. ⏳ Testar com BOS/EOS tokens habilitados
3. ⏳ Comparar valores de áudio entre Python e Rust
4. ⏳ Verificar se há algum problema com a normalização do áudio


