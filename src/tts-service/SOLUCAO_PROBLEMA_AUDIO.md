# Solução do Problema de Áudio Ininteligível

## Data: 2025-11-25

## Problema Identificado

O áudio gerado estava muito curto e ininteligível:
- **Duração inicial**: 301ms para "Hello World"
- **Duração esperada**: 500-800ms
- **Sintoma**: Áudio parecia ter "todos os fonemas colocados um em cima do outro no mesmo timestamp"

## Causa Raiz

**Ordem incorreta dos parâmetros `scales` no tensor ONNX**

### O que estava errado:
- Estávamos passando: `[length_scale, noise_scale, noise_w]`
- O modelo esperava: `[noise_scale, length_scale, noise_w]`

### Como descobrimos:
1. Pesquisamos fóruns e encontramos relatos similares
2. Verificamos o arquivo JSON do modelo (`piper-en-us.onnx.json`)
3. Encontramos a seção `inference` com a ordem correta:
   ```json
   "inference": {
     "noise_scale": 0.667,
     "length_scale": 1,
     "noise_w": 0.8
   }
   ```

## Solução Aplicada

### Mudanças no código (`piper.rs`):

**Antes:**
```rust
let length_scale = 10.0f32;
let noise_scale = 1.0f32;
let noise_w = 1.0f32;
let scales = vec![length_scale, noise_scale, noise_w]; // ORDEM ERRADA
```

**Depois:**
```rust
let noise_scale = 0.667f32; // Default from model JSON
let length_scale = 2.0f32; // Increase to slow down speech
let noise_w = 0.8f32; // Default from model JSON
let scales = vec![noise_scale, length_scale, noise_w]; // ORDEM CORRETA
```

## Resultados

### Antes da correção:
- Duração: 301ms
- Amostras: 6656
- Áudio: Ininteligível

### Depois da correção:
- Duração: 557ms ✅
- Amostras: 12288
- Áudio: Inteligível ✅

## Lições Aprendidas

1. **Sempre verificar arquivos de configuração do modelo**
   - O arquivo JSON do modelo contém informações importantes sobre parâmetros padrão e ordem

2. **Ordem dos parâmetros importa em ONNX**
   - A ordem dos parâmetros no tensor deve corresponder exatamente ao que o modelo espera

3. **Pesquisar fóruns pode ajudar**
   - Outros usuários relataram problemas similares, o que nos direcionou para a solução

4. **Valores padrão do modelo são importantes**
   - Usar os valores padrão do modelo (`noise_scale: 0.667`, `noise_w: 0.8`) em vez de `1.0` pode melhorar a qualidade

## Próximos Passos

1. ✅ Testar com diferentes textos para validar a correção
2. ✅ Verificar se o modelo PT-BR também precisa da mesma correção
3. ⚠️ Considerar tornar `length_scale` configurável via API se necessário
4. ⚠️ Documentar a ordem correta dos parâmetros para futuras referências

## Status

✅ **PROBLEMA RESOLVIDO**

O áudio agora tem duração adequada e é inteligível.


