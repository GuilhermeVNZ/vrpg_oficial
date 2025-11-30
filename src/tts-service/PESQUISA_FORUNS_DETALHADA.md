# Pesquisa Detalhada em Fóruns - Áudio Ininteligível

## Data: 2025-11-25

## Problemas Encontrados em Fóruns

### 1. Problema com Phonemização
- Alguns usuários relataram problemas com phonemização incorreta
- O `piper-phonemize` pode estar gerando phonemes IPA incorretos
- Verificar se o `espeak-ng` está configurado corretamente

### 2. Problema com Formato de Saída
- Alguns usuários relataram que converter para WAV pode introduzir problemas
- Sugestão: usar saída raw e reproduzir diretamente com `aplay`

### 3. Problema com Integridade do Modelo
- Modelos corrompidos podem gerar áudio ininteligível
- Verificar se os arquivos `.onnx` e `.onnx.json` estão completos

### 4. Problema com Versão do Piper
- Versões desatualizadas podem ter bugs conhecidos
- Atualizar para a versão mais recente

### 5. Problema com Configuração de Tokens
- Tokens BOS/EOS podem estar sendo adicionados incorretamente
- Verificar se a ordem está correta

## Possíveis Causas Adicionais

### 1. Phonemes IPA Incorretos
- O espeak-ng pode estar gerando phonemes IPA incorretos para inglês
- Verificar se os phonemes gerados estão corretos

### 2. Ordem dos Phonemes
- A ordem dos phonemes pode estar incorreta
- Verificar se há necessidade de reordenar

### 3. Normalização de Áudio
- A normalização pode estar distorcendo o áudio
- Verificar se a normalização está correta

### 4. Formato do Tensor
- O formato do tensor de entrada pode estar incorreto
- Verificar se o shape está correto

## Próximos Passos

1. ✅ Verificar phonemização - adicionar logs detalhados
2. ⏳ Comparar phonemes gerados com implementação Python oficial
3. ⏳ Verificar se há necessidade de remover silêncio no início/fim
4. ⏳ Testar com modelo diferente
5. ⏳ Verificar se há problema com normalização


