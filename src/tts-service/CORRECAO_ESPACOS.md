# Correção Crítica - Mapeamento de Espaços

## Data: 2025-11-25

## Problema Encontrado

**Estávamos usando ID errado para espaços/pausas!**

### O que estava errado:
- Mapeávamos espaços `" "` para ID 0
- Mas no JSON do modelo: `" "` -> ID 3, `"_"` -> ID 0

### Impacto:
- Espaços sendo interpretados como `"_"` (underscore) em vez de espaço
- Isso causava áudio ininteligível porque os fonemas não tinham pausas corretas entre palavras

## Correção Aplicada

```rust
// ANTES (ERRADO):
if ipa_phoneme.trim().is_empty() {
    phoneme_ids.push(0);  // ❌ ID 0 é "_", não espaço!
}

// DEPOIS (CORRETO):
if ipa_phoneme.trim().is_empty() || ipa_phoneme == " " {
    if let Some(&space_id) = inner.phoneme_id_map.get(" ") {
        phoneme_ids.push(space_id);  // ✅ ID 3 (espaço correto)
    } else {
        phoneme_ids.push(3);  // Fallback
    }
}
```

## Mapeamento Correto do Modelo

Do arquivo `piper-en-us.onnx.json`:
- `"_"` -> ID 0 (underscore, não espaço!)
- `" "` -> ID 3 (espaço/pausa)

## Status

✅ **CORREÇÃO APLICADA** - Testando agora para verificar se resolve o problema de áudio ininteligível.


