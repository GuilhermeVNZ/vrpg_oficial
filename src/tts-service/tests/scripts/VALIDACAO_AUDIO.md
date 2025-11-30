# Validação de Áudio - Grid de Testes SoVITS

## ⚠️ PROBLEMA CRÍTICO DETECTADO

**Sample Rate Mismatch:**
- **Input (XTTS)**: 24000 Hz
- **Modelo SoVITS**: 44100 Hz
- **Impacto**: Isso pode causar voz robótica e artefatos!

**Solução**: O SoVITS está re-amostrando automaticamente, mas isso pode introduzir problemas. Idealmente, o XTTS deveria gerar em 44100 Hz ou o SoVITS deveria aceitar 24000 Hz.

## 📊 Testes Executados

8 de 10 testes foram concluídos com sucesso. 2 falharam por falta de modelos FCPE (não crítico).

### Arquivos para Validação

Localização: `sovits_quality_tests/`

1. **01_baseline.wav** - Parâmetros atuais
   - `noice_scale=0.4`, `auto_f0=True`, `rmvpe`
   - **Referência**: Como estávamos usando antes

2. **02_noice_0.2.wav** - noice_scale reduzido
   - `noice_scale=0.2` (mais natural)
   - **Teste**: Se reduzir noise melhora a qualidade

3. **03_noice_0.1.wav** - noice_scale muito baixo
   - `noice_scale=0.1` (muito natural, pode perder características)
   - **Teste**: Se valores muito baixos ainda mantêm características

4. **04_no_auto_f0.wav** - auto_predict_f0 desabilitado
   - `auto_predict_f0=False`
   - **Teste**: Se desabilitar auto-F0 melhora dinâmica

5. **06_f0_crepe.wav** - F0 predictor crepe
   - `f0_predictor=crepe` (mais lento, mas pode ser melhor)
   - **Teste**: Se crepe produz melhor qualidade que rmvpe

6. **07_pad_0.8.wav** - pad_seconds aumentado
   - `pad_seconds=0.8` (evita cortes de fonemas)
   - **Teste**: Se mais padding reduz artefatos de corte

7. **08_slice_-35.wav** - slice_db menos agressivo
   - `slice_db=-35` (menos cortes agressivos)
   - **Teste**: Se slice menos agressivo melhora continuidade

8. **09_optimized.wav** - Combinação otimizada ⭐
   - `noice_scale=0.2`, `pad_seconds=0.8`, `slice_db=-35`
   - **Teste**: Combinação de todas as otimizações

## 🎧 Como Validar

1. **Ouça cada arquivo** na ordem listada
2. **Compare com o baseline** (01_baseline.wav)
3. **Identifique qual soa mais natural e menos robótico**
4. **Anote o número do arquivo** que preferir

## 📝 O que Procurar

### Sinais de Melhoria ✅
- Voz mais natural e menos robótica
- Melhor dinâmica (variação de pitch)
- Menos artefatos metálicos
- Menos "cortes" ou "picotados"
- Melhor prosódia (ritmo natural)

### Sinais de Problema ❌
- Voz ainda robótica/monótona
- Artefatos metálicos ou "vibrado"
- Cortes abruptos
- Perda de características da voz
- Som "dentro de lata"

## 🔍 Análise Técnica

### Métricas dos Testes

| Teste | Tempo (s) | RMS | Descrição |
|-------|-----------|-----|-----------|
| 01_baseline | 2.19 | 0.0751 | Referência |
| 02_noice_0.2 | 0.20 | 0.0683 | ⚡ Muito mais rápido |
| 03_noice_0.1 | 0.20 | 0.0658 | ⚡ Muito mais rápido |
| 04_no_auto_f0 | 0.18 | 0.0773 | ⚡ Mais rápido, RMS maior |
| 06_f0_crepe | 4.80 | 0.0681 | 🐌 Mais lento |
| 07_pad_0.8 | 1.69 | 0.0664 | Mais padding |
| 08_slice_-35 | 0.26 | 0.0683 | ⚡ Rápido |
| 09_optimized | 0.30 | 0.0664 | ⚡ Rápido, otimizado |

**Observação**: Testes com `noice_scale` menor são significativamente mais rápidos!

## 🎯 Próximos Passos

1. **Você valida os áudios** e identifica o melhor
2. **Aplicamos os parâmetros** do melhor teste no código
3. **Investigamos o sample rate mismatch** (pode ser a causa raiz)
4. **Se necessário**: Ajustamos o XTTS para gerar em 44100 Hz

## 📄 Relatório Técnico

Relatório completo salvo em: `sovits_quality_tests/test_report.json`

Contém:
- Verificação de sample rate
- Métricas de cada teste
- Configurações exatas usadas
- Informações do modelo

