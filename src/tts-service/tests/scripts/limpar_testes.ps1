# Script para limpar arquivos de teste e liberar espaço

Write-Host "🧹 Limpando arquivos de teste..." -ForegroundColor Yellow

$scriptsDir = $PSScriptRoot
$sovitsDir = Join-Path $scriptsDir "..\..\..\..\assets-and-models\models\tts\sovits"

$deletedCount = 0
$deletedSize = 0

# 1. Remover áudios de teste do diretório de scripts
Write-Host "`n📁 Limpando áudios de teste em scripts..." -ForegroundColor Cyan
Get-ChildItem -Path $scriptsDir -Filter "test_*.wav" -File | ForEach-Object {
    $size = $_.Length
    Remove-Item $_.FullName -Force -ErrorAction SilentlyContinue
    $deletedCount++
    $deletedSize += $size
    Write-Host "   ✅ Removido: $($_.Name)" -ForegroundColor Green
}

# 2. Remover diretório sovits_quality_tests
$qualityTestsDir = Join-Path $scriptsDir "sovits_quality_tests"
if (Test-Path $qualityTestsDir) {
    Write-Host "`n📁 Removendo sovits_quality_tests..." -ForegroundColor Cyan
    $size = (Get-ChildItem -Path $qualityTestsDir -Recurse -File -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
    Remove-Item $qualityTestsDir -Recurse -Force -ErrorAction SilentlyContinue
    $deletedCount++
    $deletedSize += $size
    Write-Host "   ✅ Removido: sovits_quality_tests" -ForegroundColor Green
}

# 3. Remover logs do SoVITS (checkpoints podem ser re-treinados)
$logsDir = Join-Path $sovitsDir "logs\44k"
if (Test-Path $logsDir) {
    Write-Host "`n📁 Removendo logs do SoVITS (checkpoints)..." -ForegroundColor Cyan
    $size = (Get-ChildItem -Path $logsDir -Recurse -File -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
    Remove-Item $logsDir -Recurse -Force -ErrorAction SilentlyContinue
    $deletedCount++
    $deletedSize += $size
    Write-Host "   ✅ Removido: logs/44k (checkpoints podem ser re-treinados)" -ForegroundColor Green
}

# 4. Remover diretório raw do SoVITS (testes)
$rawDir = Join-Path $sovitsDir "raw"
if (Test-Path $rawDir) {
    Write-Host "`n📁 Removendo raw do SoVITS (testes)..." -ForegroundColor Cyan
    $size = (Get-ChildItem -Path $rawDir -Recurse -File -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
    Remove-Item $rawDir -Recurse -Force -ErrorAction SilentlyContinue
    $deletedCount++
    $deletedSize += $size
    Write-Host "   ✅ Removido: raw (testes)" -ForegroundColor Green
}

# 5. Remover arquivos temporários
Write-Host "`n📁 Removendo arquivos temporários..." -ForegroundColor Cyan
$tempFile = Join-Path $scriptsDir "temp_xtts_44100.wav"
if (Test-Path $tempFile) {
    $size = (Get-Item $tempFile).Length
    Remove-Item $tempFile -Force -ErrorAction SilentlyContinue
    $deletedCount++
    $deletedSize += $size
    Write-Host "   ✅ Removido: temp_xtts_44100.wav" -ForegroundColor Green
}

# Resumo
Write-Host "`n================================================================" -ForegroundColor Yellow
Write-Host "✅ LIMPEZA CONCLUÍDA!" -ForegroundColor Green
Write-Host "   Arquivos removidos: $deletedCount" -ForegroundColor Cyan
Write-Host "   Espaço liberado: $([math]::Round($deletedSize / 1MB, 2)) MB" -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Yellow

Write-Host "`n📋 Arquivos importantes MANTIDOS:" -ForegroundColor Green
Write-Host "   ✅ dungeon_master_en_xtts_reference_clean.wav" -ForegroundColor White
Write-Host "   ✅ dungeon_master_en_xtts_reference.wav" -ForegroundColor White
Write-Host "   ✅ Scripts Python essenciais" -ForegroundColor White
Write-Host "   ✅ Documentação (DESCOBERTA_RAW.md, REGISTRO_FINAL_RAW.md)" -ForegroundColor White
Write-Host "   ✅ Config do SoVITS (configs/config.json)" -ForegroundColor White
Write-Host "   ✅ Pretrain do SoVITS (pretrain/)" -ForegroundColor White
Write-Host "   ✅ Dataset do SoVITS (dataset/, dataset_raw/)" -ForegroundColor White
Write-Host "   ✅ Filelists do SoVITS (filelists/)" -ForegroundColor White
