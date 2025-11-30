# Cargo command runner - Isolated execution
# Usage: .\scripts\run-cargo.ps1 <command> [args...]
# Example: .\scripts\run-cargo.ps1 "clippy --workspace"

param(
    [Parameter(Mandatory=$true)]
    [string]$Command,
    
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$Args
)

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot\..

Write-Host "=== Running: cargo $Command $($Args -join ' ') ===" -ForegroundColor Cyan

$fullCommand = "cargo $Command"
if ($Args.Count -gt 0) {
    $fullCommand += " " + ($Args -join " ")
}

Invoke-Expression $fullCommand

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Command failed with exit code $LASTEXITCODE" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "`n=== Command completed successfully ===" -ForegroundColor Green
exit 0

