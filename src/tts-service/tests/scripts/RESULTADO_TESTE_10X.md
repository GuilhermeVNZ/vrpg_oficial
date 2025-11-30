# Resultado do Teste de Latência - 10 Execuções

**Data**: 2025-11-29  
**Objetivo**: Verificar se `torch.compile()` melhora a latência após múltiplas execuções

---

## 📊 Estatísticas Gerais

| Métrica | Valor |
|---------|-------|
| **Média** | 3.125s |
| **Mediana** | 3.070s |
| **Mínimo** | 2.527s (execução 7) |
| **Máximo** | 4.005s (execução 9) |
| **Desvio Padrão** | 0.444s |

---

## 📈 Evolução da Latência

| Execução | Latência | Tendência |
|----------|----------|-----------|
| 1 | 3.500s | ➡️ |
| 2 | 2.996s | 📉 |
| 3 | 3.239s | 📈 |
| 4 | 2.893s | 📉 |
| 5 | 3.000s | 📈 |
| 6 | 2.550s | 📉 |
| 7 | 2.527s | 📉 (melhor) |
| 8 | 3.140s | 📈 |
| 9 | 4.005s | 📈 (pior) |
| 10 | 3.400s | 📉 |

---

## 🔍 Análise de Melhoria

### Comparação Primeiras vs Últimas Execuções

- **Primeiras 3 execuções (média)**: 3.245s
- **Últimas 3 execuções (média)**: 3.515s
- **Melhoria**: **-0.270s (-8.3%)** ❌ **PIOROU**

### Conclusões

1. **`torch.compile()` NÃO está melhorando a latência**
   - A latência não diminuiu com múltiplas execuções
   - Na verdade, piorou ligeiramente (-8.3%)

2. **Alta variação (desvio padrão: 0.444s)**
   - Indica que há outros fatores afetando a latência além do `torch.compile()`
   - Possíveis causas:
     - Variação normal do sistema operacional
     - Overhead do `torch.compile()` (compilação dinâmica)
     - Estado/cache da GPU variando entre execuções
     - Outros processos concorrentes

3. **Melhor latência observada**: 2.527s (execução 7)
   - Ainda está **muito acima** do target de ≤ 0.8s
   - Gap: ~1.7s a reduzir

4. **`torch.compile()` pode estar causando overhead**
   - A compilação dinâmica pode adicionar latência
   - Pode não ser adequado para este caso de uso (primeira inferência rápida)

---

## 💡 Recomendações

### 1. Remover `torch.compile()`

**Razão**: Não está melhorando a latência e pode estar causando overhead.

**Ação**: Remover a compilação do modelo e focar em outras otimizações.

### 2. Focar em Outras Otimizações

#### A. Reduzir Tamanho do Primeiro Chunk
- **Atual**: 20 chars → ~2.5s de áudio
- **Proposta**: 10-15 chars → ~1-1.5s de áudio
- **Impacto esperado**: -0.5s a -1.0s

#### B. Otimizar XTTS Diretamente
- Verificar configurações do modelo XTTS
- Ajustar parâmetros de inferência
- Usar batch size otimizado

#### C. Verificar FP16 Realmente Ativo
- **Atual**: Usando autocast como fallback
- **Ação**: Garantir que modelo está em `torch.float16`
- **Impacto esperado**: -0.3s a -0.5s

#### D. Pre-buffer Mínimo
- **Atual**: 100ms
- **Status**: ✅ Já implementado
- **Impacto**: Já reduzido ao mínimo

#### E. Pre-load Speaker Embedding
- **Status**: ✅ Implementado (via warm-up)
- **Verificar**: Se está realmente cacheando

### 3. Investigar Outras Fontes de Latência

- **Qwen 1.5B**: 0.1s (OK)
- **XTTS primeiro chunk**: ~2.5s (GARGALO)
- **Pre-buffer**: 0.1s (OK)
- **Overhead**: ~0.3s

**Foco principal**: Reduzir tempo de geração do primeiro chunk XTTS.

---

## 📋 Próximos Passos

1. ✅ Remover `torch.compile()` (não está ajudando)
2. ✅ Reduzir primeiro chunk para 10-15 chars
3. ✅ Garantir FP16 totalmente ativo
4. ✅ Otimizar configurações do XTTS
5. ✅ Investigar outras otimizações específicas do XTTS

---

## 🎯 Target vs Realidade

| Métrica | Target | Atual (melhor) | Gap |
|---------|--------|----------------|-----|
| **Latência total** | ≤ 0.8s | 2.527s | **1.727s** |
| **Primeiro chunk** | ≤ 0.5s | ~2.5s | **2.0s** |

**Conclusão**: Ainda há muito trabalho a fazer para atingir o target de ≤ 0.8s.

---

**Última atualização**: 2025-11-29



