// D&D 5e Character Type Definitions

export type DiceType = 'd4' | 'd6' | 'd8' | 'd10' | 'd12' | 'd20';

export type AbilityType = 'strength' | 'dexterity' | 'constitution' | 'intelligence' | 'wisdom' | 'charisma';

export type SkillType =
    | 'acrobatics' | 'animalHandling' | 'arcana' | 'athletics'
    | 'deception' | 'history' | 'insight' | 'intimidation'
    | 'investigation' | 'medicine' | 'nature' | 'perception'
    | 'performance' | 'persuasion' | 'religion' | 'sleightOfHand'
    | 'stealth' | 'survival';

export type Alignment =
    | 'lawful-good' | 'neutral-good' | 'chaotic-good'
    | 'lawful-neutral' | 'true-neutral' | 'chaotic-neutral'
    | 'lawful-evil' | 'neutral-evil' | 'chaotic-evil';

export type DamageType =
    | 'acid' | 'bludgeoning' | 'cold' | 'fire' | 'force'
    | 'lightning' | 'necrotic' | 'piercing' | 'poison'
    | 'psychic' | 'radiant' | 'slashing' | 'thunder';

export type Condition =
    | 'blinded' | 'charmed' | 'deafened' | 'frightened'
    | 'grappled' | 'incapacitated' | 'invisible' | 'paralyzed'
    | 'petrified' | 'poisoned' | 'prone' | 'restrained'
    | 'stunned' | 'unconscious';

export type SpellSchool =
    | 'abjuration' | 'conjuration' | 'divination' | 'enchantment'
    | 'evocation' | 'illusion' | 'necromancy' | 'transmutation';

export type ItemRarity = 'common' | 'uncommon' | 'rare' | 'very-rare' | 'legendary' | 'artifact';

export type ArmorType = 'light' | 'medium' | 'heavy' | 'shield';

export type WeaponType = 'simple-melee' | 'simple-ranged' | 'martial-melee' | 'martial-ranged';

export type ProficiencyLevel = 'none' | 'proficient' | 'expertise';

// Core Ability Scores
export interface AbilityScores {
    strength: number;
    dexterity: number;
    constitution: number;
    intelligence: number;
    wisdom: number;
    charisma: number;
}

// Saving Throws
export interface SavingThrows {
    strength: ProficiencyLevel;
    dexterity: ProficiencyLevel;
    constitution: ProficiencyLevel;
    intelligence: ProficiencyLevel;
    wisdom: ProficiencyLevel;
    charisma: ProficiencyLevel;
}

// Skills
export interface Skills {
    acrobatics: ProficiencyLevel;
    animalHandling: ProficiencyLevel;
    arcana: ProficiencyLevel;
    athletics: ProficiencyLevel;
    deception: ProficiencyLevel;
    history: ProficiencyLevel;
    insight: ProficiencyLevel;
    intimidation: ProficiencyLevel;
    investigation: ProficiencyLevel;
    medicine: ProficiencyLevel;
    nature: ProficiencyLevel;
    perception: ProficiencyLevel;
    performance: ProficiencyLevel;
    persuasion: ProficiencyLevel;
    religion: ProficiencyLevel;
    sleightOfHand: ProficiencyLevel;
    stealth: ProficiencyLevel;
    survival: ProficiencyLevel;
}

// Skill to Ability mapping
export const SKILL_ABILITIES: Record<SkillType, AbilityType> = {
    acrobatics: 'dexterity',
    animalHandling: 'wisdom',
    arcana: 'intelligence',
    athletics: 'strength',
    deception: 'charisma',
    history: 'intelligence',
    insight: 'wisdom',
    intimidation: 'charisma',
    investigation: 'intelligence',
    medicine: 'wisdom',
    nature: 'intelligence',
    perception: 'wisdom',
    performance: 'charisma',
    persuasion: 'charisma',
    religion: 'intelligence',
    sleightOfHand: 'dexterity',
    stealth: 'dexterity',
    survival: 'wisdom',
};

// Character Class
export interface CharacterClass {
    name: string;
    level: number;
    hitDie: DiceType;
    spellcastingAbility?: AbilityType;
}

