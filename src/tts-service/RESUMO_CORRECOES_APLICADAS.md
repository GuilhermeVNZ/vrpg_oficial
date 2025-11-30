# Resumo das Correções Aplicadas

**Data:** 2025-11-25

## Problema Reportado
O áudio gerado pelo Piper está "MUITO rápido" e parece que "todos os fonemas foram colocados um em cima do outro no mesmo timestamp", tornando o áudio ininteligível.

## Correções Aplicadas

### 1. Correção do Cálculo da Duração
- **Problema:** O código estava usando `22050.0` fixo para calcular a duração do áudio, em vez do `sample_rate` real do modelo.
- **Correção:** Atualizado para usar `inner.sample_rate` do modelo carregado.
- **Arquivo:** `vrpg-client/src/tts-service/src/piper.rs` (linha 784)

### 2. Logs de Diagnóstico para Mapeamento de Fonemas
- **Problema:** Não havia alertas claros quando muitos fonemas estavam sendo pulados.
- **Correção:** Adicionado log crítico que alerta quando >30% dos fonemas são pulados, e aviso quando >10% são pulados.
- **Arquivo:** `vrpg-client/src/tts-service/src/piper.rs` (após linha 505)

### 3. Scripts de Teste e Diagnóstico
- **Criado:** `test_audio_speed_diagnosis.ps1` - Testa com textos curtos e médios, compara duração esperada vs obtida.
- **Criado:** `reiniciar_e_testar.ps1` - Reinicia o servidor e executa testes automaticamente.
- **Criado:** `DIAGNOSTICO_VELOCIDADE.md` - Documentação do problema e próximos passos.

## Próximos Passos

1. **Verificar os logs do servidor** para ver:
   - Quantos fonemas IPA foram gerados
   - Quantos foram mapeados vs pulados
   - A duração real do áudio gerado
   - Se há avisos sobre muitos fonemas sendo pulados

2. **Executar o teste de diagnóstico:**
   ```powershell
   .\test_audio_speed_diagnosis.ps1
   ```

3. **Se muitos fonemas estão sendo pulados:**
   - Verificar o `phoneme_id_map` do modelo
   - Adicionar mais fallbacks para fonemas desconhecidos
   - Verificar se a phonemização está gerando fonemas corretos

4. **Se a duração está correta mas o áudio está ininteligível:**
   - Verificar a ordem dos parâmetros `scales`
   - Testar diferentes valores de `length_scale` (0.5, 1.0, 1.5, 2.0)
   - Verificar se há algum problema com o processamento do áudio após a geração

## Logs Importantes a Verificar

- `🔍 PIPER DIAGNOSTIC - PHONEMES (IPA): X total`
- `Phoneme mapping: X known, Y unknown (skipped)`
- `⚠️ CRITICAL: X% of phonemes were skipped!`
- `Piper generated X samples, duration: Ys (from Z phoneme IDs, sample_rate: W Hz)`

## Status
- ✅ Correções aplicadas e compiladas
- ⏳ Aguardando teste do servidor e análise dos logs



