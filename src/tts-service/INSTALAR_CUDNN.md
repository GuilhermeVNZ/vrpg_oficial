# Como Instalar cuDNN para Habilitar CUDA/VRAM no Piper

## Problema

O ONNX Runtime está tentando usar CUDA, mas está falhando com o erro:
```
Error loading "onnxruntime_providers_cuda.dll" which depends on "cudnn64_9.dll" which is missing.
```

Isso significa que o **cuDNN** (CUDA Deep Neural Network library) não está instalado ou não está no PATH.

## Solução: Instalar cuDNN

### Passo 1: Verificar Versão do CUDA

1. Abra o PowerShell e execute:
   ```powershell
   nvidia-smi
   ```
2. Anote a versão do CUDA mostrada (ex: "CUDA Version: 12.8")

### Passo 2: Baixar cuDNN

1. Acesse: https://developer.nvidia.com/cudnn
2. Faça login (criação de conta gratuita é necessária)
3. Baixe a versão do cuDNN compatível com sua versão do CUDA
   - Para CUDA 12.x: baixe cuDNN 9.x
   - Para CUDA 11.x: baixe cuDNN 8.x

### Passo 3: Instalar cuDNN

1. Extraia o arquivo ZIP baixado
2. Copie os arquivos para o diretório de instalação do CUDA:
   - **Caminho típico**: `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.x\`
   - Copie:
     - `bin\cudnn64_9.dll` → `CUDA\v12.x\bin\`
     - `include\cudnn.h` → `CUDA\v12.x\include\`
     - `lib\cudnn.lib` → `CUDA\v12.x\lib\`

### Passo 4: Adicionar ao PATH (se necessário)

1. Adicione o diretório `bin` do CUDA ao PATH do sistema:
   - Exemplo: `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.8\bin`
2. Reinicie o terminal/PowerShell após adicionar ao PATH

### Passo 5: Verificar Instalação

1. Verifique se `cudnn64_9.dll` existe:
   ```powershell
   Test-Path "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.8\bin\cudnn64_9.dll"
   ```
2. Se retornar `True`, o cuDNN está instalado corretamente

### Passo 6: Reiniciar o Servidor TTS

Após instalar cuDNN, reinicie o servidor TTS. Você deve ver nos logs:
- ✅ CUDA execution provider registrado com sucesso
- ✅ O modelo rodará em VRAM (não em RAM)

## Alternativa: Usar CPU (sem CUDA)

Se você não quiser instalar cuDNN agora, o sistema continuará funcionando usando CPU/RAM. O desempenho será menor, mas funcional.

## Verificação

Após instalar cuDNN e reiniciar o servidor, verifique os logs:
- ❌ **Se CUDA falhou**: `Adding default CPU execution provider` (rodando em RAM)
- ✅ **Se CUDA funcionou**: Não aparecerá a mensagem acima, e você verá uso de VRAM no Task Manager



