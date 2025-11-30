# Solução: Sample Rate Mismatch

## 🔍 Problema Identificado

**Sample Rate Mismatch** é a causa raiz do som metálico:

- **XTTS gera**: 24000 Hz
- **SoVITS espera**: 44100 Hz
- **Re-amostragem**: 1.8375x (introduz artefatos metálicos)

## ✅ Solução Testada

Converter o áudio do XTTS para **44100 Hz ANTES** de passar para o SoVITS, evitando re-amostragem no modelo.

### Arquivo de Teste

**`FIXED_sample_rate_44100.wav`** - Gerado SEM re-amostragem no SoVITS

### Comparação

| Arquivo | Sample Rate Input | Re-amostragem | Status |
|---------|------------------|---------------|--------|
| `01_baseline.wav` | 24000 Hz | Sim (no SoVITS) | ❌ Metálico |
| `FIXED_sample_rate_44100.wav` | 44100 Hz | Não | ✅ Teste |

## 🎧 Validação

**Compare os arquivos:**
1. `01_baseline.wav` - Com re-amostragem (como estava)
2. `FIXED_sample_rate_44100.wav` - Sem re-amostragem (corrigido)

**Se o arquivo corrigido soar melhor**, aplicamos a correção no código.

## 📝 Implementação

Se validado, precisamos:

1. **Modificar XTTS** para gerar em 44100 Hz, OU
2. **Re-amostrar no pipeline** antes de passar para SoVITS

A opção 2 é mais simples e não requer mudanças no XTTS.

### Código Necessário

```rust
// No pipeline.rs, antes de chamar SoVITS:
if xtts_audio.sample_rate != 44100 {
    // Re-amostrar para 44100 Hz usando scipy ou similar
    xtts_audio = resample_to_44100(xtts_audio);
}
```

## 🔄 Próximos Passos

1. ✅ Diagnóstico completo
2. ✅ Teste de correção executado
3. ⏳ **Validação do usuário** (ouça FIXED_sample_rate_44100.wav)
4. ⏳ Aplicar correção no código (se validado)
5. ⏳ Testar pipeline completo

## 📊 Análise Técnica

### Distribuição Espectral

**Input (XTTS 24000 Hz):**
- Baixas (< 1kHz): 73.6%
- Médias (1-5kHz): 20.6%
- Altas (> 5kHz): 5.7%
- ✅ Distribuição normal

**Output (SoVITS 44100 Hz):**
- Baixas (< 1kHz): 65.9%
- Médias (1-5kHz): 27.4%
- Altas (> 5kHz): 6.6%
- ⚠️ Ligeira mudança (pode ser da re-amostragem)

### Métricas

| Métrica | Baseline | Fixed |
|---------|----------|-------|
| Sample Rate | 44100 Hz | 44100 Hz |
| Max Amplitude | 0.6377 | 0.6578 |
| RMS | 0.0664 | 0.0650 |
| Re-amostragem | Sim | Não |

## 💡 Outras Possíveis Causas (se ainda metálico)

Se mesmo com sample rate corrigido ainda estiver metálico:

1. **Problema no modelo treinado**
   - Testar com checkpoint anterior
   - Verificar overfitting

2. **Problema no dataset**
   - Vocal extraído de música?
   - Compressão excessiva?
   - Qualidade dos arquivos WAV

3. **Problema no XTTS**
   - Áudio de entrada já metálico?
   - Testar com áudio original do dataset

