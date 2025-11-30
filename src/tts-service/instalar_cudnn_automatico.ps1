# Script para instalar cuDNN automaticamente
# Procura o arquivo baixado e copia para o diretório do CUDA

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  INSTALAÇÃO AUTOMÁTICA DO CUDNN" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

# 1. Verificar versão do CUDA
Write-Host "`n1. Verificando versão do CUDA..." -ForegroundColor Cyan
$cudaBase = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA"
if (-not (Test-Path $cudaBase)) {
    Write-Host "❌ CUDA Toolkit não encontrado em: $cudaBase" -ForegroundColor Red
    Write-Host "   Instale o CUDA Toolkit primeiro: https://developer.nvidia.com/cuda-downloads" -ForegroundColor Yellow
    exit 1
}

$versions = Get-ChildItem $cudaBase -Directory | Sort-Object Name -Descending
if ($versions.Count -eq 0) {
    Write-Host "❌ Nenhuma versão do CUDA encontrada" -ForegroundColor Red
    exit 1
}

# Usar a versão mais recente (ou 12.8 se disponível)
$targetVersion = $versions | Where-Object { $_.Name -eq "v12.8" } | Select-Object -First 1
if (-not $targetVersion) {
    $targetVersion = $versions[0]
}
$cudaVersion = $targetVersion.Name
$cudaBinPath = "$cudaBase\$cudaVersion\bin"

Write-Host "✅ CUDA encontrado: $cudaVersion" -ForegroundColor Green
Write-Host "   Diretório bin: $cudaBinPath" -ForegroundColor White

# 2. Verificar se cuDNN já está instalado
Write-Host "`n2. Verificando se cuDNN já está instalado..." -ForegroundColor Cyan
$cudnnDll = Join-Path $cudaBinPath "cudnn64_9.dll"
if (Test-Path $cudnnDll) {
    Write-Host "✅ cuDNN já está instalado em: $cudnnDll" -ForegroundColor Green
    Write-Host "   Reinicie o servidor TTS para usar CUDA!" -ForegroundColor Yellow
    exit 0
}

# 3. Procurar arquivo cuDNN baixado
Write-Host "`n3. Procurando arquivo cuDNN baixado..." -ForegroundColor Cyan
$searchLocations = @(
    "$env:USERPROFILE\Downloads",
    "$env:USERPROFILE\Desktop",
    "C:\",
    "G:\"
)

$cudnnZip = $null
$cudnnFolder = $null

# Procurar arquivo ZIP
foreach ($loc in $searchLocations) {
    if (Test-Path $loc) {
        $zip = Get-ChildItem $loc -Filter "*cudnn*.zip" -Recurse -ErrorAction SilentlyContinue -Depth 2 | Select-Object -First 1
        if ($zip) {
            Write-Host "✅ Arquivo ZIP encontrado: $($zip.FullName)" -ForegroundColor Green
            $cudnnZip = $zip
            break
        }
    }
}

# Procurar pasta extraída
if (-not $cudnnZip) {
    foreach ($loc in $searchLocations) {
        if (Test-Path $loc) {
            $folder = Get-ChildItem $loc -Directory -Filter "*cudnn*" -Recurse -ErrorAction SilentlyContinue -Depth 2 | Select-Object -First 1
            if ($folder) {
                $dll = Get-ChildItem $folder.FullName -Recurse -Filter "cudnn64_9.dll" -ErrorAction SilentlyContinue | Select-Object -First 1
                if ($dll) {
                    Write-Host "✅ Pasta cuDNN encontrada: $($folder.FullName)" -ForegroundColor Green
                    $cudnnFolder = $folder
                    break
                }
            }
        }
    }
}

if (-not $cudnnZip -and -not $cudnnFolder) {
    Write-Host "`n❌ cuDNN não encontrado!" -ForegroundColor Red
    Write-Host "`nPor favor:" -ForegroundColor Yellow
    Write-Host "1. Baixe o cuDNN de: https://developer.nvidia.com/cudnn-downloads" -ForegroundColor White
    Write-Host "2. Escolha a versão compatível com CUDA $cudaVersion" -ForegroundColor White
    Write-Host "3. Baixe o arquivo ZIP" -ForegroundColor White
    Write-Host "4. Execute este script novamente" -ForegroundColor White
    Write-Host "`nOu, se já extraiu o cuDNN, coloque-o em uma pasta acessível (Downloads, Desktop, etc.)" -ForegroundColor Yellow
    exit 1
}

