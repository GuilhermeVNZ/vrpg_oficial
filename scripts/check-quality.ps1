# Quality check script for VRPG Client
# This script runs all quality checks in a safe, isolated way

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot\..

Write-Host "=== Running Rust Format Check ===" -ForegroundColor Cyan
cargo fmt --check --all
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: rustfmt check failed" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== Running Clippy ===" -ForegroundColor Cyan
cargo clippy --workspace -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: clippy check failed" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== Running TypeScript Type Check ===" -ForegroundColor Cyan
npm run type-check
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: TypeScript type check failed" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== Running ESLint ===" -ForegroundColor Cyan
npm run lint
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: ESLint check failed" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== All quality checks passed! ===" -ForegroundColor Green
exit 0

