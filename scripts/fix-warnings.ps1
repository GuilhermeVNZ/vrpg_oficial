# Auto-fix warnings script for VRPG Client
# This script automatically fixes fixable warnings

$ErrorActionPreference = "Continue"
Set-Location $PSScriptRoot\..

Write-Host "=== Auto-fixing Rust Format ===" -ForegroundColor Cyan
cargo fmt --all

Write-Host "`n=== Auto-fixing Clippy Warnings ===" -ForegroundColor Cyan
cargo clippy --workspace --fix --allow-dirty --allow-staged

Write-Host "`n=== Auto-fixing ESLint ===" -ForegroundColor Cyan
npm run lint -- --fix

Write-Host "`n=== Fixes applied! ===" -ForegroundColor Green
exit 0

