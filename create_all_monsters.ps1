# D&D 5e Monster Manual - LISTA COMPLETA
# Todos os monstros organizados alfabeticamente

$monsters = @(
    "aboleth", "acolyte", "adult_black_dragon", "adult_blue_dragon", "adult_brass_dragon",
    "adult_bronze_dragon", "adult_copper_dragon", "adult_gold_dragon", "adult_green_dragon",
    "adult_red_dragon", "adult_silver_dragon", "adult_white_dragon", "air_elemental",
    "ancient_black_dragon", "ancient_blue_dragon", "ancient_brass_dragon", "ancient_bronze_dragon",
    "ancient_copper_dragon", "ancient_gold_dragon", "ancient_green_dragon", "ancient_red_dragon",
    "ancient_silver_dragon", "ancient_white_dragon", "androsphinx", "animated_armor",
    "ankheg", "ape", "archmage", "assassin", "awakened_shrub", "awakened_tree",
    "axe_beak", "azer", "balor", "bandit", "bandit_captain", "banshee", "barbed_devil",
    "basilisk", "bat", "bearded_devil", "behir", "berserker", "black_bear", "black_dragon_wyrmling",
    "black_pudding", "blink_dog", "blood_hawk", "blue_dragon_wyrmling", "boar", "bone_devil",
    "brass_dragon_wyrmling", "bronze_dragon_wyrmling", "brown_bear", "bugbear", "bulette",
    "camel", "cat", "centaur", "chain_devil", "chimera", "chuul", "clay_golem", "cloaker",
    "cloud_giant", "cockatrice", "commoner", "constrictor_snake", "copper_dragon_wyrmling",
    "couatl", "crab", "crocodile", "cult_fanatic", "cultist", "darkmantle", "death_dog",
    "deep_gnome", "deer", "deva", "dire_wolf", "displacer_beast", "djinni", "doppelganger",
    "draft_horse", "dragon_turtle", "dretch", "drider", "drow", "drow_elite_warrior",
    "drow_mage", "drow_priestess_of_lolth", "druid", "dryad", "duergar", "duodrone",
    "dust_mephit", "eagle", "earth_elemental", "efreeti", "elephant", "elk", "erinyes",
    "ettercap", "ettin", "fire_elemental", "fire_giant", "fire_snake", "flameskull",
    "flesh_golem", "flying_snake", "flying_sword", "frog", "frost_giant", "gargoyle",
    "gas_spore", "gelatinous_cube", "ghast", "ghost", "ghoul", "giant_ape", "giant_badger",
    "giant_bat", "giant_boar", "giant_centipede", "giant_constrictor_snake", "giant_crab",
    "giant_crocodile", "giant_eagle", "giant_elk", "giant_fire_beetle", "giant_frog",
    "giant_goat", "giant_hyena", "giant_lizard", "giant_octopus", "giant_owl", "giant_poisonous_snake",
    "giant_rat", "giant_scorpion", "giant_sea_horse", "giant_shark", "giant_spider",
    "giant_toad", "giant_vulture", "giant_wasp", "giant_weasel", "giant_wolf_spider",
    "gibbering_mouther", "glabrezu", "gladiator", "gnoll", "gnoll_pack_lord", "goat",
    "goblin", "gold_dragon_wyrmling", "gorgon", "gray_ooze", "green_dragon_wyrmling",
    "green_hag", "grick", "grick_alpha", "griffon", "grimlock", "guard", "guardian_naga",
    "gynosphinx", "half_red_dragon_veteran", "harpy", "hawk", "hell_hound", "hezrou",
    "hill_giant", "hippogriff", "hobgoblin", "hobgoblin_captain", "homunculus", "horned_devil",
    "hunter_shark", "hydra", "hyena", "ice_devil", "ice_mephit", "imp", "invisible_stalker",
    "iron_golem", "jackal", "killer_whale", "knight", "kobold", "kraken", "lamia",
    "lemure", "lich", "lion", "lizard", "lizardfolk", "lizardfolk_shaman", "mage",
    "magma_mephit", "magmin", "mammoth", "manticore", "marilith", "mastiff", "medusa",
    "merfolk", "merrow", "mimic", "minotaur", "minotaur_skeleton", "monodrone", "mule",
    "mummy", "mummy_lord", "nalfeshnee", "night_hag", "nightmare", "noble", "nothic",
    "ochre_jelly", "octopus", "ogre", "ogre_zombie", "oni", "orc", "orc_eye_of_gruumsh",
    "orc_war_chief", "otyugh", "owl", "owlbear", "panther", "pegasus", "pentadrone",
    "peryton", "phase_spider", "pit_fiend", "pixie", "planetar", "plesiosaurus",
    "poisonous_snake", "polar_bear", "pony", "priest", "pseudodragon", "purple_worm",
    "quadrone", "quaggoth", "quaggoth_thonot", "quasit", "quipper", "rakshasa", "rat",
    "raven", "red_dragon_wyrmling", "reef_shark", "remorhaz", "rhinoceros", "riding_horse",
    "roc", "roper", "rug_of_smothering", "rust_monster", "saber_toothed_tiger", "sahuagin",
    "sahuagin_baron", "sahuagin_priestess", "salamander", "satyr", "scorpion", "scout",
    "sea_hag", "sea_horse", "shadow", "shadow_demon", "shambling_mound", "shield_guardian",
    "shrieker", "silver_dragon_wyrmling", "skeleton", "slaad_tadpole", "smoke_mephit",
    "solar", "specter", "spider", "spirit_naga", "sprite", "spy", "steam_mephit",
    "stirge", "stone_giant", "stone_golem", "storm_giant", "succubus_incubus", "swarm_of_bats",
    "swarm_of_insects", "swarm_of_poisonous_snakes", "swarm_of_quippers", "swarm_of_rats",
    "swarm_of_ravens", "tarrasque", "thug", "tiger", "treant", "tridrone", "triceratops",
    "troll", "twig_blight", "tyrannosaurus_rex", "umber_hulk", "unicorn", "vampire",
    "vampire_spawn", "veteran", "violet_fungus", "vrock", "vulture", "warhorse",
    "warhorse_skeleton", "water_elemental", "weasel", "werebear", "wereboar", "wererat",
    "weretiger", "werewolf", "white_dragon_wyrmling", "wight", "will_o_wisp", "winged_kobold",
    "wolf", "worg", "wraith", "wyvern", "xorn", "yeti", "young_black_dragon",
    "young_blue_dragon", "young_brass_dragon", "young_bronze_dragon", "young_copper_dragon",
    "young_gold_dragon", "young_green_dragon", "young_red_dragon", "young_remorhaz",
    "young_silver_dragon", "young_white_dragon", "yuan_ti_abomination", "yuan_ti_malison",
    "yuan_ti_pureblood", "zombie"
)

