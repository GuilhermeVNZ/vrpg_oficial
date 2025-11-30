# Problema Identificado - Modelo GB vs US

## Data: 2025-11-25

## Problema Encontrado

O arquivo `piper-en-us.onnx` **NÃO é um modelo americano**, mas sim **inglês britânico (GB)**!

### Detalhes do Modelo Real:
- **Nome do arquivo**: `piper-en-us.onnx` (sugere US)
- **Modelo real**: `en_GB-northern_english_male-medium`
- **Voice**: `en-gb-x-rp` (inglês britânico, Received Pronunciation)
- **Dataset**: `northern_english_male` (inglês do norte da Inglaterra)
- **Language region**: `GB` (Great Britain, não US!)

## Impacto

Isso pode causar problemas porque:
1. O código pode estar usando configurações de phonemização para US quando deveria ser GB
2. Phonemes IPA podem ser diferentes entre GB e US
3. O modelo espera phonemes em formato GB, mas estamos gerando em formato US

## Exemplo de Diferença

- **US**: "Hello" -> `həˈloʊ` (com ə e oʊ)
- **GB**: "Hello" -> pode ser diferente, especialmente o "o" final

## Solução

1. **Opção 1**: Usar um modelo verdadeiramente americano (en-US)
2. **Opção 2**: Ajustar a phonemização para usar `en-gb` em vez de `en-us`
3. **Opção 3**: Renomear o arquivo para `piper-en-gb.onnx` e ajustar o código

## Próximos Passos

1. Verificar se a phonemização está usando `en-us` quando deveria ser `en-gb`
2. Testar com phonemização `en-gb` para ver se melhora
3. Considerar baixar um modelo verdadeiramente americano se necessário


