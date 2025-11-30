# Correções Aplicadas para Áudio Ininteligível

## Correções Implementadas

### 1. Script Python de Phonemização ✅
**Problema**: O script estava dividindo cada caractere individualmente, quebrando digraphs como "oʊ", "aɪ", "tʃ", etc.

**Solução**: Corrigido para preservar digraphs conhecidos antes de dividir em caracteres individuais.

**Arquivo**: `scripts/phonemize_for_piper.py`

### 2. Verificação de Tokens BOS/EOS ✅
**Problema**: O modelo ONNX pode esperar tokens de início/fim de sequência.

**Solução**: Adicionada verificação no `phoneme_id_map` para tokens BOS/EOS. Se encontrados, são adicionados automaticamente.

**Arquivo**: `src/piper.rs` (função `run_inference`)

## Próximos Passos para Teste

1. **Reiniciar o servidor TTS** com as novas correções
2. **Testar com texto simples**: "Hello world"
3. **Verificar logs** para:
   - Se BOS/EOS tokens foram encontrados e adicionados
   - Se os fonemas estão sendo divididos corretamente
   - Se o mapeamento está correto

## Possíveis Problemas Restantes

1. **Mapeamento de fonemas**: O fonema "ɜ" para "world" pode estar incorreto
2. **Ordem dos fonemas**: Pode haver problema com a ordem em que os fonemas são passados
3. **Formato dos inputs**: Pode precisar de ajustes no formato dos tensores

## Como Testar

```powershell
# 1. Reiniciar servidor
cd G:\vrpg\vrpg-client\target\release
.\tts-server.exe

# 2. Em outro terminal, testar
cd G:\vrpg\vrpg-client\src\tts-service
.\test_phoneme_mapping.ps1
```

## Logs para Verificar

Procure por:
- `Adding BOS token (ID X) at beginning` - Se aparecer, BOS foi adicionado
- `Adding EOS token (ID X) at end` - Se aparecer, EOS foi adicionado
- `No BOS/EOS tokens found` - Se aparecer, não há tokens especiais
- `First 50 IPA phonemes` - Verificar se digraphs estão preservados
- `Phoneme mapping: X known, Y unknown` - Verificar se mapeamento está correto



