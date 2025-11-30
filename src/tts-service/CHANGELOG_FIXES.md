# Correções de Pronúncia e Integração SoVITS

## Data: 2025-01-25

### Problemas Resolvidos

1. **Pronúncia horrível e incompreensível**
   - ✅ Corrigido: Agora usa `phonemizer` Python (mesma biblioteca do Piper Python)
   - ✅ Script: `scripts/phonemize_for_piper.py`
   - ✅ Fallback: Se Python falhar, usa espeak-ng direto

2. **Voz não se parece com a treinada**
   - ✅ Corrigido: Agora usa modelo SoVITS Python real
   - ✅ Script: `scripts/sovits_convert.py`
   - ✅ Fallback: Se SoVITS falhar, usa áudio do Piper diretamente

### Mudanças Implementadas

#### 1. Phonemização Melhorada
- **Arquivo**: `src/tts-service/src/piper.rs`
- **Mudança**: `text_to_phonemes()` agora chama script Python `phonemize_for_piper.py`
- **Benefício**: Phonemes corretos, mesma qualidade do Piper Python oficial

#### 2. Integração SoVITS Real
- **Arquivo**: `src/tts-service/src/pipeline.rs`
- **Mudança**: `convert_with_sovits_python()` chama script Python `sovits_convert.py`
- **Benefício**: Voz convertida usando modelo treinado real

#### 3. Scripts Python Criados
- `scripts/phonemize_for_piper.py`: Phonemização usando phonemizer
- `scripts/sovits_convert.py`: Conversão de áudio usando SoVITS

#### 4. Detecção Automática de Ambiente
- Detecta automaticamente o ambiente virtual do SoVITS (`venv310`)
- Usa Python do venv quando disponível (tem phonemizer instalado)
- Fallback para Python do sistema se venv não encontrado

### Dependências

1. **phonemizer** (instalado no venv310 do SoVITS)
   ```bash
   cd G:\vrpg\vrpg-client\assets-and-models\models\tts\sovits
   .\venv310\Scripts\python.exe -m pip install phonemizer
   ```

2. **Modelo SoVITS treinado**
   - Localização: `assets-and-models/models/tts/sovits/dungeon_master_en.pth`
   - Config: `assets-and-models/models/tts/sovits/config.json`

### Como Testar

1. **Iniciar o servidor TTS**:
   ```bash
   cd G:\vrpg\vrpg-client
   cargo run --bin tts-server
   ```

2. **Testar com script PowerShell**:
   ```powershell
   cd G:\vrpg\vrpg-client\src\tts-service
   .\test_and_save_audio.ps1
   ```

3. **Verificar logs**:
   - Deve mostrar "Phoneme mapping: X known, Y unknown"
   - Deve mostrar "SoVITS conversion successful" se funcionar
   - Se falhar, mostrará "SoVITS Python conversion failed" e usará Piper direto

### Próximos Passos (Opcional)

1. **Otimização de Performance**:
   - Cache de phonemes frequentes
   - Pool de processos Python para SoVITS
   - Pré-carregar modelo SoVITS em memória

2. **Melhorias de Qualidade**:
   - Ajustar parâmetros do SoVITS (noice_scale, f0_predictor)
   - Suporte a múltiplos speakers
   - Suporte a emoções e estilos

3. **Monitoramento**:
   - Métricas de latência por etapa
   - Taxa de sucesso/falha do SoVITS
   - Qualidade de áudio gerado

### Notas Técnicas

- O sistema faz fallback gracioso: se Python falhar, usa métodos alternativos
- Logs detalhados ajudam a diagnosticar problemas
- Scripts Python são independentes e podem ser testados separadamente
- Compatível com Windows (usa `.exe` para Python do venv)

