# Script de Verificação GPU - VRPG Client
# Verifica se todos os componentes estão configurados para usar GPU

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "🔍 Verificação de Configuração GPU" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 1. Verificar NVIDIA GPU
Write-Host "1️⃣ Verificando GPU NVIDIA..." -ForegroundColor Yellow
try {
    $nvidiaSmi = nvidia-smi --query-gpu=name,driver_version,memory.total --format=csv,noheader 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   ✅ GPU NVIDIA detectada:" -ForegroundColor Green
        $nvidiaSmi | ForEach-Object { Write-Host "      $_" -ForegroundColor White }
    } else {
        Write-Host "   ❌ GPU NVIDIA não detectada ou nvidia-smi não encontrado" -ForegroundColor Red
        Write-Host "      Instale os drivers NVIDIA: https://www.nvidia.com/drivers" -ForegroundColor Yellow
    }
} catch {
    Write-Host "   ❌ Erro ao verificar GPU: $_" -ForegroundColor Red
}
Write-Host ""

# 2. Verificar PyTorch com CUDA
Write-Host "2️⃣ Verificando PyTorch com CUDA..." -ForegroundColor Yellow
try {
    $pythonCheck = python -c "import torch; print(f'PyTorch: {torch.__version__}'); print(f'CUDA Available: {torch.cuda.is_available()}'); print(f'CUDA Version: {torch.version.cuda if torch.cuda.is_available() else \"N/A\"}'); print(f'GPU Count: {torch.cuda.device_count() if torch.cuda.is_available() else 0}'); [print(f'GPU {i}: {torch.cuda.get_device_name(i)}') for i in range(torch.cuda.device_count())] if torch.cuda.is_available() else None" 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   ✅ PyTorch instalado:" -ForegroundColor Green
        $pythonCheck | ForEach-Object { Write-Host "      $_" -ForegroundColor White }
        
        if ($pythonCheck -match "CUDA Available: True") {
            Write-Host "   ✅ CUDA está disponível no PyTorch!" -ForegroundColor Green
        } else {
            Write-Host "   ⚠️  CUDA não está disponível no PyTorch" -ForegroundColor Yellow
            Write-Host "      Reinstale PyTorch com CUDA:" -ForegroundColor Yellow
            Write-Host "      pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121" -ForegroundColor Cyan
        }
    } else {
        Write-Host "   ❌ Erro ao verificar PyTorch: $pythonCheck" -ForegroundColor Red
    }
} catch {
    Write-Host "   ❌ Erro ao executar verificação PyTorch: $_" -ForegroundColor Red
}
Write-Host ""

# 3. Verificar Coqui TTS (XTTS)
Write-Host "3️⃣ Verificando Coqui TTS (XTTS)..." -ForegroundColor Yellow
try {
    $ttsCheck = python -c "from TTS.api import TTS; import torch; print(f'TTS Version: OK'); print(f'PyTorch CUDA: {torch.cuda.is_available()}'); tts = TTS('tts_models/multilingual/multi-dataset/xtts_v2', gpu=torch.cuda.is_available()); print(f'XTTS GPU Mode: {torch.cuda.is_available()}')" 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   ✅ Coqui TTS instalado e funcionando" -ForegroundColor Green
        $ttsCheck | ForEach-Object { Write-Host "      $_" -ForegroundColor White }
    } else {
        Write-Host "   ⚠️  Coqui TTS não está instalado ou há problemas" -ForegroundColor Yellow
        Write-Host "      Instale com: pip install TTS" -ForegroundColor Cyan
        Write-Host "      Erro: $ttsCheck" -ForegroundColor Red
    }
} catch {
    Write-Host "   ⚠️  Não foi possível verificar Coqui TTS" -ForegroundColor Yellow
}
Write-Host ""

# 4. Verificar SoVITS
Write-Host "4️⃣ Verificando SoVITS..." -ForegroundColor Yellow
$sovitsPath = "assets-and-models\models\tts\sovits"
if (Test-Path $sovitsPath) {
    Write-Host "   ✅ Diretório SoVITS encontrado" -ForegroundColor Green
    
    $venvPython = "$sovitsPath\venv310\Scripts\python.exe"
    if (Test-Path $venvPython) {
        Write-Host "   ✅ Python do venv SoVITS encontrado" -ForegroundColor Green
        
        try {
            $sovitsCheck = & $venvPython -c "import torch; print(f'PyTorch: {torch.__version__}'); print(f'CUDA Available: {torch.cuda.is_available()}'); print(f'GPU: {torch.cuda.get_device_name(0) if torch.cuda.is_available() else \"N/A\"}')" 2>&1
            
            if ($LASTEXITCODE -eq 0) {
                Write-Host "   ✅ SoVITS venv configurado:" -ForegroundColor Green
                $sovitsCheck | ForEach-Object { Write-Host "      $_" -ForegroundColor White }
            } else {
                Write-Host "   ⚠️  Erro ao verificar SoVITS venv: $sovitsCheck" -ForegroundColor Yellow
            }
        } catch {
            Write-Host "   ⚠️  Erro ao executar verificação SoVITS: $_" -ForegroundColor Yellow
        }
    } else {
        Write-Host "   ⚠️  Python do venv SoVITS não encontrado em: $venvPython" -ForegroundColor Yellow
    }
} else {
    Write-Host "   ⚠️  Diretório SoVITS não encontrado: $sovitsPath" -ForegroundColor Yellow
}
Write-Host ""

