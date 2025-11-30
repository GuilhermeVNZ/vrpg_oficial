# Comparação Python vs Rust

## Teste com mesmos fonemas: [20, 59, 24, 27, 100, 3, 35, 62, 24, 17]

### Python (test_piper_python.py)
- Shape: `[1, 1, 1, 12288]`
- Amostras: 12,288
- Duração: 0.56s
- Range: `[-0.606, 0.496]`

### Rust (última execução)
- Shape: `[1, 1, 1, 11264]`
- Amostras: 11,264
- Duração: 0.51s
- Range: `[-0.594067, 0.414122]`

## Diferenças Observadas
1. **Número de amostras**: Python gera mais amostras (12,288 vs 11,264)
2. **Range de valores**: Similar, mas Python tem valores ligeiramente maiores

## Possíveis Causas
1. ⏳ Diferença no número de amostras pode indicar problema na inferência
2. ⏳ Pode haver diferença na forma como os tensores são passados
3. ⏳ Pode haver diferença na forma como o modelo está sendo executado


