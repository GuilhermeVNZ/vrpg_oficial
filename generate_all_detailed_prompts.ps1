Write-Host "Generating DETAILED prompts for ALL monsters (except gelatinous_cube and goblin01)..."
Write-Host "This will create 348 monsters x 9 frames = 3,132 unique detailed prompts!"
Write-Host ""

$baseDir = "g:\vrpg\vrpg-client\assets-and-models\sprites\monsters"
$exclude = @("gelatinous_cube", "goblin01")

# Get all monster directories
$monsterDirs = Get-ChildItem -Path $baseDir -Directory | Where-Object { $_.Name -notin $exclude }

$count = 0
$total = $monsterDirs.Count

foreach ($dir in $monsterDirs) {
    $count++
    $monsterName = $dir.Name
    $displayName = $monsterName.Replace('_', ' ')
    
    Write-Progress -Activity "Generating detailed prompts" -Status "Processing $displayName ($count/$total)" -PercentComplete (($count / $total) * 100)
    
    # Monster-specific descriptions and animations will be generated
    $promptFile = Join-Path $dir.FullName "animation_prompts.txt"
    
    # Generate detailed content based on monster type
    $content = Generate-MonsterPrompts -MonsterName $monsterName
    
    Set-Content -Path $promptFile -Value $content -Encoding UTF8
}

Write-Host ""
Write-Host "COMPLETE! Generated detailed prompts for $total monsters!"
Write-Host "Total unique prompts created: $($total * 9)"
