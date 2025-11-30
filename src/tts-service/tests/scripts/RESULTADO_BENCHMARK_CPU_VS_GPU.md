# Resultado do Benchmark: CPU vs GPU - Todas as 5 Vozes

**Data**: 2025-11-29  
**Parágrafo de teste**: 386 caracteres (novo parágrafo sobre biblioteca esquecida)

---

## 📊 Resultados Comparativos

### Tempo de Geração (segundos)

| Voz | CPU | GPU | Melhoria |
|-----|-----|-----|----------|
| **Mestre (Ana Florence)** | 214.47s | 16.06s | **⬇️ 92.5%** |
| **Lax Barros** | 190.78s | 16.04s | **⬇️ 91.6%** |
| **Common Voice Spontaneous** | 195.87s | 14.98s | **⬇️ 92.4%** |
| **Joe** | 181.69s | 22.34s | **⬇️ 87.7%** |
| **Kathleen** | 209.03s | 17.33s | **⬇️ 91.7%** |

### Real-Time Factor (RTF)

| Voz | CPU | GPU | Diferença |
|-----|-----|-----|-----------|
| **Mestre (Ana Florence)** | 8.60x | 0.64x | **13.4x melhor** |
| **Lax Barros** | 8.64x | 0.74x | **12.7x melhor** |
| **Common Voice Spontaneous** | 8.69x | 0.68x | **12.8x melhor** |
| **Joe** | 8.86x | 0.67x | **13.2x melhor** |
| **Kathleen** | 8.48x | 0.69x | **12.3x melhor** |

---

## 📈 Estatísticas Gerais

### Tempo Médio de Geração
- **CPU**: 198.37s (~3.3 minutos)
- **GPU**: 17.35s (~17 segundos)
- **Melhoria**: **⬇️ 91.3% mais rápido**
- **Speedup**: **~11.4x mais rápido na GPU**

### Real-Time Factor (RTF) Médio
- **CPU**: 8.65x (muito mais lento que tempo real)
- **GPU**: 0.68x (mais rápido que tempo real)
- **Melhoria**: **+92.1%** (RTF menor = melhor)

### Taxa de Sucesso
- **CPU**: 5/5 vozes (100%)
- **GPU**: 5/5 vozes (100%)

---

## 🎯 Conclusões

### 1. Performance
- **GPU é ~11x mais rápida que CPU** para geração de áudio XTTS
- CPU: ~3.3 minutos para gerar ~23s de áudio
- GPU: ~17 segundos para gerar ~23s de áudio

### 2. Real-Time Factor
- **CPU**: RTF 8.65x = precisa de 8.65x o tempo do áudio para gerar (muito lento)
- **GPU**: RTF 0.68x = gera mais rápido que tempo real (ideal para streaming)

### 3. Vantagens da GPU
- ✅ **91-92% mais rápido** em todas as vozes
- ✅ **RTF < 1.0** = pode gerar em tempo real
- ✅ **Todas as 5 vozes funcionam** (com monkey patch)
- ✅ **Suporte RTX 5090** (PyTorch nightly + CUDA 12.8)

### 4. Limitações da CPU
- ❌ **RTF 8.65x** = não é viável para tempo real
- ❌ **~3 minutos** para gerar 23s de áudio
- ❌ Não aproveita hardware de ponta (RTX 5090)

---

## 💡 Recomendações

1. **Usar GPU sempre que possível** - Melhoria de 91%+ é significativa
2. **RTF 0.68x na GPU** permite streaming em tempo real
3. **Monkey patch necessário** para embeddings customizados funcionarem
4. **PyTorch nightly** requerido para suporte RTX 5090

---

## 📁 Arquivos Gerados

- **CPU**: `benchmark_cpu_vs_gpu_20251129_092749/cpu_*.wav`
- **GPU**: `benchmark_cpu_vs_gpu_20251129_092749/gpu_*.wav`

Total: 10 arquivos de áudio (5 vozes × 2 devices)

---

**Última atualização**: 2025-11-29

