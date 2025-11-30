# Diagnóstico: Áudio Ininteligível

## Problema
O áudio está sendo gerado e é reproduzível, mas é **ininteligível** - é uma coleção monótona de fonemas sem sentido, onde as palavras não são formadas.

## O que já foi corrigido
1. ✅ Divisão de diphthongs (eɪ → e + ɪ, aɪ → a + ɪ, oʊ → o + ʊ, tʃ → t + ʃ, dʒ → d + ʒ)
2. ✅ Inserção de pausas entre palavras (ID 0)
3. ✅ Mapeamento de fonemas IPA para phoneme IDs
4. ✅ Formato WAV correto (bits per sample = 16)

## Possíveis causas

### 1. Formato dos inputs do modelo ONNX
- **Problema**: Os inputs podem estar sendo passados em formato incorreto
- **Verificar**: Se o modelo espera inputs nomeados em vez de posicionais
- **Ação**: Verificar os nomes dos inputs do modelo ONNX

### 2. Tokens especiais (BOS/EOS)
- **Problema**: O Piper pode esperar tokens especiais no início/fim da sequência
- **Verificar**: Se há tokens como `_BOS`, `_EOS`, `_PAD` no `phoneme_id_map`
- **Ação**: Adicionar tokens especiais se necessário

### 3. Ordem dos phoneme IDs
- **Problema**: Os phoneme IDs podem estar sendo passados na ordem errada
- **Verificar**: Se a ordem dos fonemas está correta
- **Ação**: Verificar se o Piper Python usa alguma ordenação especial

### 4. Formato dos tensors
- **Problema**: Os tensors podem estar em formato incorreto (shape, dtype)
- **Verificar**: Se os shapes dos inputs estão corretos
- **Ação**: Verificar se `[batch_size, phoneme_count]` está correto

### 5. Mapeamento de fonemas
- **Problema**: Os phoneme IDs podem não corresponder ao que o modelo espera
- **Verificar**: Se o `phoneme_id_map` está correto
- **Ação**: Comparar com o que o Piper Python usa

## Próximos passos

1. **Verificar o código do Piper Python** para ver como ele realmente faz a síntese
2. **Testar com um exemplo mínimo** (ex: "hello") para isolar o problema
3. **Adicionar logs detalhados** para ver exatamente o que está sendo passado para o modelo
4. **Verificar se há tokens especiais** no `phoneme_id_map`
5. **Comparar com um exemplo funcional** do Piper Python

## Logs atuais

Os logs mostram:
- ✅ Phonemização funcionando (263 fonemas IPA gerados)
- ✅ Mapeamento funcionando (231 fonemas conhecidos, 0 desconhecidos)
- ✅ Pausas inseridas (53 pausas)
- ✅ Áudio gerado (163840 samples, 7.43s)
- ❌ Áudio ininteligível (monótono, sem sentido)

Isso sugere que o problema está na forma como estamos passando os phoneme IDs para o modelo ONNX, não no mapeamento ou na phonemização.



