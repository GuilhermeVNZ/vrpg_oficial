import { Character, getProficiencyBonus } from '../types/Character';

// Demo D&D 5e Character for testing
export const createDemoCharacter = (): Character => {
    const level = 5;
    const proficiencyBonus = getProficiencyBonus(level);

    return {
        id: 'demo-character-1',
        name: 'Elara Moonwhisper',
        race: 'High Elf',
        classes: [
            {
                name: 'Wizard',
                level: 5,
                hitDie: 'd6',
                spellcastingAbility: 'intelligence',
            },
        ],
        background: 'Sage',
        alignment: 'neutral-good',
        experiencePoints: 6500,
        level: 5,

        // Ability Scores
        abilityScores: {
            strength: 8,
            dexterity: 14,
            constitution: 13,
            intelligence: 18,
            wisdom: 12,
            charisma: 10,
        },

        // Skills
        skills: {
            acrobatics: 'none',
            animalHandling: 'none',
            arcana: 'expertise',
            athletics: 'none',
            deception: 'none',
            history: 'proficient',
            insight: 'proficient',
            intimidation: 'none',
            investigation: 'proficient',
            medicine: 'none',
            nature: 'proficient',
            perception: 'proficient',
            performance: 'none',
            persuasion: 'none',
            religion: 'proficient',
            sleightOfHand: 'none',
            stealth: 'none',
            survival: 'none',
        },

        // Saving Throws
        savingThrows: {
            strength: 'none',
            dexterity: 'none',
            constitution: 'none',
            intelligence: 'proficient',
            wisdom: 'proficient',
            charisma: 'none',
        },

        // Combat Stats
        armorClass: 12, // 10 + 2 (Dex)
        initiative: 2, // Dex modifier
        speed: {
            walking: 30,
        },
        hitPoints: {
            current: 24,
            max: 28, // 6 + 4×5 (avg) + 8 (Con +1 × 5 levels)
            temporary: 0,
        },
        hitDice: [
            {
                current: 3,
                total: 5,
                dieType: 'd6',
            },
        ],
        deathSaves: {
            successes: 0,
            failures: 0,
        },

        proficiencyBonus,

        // Inventory
        inventory: {
            equipped: {
                mainHand: {
                    id: 'quarterstaff-1',
                    name: 'Quarterstaff',
                    description: 'A simple wooden staff',
                    quantity: 1,
                    weight: 4,
                    value: 0.2,
                    category: 'weapon',
                    weaponType: 'simple-melee',
                    damage: '1d6',
                    damageType: 'bludgeoning',
                    isMagic: false,
                    requiresAttunement: false,
                    properties: ['Versatile'],
                },
                chest: {
                    id: 'robes-1',
                    name: 'Robes',
                    description: 'Simple cloth robes',
                    quantity: 1,
                    weight: 4,
                    value: 1,
                    category: 'armor',
                    armorType: 'light',
                    baseAC: 11,
                    stealthDisadvantage: false,
                    isMagic: false,
                    requiresAttunement: false,
                },
            },
            backpack: [
                {
                    id: 'spellbook-1',
                    name: 'Spellbook',
                    description: 'Contains your wizard spells',
                    quantity: 1,
                    weight: 3,
                    value: 50,
                    category: 'gear',
                    isMagic: false,
                    requiresAttunement: false,
                },
                {
                    id: 'component-pouch-1',
                    name: 'Component Pouch',
                    description: 'Contains spell components',
                    quantity: 1,
                    weight: 2,
                    value: 25,
                    category: 'gear',
                    isMagic: false,
                    requiresAttunement: false,
                },
            ],
            attuned: [],
            currency: {
                copper: 50,
                silver: 20,
                electrum: 0,
                gold: 125,
                platinum: 0,
            },
            carryCapacity: {
                current: 13, // Total weight of items
                max: 120, // Str (8) × 15
            },
        },

        // Features
        features: [
            {
                id: 'arcane-recovery',
                name: 'Arcane Recovery',
                description:
                    'Once per day when you finish a short rest, you can recover expended spell slots with a combined level equal to or less than half your wizard level (rounded up).',
                source: 'class',
                uses: {
                    current: 1,
                    max: 1,
                    recharge: 'long-rest',
                },
            },
            {
                id: 'darkvision',
                name: 'Darkvision',
                description: 'You can see in dim light within 60 feet as if it were bright light.',
                source: 'race',
            },
            {
                id: 'fey-ancestry',
                name: 'Fey Ancestry',
                description:
                    'You have advantage on saving throws against being charmed, and magic cannot put you to sleep.',
                source: 'race',
            },
        ],

        proficiencies: {
            armor: [],
            weapons: ['Dagger', 'Dart', 'Sling', 'Quarterstaff', 'Light Crossbow'],
            tools: [],
            languages: ['Common', 'Elvish', 'Draconic', 'Celestial'],
        },

        // Spellcasting
        spellcasting: {
            ability: 'intelligence',
            saveDC: 15, // 8 + 3 (prof) + 4 (Int)
            attackBonus: 7, // 3 (prof) + 4 (Int)
            spellSlots: {
                level1: { current: 4, max: 4 },
                level2: { current: 2, max: 3 },
                level3: { current: 2, max: 2 },
                level4: { current: 0, max: 0 },
                level5: { current: 0, max: 0 },
                level6: { current: 0, max: 0 },
                level7: { current: 0, max: 0 },
                level8: { current: 0, max: 0 },
                level9: { current: 0, max: 0 },
            },
            cantrips: [
                {
                    id: 'fire-bolt',
                    name: 'Fire Bolt',
                    level: 0,
                    school: 'evocation',
                    castingTime: '1 action',
                    range: '120 feet',
                    components: {
                        verbal: true,
                        somatic: true,
                        material: false,
                    },
                    duration: 'Instantaneous',
                    concentration: false,
                    ritual: false,
                    description:
                        'You hurl a mote of fire at a creature or object within range. Make a ranged spell attack. On a hit, the target takes 1d10 fire damage.',
                    damage: {
                        base: '1d10',
                        type: 'fire',
                        scaling: '+1d10 at 5th, 11th, and 17th level',
                    },
                },
                {
                    id: 'mage-hand',
                    name: 'Mage Hand',
                    level: 0,
                    school: 'conjuration',
                    castingTime: '1 action',
                    range: '30 feet',
                    components: {
                        verbal: true,
                        somatic: true,
                        material: false,
                    },
                    duration: '1 minute',
                    concentration: false,
                    ritual: false,
                    description:
                        'A spectral, floating hand appears at a point you choose within range.',
                },
            ],
            knownSpells: [
                {
                    id: 'shield',
                    name: 'Shield',
                    level: 1,
                    school: 'abjuration',
                    castingTime: '1 reaction',
                    range: 'Self',
                    components: {
                        verbal: true,
                        somatic: true,
                        material: false,
                    },
                    duration: '1 round',
                    concentration: false,
                    ritual: false,
                    description:
                        'An invisible barrier of magical force appears and protects you. Until the start of your next turn, you have a +5 bonus to AC.',
                },
                {
                    id: 'magic-missile',
                    name: 'Magic Missile',
                    level: 1,
                    school: 'evocation',
                    castingTime: '1 action',
                    range: '120 feet',
                    components: {
                        verbal: true,
                        somatic: true,
                        material: false,
                    },
                    duration: 'Instantaneous',
                    concentration: false,
                    ritual: false,
                    description:
                        'You create three glowing darts of magical force. Each dart hits a creature of your choice that you can see within range.',
                    damage: {
                        base: '3×(1d4+1)',
                        type: 'force',
                        scaling: '+1 dart per spell level above 1st',
                    },
                    atHigherLevels: 'The spell creates one more dart for each slot level above 1st.',
                },
            ],
            preparedSpells: ['shield', 'magic-missile'],
        },

        inspiration: false,
        conditions: [],
        notes: 'A scholarly wizard seeking ancient knowledge.',
    };
};
