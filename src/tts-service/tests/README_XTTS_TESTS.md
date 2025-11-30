# XTTS Test Suite

Este documento descreve os testes para o módulo XTTS e como executá-los.

## 📁 Estrutura de Testes

```
tests/
├── unit/
│   └── xtts_test.rs              # Testes unitários do módulo XTTS
├── integration/
│   ├── xtts_integration_test.rs   # Testes de integração XTTS
│   └── xtts_pipeline_fallback_test.rs  # Testes de fallback no pipeline
└── scripts/
    ├── test_xtts_python.py        # Teste standalone do Coqui XTTS
    └── test_xtts_rust_bridge.py   # Simula bridge Python usado pelo Rust
```

## 🧪 Tipos de Testes

### 1. Testes Unitários (`tests/unit/xtts_test.rs`)

Testam a funcionalidade básica do módulo XTTS:

- ✅ Criação e carregamento de modelo
- ✅ Síntese de áudio básica
- ✅ Diferentes vozes
- ✅ Diferentes velocidades e pitches
- ✅ Sistema de cache
- ✅ Tratamento de erros
- ✅ Validação de estrutura de áudio

**Executar:**
```bash
cargo test --test xtts_test
# ou
cargo test --lib xtts
```

### 2. Testes de Integração (`tests/integration/xtts_integration_test.rs`)

Testam a integração do XTTS com outros componentes:

- ✅ Modelo compartilhado (SharedXttsModel)
- ✅ Síntese concorrente
- ✅ Estrutura de integração com pipeline
- ✅ Propagação de erros
- ✅ Qualidade básica de áudio
- ✅ Streaming de áudio

**Executar:**
```bash
cargo test --test xtts_integration_test
```

### 3. Testes de Fallback (`tests/integration/xtts_pipeline_fallback_test.rs`)

Testam o mecanismo de fallback Piper → XTTS:

- ✅ Estrutura de fallback no pipeline
- ✅ Fallback automático quando Piper falha
- ✅ Uso direto do XTTS
- ✅ Tratamento de erros no pipeline

**Executar:**
```bash
cargo test --test xtts_pipeline_fallback_test
```

**Nota:** Estes testes estão marcados com `#[ignore]` e requerem modelos reais.

### 4. Testes Python (`tests/scripts/`)

Scripts Python para validar a instalação e funcionamento do Coqui XTTS:

#### `test_xtts_python.py`
Testa a instalação e funcionalidade básica do Coqui XTTS:

```bash
# Instalar dependências primeiro
pip install TTS

# Executar teste
python tests/scripts/test_xtts_python.py

# Com teste multilíngue
python tests/scripts/test_xtts_python.py --multilingual
```

**O que testa:**
- ✅ Instalação do Coqui TTS
- ✅ Carregamento do modelo XTTS
- ✅ Síntese básica de áudio
- ✅ Suporte multilíngue (opcional)
- ✅ Formato de saída compatível com Rust

#### `test_xtts_rust_bridge.py`
Simula exatamente o que o Rust fará ao chamar o Python bridge:

```bash
# Criar arquivo JSON de entrada (como Rust faria)
echo '{"text": "Hello", "language": "en", "speaker": null, "use_gpu": false}' > test_input.json

# Executar bridge
python tests/scripts/test_xtts_rust_bridge.py test_input.json
```

## 🚀 Executando Todos os Testes

### Testes Rust (sem dependências externas)
```bash
# Todos os testes unitários
cargo test --lib xtts

# Todos os testes de integração
cargo test --test xtts_integration_test

# Testes ignorados (requerem modelos)
cargo test --test xtts_integration_test -- --ignored
```

### Testes Python (requerem Coqui TTS)
```bash
# Verificar instalação
python -c "import TTS; print('OK')"

# Teste completo
python tests/scripts/test_xtts_python.py
```

## ⚠️ Testes que Requerem Dependências

Alguns testes são marcados com `#[ignore]` porque requerem:

1. **Coqui TTS instalado**: `pip install TTS`
2. **Modelo XTTS baixado**: Baixado automaticamente na primeira execução
3. **GPU opcional**: Testes funcionam com CPU, mas GPU é mais rápido

### Executar Testes Ignorados

```bash
# Executar apenas testes ignorados
cargo test --test xtts_integration_test -- --ignored

# Executar todos (incluindo ignorados)
cargo test --test xtts_integration_test -- --include-ignored
```

## 📊 Cobertura de Testes

### Cobertura Atual

- ✅ **Estrutura básica**: 100%
- ✅ **Cache**: 100%
- ✅ **Tratamento de erros**: 100%
- ✅ **Validação de dados**: 100%
- ⚠️ **Python bridge**: Parcial (requer Coqui TTS)
- ⚠️ **Integração pipeline**: Estrutural apenas
- ⚠️ **Fallback**: Estrutural apenas

### Cobertura Futura (após implementação completa)

- [ ] Python bridge com Coqui XTTS real
- [ ] Integração completa com pipeline
- [ ] Fallback funcional Piper → XTTS
- [ ] Testes de performance
- [ ] Testes de qualidade de áudio
- [ ] Testes multilíngue completos

## 🐛 Troubleshooting

### Erro: "Coqui TTS not available"
```bash
# Instalar Coqui TTS
pip install TTS

# Verificar instalação
python -c "from TTS.api import TTS; print('OK')"
```

### Erro: "Model not found"
O modelo XTTS é baixado automaticamente na primeira execução.
Isso pode levar alguns minutos e requer ~1.5GB de espaço.

### Erro: "GPU not available"
Os testes funcionam com CPU. GPU é opcional mas recomendado para performance.

### Testes falhando com timeout
Alguns testes podem demorar se o modelo precisar ser baixado.
Aumente o timeout:
```bash
cargo test --test xtts_integration_test -- --test-threads=1 --timeout 300
```

## 📝 Adicionando Novos Testes

### Teste Unitário
Adicione em `tests/unit/xtts_test.rs`:
```rust
#[tokio::test]
async fn test_new_feature() {
    // Seu teste aqui
}
```

### Teste de Integração
Adicione em `tests/integration/xtts_integration_test.rs`:
```rust
#[tokio::test]
async fn test_integration_feature() {
    // Seu teste aqui
}
```

### Teste que Requer Dependências
Marque com `#[ignore]`:
```rust
#[tokio::test]
#[ignore] // Requer Coqui TTS
async fn test_requires_coqui() {
    // Seu teste aqui
}
```

## ✅ Checklist de Testes

Antes de considerar a implementação completa:

- [ ] Todos os testes unitários passam
- [ ] Testes de integração básicos passam
- [ ] Teste Python standalone passa
- [ ] Teste de bridge Python passa
- [ ] Testes de fallback (estruturais) passam
- [ ] Testes com modelos reais (quando disponível)
- [ ] Cobertura de código adequada
- [ ] Documentação atualizada


