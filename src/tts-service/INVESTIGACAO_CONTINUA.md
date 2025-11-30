# Investigação Contínua - Áudio Ininteligível

## Data: 2025-11-25

## Correções Já Aplicadas

1. ✅ Ordem dos parâmetros `scales` corrigida
2. ✅ Espaço mapeado para ID 3 (não ID 0)
3. ✅ Tokens BOS/EOS adicionados (^ e $)

## Problemas Identificados em Fóruns

### 1. Phonemização Incorreta
- O `espeak-ng` pode estar gerando phonemes IPA incorretos
- Verificar se os phonemes gerados estão corretos para "Hello World"
- Esperado: algo como `həloʊ wɜld`

### 2. Formato de Saída
- Converter para WAV pode introduzir problemas
- Alguns usuários sugerem usar saída raw

### 3. Normalização de Áudio
- A normalização pode estar distorcendo o áudio
- Verificar se há clipping ou distorção

## Próximos Passos de Investigação

1. ⏳ Adicionar logs detalhados dos phonemes IPA gerados
2. ⏳ Comparar phonemes gerados com o esperado
3. ⏳ Verificar se há problema com normalização
4. ⏳ Testar sem normalização
5. ⏳ Verificar se há problema com o formato do tensor de entrada

## Diagnósticos Adicionados

- Log completo da string IPA gerada
- Comparação com IPA esperado para "Hello World"
- Verificação de normalização de áudio


