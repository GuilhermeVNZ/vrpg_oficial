# Instruções para Reiniciar e Testar

## Problemas Identificados e Corrigidos

### 1. SoVITS não estava convertendo
**Causa**: Python do sistema sendo usado (sem soundfile)
**Correção**: Caminho do Python do venv310 corrigido

### 2. Pronúncia sem pausas
**Causa**: Phonemes sem pausas entre palavras
**Correção**: Pausas adicionadas na phonemização

## Como Reiniciar o Servidor

1. **Parar o servidor atual**:
   ```powershell
   Get-Process | Where-Object { $_.ProcessName -like "*tts-server*" } | Stop-Process -Force
   ```

2. **Recompilar** (se necessário):
   ```powershell
   cd G:\vrpg\vrpg-client
   cargo build --package tts-service
   ```

3. **Iniciar o servidor**:
   ```powershell
   cd G:\vrpg\vrpg-client
   $env:RUST_LOG='info'
   cargo run --bin tts-server
   ```

4. **Testar**:
   ```powershell
   cd G:\vrpg\vrpg-client\src\tts-service
   $testText = "Hello world. This is a test."
   $body = @{ text = '<VOICE actor="dungeon_master_en" emotion="neutral" style="narrative">' + $testText + '</VOICE>'; language = "en" } | ConvertTo-Json
   $response = Invoke-RestMethod -Uri "http://localhost:3002/speak" -Method Post -Body $body -ContentType "application/json" -TimeoutSec 60
   ```

## O que Verificar nos Logs

Após reiniciar, os logs devem mostrar:

1. **Python correto sendo usado**:
   ```
   Using SoVITS venv Python: "G:\\vrpg\\vrpg-client\\assets-and-models\\models\\tts\\sovits\\venv310\\Scripts\\python.exe"
   ```

2. **SoVITS funcionando**:
   ```
   SoVITS conversion successful for character: dungeon_master_en
   ```

3. **Pausas na phonemização**:
   ```
   Phonemized '...' -> X IPA phonemes (with pauses: Y)
   ```

## Se Ainda Não Funcionar

1. Verifique se o Python do venv existe:
   ```powershell
   Test-Path "G:\vrpg\vrpg-client\assets-and-models\models\tts\sovits\venv310\Scripts\python.exe"
   ```

2. Teste o script Python diretamente:
   ```powershell
   cd G:\vrpg\vrpg-client\assets-and-models\models\tts\sovits
   .\venv310\Scripts\python.exe ..\..\..\..\..\src\tts-service\scripts\sovits_convert.py --help
   ```

3. Verifique os logs do servidor para erros específicos

