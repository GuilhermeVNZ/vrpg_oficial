# Run cargo commands in isolated PowerShell session
# This prevents terminal disconnection issues
# Usage: .\scripts\run-cargo-isolated.ps1 <command> [args...]

param(
    [Parameter(Mandatory=$true)]
    [string]$Command,
    
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$Args
)

$ErrorActionPreference = "Stop"
$scriptPath = $PSScriptRoot
$projectRoot = Join-Path $scriptPath ".."

# Create a temporary script file to run in isolated session
$tempScript = Join-Path $env:TEMP "vrpg-cargo-$(Get-Random).ps1"

try {
    # Write the command to temp script
    $scriptContent = @"
Set-Location "$projectRoot"
`$ErrorActionPreference = "Stop"

Write-Host "=== Running: cargo $Command $($Args -join ' ') ===" -ForegroundColor Cyan
Write-Host ""

`$fullCommand = "cargo $Command"
if (`$Args.Count -gt 0) {
    `$fullCommand += " " + (`$Args -join " ")
}

Invoke-Expression `$fullCommand

if (`$LASTEXITCODE -ne 0) {
    Write-Host "`nERROR: Command failed with exit code `$LASTEXITCODE" -ForegroundColor Red
    exit `$LASTEXITCODE
}

Write-Host "`n=== Command completed successfully ===" -ForegroundColor Green
exit 0
"@
    
    Set-Content -Path $tempScript -Value $scriptContent -Encoding UTF8
    
    # Run in new PowerShell session
    $process = Start-Process powershell.exe -ArgumentList @(
        "-NoProfile",
        "-ExecutionPolicy", "Bypass",
        "-File", $tempScript
    ) -Wait -PassThru -NoNewWindow
    
    exit $process.ExitCode
} finally {
    # Clean up temp script
    if (Test-Path $tempScript) {
        Remove-Item $tempScript -Force -ErrorAction SilentlyContinue
    }
}

