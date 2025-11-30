# Problema - Fonemas Sobrepostos

## Data: 2025-11-25

## Sintoma

O áudio soa como "eoud" e parece que todos os fonemas estão sendo falados ao mesmo tempo, como se estivessem sobrepostos em um único instante.

## Análise

### Phonemes Gerados (en-gb):
- `["h", "ə", "l", "ə", "ʊ", " ", "w", "ɜ", "l", "d"]`
- Sequência de IDs: `[20, 59, 24, 59, 100, 3, 35, 62, 24, 17]`
- Com BOS/EOS: `[1, 20, 59, 24, 59, 100, 3, 35, 62, 24, 17, 2]`

### Possíveis Causas

1. **Ordem dos Fonemas**: Os fonemas podem estar na ordem errada
2. **Formato do Tensor**: O tensor pode estar em formato errado (shape ou layout)
3. **Interpretação do Modelo**: O modelo pode estar interpretando os fonemas de forma errada
4. **Problema com BOS/EOS**: Os tokens BOS/EOS podem estar causando problemas
5. **Problema com Espaços**: O espaço (ID 3) pode estar sendo interpretado incorretamente

## Próximos Passos

1. ⏳ Verificar se o tensor precisa estar em formato diferente
2. ⏳ Testar sem BOS/EOS tokens
3. ⏳ Verificar se há necessidade de padding ou formatação especial
4. ⏳ Comparar com implementação Python oficial
5. ⏳ Verificar se os fonemas precisam estar em ordem diferente