$baseDir = "g:\vrpg\vrpg-client\assets-and-models\sprites\monsters"

Write-Host "Creating folders and prompt files for $($monsters.Count) monsters..."

foreach ($monster in $monsters) {
    $monsterDir = Join-Path $baseDir $monster
    
    # Create directory if it doesn't exist
    if (-not (Test-Path $monsterDir)) {
        New-Item -ItemType Directory -Path $monsterDir -Force | Out-Null
    }
    
    # Create animation prompts file
    $promptFile = Join-Path $monsterDir "animation_prompts.txt"
    
    $content = @"
# $($monster.Replace('_', ' ').ToUpper()) - Animation Prompts (9 Frames)

Base: D&D 5e dark fantasy anime illustration, $($monster.Replace('_', ' ')), top-down view (80-90 degrees), 2D painted style, solid dark grey background NO CHECKERED PATTERN, 1:1 aspect ratio.

Frame 1: Idle rest position
Frame 2: Begin movement/action
Frame 3: Movement building
Frame 4: Peak of action
Frame 5: Hold peak position
Frame 6: Begin return
Frame 7: Returning to rest
Frame 8: Almost at rest
Frame 9: Complete loop (identical to Frame 1)

Generate each frame by combining the base prompt with the specific frame description.
"@
    
    Set-Content -Path $promptFile -Value $content -Encoding UTF8
}

Write-Host "DONE! Created $($monsters.Count) monster folders with animation prompts!"
Write-Host "Total monsters: $($monsters.Count)"
