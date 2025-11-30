# Setup Rápido GPU - VRPG Client

## ⚠️ Status Atual

Você tem PyTorch instalado, mas é a versão **CPU-only** (`2.9.1+cpu`). Para atingir < 1.5s de latência, precisamos reinstalar PyTorch com suporte CUDA.

## 🚀 Instalação Rápida

### Opção 1: Script Automatizado (Recomendado)

```powershell
# Execute o script de instalação
.\scripts\install_pytorch_cuda.ps1
```

### Opção 2: Manual

```powershell
# 1. Desinstalar PyTorch CPU-only
pip uninstall torch torchvision torchaudio -y

# 2. Instalar PyTorch com CUDA 12.1
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121

# 3. Verificar instalação
python scripts\check_pytorch_cuda.py
```

## ✅ Verificação

Após instalar, execute:

```powershell
# Verificar configuração completa
.\scripts\verify_gpu_setup.ps1

# Testar latência
.\scripts\test_gpu_latency.ps1
```

## 📋 Checklist

- [ ] PyTorch com CUDA instalado
- [ ] Coqui TTS instalado
- [ ] SoVITS venv configurado
- [ ] Variáveis de ambiente configuradas no `.env`
- [ ] Teste de latência < 1.5s

## 🔧 Variáveis de Ambiente

Crie/edite o arquivo `.env` na raiz do projeto:

```bash
# GPU Configuration
VRPG_GPU_ENABLED=true
VRPG_TTS_USE_GPU=true
VRPG_ASR_USE_GPU=true
VRPG_LLM_USE_GPU=true
VRPG_SOVITS_USE_GPU=true
```

## 📖 Documentação Completa

Veja `docs/OTIMIZACAO_GPU_1.5S.md` para detalhes completos.

