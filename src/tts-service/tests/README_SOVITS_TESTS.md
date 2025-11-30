# Testes SoVITS - Hello World → Dungeon Master

## Visão Geral

Este conjunto de testes valida a conversão de áudio neutro (XTTS) para voz de personagem usando o modelo SoVITS treinado do Dungeon Master.

## Estrutura de Testes

### Teste Python (Standalone)

**Arquivo**: `tests/scripts/test_hello_world_sovits.py`

**Execução**:
```powershell
# Via script PowerShell (recomendado)
.\tests\scripts\test_hello_world_sovits.ps1

# Ou diretamente com venv do SoVITS
cd assets-and-models/models/tts/sovits
.\venv310\Scripts\activate
python ../../../../src/tts-service/tests/scripts/test_hello_world_sovits.py
```

**O que testa**:
- ✅ Encontra áudio "Hello World" gerado pelo XTTS
- ✅ Carrega modelo SoVITS do dungeon master
- ✅ Converte áudio usando GPU (se disponível)
- ✅ Valida speakers disponíveis
- ✅ Mede latência de conversão
- ✅ Salva áudio convertido para verificação

**Resultado esperado**:
- Áudio convertido salvo em `test_hello_world_sovits_output.wav`
- Latência de conversão: ~5-6s (primeira vez), ~1-2s (subsequentes)
- Speaker: `dungeon_master_en`
- Sample rate de saída: 44100 Hz

### Teste Rust (Integration)

**Arquivo**: `tests/integration/hello_world_sovits_test.rs`

**Execução**:
```bash
# Executar teste específico
cargo test --package tts-service --test hello_world_sovits_test -- --ignored

# Ou via cargo test com filtro
cargo test --package tts-service test_hello_world_xtts_to_sovits -- --ignored
```

**O que testa**:
- ✅ Pipeline completo XTTS → SoVITS
- ✅ Validação de resposta (áudio, sample rate, duração)
- ✅ Validação de latência (< 1.5s target)
- ✅ Validação de qualidade básica (amplitude)

## Requisitos

### Modelos Necessários

1. **Modelo SoVITS**: `dungeon_master_en.pth`
   - Localização: `assets-and-models/models/tts/sovits/dungeon_master_en.pth`
   - Config: `assets-and-models/models/tts/sovits/config.json`

2. **Áudio XTTS**: `test_hello_world_xtts_real.wav`
   - Localização: `src/tts-service/tests/scripts/test_hello_world_xtts_real.wav`
   - Gerado pelo XTTS com texto "Hello World"

### Ambiente

- **Python venv**: SoVITS venv310 ativado
- **GPU**: Recomendado (CUDA) para latência < 1.5s
- **Dependências**: PyTorch com CUDA, soundfile, numpy

## Resultados Esperados

### Performance (GPU)

| Métrica | Target | Real |
|---------|--------|------|
| Carregamento modelo | < 10s | ~5s |
| Conversão (primeira) | < 10s | ~5s |
| Conversão (subsequente) | < 2s | ~1-2s |
| **Total pipeline** | < 1.5s | ~6s (primeira), ~2s (subsequente) |

### Qualidade

- ✅ Áudio convertido mantém duração original
- ✅ Sample rate correto (44100 Hz)
- ✅ Amplitude normalizada (sem clipping)
- ✅ Voz do dungeon master aplicada

## Troubleshooting

### Erro: "Modelo não encontrado"

Verifique se o modelo está em:
```
assets-and-models/models/tts/sovits/dungeon_master_en.pth
```

### Erro: "Speaker não encontrado"

Verifique speakers disponíveis no modelo:
```python
print(svc_model.spk2id.keys())
```

Speakers esperados:
- `dungeon_master_en`
- `dungeon_master_en_backup`

### Erro: "Dependências não encontradas"

Ative o venv do SoVITS:
```powershell
cd assets-and-models/models/tts/sovits
.\venv310\Scripts\activate
```

### Latência alta

- Verifique se GPU está sendo usada: `device: cuda`
- Primeira execução sempre será mais lenta (carregamento)
- Subsequentes devem ser mais rápidas

## Próximos Passos

1. ✅ Teste básico funcionando
2. ⏳ Testar com diferentes emoções
3. ⏳ Testar com texto longo
4. ⏳ Integrar no pipeline completo
5. ⏳ Medir latência end-to-end