# 4. Extrair ZIP se necessário
if ($cudnnZip) {
    Write-Host "`n4. Extraindo arquivo ZIP..." -ForegroundColor Cyan
    $extractPath = Join-Path $env:TEMP "cudnn_extract_$(Get-Random)"
    New-Item -ItemType Directory -Path $extractPath -Force | Out-Null
    
    try {
        Expand-Archive -Path $cudnnZip.FullName -DestinationPath $extractPath -Force
        Write-Host "✅ Extração concluída" -ForegroundColor Green
        
        # Procurar a pasta bin dentro da extração
        $binFolder = Get-ChildItem $extractPath -Recurse -Directory -Filter "bin" -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($binFolder) {
            $cudnnFolder = $binFolder.Parent
        } else {
            # Se não encontrar bin, procurar diretamente pelo DLL
            $dll = Get-ChildItem $extractPath -Recurse -Filter "cudnn64_9.dll" -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($dll) {
                $cudnnFolder = $dll.Directory.Parent
            }
        }
    } catch {
        Write-Host "❌ Erro ao extrair: $_" -ForegroundColor Red
        exit 1
    }
}

# 5. Copiar arquivos para o diretório do CUDA
if ($cudnnFolder) {
    Write-Host "`n5. Copiando arquivos cuDNN para CUDA..." -ForegroundColor Cyan
    
    # Procurar os arquivos necessários
    $binPath = Join-Path $cudnnFolder.FullName "bin"
    $includePath = Join-Path $cudnnFolder.FullName "include"
    $libPath = Join-Path $cudnnFolder.FullName "lib"
    
    if (-not (Test-Path $binPath)) {
        # Se não tem estrutura de pastas, procurar diretamente
        $dll = Get-ChildItem $cudnnFolder.FullName -Recurse -Filter "cudnn64_9.dll" -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($dll) {
            $binPath = $dll.DirectoryName
        }
    }
    
    if (Test-Path $binPath) {
        Write-Host "   Copiando DLLs de: $binPath" -ForegroundColor White
        $dlls = Get-ChildItem $binPath -Filter "cudnn*.dll" -ErrorAction SilentlyContinue
        
        # Tentar copiar para CUDA primeiro
        $copiedToCuda = $false
        $copiedToRelease = $false
        
        foreach ($dll in $dlls) {
            $destCuda = Join-Path $cudaBinPath $dll.Name
            try {
                Copy-Item $dll.FullName $destCuda -Force -ErrorAction Stop
                Write-Host "   ✅ Copiado para CUDA: $($dll.Name)" -ForegroundColor Green
                $copiedToCuda = $true
            } catch {
                Write-Host "   ⚠️ Falha ao copiar para CUDA (permissão negada): $($dll.Name)" -ForegroundColor Yellow
                # Fallback: copiar para target/release/
                $releasePath = Join-Path (Get-Location) "..\..\target\release"
                if (-not (Test-Path $releasePath)) {
                    New-Item -ItemType Directory -Path $releasePath -Force | Out-Null
                }
                $destRelease = Join-Path $releasePath $dll.Name
                try {
                    Copy-Item $dll.FullName $destRelease -Force -ErrorAction Stop
                    Write-Host "   ✅ Copiado para target/release/: $($dll.Name)" -ForegroundColor Green
                    $copiedToRelease = $true
                } catch {
                    Write-Host "   ❌ Falha ao copiar para target/release/: $($dll.Name) - $_" -ForegroundColor Red
                }
            }
        }
        
        if ($copiedToRelease -and -not $copiedToCuda) {
            Write-Host "`n⚠️ DLLs copiados para target/release/ (não foi possível copiar para CUDA devido a permissões)" -ForegroundColor Yellow
            Write-Host "   O servidor TTS encontrará os DLLs no mesmo diretório do executável" -ForegroundColor White
        } elseif ($copiedToCuda) {
            Write-Host "`n✅ DLLs copiados para CUDA Toolkit" -ForegroundColor Green
        }
    } else {
        Write-Host "❌ Pasta 'bin' não encontrada em: $($cudnnFolder.FullName)" -ForegroundColor Red
        Write-Host "   Estrutura esperada: cudnn/bin/cudnn64_9.dll" -ForegroundColor Yellow
        exit 1
    }
    
    # Limpar arquivos temporários
    if ($cudnnZip) {
        Write-Host "`n6. Limpando arquivos temporários..." -ForegroundColor Cyan
        Remove-Item $extractPath -Recurse -Force -ErrorAction SilentlyContinue
    }
    
    Write-Host "`n✅ cuDNN instalado com sucesso!" -ForegroundColor Green
    Write-Host "`n📋 PRÓXIMOS PASSOS:" -ForegroundColor Yellow
    Write-Host "1. Reinicie o terminal/PowerShell (para recarregar PATH)" -ForegroundColor White
    Write-Host "2. Reinicie o servidor TTS" -ForegroundColor White
    Write-Host "3. Verifique os logs para confirmar que CUDA está funcionando" -ForegroundColor White
} else {
    Write-Host "❌ Não foi possível localizar os arquivos cuDNN" -ForegroundColor Red
    exit 1
}

