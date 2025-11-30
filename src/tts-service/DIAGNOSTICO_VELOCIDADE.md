# Diagnóstico: Áudio Muito Rápido e Ininteligível

**Data:** 2025-11-25

**Problema Reportado:**
O áudio gerado pelo Piper está "MUITO rápido" e parece que "todos os fonemas foram colocados um em cima do outro no mesmo timestamp", tornando o áudio ininteligível.

## Possíveis Causas

### 1. Muitos Fonemas Sendo Pulados
- Se muitos fonemas IPA não estão sendo mapeados para IDs do Piper, o áudio resultante será muito curto e ininteligível.
- **Verificação:** Verificar os logs do servidor para ver a proporção de fonemas mapeados vs pulados.
- **Solução:** Melhorar o mapeamento de fonemas ou adicionar fallbacks mais robustos.

### 2. Ordem Incorreta dos Parâmetros `scales`
- O modelo ONNX pode esperar os parâmetros `scales` em uma ordem diferente.
- Atualmente estamos usando: `[length_scale, noise_scale, noise_w] = [1.0, 1.0, 1.0]`
- **Verificação:** Testar diferentes ordens e valores de `scales`.
- **Solução:** Verificar a documentação do modelo ou testar empiricamente.

### 3. Formato Incorreto dos Tensores de Entrada
- Os tensores podem estar sendo criados com shapes ou tipos incorretos.
- **Verificação:** Verificar os logs do servidor para ver os shapes e tipos dos tensores.
- **Solução:** Garantir que os tensores correspondem exatamente ao que o modelo espera.

### 4. Sample Rate Incorreto
- O cálculo da duração estava usando `22050.0` fixo em vez do `sample_rate` real do modelo.
- **Correção Aplicada:** Atualizado para usar `inner.sample_rate`.
- **Verificação:** Verificar se o sample_rate está correto nos logs.

## Próximos Passos

1. **Executar o teste de diagnóstico:**
   ```powershell
   .\test_audio_speed_diagnosis.ps1
   ```

2. **Verificar os logs do servidor para:**
   - Quantos fonemas IPA foram gerados
   - Quantos foram mapeados vs pulados
   - A duração real do áudio gerado
   - O sample_rate usado

3. **Comparar a duração esperada vs obtida:**
   - Texto curto ("Hello world"): Esperado ~300-600ms
   - Texto médio (50 caracteres): Esperado ~1.5-3s

4. **Se muitos fonemas estão sendo pulados:**
   - Verificar o `phoneme_id_map` do modelo
   - Adicionar mais fallbacks para fonemas desconhecidos
   - Verificar se a phonemização está gerando fonemas corretos

5. **Se a duração está correta mas o áudio está ininteligível:**
   - Verificar a ordem dos parâmetros `scales`
   - Testar diferentes valores de `length_scale` (0.5, 1.0, 1.5, 2.0)
   - Verificar se há algum problema com o processamento do áudio após a geração

## Logs Importantes a Verificar

- `🔍 PIPER DIAGNOSTIC - PHONEMES (IPA): X total`
- `Phoneme mapping: X known, Y unknown (skipped)`
- `⚠️ CRITICAL: X% of phonemes were skipped!`
- `Piper generated X samples, duration: Ys (from Z phoneme IDs, sample_rate: W Hz)`

## Correções Aplicadas

1. ✅ Corrigido cálculo da duração para usar `inner.sample_rate` em vez de `22050.0` fixo
2. ✅ Adicionado log crítico para alertar quando >30% dos fonemas são pulados
3. ✅ Criado script de teste para diagnosticar velocidade e duração do áudio



