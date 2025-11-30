# Test runner script for VRPG Client
# This script runs all tests in a safe, isolated way

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot\..

Write-Host "=== Running Rust Tests ===" -ForegroundColor Cyan
cargo test --workspace
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Rust tests failed" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== Running TypeScript Tests ===" -ForegroundColor Cyan
npm test
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: TypeScript tests failed" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== All tests passed! ===" -ForegroundColor Green
exit 0

