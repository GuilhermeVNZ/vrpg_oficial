# Mudanças para Reduzir Velocidade do Áudio

**Data:** 2025-11-25

## Mudanças Implementadas

### 1. Aumento de Pausas entre Palavras (`phonemizer.rs`)
- **Antes:** 1 pausa (silence ID 0) entre palavras
- **Agora:** 2-3 pausas entre palavras
  - 2 pausas padrão entre todas as palavras
  - 1 pausa extra a cada 6 palavras (para criar quebras de frase)
- **Objetivo:** Aumentar a duração percebida do áudio adicionando mais silêncio entre palavras

### 2. Ajuste do `length_scale` (`piper.rs`)
- **Antes:** `length_scale = 0.3` (testando valores baixos)
- **Agora:** `length_scale = 5.0` (valores muito altos)
- **Motivo:** Testes anteriores mostraram que valores > 1.0 tornavam o áudio mais rápido (comportamento inesperado)
- **Hipótese:** Talvez valores muito altos (5.0-10.0) tenham um efeito diferente, ou haja um threshold

### 3. Logs Melhorados (`piper.rs`)
- Adicionados logs para mostrar os nomes dos inputs do modelo ONNX
- Isso ajuda a verificar se estamos passando os parâmetros na ordem correta

## Próximos Testes

1. Testar com `length_scale = 5.0` e pausas aumentadas
2. Se ainda estiver rápido, tentar `length_scale = 10.0`
3. Se ainda não funcionar, investigar se o problema está na ordem dos parâmetros `scales`
4. Considerar pós-processamento do áudio (time-stretching) como alternativa

## Observações

- O `length_scale` não está tendo o efeito esperado (valores maiores tornam o áudio mais rápido, não mais lento)
- Isso sugere que:
  - A ordem dos parâmetros pode estar errada
  - O modelo ONNX pode não estar interpretando o parâmetro corretamente
  - Pode haver um problema na forma como estamos passando os parâmetros