# 5. Verificar Variáveis de Ambiente
Write-Host "5️⃣ Verificando Variáveis de Ambiente..." -ForegroundColor Yellow
$envVars = @(
    "VRPG_GPU_ENABLED",
    "VRPG_TTS_USE_GPU",
    "VRPG_ASR_USE_GPU",
    "VRPG_LLM_USE_GPU",
    "VRPG_SOVITS_USE_GPU"
)

$allSet = $true
foreach ($var in $envVars) {
    $value = [Environment]::GetEnvironmentVariable($var, "User")
    if ($value) {
        Write-Host "   ✅ $var = $value" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  $var não está definida" -ForegroundColor Yellow
        $allSet = $false
    }
}

if (-not $allSet) {
    Write-Host ""
    Write-Host "   💡 Adicione ao seu arquivo .env:" -ForegroundColor Cyan
    Write-Host "      VRPG_GPU_ENABLED=true" -ForegroundColor White
    Write-Host "      VRPG_TTS_USE_GPU=true" -ForegroundColor White
    Write-Host "      VRPG_ASR_USE_GPU=true" -ForegroundColor White
    Write-Host "      VRPG_LLM_USE_GPU=true" -ForegroundColor White
    Write-Host "      VRPG_SOVITS_USE_GPU=true" -ForegroundColor White
}
Write-Host ""

# 6. Teste Rápido de Performance
Write-Host "6️⃣ Teste Rápido de Performance..." -ForegroundColor Yellow
Write-Host "   Executando teste de síntese XTTS com GPU..." -ForegroundColor White

try {
    $testScript = @"
import time
import torch
from TTS.api import TTS

print(f'CUDA Available: {torch.cuda.is_available()}')
if torch.cuda.is_available():
    print(f'GPU: {torch.cuda.get_device_name(0)}')

# Carregar modelo
start = time.time()
tts = TTS('tts_models/multilingual/multi-dataset/xtts_v2', gpu=torch.cuda.is_available())
load_time = time.time() - start
print(f'Model load time: {load_time:.2f}s')

# Teste de síntese
start = time.time()
audio = tts.tts(text='Hello World', speaker='Ana Florence', language='en')
synthesis_time = time.time() - start
print(f'Synthesis time: {synthesis_time:.2f}s')
print(f'Audio length: {len(audio)} samples')
print(f'Device used: {\"GPU\" if torch.cuda.is_available() else \"CPU\"}')
"@

    $testResult = python -c $testScript 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   ✅ Teste de síntese concluído:" -ForegroundColor Green
        $testResult | ForEach-Object { Write-Host "      $_" -ForegroundColor White }
        
        # Verificar se está usando GPU
        if ($testResult -match "Device used: GPU") {
            Write-Host "   ✅ XTTS está usando GPU!" -ForegroundColor Green
        } else {
            Write-Host "   ⚠️  XTTS está usando CPU (mais lento)" -ForegroundColor Yellow
        }
        
        # Verificar tempo de síntese
        if ($testResult -match "Synthesis time: (\d+\.\d+)s") {
            $synthTime = [double]$matches[1]
            if ($synthTime -lt 2.0) {
                Write-Host "   ✅ Tempo de síntese aceitável (< 2s)" -ForegroundColor Green
            } else {
                Write-Host "   ⚠️  Tempo de síntese alto (> 2s) - verifique GPU" -ForegroundColor Yellow
            }
        }
    } else {
        Write-Host "   ⚠️  Erro no teste: $testResult" -ForegroundColor Yellow
    }
} catch {
    Write-Host "   ⚠️  Erro ao executar teste: $_" -ForegroundColor Yellow
}
Write-Host ""

# Resumo
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "📊 Resumo da Verificação" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "✅ Componentes verificados:" -ForegroundColor Green
Write-Host "   - GPU NVIDIA" -ForegroundColor White
Write-Host "   - PyTorch com CUDA" -ForegroundColor White
Write-Host "   - Coqui TTS (XTTS)" -ForegroundColor White
Write-Host "   - SoVITS" -ForegroundColor White
Write-Host "   - Variáveis de Ambiente" -ForegroundColor White
Write-Host ""
Write-Host "💡 Próximos passos:" -ForegroundColor Cyan
Write-Host "   1. Configure as variáveis de ambiente no .env" -ForegroundColor White
Write-Host "   2. Reinicie o serviço TTS para aplicar configurações" -ForegroundColor White
Write-Host "   3. Monitore a latência durante uso real" -ForegroundColor White
Write-Host ""
Write-Host "📖 Documentação completa: docs/OTIMIZACAO_GPU_1.5S.md" -ForegroundColor Cyan
Write-Host ""

