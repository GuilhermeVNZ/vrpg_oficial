# Verify setup-project-base completion criteria
# This script checks all completion criteria for the setup-project-base task

$ErrorActionPreference = "Continue"
Set-Location $PSScriptRoot\..

$allPassed = $true
$failures = @()

Write-Host "=== Verifying setup-project-base completion criteria ===" -ForegroundColor Cyan
Write-Host ""

# 1. Project Structure
Write-Host "1. Checking project structure..." -ForegroundColor Yellow
$requiredDirs = @("src", "tests", "docs", "config", "shared", "assets-and-models", "logs", "scripts")
foreach ($dir in $requiredDirs) {
    if (Test-Path $dir) {
        Write-Host "  [OK] $dir/ exists" -ForegroundColor Green
    } else {
        Write-Host "  [FAIL] $dir/ missing" -ForegroundColor Red
        $allPassed = $false
        $failures += "$dir/ directory missing"
    }
}

# 2. Rust Workspace
Write-Host "`n2. Checking Rust workspace..." -ForegroundColor Yellow
if (Test-Path "Cargo.toml") {
    Write-Host "  [OK] Cargo.toml exists" -ForegroundColor Green
    $cargoContent = Get-Content "Cargo.toml" -Raw
    if ($cargoContent -match "\[workspace\]") {
        Write-Host "  [OK] Workspace configured" -ForegroundColor Green
    } else {
        Write-Host "  [FAIL] Workspace not configured" -ForegroundColor Red
        $allPassed = $false
        $failures += "Cargo.toml workspace not configured"
    }
} else {
    Write-Host "  [FAIL] Cargo.toml missing" -ForegroundColor Red
    $allPassed = $false
    $failures += "Cargo.toml missing"
}

if (Test-Path "rustfmt.toml") {
    Write-Host "  [OK] rustfmt.toml exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] rustfmt.toml missing" -ForegroundColor Red
    $allPassed = $false
    $failures += "rustfmt.toml missing"
}

if (Test-Path ".clippy.toml") {
    Write-Host "  [OK] .clippy.toml exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] .clippy.toml missing" -ForegroundColor Red
    $allPassed = $false
    $failures += ".clippy.toml missing"
}

# 3. TypeScript Configuration
Write-Host "`n3. Checking TypeScript configuration..." -ForegroundColor Yellow
if (Test-Path "package.json") {
    Write-Host "  [OK] package.json exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] package.json missing" -ForegroundColor Red
    $allPassed = $false
    $failures += "package.json missing"
}

if (Test-Path "tsconfig.json") {
    Write-Host "  [OK] tsconfig.json exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] tsconfig.json missing" -ForegroundColor Red
    $allPassed = $false
    $failures += "tsconfig.json missing"
}

if (Test-Path "vitest.config.ts") {
    Write-Host "  [OK] vitest.config.ts exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] vitest.config.ts missing" -ForegroundColor Red
    $allPassed = $false
    $failures += "vitest.config.ts missing"
}

if (Test-Path ".eslintrc.json") {
    Write-Host "  [OK] .eslintrc.json exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] .eslintrc.json missing" -ForegroundColor Red
    $allPassed = $false
    $failures += ".eslintrc.json missing"
}

if (Test-Path ".prettierrc.json") {
    Write-Host "  [OK] .prettierrc.json exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] .prettierrc.json missing" -ForegroundColor Red
    $allPassed = $false
    $failures += ".prettierrc.json missing"
}

# 4. Development Tooling
Write-Host "`n4. Checking development tooling..." -ForegroundColor Yellow
if (Test-Path ".gitignore") {
    Write-Host "  [OK] .gitignore exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] .gitignore missing" -ForegroundColor Red
    $allPassed = $false
    $failures += ".gitignore missing"
}

if (Test-Path ".editorconfig") {
    Write-Host "  [OK] .editorconfig exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] .editorconfig missing" -ForegroundColor Red
    $allPassed = $false
    $failures += ".editorconfig missing"
}

# 5. Environment Configuration
Write-Host "`n5. Checking environment configuration..." -ForegroundColor Yellow
if (Test-Path "config/vrpg.json") {
    Write-Host "  [OK] config/vrpg.json exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] config/vrpg.json missing" -ForegroundColor Red
    $allPassed = $false
    $failures += "config/vrpg.json missing"
}

if (Test-Path "config/services.json") {
    Write-Host "  [OK] config/services.json exists" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] config/services.json missing" -ForegroundColor Red
    $allPassed = $false
    $failures += "config/services.json missing"
}

# 6. Quality Checks (run actual commands)
Write-Host "`n6. Running quality checks..." -ForegroundColor Yellow
Write-Host "  Running cargo check..." -ForegroundColor Gray
$cargoCheck = & cargo check --workspace 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  [OK] cargo check passed" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] cargo check failed" -ForegroundColor Red
    $allPassed = $false
    $failures += "cargo check failed"
}

Write-Host "  Running cargo fmt --check..." -ForegroundColor Gray
$cargoFmt = & cargo fmt --check --all 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  [OK] cargo fmt --check passed" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] cargo fmt --check failed (run 'cargo fmt --all' to fix)" -ForegroundColor Red
    $allPassed = $false
    $failures += "cargo fmt --check failed"
}

Write-Host "  Running cargo clippy..." -ForegroundColor Gray
$cargoClippy = & cargo clippy --workspace -- -D warnings 2>&1 | Select-Object -Last 5
if ($LASTEXITCODE -eq 0) {
    Write-Host "  [OK] cargo clippy passed" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] cargo clippy failed" -ForegroundColor Red
    $allPassed = $false
    $failures += "cargo clippy failed"
}

Write-Host "  Running npm type-check..." -ForegroundColor Gray
$npmTypeCheckResult = cmd /c "npm run type-check 2>&1 && echo SUCCESS || echo FAILED"
if ($npmTypeCheckResult -match "SUCCESS" -or $npmTypeCheckResult -notmatch "error TS") {
    Write-Host "  [OK] npm type-check passed" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] npm type-check failed" -ForegroundColor Red
    $allPassed = $false
    $failures += "npm type-check failed"
}

Write-Host "  Running npm lint..." -ForegroundColor Gray
$npmLintResult = cmd /c "npm run lint 2>&1 && echo SUCCESS || echo FAILED"
if ($npmLintResult -match "SUCCESS" -or ($npmLintResult -notmatch "error" -and $npmLintResult -notmatch "FAILED")) {
    Write-Host "  [OK] npm lint passed" -ForegroundColor Green
} else {
    Write-Host "  [FAIL] npm lint failed" -ForegroundColor Red
    $allPassed = $false
    $failures += "npm lint failed"
}

# Summary
Write-Host "`n=== Summary ===" -ForegroundColor Cyan
if ($allPassed) {
    Write-Host "[SUCCESS] All checks passed! setup-project-base is complete." -ForegroundColor Green
    exit 0
} else {
    Write-Host "[FAILED] Some checks failed:" -ForegroundColor Red
    foreach ($failure in $failures) {
        Write-Host "  - $failure" -ForegroundColor Red
    }
    exit 1
}

