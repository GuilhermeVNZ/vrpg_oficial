#!/usr/bin/env pwsh
# Script para testar os servidores MCP

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Teste dos Servidores MCP" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Verificar Vectorizer
Write-Host "Testando Vectorizer (porta 15002)..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://127.0.0.1:15002/health" -Method GET -TimeoutSec 3 -UseBasicParsing
    Write-Host "✅ Vectorizer: Online (Status $($response.StatusCode))" -ForegroundColor Green
} catch {
    Write-Host "❌ Vectorizer: Offline - $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""

# Verificar Synap
Write-Host "Testando Synap (porta 15500)..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://127.0.0.1:15500/health" -Method GET -TimeoutSec 3 -UseBasicParsing
    Write-Host "✅ Synap: Online (Status $($response.StatusCode))" -ForegroundColor Green
} catch {
    Write-Host "❌ Synap: Offline - $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""

# Verificar Python
Write-Host "Verificando Python..." -ForegroundColor Yellow
try {
    $pythonVersion = python --version 2>&1
    Write-Host "✅ Python: $pythonVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Python: Não encontrado" -ForegroundColor Red
}

Write-Host ""

# Verificar aiohttp
Write-Host "Verificando aiohttp..." -ForegroundColor Yellow
try {
    python -c "import aiohttp; print('aiohttp OK')" 2>&1 | Out-Null
    Write-Host "✅ aiohttp: Instalado" -ForegroundColor Green
} catch {
    Write-Host "❌ aiohttp: Não instalado - Execute: pip install -r requirements-mcp.txt" -ForegroundColor Red
}

Write-Host ""

# Verificar scripts
Write-Host "Verificando scripts MCP..." -ForegroundColor Yellow
$scripts = @(
    @{Name="synap-mcp-simple.py"; Path="G:\vrpg\vrpg-client\synap-mcp-simple.py"},
    @{Name="vectorizer-mcp.py"; Path="G:\vrpg\vrpg-client\vectorizer-mcp.py"},
    @{Name="context7-mcp.py"; Path="G:\vrpg\vrpg-client\context7-mcp.py"}
)

foreach ($script in $scripts) {
    if (Test-Path $script.Path) {
        Write-Host "✅ $($script.Name): Existe" -ForegroundColor Green
    } else {
        Write-Host "❌ $($script.Name): Não encontrado" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Configuração MCP" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "O arquivo mcp_servers.json está configurado com:" -ForegroundColor White
Write-Host "  - synap" -ForegroundColor Cyan
Write-Host "  - vectorizer" -ForegroundColor Cyan
Write-Host "  - context7" -ForegroundColor Cyan
Write-Host ""
Write-Host "Para usar no Cursor:" -ForegroundColor Yellow
Write-Host "1. Copie o conteúdo de mcp_servers.json para:" -ForegroundColor White
Write-Host "   %APPDATA%\Cursor\User\globalStorage\rooveterinaryinc.roo-cline\settings\cline_mcp_settings.json" -ForegroundColor Gray
Write-Host ""
Write-Host "2. Ou configure manualmente em:" -ForegroundColor White
Write-Host "   Settings → Cursor Settings → MCP" -ForegroundColor Gray
Write-Host ""
Write-Host "3. Reinicie o Cursor" -ForegroundColor White
Write-Host ""






