// Hit Points
export interface HitPoints {
    current: number;
    max: number;
    temporary: number;
}

// Hit Dice
export interface HitDice {
    current: number;
    total: number;
    dieType: DiceType;
}

// Death Saves
export interface DeathSaves {
    successes: number; // 0-3
    failures: number; // 0-3
}

// Speed
export interface Speed {
    walking: number;
    flying?: number;
    swimming?: number;
    climbing?: number;
    burrowing?: number;
}

// Currency
export interface Currency {
    copper: number;
    silver: number;
    electrum: number;
    gold: number;
    platinum: number;
}

// Inventory Item
export interface InventoryItem {
    id: string;
    name: string;
    description: string;
    quantity: number;
    weight: number; // per item in lbs
    value: number; // in GP
    category: 'weapon' | 'armor' | 'tool' | 'consumable' | 'gear' | 'treasure' | 'magic-item';
    isMagic: boolean;
    requiresAttunement: boolean;
    rarity?: ItemRarity;
    properties?: string[]; // e.g., "Versatile", "Finesse", "Heavy"
}

// Weapon
export interface Weapon extends InventoryItem {
    category: 'weapon';
    weaponType: WeaponType;
    damage: string; // e.g., "1d8"
    damageType: DamageType;
    range?: { normal: number; long?: number }; // in feet
    properties: string[];
}

// Armor
export interface Armor extends InventoryItem {
    category: 'armor';
    armorType: ArmorType;
    baseAC: number;
    maxDexBonus?: number; // undefined = no limit (light armor)
    strengthRequirement?: number;
    stealthDisadvantage: boolean;
}

// Attuned Item
export interface AttunedItem extends InventoryItem {
    attunedBy: string; // character ID
    charges?: {
        current: number;
        max: number;
        recharge: 'dawn' | 'dusk' | 'short-rest' | 'long-rest' | 'never';
    };
}

// Equipment Slots
export interface EquippedItems {
    head?: InventoryItem;
    neck?: InventoryItem;
    chest?: Armor;
    back?: InventoryItem;
    hands?: InventoryItem;
    waist?: InventoryItem;
    legs?: InventoryItem;
    feet?: InventoryItem;
    ring1?: InventoryItem;
    ring2?: InventoryItem;
    mainHand?: Weapon | InventoryItem;
    offHand?: Weapon | Armor | InventoryItem; // Shield or weapon
}

// Inventory
export interface Inventory {
    equipped: EquippedItems;
    backpack: InventoryItem[];
    attuned: AttunedItem[]; // max 3
    currency: Currency;
    carryCapacity: {
        current: number; // in lbs
        max: number; // Strength × 15
    };
}

// Feature/Trait
export interface Feature {
    id: string;
    name: string;
    description: string;
    source: 'race' | 'class' | 'feat' | 'background' | 'other';
    uses?: {
        current: number;
        max: number;
        recharge: 'short-rest' | 'long-rest' | 'dawn' | 'never';
    };
}

// Proficiencies
export interface Proficiencies {
    armor: ArmorType[];
    weapons: string[]; // e.g., "Simple weapons", "Longsword", "Martial weapons"
    tools: string[]; // e.g., "Thieves' Tools", "Smith's Tools"
    languages: string[]; // e.g., "Common", "Elvish", "Draconic"
}

// Spell
export interface Spell {
    id: string;
    name: string;
    level: number; // 0 = cantrip, 1-9 = spell level
    school: SpellSchool;
    castingTime: string; // e.g., "1 action", "1 bonus action", "1 minute"
    range: string; // e.g., "Self", "Touch", "30 feet", "120 feet"
    components: {
        verbal: boolean;
        somatic: boolean;
        material: boolean;
        materialDescription?: string;
        materialCost?: number; // in GP, if material has cost
    };
    duration: string; // e.g., "Instantaneous", "Concentration, up to 1 minute", "8 hours"
    concentration: boolean;
    ritual: boolean;
    description: string;
    atHigherLevels?: string; // Upcasting description
    damage?: {
        base: string; // e.g., "8d6"
        type: DamageType;
        scaling?: string; // e.g., "+1d6 per level"
    };
    savingThrow?: {
        ability: AbilityType;
        effect: string; // e.g., "half damage", "negates"
    };
}

