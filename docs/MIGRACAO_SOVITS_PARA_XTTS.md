# Migração: SoVITS → XTTS com Embeddings

**Data**: 2025-11-28  
**Status**: ✅ Migração concluída

## 📋 Resumo da Mudança

O pipeline de TTS foi simplificado e melhorado, removendo SoVITS e usando apenas **XTTS (Coqui) com embeddings personalizados**.

### Antes (Pipeline Antigo):
```
Qwen → XTTS (neutro) → SoVITS (conversão) → Áudio Final
```

### Agora (Pipeline Novo):
```
Qwen → XTTS (com embedding do personagem) → Áudio Final (RAW)
```

## ✅ Vantagens da Nova Abordagem

1. **Qualidade Superior**
   - Áudio RAW do XTTS é infinitamente melhor que qualquer processamento
   - Sem artefatos de múltiplas camadas de processamento
   - Voz natural e consistente

2. **Latência Menor**
   - Síntese direta sem camadas adicionais
   - 350-800ms (prelúdio) vs 600-1200ms anterior
   - 1.8-4.7s (narrativa) vs 2.5-6s anterior

3. **Mais Simples**
   - Apenas um arquivo WAV por personagem (vs modelo treinado complexo)
   - Fácil criar novos embeddings
   - Sem necessidade de treinamento

4. **Escalável**
   - Fácil adicionar novos personagens
   - Embeddings compartilháveis entre NPCs similares
   - Gerenciamento simples

## 📁 Estrutura de Arquivos

### Antes (SoVITS):
```
sovits/
├── narrator_default.pth  (modelo treinado, ~50-200MB)
├── npc_guard.pth
└── npc_barkeep.pth
```

### Agora (XTTS Embeddings):
```
xtts_embeddings/
├── narrator_default_xtts_reference_clean.wav  (embedding, ~5-50MB)
├── npc_guard_xtts_reference_clean.wav
└── npc_barkeep_xtts_reference_clean.wav
```

## 🔧 Como Criar Embeddings

1. **Coletar áudio** (5-10 minutos do personagem)
2. **Processar** com `create_clean_xtts_embedding.py`
3. **Salvar** em `xtts_embeddings/{character_id}_xtts_reference_clean.wav`

**Guia completo**: `assets-and-models/models/tts/COMO_CRIAR_EMBEDDINGS_XTTS.md`

## 📝 Arquivos Atualizados

### Documentação:
- ✅ `docs/AUDIO_PIPELINE.md` - Pipeline atualizado (removido SoVITS)
- ✅ `assets-and-models/models/tts/README.md` - Novo pipeline XTTS
- ✅ `assets-and-models/models/tts/COMO_CRIAR_EMBEDDINGS_XTTS.md` - Novo guia
- ✅ `src/tts-service/src/pipeline.rs` - Comentários atualizados

### Scripts:
- ✅ `scripts/download-tts-models.ps1` - Atualizado para XTTS embeddings

### Removidos:
- ❌ `assets-and-models/models/tts/COMO_BAIXAR_SOVITS.md` - Não mais necessário

## 🎯 Descoberta Importante

**Áudio RAW (sem processamento) é infinitamente melhor!**

Após extensos testes, descobrimos que:
- Processamento adicional (filtros, normalização, fade, etc.) **degradam a qualidade**
- O XTTS já gera áudio perfeito
- **Solução**: Usar áudio RAW direto do XTTS

**Documentação**: `src/tts-service/tests/scripts/DESCOBERTA_RAW.md`

## 🚀 Próximos Passos

1. ✅ Usar XTTS com embeddings como padrão
2. ✅ Criar embeddings para personagens principais
3. ✅ Integrar na pipeline de produção
4. ✅ Testar com diferentes textos e vozes

## 📚 Recursos

- **Guia de Criação**: `assets-and-models/models/tts/COMO_CRIAR_EMBEDDINGS_XTTS.md`
- **Script de Criação**: `src/tts-service/tests/scripts/create_clean_xtts_embedding.py`
- **Script de Teste**: `src/tts-service/tests/scripts/test_xtts_book_paragraph.py`
- **Descoberta RAW**: `src/tts-service/tests/scripts/DESCOBERTA_RAW.md`
- **Documentação Pipeline**: `docs/AUDIO_PIPELINE.md`

---

**Última atualização**: 2025-11-28



