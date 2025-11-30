// Tooltip Definitions for D&D 5e Terms
// Used by TooltipTerm component for nested tooltips

export interface TooltipDefinition {
    title: string;
    type?: 'condition' | 'mechanic' | 'spell-school' | 'damage-type' | 'property' | 'skill' | 'ability';
    content: string | React.ReactNode;
}

export const TOOLTIP_DEFINITIONS = {
    // ===== CONDITIONS =====
    Blinded: {
        title: 'Blinded',
        type: 'condition',
        content: `• Can't see, auto-fail sight-based Perception
• Attack rolls have Disadvantage
• Attacks against you have Advantage`,
    },

    Charmed: {
        title: 'Charmed',
        type: 'condition',
        content: `• Can't attack the charmer
• Charmer has Advantage on social checks against you`,
    },

    Deafened: {
        title: 'Deafened',
        type: 'condition',
        content: `• Can't hear
• Auto-fail hearing-based Perception checks`,
    },

    Frightened: {
        title: 'Frightened',
        type: 'condition',
        content: `• Disadvantage on ability checks and attack rolls while source is in sight
• Can't willingly move closer to the source of fear`,
    },

    Grappled: {
        title: 'Grappled',
        type: 'condition',
        content: `• Speed becomes 0
• Can't benefit from speed bonuses
• Ends if grappler is Incapacitated
• Ends if effect removes you from grappler's reach`,
    },

    Incapacitated: {
        title: 'Incapacitated',
        type: 'condition',
        content: `• Can't take actions or reactions
• Can still move (unless another condition prevents it)`,
    },

    Invisible: {
        title: 'Invisible',
        type: 'condition',
        content: `• Impossible to see without magic or special sense
• Heavily obscured for hiding purposes
• Attack rolls have Advantage
• Attacks against you have Disadvantage`,
    },

    Paralyzed: {
        title: 'Paralyzed',
        type: 'condition',
        content: `• Incapacitated, can't move or speak
• Auto-fail Strength & Dexterity saves
• Attack rolls against you have Advantage
• Attacks within 5 ft auto-crit if they hit`,
    },

    Petrified: {
        title: 'Petrified',
        type: 'condition',
        content: `• Transformed into solid inanimate substance (usually stone)
• Weight increases by ×10
• Incapacitated, can't move or speak
• Unaware of surroundings
• Auto-fail Str & Dex saves
• Resistance to all damage
• Immune to poison and disease`,
    },

    Poisoned: {
        title: 'Poisoned',
        type: 'condition',
        content: `• Disadvantage on attack rolls
• Disadvantage on ability checks
Does not affect saving throws.`,
    },

    Prone: {
        title: 'Prone',
        type: 'condition',
        content: `• Disadvantage on attack rolls
• Attacks within 5 ft have Advantage
• Ranged attacks beyond 5 ft have Disadvantage
• Costs half movement to stand up`,
    },

    Restrained: {
        title: 'Restrained',
        type: 'condition',
        content: `• Speed becomes 0
• Attack rolls have Disadvantage
• Dex saves have Disadvantage
• Attacks against you have Advantage`,
    },

    Stunned: {
        title: 'Stunned',
        type: 'condition',
        content: `• Incapacitated, can't move
• Can speak only falteringly
• Auto-fail Str & Dex saves
• Attack rolls against you have Advantage`,
    },

    Unconscious: {
        title: 'Unconscious',
        type: 'condition',
        content: `• Incapacitated, can't move or speak
• Unaware of surroundings
• Drop everything held, fall prone
• Auto-fail Str & Dex saves
• Attack rolls against you have Advantage
• Attacks within 5 ft auto-crit if they hit`,
    },

    // ===== CHARACTER STATS =====
    'Armor Class': {
        title: 'Armor Class (AC)',
        type: 'mechanic',
        content: `How hard it is to hit you.
        
Attack Roll must equal or exceed AC to hit.

Calculation:
• Unarmored: 10 + Dex mod
• Light Armor: Armor + Dex mod
• Medium Armor: Armor + Dex mod (max +2)
• Heavy Armor: Armor value only`,
    },

    Initiative: {
        title: 'Initiative',
        type: 'mechanic',
        content: `Determines turn order in combat.
        
Roll: d20 + Dexterity modifier

Rolled at the start of combat. Highest goes first.`,
    },

    Speed: {
        title: 'Speed',
        type: 'mechanic',
        content: `Distance you can move in one round (6 seconds).
        
• Can be broken up before/after action
• Difficult terrain costs 2 ft per 1 ft
• Climbing/Swimming costs 2 ft per 1 ft (unless you have that speed)`,
    },

    'Hit Points': {
        title: 'Hit Points (HP)',
        type: 'mechanic',
        content: `Measure of health and durability.
        
• Current HP: How much damage you can take before falling unconscious
• Max HP: Your total health pool
• 0 HP: Unconscious and dying (make Death Saves)`,
    },

    'Temporary Hit Points': {
        title: 'Temporary Hit Points',
        type: 'mechanic',
        content: `Buffer against damage.
        
• Lost first before real HP
• Do NOT stack (take higher value)
• Cannot be healed
• Disappear after a Long Rest`,
    },

    'Hit Dice': {
        title: 'Hit Dice',
        type: 'mechanic',
        content: `Used to heal during Short Rests.
        
• Total = Character Level
• Die type depends on Class (d6-d12)
• Regain half of total Hit Dice on Long Rest`,
    },

    Inspiration: {
        title: 'Inspiration',
        type: 'mechanic',
        content: `Awarded by DM for good roleplay or heroism.
        
Spend to gain Advantage on:
• Attack roll
• Saving throw
• Ability check

You can only have 1 Inspiration at a time.`,
    },

    'Passive Perception': {
        title: 'Passive Perception',
        type: 'mechanic',
        content: `Your constant awareness of surroundings.
        
Calculation: 10 + Perception modifier
(+5 if you have Advantage, -5 if Disadvantage)

Used by DM to determine if you notice hidden threats without rolling.`,
    },

    'Proficiency Bonus': {
        title: 'Proficiency Bonus',
        type: 'mechanic',
        content: `Bonus added to rolls you are proficient in.
        
Applies to:
• Attack rolls (weapons/spells)
• Skill checks
• Saving throws
• DC for your spells/abilities

Increases with total character level (+2 to +6).`,
    },

    // ===== MECHANICS =====
    Advantage: {
        title: 'Advantage',
        type: 'mechanic',
        content: `Roll 2d20, use the HIGHER result.

Advantage cancels Disadvantage (and vice-versa).
Multiple sources don't stack.

Probability: ~75% to roll 11+ (vs 50% normal)`,
    },

    Disadvantage: {
        title: 'Disadvantage',
        type: 'mechanic',
        content: `Roll 2d20, use the LOWER result.

Disadvantage cancels Advantage (and vice-versa).
Multiple sources don't stack.

Probability: ~25% to roll 11+ (vs 50% normal)`,
    },

    Concentration: {
        title: 'Concentration',
        type: 'mechanic',
        content: `Only 1 concentration spell active at a time.

You lose concentration if:
• Take damage: Con save DC 10 or half damage (whichever is higher)
• Cast another concentration spell
• Incapacitated or killed
• Environmental factors (DM discretion)

Improve with: War Caster feat, Resilient (Con)`,
    },

    Proficiency: {
        title: 'Proficiency',
        type: 'mechanic',
        content: `Add proficiency bonus to rolls you're proficient in.

Based on total character level:
Level 1-4: +2
Level 5-8: +3
Level 9-12: +4
Level 13-16: +5
Level 17-20: +6`,
    },

    Expertise: {
        title: 'Expertise',
        type: 'mechanic',
        content: `Double proficiency bonus for specific skills.

Available to: Rogues, Bards, some feats

Example: Level 5 Rogue
Normal proficiency: +3
Expertise: +6`,
    },

    'Critical Hit': {
        title: 'Critical Hit',
        type: 'mechanic',
        content: `Natural 20 on attack roll = Critical Hit

Effect:
• Roll ALL damage dice TWICE
• Add modifiers normally (only once)

Example: Longsword (1d8+3)
Normal: 1d8+3 = 7 avg
Crit: 2d8+3 = 14 avg

Spell attacks can crit!
Saving throw spells cannot.`,
    },

    'Opportunity Attack': {
        title: 'Opportunity Attack',
        type: 'mechanic',
        content: `Reaction when hostile creature you can see moves out of your reach.

Make one melee attack against the provoking creature.

Prevented by:
• Disengage action
• Teleportation
• Being moved without using movement`,
    },

    Reaction: {
        title: 'Reaction',
        type: 'mechanic',
        content: `Special action taken in response to a trigger.
1 reaction per round (resets on your turn).

Common reactions:
• Opportunity Attack
• Shield spell
• Counterspell
• Readied action`,
    },

    'Bonus Action': {
        title: 'Bonus Action',
        type: 'mechanic',
        content: `Special action that takes less time than a full action.

You can only take 1 bonus action per turn.
Only available if a feature/spell grants it.

Common bonus actions:
• Two-Weapon Fighting (offhand attack)
• Cunning Action (Rogue)
• Healing Word spell
• Misty Step spell`,
    },

    Action: {
        title: 'Action',
        type: 'mechanic',
        content: `Main activity on your turn.

Common actions:
• Attack (weapon attack, or Extra Attack)
• Cast a Spell (if casting time = 1 action)
• Dash (move again, up to your speed)
• Disengage (move without opportunity attacks)
• Dodge (attacks have Disadvantage)
• Help (give ally Advantage)
• Hide (Stealth check)
• Ready (prepare action for trigger)
• Search (Perception/Investigation)
• Use an Object`,
    },

    'Short Rest': {
        title: 'Short Rest',
        type: 'mechanic',
        content: `Minimum 1 hour of light activity.

Benefits:
• Spend Hit Dice to recover HP
• Recover features that recharge on short rest
  (Fighter: Action Surge, Warlock: Spell Slots)
• Attune to magic items

No limit per day.
Interrupted by combat = no benefit.`,
    },

    'Long Rest': {
        title: 'Long Rest',
        type: 'mechanic',
        content: `Minimum 8 hours (6 sleep + 2 light activity).

Benefits:
• HP: Restore to maximum
• Hit Dice: Regain half (round up)
• Spell Slots: ALL restored
• Class Features: ALL restored
• Prepared Spells: Can change

Limit: 1 per 24 hours
Interrupted by 1+ hours combat = canceled`,
    },

    'Death Saves': {
        title: 'Death Saves',
        type: 'mechanic',
        content: `When at 0 HP, roll d20 each turn:
• 10+: Success (need 3 to stabilize)
• 9 or less: Failure (3 = death)

Taking damage while down = 1 failure
Critical hit while down = 2 failures
Natural 20 = regain 1 HP

Stabilized: No more saves, but still unconscious at 0 HP`,
    },

    Attunement: {
        title: 'Attunement',
        type: 'mechanic',
        content: `Some magic items require attunement.

Process:
• Spend 1 Short Rest (1 hour) focusing on item

Limit: Maximum 3 items simultaneously

Break attunement:
• Spend 1 Short Rest
• 24+ hours more than 100 ft from item
• You die`,
    },

    Upcasting: {
        title: 'Upcasting',
        type: 'mechanic',
        content: `Cast a spell using a higher-level slot.

Effect varies by spell (check "At Higher Levels").

Examples:
• Cure Wounds: +1d8 per level
• Magic Missile: +1 dart per level
• Fireball: +1d6 per level

Not all spells can be upcast.`,
    },

    'Ritual Casting': {
        title: 'Ritual Casting',
        type: 'mechanic',
        content: `Cast a spell as a ritual:
• +10 minutes to casting time
• Does NOT consume spell slot

Requirements:
• Spell has "ritual" tag
• Wizard: Any ritual spell in spellbook
• Cleric/Druid: Only prepared ritual spells

Common ritual spells:
Detect Magic, Identify, Find Familiar, Alarm`,
    },

    // ===== SPELL SCHOOLS =====
    Abjuration: {
        title: 'Abjuration',
        type: 'spell-school',
        content: `Protective magic, wards, and barriers.

Common spells:
• Shield, Counterspell, Dispel Magic
• Mage Armor, Protection from Evil
• Banishment, Globe of Invulnerability

Wizard subclass: Arcane Ward, Spell Resistance`,
    },

    Conjuration: {
        title: 'Conjuration',
        type: 'spell-school',
        content: `Summoning creatures and objects, teleportation.

Common spells:
• Misty Step, Dimension Door
• Summon Beast, Conjure Animals
• Find Familiar, Unseen Servant

Wizard subclass: Minor Conjuration, Benign Transposition`,
    },

    Divination: {
        title: 'Divination',
        type: 'spell-school',
        content: `Revealing information, seeing the future.

Common spells:
• Detect Magic, Identify, Scrying
• Clairvoyance, Arcane Eye
• Foresight, True Seeing

Wizard subclass: Portent, Expert Divination`,
    },

    Enchantment: {
        title: 'Enchantment',
        type: 'spell-school',
        content: `Charm and mind control magic.

Common spells:
• Charm Person, Suggestion
• Hold Person, Dominate Person
• Sleep, Hypnotic Pattern

Wizard subclass: Hypnotic Gaze, Instinctive Charm`,
    },

    Evocation: {
        title: 'Evocation',
        type: 'spell-school',
        content: `Manipulating energy for damage.

Common spells:
• Fireball, Lightning Bolt, Magic Missile
• Scorching Ray, Cone of Cold
• Wall of Fire, Chain Lightning

Wizard subclass: Sculpt Spells, Empowered Evocation`,
    },

    Illusion: {
        title: 'Illusion',
        type: 'spell-school',
        content: `Creating false images and sounds.

Common spells:
• Invisibility, Mirror Image
• Major Image, Phantasmal Force
• Mislead, Programmed Illusion

Wizard subclass: Improved Illusions, Illusory Self`,
    },

    Necromancy: {
        title: 'Necromancy',
        type: 'spell-school',
        content: `Life, death, and undead magic.

Common spells:
• Animate Dead, Raise Dead
• Blight, Vampiric Touch
• Revivify, Resurrection

Wizard subclass: Grim Harvest, Undead Thralls`,
    },

    Transmutation: {
        title: 'Transmutation',
        type: 'spell-school',
        content: `Transforming creatures and objects.

Common spells:
• Polymorph, Haste, Slow
• Enlarge/Reduce, Alter Self
• Stone Shape, Flesh to Stone

Wizard subclass: Minor Alchemy, Transmuter's Stone`,
    },

    // ===== DAMAGE TYPES =====
    Acid: {
        title: 'Acid',
        type: 'damage-type',
        content: `Corrosive damage from acids and digestive enzymes.

Common sources:
• Black dragon breath
• Acid Arrow spell
• Oozes and slimes`,
    },

    Bludgeoning: {
        title: 'Bludgeoning',
        type: 'damage-type',
        content: `Blunt force trauma.

Common sources:
• Clubs, maces, hammers
• Falling damage
• Unarmed strikes`,
    },

    Cold: {
        title: 'Cold',
        type: 'damage-type',
        content: `Freezing damage.

Common sources:
• White dragon breath
• Cone of Cold spell
• Ice elementals`,
    },

    Fire: {
        title: 'Fire',
        type: 'damage-type',
        content: `Burning damage.

Common sources:
• Red dragon breath
• Fireball spell
• Fire elementals`,
    },

    Force: {
        title: 'Force',
        type: 'damage-type',
        content: `Pure magical energy.

Rarely resisted.

Common sources:
• Magic Missile
• Eldritch Blast
• Wall of Force`,
    },

    Lightning: {
        title: 'Lightning',
        type: 'damage-type',
        content: `Electrical damage.

Common sources:
• Blue dragon breath
• Lightning Bolt spell
• Call Lightning`,
    },

    Necrotic: {
        title: 'Necrotic',
        type: 'damage-type',
        content: `Life-draining dark energy.

Common sources:
• Undead attacks
• Blight spell
• Vampiric Touch`,
    },

    Piercing: {
        title: 'Piercing',
        type: 'damage-type',
        content: `Puncture wounds.

Common sources:
• Arrows, bolts, spears
• Rapiers, daggers
• Bite attacks`,
    },

    Poison: {
        title: 'Poison',
        type: 'damage-type',
        content: `Toxic damage.

Many creatures are resistant or immune.

Common sources:
• Poisoned weapons
• Poison Spray cantrip
• Venomous creatures`,
    },

    Psychic: {
        title: 'Psychic',
        type: 'damage-type',
        content: `Mental damage.

Common sources:
• Mind Blast
• Psychic Scream
• Mind Flayer attacks`,
    },

    Radiant: {
        title: 'Radiant',
        type: 'damage-type',
        content: `Holy/divine light energy.

Effective against undead.

Common sources:
• Sacred Flame
• Guiding Bolt
• Celestial attacks`,
    },

    Slashing: {
        title: 'Slashing',
        type: 'damage-type',
        content: `Cutting damage.

Common sources:
• Swords, axes, claws
• Longswords, greatswords
• Slash attacks`,
    },

    Thunder: {
        title: 'Thunder',
        type: 'damage-type',
        content: `Concussive sound damage.

Common sources:
• Thunderwave spell
• Shatter spell
• Thunder damage weapons`,
    },

    // ===== WEAPON PROPERTIES =====
    Versatile: {
        title: 'Versatile',
        type: 'property',
        content: `Can be wielded with 1 or 2 hands.

1-hand: Listed damage
2-hands: Increased damage (shown in parentheses)

Example: Longsword
1-hand: 1d8
2-hands: 1d10

Your other hand must be free to use 2-hands.`,
    },

    Finesse: {
        title: 'Finesse',
        type: 'property',
        content: `Use Str OR Dex for attack/damage (your choice).

Best for: Dex-based fighters, rogues
Allows: Sneak Attack damage (requires finesse)

Examples: Rapier, Dagger, Shortsword`,
    },

    Reach: {
        title: 'Reach',
        type: 'property',
        content: `+5 ft reach (10 ft total instead of 5 ft).

You can attack enemies 10 ft away.
Opportunity attacks trigger at 10 ft.

Examples: Glaive, Halberd, Whip`,
    },

    Heavy: {
        title: 'Heavy',
        type: 'property',
        content: `Small creatures have Disadvantage on attack rolls.

Examples: Greatsword, Greataxe, Maul`,
    },

    Light: {
        title: 'Light',
        type: 'property',
        content: `Allows Two-Weapon Fighting.

When you attack with a light weapon, you can use
a bonus action to attack with a different light
weapon in your other hand.

Examples: Shortsword, Dagger, Handaxe`,
    },

    Loading: {
        title: 'Loading',
        type: 'property',
        content: `Can only fire 1 attack per turn.

Even with Extra Attack, you can only attack once
with this weapon per turn.

Crossbow Expert feat removes this limitation.

Examples: Crossbow (Hand, Light, Heavy)`,
    },

    Thrown: {
        title: 'Thrown',
        type: 'property',
        content: `Requires both hands to use.

Cannot use a shield while wielding.

Examples: Greatsword, Greataxe, Longbow`,
    },

    // ===== INVENTORY & ECONOMY =====
    Weight: {
        title: 'Carrying Capacity',
        type: 'mechanic',
        content: `Your Strength score determines how much you can carry.

Carrying Capacity = Strength Score × 15 lbs.

If you carry more than this, your speed drops to 5 ft.`,
    },

    Encumbrance: {
        title: 'Encumbrance',
        type: 'mechanic',
        content: `Variant Rule: Encumbrance

• Encumbered: Weight > Strength × 5
  (Speed drops by 10 ft)

• Heavily Encumbered: Weight > Strength × 10
  (Speed drops by 20 ft, Disadvantage on Str/Dex/Con checks)

• Maximum: Weight > Strength × 15
  (Speed drops to 5 ft)`,
    },

    Currency: {
        title: 'Currency',
        type: 'mechanic',
        content: `Standard exchange rates:

1 pp (Platinum) = 10 gp
1 gp (Gold) = 10 ep
1 ep (Electrum) = 2 sp
1 sp (Silver) = 10 cp
1 cp (Copper)

1 gp = 100 cp`,
    },

    cp: {
        title: 'Copper Piece (cp)',
        type: 'mechanic',
        content: `The lowest denomination.
Common laborers earn about 1 sp (10 cp) per day.
A candle costs 1 cp.`,
    },

    sp: {
        title: 'Silver Piece (sp)',
        type: 'mechanic',
        content: `Standard trade coin.
1 sp = 10 cp.
A flask of oil costs 1 sp.`,
    },

    ep: {
        title: 'Electrum Piece (ep)',
        type: 'mechanic',
        content: `Old or foreign currency, rarely used.
1 ep = 5 sp = 50 cp.
Often found in ancient dungeons.`,
    },

    gp: {
        title: 'Gold Piece (gp)',
        type: 'mechanic',
        content: `Standard adventuring currency.
1 gp = 10 sp = 100 cp.
A longsword costs 15 gp.`,
    },

    pp: {
        title: 'Platinum Piece (pp)',
        type: 'mechanic',
        content: `High value currency.
1 pp = 10 gp = 100 sp.
Used for large transactions or magic items.`,
    },

    Property: {
        title: 'Landholdings & Property',
        type: 'mechanic',
        content: `Real estate owned by the character.

Properties can generate income, provide a base of operations, or require maintenance costs.
See DMG p. 126 for recurring expenses.`,
    },

    Companion: {
        title: 'Companions & Mounts',
        type: 'mechanic',
        content: `Creatures that accompany the character.

• Mounts: Horses, griffons, etc.
• Familiars: Magical spirits (Find Familiar)
• Pets: Mundane animals
• Hirelings: Paid NPCs

If a companion engages in combat, it shares your initiative count but takes its turn immediately after yours.`,
    },

    // ===== ARMOR TYPES =====
    'Light Armor': {
        title: 'Light Armor',
        type: 'property',
        content: `AC = Armor + Full Dex modifier

Types:
• Padded: AC 11 + Dex
• Leather: AC 11 + Dex
• Studded Leather: AC 12 + Dex

No Strength requirement.
No Stealth disadvantage (except Padded).`,
    },

    'Medium Armor': {
        title: 'Medium Armor',
        type: 'property',
        content: `AC = Armor + Dex (max +2)

Types:
• Hide: AC 12 + Dex (max +2)
• Chain Shirt: AC 13 + Dex (max +2)
• Scale Mail: AC 14 + Dex (max +2)
• Breastplate: AC 14 + Dex (max +2)
• Half Plate: AC 15 + Dex (max +2)

Some have Stealth disadvantage.`,
    },

    'Heavy Armor': {
        title: 'Heavy Armor',
        type: 'property',
        content: `AC = Armor only (no Dex bonus)

Types:
• Ring Mail: AC 14
• Chain Mail: AC 16 (Str 13 req)
• Splint: AC 17 (Str 15 req)
• Plate: AC 18 (Str 15 req)

Strength requirement if listed.
All have Stealth disadvantage.`,
    },

    Shield: {
        title: 'Shield',
        type: 'property',
        content: `+2 AC

Requires one free hand.
Cannot use with two-handed weapons.

Can be donned/doffed as an action.`,
    },

    // ===== SKILLS =====
    Acrobatics: {
        title: 'Acrobatics (Dex)',
        type: 'skill',
        content: `Your Dexterity (Acrobatics) check covers your attempt to stay on your feet in a tricky situation, such as when you're trying to run across a sheet of ice, balance on a tightrope, or stay upright on a rocking ship. The DM might also call for a Dexterity (Acrobatics) check to see if you can perform acrobatic stunts, including dives, rolls, somersaults, and flips.`,
    },
    'Animal Handling': {
        title: 'Animal Handling (Wis)',
        type: 'skill',
        content: `When there is any question whether you can calm down a domesticated animal, keep a mount from getting spooked, or intuit an animal's intentions, the DM might call for a Wisdom (Animal Handling) check. You also make a Wisdom (Animal Handling) check to control your mount when you attempt a risky maneuver.`,
    },
    Arcana: {
        title: 'Arcana (Int)',
        type: 'skill',
        content: `Your Intelligence (Arcana) check measures your ability to recall lore about spells, magic items, eldritch symbols, magical traditions, the planes of existence, and the inhabitants of those planes.`,
    },
    Athletics: {
        title: 'Athletics (Str)',
        type: 'skill',
        content: `Your Strength (Athletics) check covers difficult situations you encounter while climbing, jumping, or swimming. Examples include attempting to climb a sheer or slippery cliff, avoid hazards while scaling a wall, or cling to a surface while something is trying to knock you off.`,
    },
    Deception: {
        title: 'Deception (Cha)',
        type: 'skill',
        content: `Your Charisma (Deception) check determines whether you can convincingly hide the truth, either verbally or through your actions. This deception can encompass everything from misleading others through ambiguity to telling outright lies. Typical situations include trying to fast-talk a guard, con a merchant, earn money through gambling, pass yourself off in a disguise, dull someone's suspicions with false assurances, or maintain a straight face while telling a blatant lie.`,
    },
    History: {
        title: 'History (Int)',
        type: 'skill',
        content: `Your Intelligence (History) check measures your ability to recall lore about historical events, legendary people, ancient kingdoms, past disputes, recent wars, and lost civilizations.`,
    },
    Insight: {
        title: 'Insight (Wis)',
        type: 'skill',
        content: `Your Wisdom (Insight) check decides whether you can determine the true intentions of a creature, such as when searching out a lie or predicting someone's next move. Doing so involves gleaning clues from body language, speech habits, and changes in mannerisms.`,
    },
    Intimidation: {
        title: 'Intimidation (Cha)',
        type: 'skill',
        content: `When you attempt to influence someone through overt threats, hostile actions, and physical violence, the DM might ask you to make a Charisma (Intimidation) check. Examples include trying to pry information out of a prisoner, convincing street thugs to back down from a confrontation, or using the edge of a broken bottle to convince a sneering vizier to reconsider a decision.`,
    },
    Investigation: {
        title: 'Investigation (Int)',
        type: 'skill',
        content: `When you look around for clues and make deductions based on those clues, you make an Intelligence (Investigation) check. You might deduce the location of a hidden object, discern from the appearance of a wound what kind of weapon dealt it, or determine the weakest point in a tunnel that could cause it to collapse. Poring through ancient scrolls in search of a hidden fragment of knowledge might also call for an Intelligence (Investigation) check.`,
    },
    Medicine: {
        title: 'Medicine (Wis)',
        type: 'skill',
        content: `A Wisdom (Medicine) check lets you try to stabilize a dying companion or diagnose an illness.`,
    },
    Nature: {
        title: 'Nature (Int)',
        type: 'skill',
        content: `Your Intelligence (Nature) check measures your ability to recall lore about terrain, plants and animals, the weather, and natural cycles.`,
    },
    Perception: {
        title: 'Perception (Wis)',
        type: 'skill',
        content: `Your Wisdom (Perception) check lets you spot, hear, or otherwise detect the presence of something. It measures your general awareness of your surroundings and the keenness of your senses. For example, you might try to hear a conversation through a closed door, eavesdrop under an open window, or hear monsters moving stealthily in the forest. Or you might try to spot things that are obscured or easy to miss, whether they are orcs lying in ambush on a road, thugs hiding in the shadows of an alley, or candlelight under a closed secret door.`,
    },
    Performance: {
        title: 'Performance (Cha)',
        type: 'skill',
        content: `Your Charisma (Performance) check determines how well you can delight an audience with music, dance, acting, storytelling, or some other form of entertainment.`,
    },
    Persuasion: {
        title: 'Persuasion (Cha)',
        type: 'skill',
        content: `When you attempt to influence someone or a group of people with tact, social graces, or good nature, the DM might ask you to make a Charisma (Persuasion) check. Typically, you use persuasion when acting in good faith, to foster friendships, make cordial requests, or exhibit proper etiquette. Examples of persuading others include convincing a chamberlain to let your party see the king, negotiating peace between warring tribes, or inspiring a crowd of townsfolk.`,
    },
    Religion: {
        title: 'Religion (Int)',
        type: 'skill',
        content: `Your Intelligence (Religion) check measures your ability to recall lore about deities, rites and prayers, religious hierarchies, holy symbols, and the practices of secret cults.`,
    },
    'Sleight of Hand': {
        title: 'Sleight of Hand (Dex)',
        type: 'skill',
        content: `Whenever you attempt an act of legerdemain or manual trickery, such as planting something on someone else or concealing an object on your person, make a Dexterity (Sleight of Hand) check. The DM might also call for a Dexterity (Sleight of Hand) check to determine whether you can lift a coin purse off another person or slip something out of another person's pocket.`,
    },
    Stealth: {
        title: 'Stealth (Dex)',
        type: 'skill',
        content: `Make a Dexterity (Stealth) check when you attempt to conceal yourself from enemies, slink past guards, slip away without being noticed, or sneak up on someone without being seen or heard.`,
    },
    Survival: {
        title: 'Survival (Wis)',
        type: 'skill',
        content: `The DM might ask you to make a Wisdom (Survival) check to follow tracks, hunt wild game, guide your group through frozen wastelands, identify signs that owlbears live nearby, predict the weather, or avoid quicksand and other natural hazards.`,
    },

    // ===== BACKGROUNDS =====
    Sage: {
        title: 'Sage',
        type: 'property',
        content: `You spent years learning the lore of the multiverse. You scoured manuscripts, studied scrolls, and listened to the greatest experts on the subjects that interest you. Your efforts have made you a master in your fields of study.

Skill Proficiencies: Arcana, History
Languages: Two of your choice
Equipment: A bottle of black ink, a quill, a small knife, a letter from a dead colleague posing a question you have not yet been able to answer, a set of common clothes, and a pouch containing 10 gp

Feature: Researcher
When you attempt to learn or recall a piece of lore, if you do not know that information, you often know where and from whom you can obtain it. Usually, this information comes from a library, scriptorium, university, or a sage or other learned person or creature.`,
    },

    // ===== CLASSES =====
    Wizard: {
        title: 'Wizard',
        type: 'property',
        content: `A scholarly magic-user capable of manipulating the structures of reality.

Hit Die: d6
Primary Ability: Intelligence
Saves: Intelligence, Wisdom

Class Features:
• Spellcasting: You have a spellbook containing spells that show the first glimmerings of your true power.
• Arcane Recovery: You have learned to regain some of your magical energy by studying your spellbook.
• Arcane Tradition: You choose an arcane tradition, shaping your practice of magic through one of eight schools.`,
    },

    // ===== RACES =====
    'High Elf': {
        title: 'High Elf',
        type: 'property',
        content: `Elves are a magical people of otherworldly grace, living in the world but not entirely part of it.

Traits:
• Ability Score Increase: Dex +2, Int +1
• Age: Adulthood at 100, live to 750
• Size: Medium
• Speed: 30 ft.
• Darkvision: 60 ft.
• Keen Senses: Proficiency in Perception
• Fey Ancestry: Advantage vs charm, immune to sleep magic
• Trance: Meditate for 4 hours instead of sleep
• Elf Weapon Training: Proficiency with longsword, shortsword, shortbow, longbow
• Cantrip: Know one cantrip from wizard list (Int is spellcasting ability)
• Extra Language: Speak, read, write one extra language`,
    },

} as const;

export type TooltipDefinitionKey = keyof typeof TOOLTIP_DEFINITIONS;
