# Refatoração do Pipeline de Streaming TTS

**Data**: 2025-11-29  
**Status**: ✅ Estrutura implementada, otimizações pendentes

---

## ✅ Implementado

### 1. Estrutura de Perfis TTS
- ✅ Módulo `tts_profile.rs` criado
- ✅ Perfil FAST (para Qwen 1.5B):
  - Primeiro chunk: 30 chars
  - Próximos chunks: 90 chars
  - Sample rate: 16 kHz
  - Audio block: 50 ms
  - Pre-buffer: 240 ms
- ✅ Perfil CINEMATIC (para Qwen 14B):
  - Primeiro chunk: 100 chars
  - Próximos chunks: 150 chars
  - Sample rate: 24 kHz
  - Audio block: 60 ms
  - Pre-buffer: 500 ms
- ✅ Função `from_llm_model()` para auto-seleção de perfil

### 2. Chunker Configurável
- ✅ Método `chunk_with_profile()` implementado
- ✅ Respeita limites de primeiro chunk vs próximos chunks
- ✅ Chunking por palavras (não por sentenças) para FAST

### 3. Streaming com FIFO
- ✅ Teste Python implementado com FIFO real
- ✅ Producer/Consumer threads separadas
- ✅ Pre-buffer configurável por perfil

### 4. Teste Atualizado
- ✅ Teste `test_mestre_20s_streaming_v2.py` criado
- ✅ Mede `time_to_first_audio` e `time_to_playback`
- ✅ Testa ambos os perfis (FAST e CINEMATIC)

---

## 📊 Resultados Atuais

### Perfil FAST
- **Primeiro chunk**: 29 chars ✅
- **Time to First Audio**: 1.390s ⚠️ (target: ≤ 0.8s)
- **Time to Playback**: 1.390s
- **Status**: Melhorou de 10s para 1.4s, mas ainda acima do target

### Perfil CINEMATIC
- **Primeiro chunk**: 92 chars ✅
- **Time to First Audio**: 9.165s ⚠️ (target: 1.5-3s)
- **Time to Playback**: 9.168s
- **Status**: Ainda alto, mas primeiro chunk é maior (esperado)

---

## ⚠️ Problemas Identificados

### 1. XTTS Gera Chunks Maiores que o Texto
- O XTTS está gerando ~2.3s de áudio para 29 chars (deveria ser ~0.7-1.0s)
- Isso indica que o XTTS pode estar processando o texto de forma diferente
- **Solução**: Verificar se o XTTS está respeitando o texto exato ou fazendo padding

### 2. Falta Otimização FP16
- O código Rust ainda não implementa FP16
- **Solução**: Adicionar suporte a FP16 no XTTS wrapper

### 3. Sample Rate não está sendo aplicado
- O teste Python resample após geração, mas deveria gerar direto no sample rate do perfil
- **Solução**: Passar sample_rate para o XTTS na síntese

### 4. Warm-up não está sendo usado
- O warm-up é executado, mas não está sendo usado no código Rust
- **Solução**: Implementar warm-up na inicialização do serviço

---

## 🔧 Próximos Passos

### 1. Otimizar XTTS no Rust
- [ ] Adicionar suporte a FP16 (`model.half().to("cuda")`)
- [ ] Passar sample_rate para síntese
- [ ] Implementar warm-up na inicialização

### 2. Melhorar Chunker
- [ ] Garantir que primeiro chunk seja realmente pequeno (pode cortar no meio de palavra para FAST)
- [ ] Adicionar validação de duração estimada

### 3. Otimizar Streaming
- [ ] Implementar streaming real no Rust (não apenas no teste Python)
- [ ] Adicionar suporte a blocos menores (50ms para FAST)
- [ ] Melhorar gerenciamento de pre-buffer

### 4. Integração com Orquestrador
- [ ] Passar `llm_model_name` no `StreamingRequest`
- [ ] Auto-selecionar perfil baseado no modelo LLM

---

## 📝 Notas Técnicas

### Por que FAST ainda está em 1.4s?
1. **Warm-up**: Primeira inferência sempre é mais lenta
2. **XTTS overhead**: Carregamento de modelo, processamento de texto
3. **Chunk ainda grande**: 29 chars gerando 2.3s de áudio (deveria ser ~1s)

### Como reduzir para < 0.8s?
1. **Chunk ainda menor**: 15-20 chars para primeiro chunk
2. **FP16**: Reduzir tempo de inferência em ~30-40%
3. **Sample rate 16kHz**: Reduzir custo computacional
4. **Warm-up prévio**: Garantir que modelo está "quente"
5. **Streaming mais fino**: Blocos de 25ms em vez de 50ms

---

## 🎯 Targets Finais

### FAST (Qwen 1.5B)
- ✅ Primeiro chunk: 30 chars (atual: 29 chars)
- ⚠️ Time to First Audio: ≤ 0.8s (atual: 1.39s)
- ⚠️ Time to Playback: ≤ 1.0s (atual: 1.39s)

### CINEMATIC (Qwen 14B)
- ✅ Primeiro chunk: 100 chars (atual: 92 chars)
- ⚠️ Time to First Audio: 1.5-3s (atual: 9.17s)
- ⚠️ Time to Playback: 2.0-3.5s (atual: 9.17s)

---

**Status**: Estrutura implementada, otimizações em andamento.



