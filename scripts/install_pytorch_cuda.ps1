# Script para instalar PyTorch com suporte CUDA
# Execute este script se PyTorch estiver instalado sem CUDA

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "🔧 Instalação PyTorch com CUDA" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Verificar CUDA disponível
Write-Host "1️⃣ Verificando CUDA disponível..." -ForegroundColor Yellow
try {
    $cudaVersion = nvcc --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "   ✅ CUDA detectado:" -ForegroundColor Green
        $cudaVersion | Select-String "release" | ForEach-Object { Write-Host "      $_" -ForegroundColor White }
    } else {
        Write-Host "   ⚠️  CUDA não detectado via nvcc" -ForegroundColor Yellow
        Write-Host "      Mas isso não impede de usar PyTorch com CUDA" -ForegroundColor White
    }
} catch {
    Write-Host "   ⚠️  nvcc não encontrado (normal se CUDA toolkit não estiver no PATH)" -ForegroundColor Yellow
}
Write-Host ""

# Verificar PyTorch atual
Write-Host "2️⃣ Verificando PyTorch atual..." -ForegroundColor Yellow
python -c "import torch; print(f'PyTorch: {torch.__version__}'); print(f'CUDA Available: {torch.cuda.is_available()}')"
Write-Host ""

# Instalar PyTorch com CUDA
Write-Host "3️⃣ Instalando PyTorch com CUDA 12.1..." -ForegroundColor Yellow
Write-Host "   Isso pode levar alguns minutos..." -ForegroundColor White
Write-Host ""

# Desinstalar PyTorch CPU
Write-Host "   Desinstalando PyTorch CPU-only..." -ForegroundColor White
pip uninstall torch torchvision torchaudio -y

# Instalar PyTorch com CUDA
Write-Host "   Instalando PyTorch com CUDA 12.1..." -ForegroundColor White
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121

Write-Host ""
Write-Host "4️⃣ Verificando instalação..." -ForegroundColor Yellow
python scripts\check_pytorch_cuda.py

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "✅ Instalação concluída!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "💡 Próximos passos:" -ForegroundColor Cyan
Write-Host "   1. Execute: .\scripts\verify_gpu_setup.ps1" -ForegroundColor White
Write-Host "   2. Configure variáveis de ambiente no .env" -ForegroundColor White
Write-Host "   3. Teste latência: .\scripts\test_gpu_latency.ps1" -ForegroundColor White
Write-Host ""

