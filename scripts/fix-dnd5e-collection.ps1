# Script para corrigir a collection dnd5e-rules para dimensao 512
# Este script deleta a collection antiga (384) e cria uma nova (512)

$ErrorActionPreference = "Stop"

$VectorizerUrl = "http://localhost:8002"
$CollectionName = "dnd5e-rules"

Write-Host "Corrigindo collection dnd5e-rules para dimensao 512..." -ForegroundColor Cyan
Write-Host ""

# Verificar se o Vectorizer esta rodando
try {
    $health = Invoke-WebRequest -Uri "$VectorizerUrl/health" -Method GET -ErrorAction Stop
    Write-Host "Vectorizer esta rodando" -ForegroundColor Green
} catch {
    Write-Host "ERRO: Vectorizer nao esta rodando em $VectorizerUrl" -ForegroundColor Red
    Write-Host "Inicie o Vectorizer antes de executar este script" -ForegroundColor Yellow
    exit 1
}

# Deletar collection antiga
Write-Host "Deletando collection antiga (dimensao 384)..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$VectorizerUrl/collections/$CollectionName" -Method DELETE -ErrorAction Stop
    Write-Host "Collection deletada com sucesso" -ForegroundColor Green
} catch {
    if ($_.Exception.Response.StatusCode -eq 404) {
        Write-Host "Collection nao existe (ja foi deletada ou nunca existiu)" -ForegroundColor Yellow
    } else {
        Write-Host "Erro ao deletar collection: $_" -ForegroundColor Red
        Write-Host "Tentando continuar mesmo assim..." -ForegroundColor Yellow
    }
}

# Aguardar um pouco para garantir que a delecao foi processada
Start-Sleep -Seconds 1

# Criar nova collection com dimensao 512
Write-Host "Criando nova collection com dimensao 512..." -ForegroundColor Yellow

$body = @{
    name = $CollectionName
    dimension = 512
    metric = "cosine"
} | ConvertTo-Json

try {
    $response = Invoke-WebRequest -Uri "$VectorizerUrl/collections" -Method POST -Body $body -ContentType "application/json" -ErrorAction Stop
    Write-Host "Collection criada com sucesso!" -ForegroundColor Green
    Write-Host "  Nome: $CollectionName" -ForegroundColor Gray
    Write-Host "  Dimensao: 512" -ForegroundColor Gray
    Write-Host "  Metrica: cosine" -ForegroundColor Gray
} catch {
    Write-Host "Erro ao criar collection: $_" -ForegroundColor Red
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "Resposta do servidor: $responseBody" -ForegroundColor Red
    }
    exit 1
}

Write-Host ""
Write-Host "Collection corrigida com sucesso!" -ForegroundColor Green
Write-Host "Agora voce pode inserir os chunks de D&D 5e na collection '$CollectionName'" -ForegroundColor Cyan
Write-Host ""


