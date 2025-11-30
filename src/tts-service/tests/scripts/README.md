# Scripts de Teste XTTS

Scripts Python para testar a integração do Coqui XTTS.

## 📋 Scripts Disponíveis

### `test_xtts_python.py`
Teste completo do Coqui XTTS - valida instalação, carregamento de modelo e síntese.

**Uso:**
```bash
# Teste básico
python tests/scripts/test_xtts_python.py

# Com teste multilíngue
python tests/scripts/test_xtts_python.py --multilingual
```

**Requisitos:**
- Python 3.8+
- Coqui TTS instalado: `pip install TTS`

### `test_xtts_rust_bridge.py`
Simula exatamente o que o Rust fará ao chamar o Python bridge.

**Uso:**
```bash
# Criar arquivo JSON de entrada
echo '{"text": "Hello", "language": "en", "speaker": null, "use_gpu": false}' > test_input.json

# Executar bridge
python tests/scripts/test_xtts_rust_bridge.py test_input.json
```

**Formato de entrada (JSON):**
```json
{
    "text": "Texto para sintetizar",
    "language": "en",
    "speaker": null,
    "use_gpu": false
}
```

**Formato de saída (JSON):**
```json
{
    "samples": [0.1, 0.2, ...],
    "sample_rate": 22050,
    "channels": 1
}
```

## 🔧 Instalação

```bash
# Instalar Coqui TTS
pip install TTS

# Com suporte GPU (opcional)
pip install TTS torch torchaudio --index-url https://download.pytorch.org/whl/cu118
```

## ✅ Validação

Execute os scripts para validar que tudo está funcionando antes de usar no Rust:

```bash
# Teste completo
python tests/scripts/test_xtts_python.py

# Teste de bridge (simula Rust)
echo '{"text": "Test", "language": "en"}' > test.json
python tests/scripts/test_xtts_rust_bridge.py test.json
```


