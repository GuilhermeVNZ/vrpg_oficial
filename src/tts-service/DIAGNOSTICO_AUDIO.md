# Diagnóstico: Áudio Ininteligível

## Problema Reportado
O áudio gerado pelo Piper TTS é reproduzível, mas permanece ininteligível. As palavras não são formadas corretamente.

## Análise dos Logs

### Phonemização
- **Texto de entrada**: "Hello world"
- **IPA gerado**: `["h", "ə", "l", "oʊ", " ", "w", "ɜ", "l", "d"]`
- **Status**: ✅ Phonemização parece correta

### Mapeamento para IDs
- **Phoneme IDs gerados**: `[20, 59, 24, 27, 100, 0, 35, 62, 24, 17]`
- **Mapeamento**:
  - `h` → 20 ✅
  - `ə` → 59 ✅
  - `l` → 24 ✅
  - `oʊ` → dividido em `o` (27) + `ʊ` (100) ✅
  - ` ` → 0 (pausa) ✅
  - `w` → 35 ✅
  - `ɜ` → 62 ⚠️ (pode estar incorreto - "world" geralmente usa "ɜr" ou "ɚ")
  - `l` → 24 ✅
  - `d` → 17 ✅

### Problemas Identificados

1. **Fonema "ɜ" para "world"**:
   - O espeak-ng está gerando "ɜ" para "world"
   - Mas "world" geralmente usa "ɜr" (r-colored) ou "ɚ"
   - O mapeamento para ID 62 pode estar incorreto

2. **Divisão de Diphthongs**:
   - "oʊ" está sendo dividido corretamente em "o" + "ʊ"
   - Mas pode haver um problema com a ordem ou timing

3. **Possível problema com tokens BOS/EOS**:
   - O modelo ONNX pode esperar tokens de início/fim de sequência
   - Não estamos adicionando esses tokens

4. **Formato dos inputs**:
   - Shape: `[1, 10]` para 10 phoneme IDs
   - Pode estar correto, mas precisa verificar

## Próximos Passos

1. ✅ Verificar se o modelo espera tokens BOS/EOS
2. ✅ Testar com fonemas diferentes para "world"
3. ✅ Comparar com implementação Python do Piper
4. ✅ Verificar se há problema com a ordem dos fonemas
5. ✅ Testar com texto mais simples ("hello" apenas)



