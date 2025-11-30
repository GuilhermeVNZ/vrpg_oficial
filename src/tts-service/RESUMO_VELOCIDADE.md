# Resumo: Investigação da Velocidade do Áudio

**Data:** 2025-11-25

**Problema Reportado:**
O áudio gerado pelo Piper está "MUITO rápido" e ininteligível.

**Testes Realizados:**

1. **length_scale = 1.5** (padrão era 1.0)
   - Duração: 7.72 segundos
   - Ratio: 0.16 (muito rápido)

2. **length_scale = 2.5**
   - Duração: 7.06 segundos
   - Ratio: 0.14 (ainda mais rápido - comportamento inesperado!)

3. **length_scale = 0.3**
   - Duração: 7.10 segundos
   - Ratio: 0.14 (similar aos anteriores)

**Observações:**

- A duração do áudio permanece aproximadamente constante (~7s) independentemente do valor de `length_scale`
- Valores maiores de `length_scale` (2.5) resultaram em áudio mais rápido, não mais lento
- Isso sugere que:
  1. O `length_scale` pode não ser o primeiro elemento do array `scales`
  2. O modelo ONNX pode não estar interpretando o parâmetro corretamente
  3. A ordem dos parâmetros pode estar invertida
  4. O parâmetro pode não estar sendo aplicado pelo modelo

**Próximos Passos:**

1. Verificar a documentação do Piper Python para confirmar a ordem correta dos parâmetros em `scales`
2. Testar diferentes elementos do array `scales` (talvez o segundo ou terceiro elemento controle a velocidade)
3. Verificar se há alguma configuração no modelo ONNX que precise ser ajustada
4. Comparar com a implementação Python do Piper para ver como os `scales` são passados

**Arquivo de Teste:**
- `test_velocidade_corrigida.wav` (última versão testada)



