# Testes para Task M1.2: Adicionar Suporte a Qwen-1.5B no LLM Core

## Visão Geral

Este documento descreve os testes criados para a task **M1.2: add-qwen-1-5b-support**, seguindo os padrões do rulebook e as especificações da documentação.

**Arquivo de Testes**: `src/llm-core/src/inference_1_5b_tests.rs`

## Testes Implementados

### 1. Teste de Carregamento Simultâneo de Ambos Modelos
**Teste**: `test_load_both_models_simultaneously`

**Descrição**: Verifica que ambos os modelos (1.5B e 14B) podem ser carregados simultaneamente sem interferência.

**Critérios de Sucesso**:
- ✅ 14B model carrega corretamente
- ✅ 1.5B model carrega corretamente
- ✅ Ambos permanecem carregados após carregamento do outro

**Status**: ✅ Implementado

---

### 2. Teste de Latência de Inferência 1.5B
**Teste**: `test_1_5b_inference_latency`

**Descrição**: Verifica que a inferência do 1.5B completa em < 1.2s total.

**Critérios de Sucesso**:
- ✅ Inferência completa dentro do timeout de 1200ms
- ✅ Latência medida e registrada
- ✅ Resposta é válida (não vazia)

**Feature**: `s2s` (requer servidor Synap ativo)

**Status**: ✅ Implementado

---

### 3. Teste de Formato de Resposta Emocional
**Teste**: `test_1_5b_emotional_response_format`

**Descrição**: Verifica que o 1.5B gera resposta emocional (1-2 frases, max 40 tokens).

**Critérios de Sucesso**:
- ✅ Tokens estimados ≤ 40
- ✅ Palavras ≤ 45 (1-2 frases)
- ✅ Resposta é prelúdio/emocional (não resolução final)
- ✅ Resposta não está vazia

**Feature**: `s2s` (requer servidor Synap ativo)

**Status**: ✅ Implementado

---

### 4. Teste de Não Geração de Resultados Finais
**Teste**: `test_1_5b_no_final_results`

**Descrição**: Verifica que o 1.5B NÃO gera resultados finais ou consequências.

**Critérios de Sucesso**:
- ✅ Resposta não contém palavras-chave de resolução final
- ✅ Testado com múltiplos prompts que poderiam trigger resolução
- ✅ Todas as respostas são prelúdios emocionais

**Feature**: `s2s` (requer servidor Synap ativo)

**Status**: ✅ Implementado

---

### 5. Teste de Uso de Memória com Ambos Modelos
**Teste**: `test_memory_usage_both_models`

**Descrição**: Verifica uso de memória quando ambos modelos estão carregados.

**Critérios de Sucesso**:
- ✅ Ambos modelos carregam corretamente
- ✅ Ambos podem ser usados independentemente
- ✅ Ambos permanecem carregados após uso

**Status**: ✅ Implementado

---

### 6. Teste de Configuração do Endpoint Prelude
**Teste**: `test_prelude_endpoint_configuration`

**Descrição**: Verifica que os parâmetros do endpoint `/llm/prelude` estão configurados corretamente.

**Critérios de Sucesso**:
- ✅ Max tokens = 40
- ✅ Temperature = 0.8
- ✅ Top-p = 0.9
- ✅ Timeout = 1200ms

**Status**: ✅ Implementado

---

### 7. Teste de Logging e Métricas
**Teste**: `test_1_5b_logging_and_metrics`

**Descrição**: Verifica que logging e métricas são registrados corretamente.

**Critérios de Sucesso**:
- ✅ Latência é mensurável
- ✅ Tokens usados são rastreados
- ✅ Persona está correta na resposta

**Feature**: `s2s` (requer servidor Synap ativo)

**Status**: ✅ Implementado

---

### 8. Teste de Estilo de Resposta
**Teste**: `test_1_5b_response_style`

**Descrição**: Verifica que a resposta é emocional, human-like, não formulaica.

**Critérios de Sucesso**:
- ✅ Resposta não está vazia
- ✅ Resposta não é formulaica (soft check)

**Feature**: `s2s` (requer servidor Synap ativo)

**Status**: ✅ Implementado

---

### 9. Teste de Múltiplas Requisições em Sequência
**Teste**: `test_1_5b_multiple_requests_sequence`

**Descrição**: Verifica que múltiplas requisições 1.5B em sequência funcionam corretamente.

**Critérios de Sucesso**:
- ✅ Cada resposta é válida
- ✅ Cada resposta completa em < 1.2s
- ✅ Cada resposta é prelúdio emocional

**Feature**: `s2s` (requer servidor Synap ativo)

**Status**: ✅ Implementado

---

### 10. Teste com Diferentes Personas
**Teste**: `test_1_5b_different_personas`

**Descrição**: Verifica que o 1.5B funciona com diferentes personas.

**Critérios de Sucesso**:
- ✅ Resposta tem persona correta para cada tipo
- ✅ Testado com DungeonMaster, NPC, Narrator

**Feature**: `s2s` (requer servidor Synap ativo)

**Status**: ✅ Implementado

---

### 11. Teste de Tratamento de Erros
**Teste**: `test_1_5b_error_handling`

**Descrição**: Verifica tratamento de erros (modelo não carregado, etc.).

**Critérios de Sucesso**:
- ✅ Erro retornado quando modelo não está carregado
- ✅ Sucesso quando modelo está carregado

**Status**: ✅ Implementado

---

## Execução dos Testes

### Testes Rápidos (sem servidor Synap)
```bash
cd vrpg-client/src/llm-core
cargo test --lib inference_1_5b_tests
```

### Testes S2S (requer servidor Synap ativo)
```bash
cd vrpg-client/src/llm-core
cargo test --lib inference_1_5b_tests --features s2s
```

### Todos os Testes
```bash
cd vrpg-client/src/llm-core
cargo test --lib inference_1_5b_tests --features s2s
```

## Cobertura de Testes

**Meta**: ≥ 95% (conforme rulebook)

**Testes Implementados**: 11 testes
- **Testes rápidos**: 3 (não requerem servidor)
- **Testes S2S**: 8 (requerem servidor Synap ativo)

## Critérios de Sucesso da Task M1.2

Conforme [TASKS_PIPELINE_MIGRATION.md](../TASKS_PIPELINE_MIGRATION.md):

- ✅ Teste de carregamento de ambos modelos simultaneamente
- ✅ Teste de inferência 1.5B < 1.2s total
- ✅ Teste de geração de resposta emocional (1-2 frases, max 40 tokens)
- ✅ Teste de que 1.5B não gera resultados finais ou consequências
- ✅ Teste de uso de memória com ambos modelos carregados
- ✅ Teste de cobertura (95%+) - **Pendente**: Requer implementação completa

## Próximos Passos

1. **Implementar funcionalidade**: Adicionar suporte real ao Qwen-1.5B no `inference.rs`
2. **Executar testes S2S**: Quando servidor Synap estiver disponível
3. **Verificar cobertura**: Executar `cargo llvm-cov --all` após implementação
4. **Atualizar documentação**: Atualizar STATUS.md quando task for concluída

## Referências

- [TASKS_PIPELINE_MIGRATION.md](../TASKS_PIPELINE_MIGRATION.md) - Task M1.2
- [QWEN_1_5B_SPEC.md](../QWEN_1_5B_SPEC.md) - Especificação do Qwen-1.5B
- [TESTS_MASTER.md](../TESTS_MASTER.md) - Plano completo de testes
- [RUST.md](../../rulebook/RUST.md) - Padrões de testes Rust






