// Spell Slots
export interface SpellSlots {
    level1: { current: number; max: number };
    level2: { current: number; max: number };
    level3: { current: number; max: number };
    level4: { current: number; max: number };
    level5: { current: number; max: number };
    level6: { current: number; max: number };
    level7: { current: number; max: number };
    level8: { current: number; max: number };
    level9: { current: number; max: number };
}

// Spellcasting Info
export interface SpellcastingInfo {
    ability: AbilityType; // 'intelligence', 'wisdom', or 'charisma'
    saveDC: number; // 8 + proficiency + ability modifier
    attackBonus: number; // proficiency + ability modifier
    spellSlots: SpellSlots;
    cantrips: Spell[];
    knownSpells: Spell[];
    preparedSpells: string[]; // spell IDs
    concentratingOn?: {
        spellId: string;
        roundsRemaining: number;
    };
}

// Main Character Interface
export interface Character {
    // Basic Info
    id: string;
    name: string;
    race: string;
    classes: CharacterClass[]; // for multiclassing
    background: string;
    alignment: Alignment;
    experiencePoints: number;
    level: number; // total level (sum of all class levels)

    // Ability Scores
    abilityScores: AbilityScores;

    // Skills & Saves
    skills: Skills;
    savingThrows: SavingThrows;

    // Combat Stats
    armorClass: number;
    initiative: number;
    speed: Speed;
    hitPoints: HitPoints;
    hitDice: HitDice[];
    deathSaves: DeathSaves;

    // Proficiency
    proficiencyBonus: number; // based on level

    // Inventory
    inventory: Inventory;

    // Features & Traits
    features: Feature[];
    proficiencies: Proficiencies;

    // Spellcasting (optional, only for casters)
    spellcasting?: SpellcastingInfo;

    // Other
    inspiration: boolean;
    conditions: Condition[]; // active conditions
    notes?: string;
}

// Helper function to calculate ability modifier
export function getAbilityModifier(score: number): number {
    return Math.floor((score - 10) / 2);
}

// Helper function to calculate proficiency bonus
export function getProficiencyBonus(level: number): number {
    return Math.ceil(level / 4) + 1;
}

// Helper function to calculate skill bonus
export function getSkillBonus(
    character: Character,
    skill: SkillType
): number {
    const ability = SKILL_ABILITIES[skill];
    const abilityMod = getAbilityModifier(character.abilityScores[ability]);
    const proficiency = character.skills[skill];

    let bonus = abilityMod;

    if (proficiency === 'proficient') {
        bonus += character.proficiencyBonus;
    } else if (proficiency === 'expertise') {
        bonus += character.proficiencyBonus * 2;
    }

    return bonus;
}

// Helper function to calculate saving throw bonus
export function getSavingThrowBonus(
    character: Character,
    ability: AbilityType
): number {
    const abilityMod = getAbilityModifier(character.abilityScores[ability]);
    const proficiency = character.savingThrows[ability];

    let bonus = abilityMod;

    if (proficiency === 'proficient') {
        bonus += character.proficiencyBonus;
    } else if (proficiency === 'expertise') {
        bonus += character.proficiencyBonus * 2;
    }

    return bonus;
}

// Helper function to calculate passive perception
export function getPassivePerception(character: Character): number {
    return 10 + getSkillBonus(character, 'perception');
}

// Helper function to calculate spell save DC
export function getSpellSaveDC(character: Character): number {
    if (!character.spellcasting) return 0;

    const abilityMod = getAbilityModifier(
        character.abilityScores[character.spellcasting.ability]
    );

    return 8 + character.proficiencyBonus + abilityMod;
}

// Helper function to calculate spell attack bonus
export function getSpellAttackBonus(character: Character): number {
    if (!character.spellcasting) return 0;

    const abilityMod = getAbilityModifier(
        character.abilityScores[character.spellcasting.ability]
    );

    return character.proficiencyBonus + abilityMod;
}
