import React, { useState } from 'react';
import { useCharacter } from '../../context/CharacterContext';
import { StatBlock } from '../character/StatBlock';
import { SkillRow } from '../character/SkillRow';
import { TooltipTerm } from '../common/TooltipTerm';
import {
    getAbilityModifier,
    getSkillBonus,
    getSavingThrowBonus,
    getPassivePerception,
    AbilityType,
    SkillType,
    ProficiencyLevel,
    Weapon,
    Character,
} from '../../types/Character';
import { TooltipDefinitionKey } from '../../data/tooltipDefinitions';
import './CharacterSheetModal.css';

interface CharacterSheetModalProps {
    isOpen: boolean;
    onClose: () => void;
}

type CharacterTab = 'information' | 'skills' | 'biography';

const CharacterSheetModal: React.FC<CharacterSheetModalProps> = ({
    isOpen,
    onClose,
}) => {
    const { character, updateCharacter } = useCharacter();
    const [activeTab, setActiveTab] = useState<CharacterTab>('information');

    if (!isOpen || !character) return null;

    const abilities: AbilityType[] = [
        'strength',
        'dexterity',
        'constitution',
        'intelligence',
        'wisdom',
        'charisma',
    ];

    const skills: SkillType[] = [
        'acrobatics',
        'animalHandling',
        'arcana',
        'athletics',
        'deception',
        'history',
        'insight',
        'intimidation',
        'investigation',
        'medicine',
        'nature',
        'perception',
        'performance',
        'persuasion',
        'religion',
        'sleightOfHand',
        'stealth',
        'survival',
    ];

    const handleAbilityScoreChange = (ability: AbilityType, newScore: number) => {
        updateCharacter({
            abilityScores: {
                ...character.abilityScores,
                [ability]: newScore,
            },
        });
    };

    const handleSkillProficiencyChange = (
        skill: SkillType,
        newProficiency: ProficiencyLevel
    ) => {
        updateCharacter({
            skills: {
                ...character.skills,
                [skill]: newProficiency,
            },
        });
    };

    const handleHPChange = (field: 'current' | 'max' | 'temporary', value: number) => {
        updateCharacter({
            hitPoints: {
                ...character.hitPoints,
                [field]: value,
            },
        });
    };

    const handleDeathSaveChange = (type: 'successes' | 'failures', value: number) => {
        updateCharacter({
            deathSaves: {
                ...character.deathSaves,
                [type]: Math.max(0, Math.min(3, value)),
            },
        });
    };

    const passivePerception = getPassivePerception(character);

    return (
        <div className="modal-overlay" onClick={onClose}>
            <div
                className="character-sheet-modal"
                onClick={(e) => e.stopPropagation()}
                style={{
                    background: 'rgba(20, 20, 20, 0.6)',
                    backdropFilter: 'blur(24px)',
                    padding: '24px',
                    borderRadius: '24px',
                    border: 'none',
                    boxShadow: '0 20px 50px rgba(0,0,0,0.3)',
                    maxWidth: '1000px',
                    width: '100%',
                    height: '70vh',
                    display: 'flex',
                    flexDirection: 'column',
                }}
            >
                {/* Close Button */}
                <button
                    onClick={onClose}
                    style={{
                        position: 'absolute',
                        top: '16px',
                        right: '16px',
                        background: 'rgba(255, 255, 255, 0.1)',
                        border: 'none',
                        borderRadius: '50%',
                        width: '28px',
                        height: '28px',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        cursor: 'pointer',
                        color: 'rgba(255, 255, 255, 0.6)',
                        fontSize: '14px',
                        transition: 'all 0.2s ease',
                        zIndex: 10,
                    }}
                    onMouseEnter={(e) => {
                        e.currentTarget.style.background = 'rgba(255, 255, 255, 0.2)';
                        e.currentTarget.style.color = '#FFF';
                    }}
                    onMouseLeave={(e) => {
                        e.currentTarget.style.background = 'rgba(255, 255, 255, 0.1)';
                        e.currentTarget.style.color = 'rgba(255, 255, 255, 0.6)';
                    }}
                >
                    ✕
                </button>

                {/* Header */}
                <div style={{ marginBottom: '16px' }}>
                    <input
                        type="text"
                        value={character.name}
                        onChange={(e) => updateCharacter({ name: e.target.value })}
                        style={{
                            fontSize: '28px',
                            fontWeight: 'bold',
                            margin: '0 0 4px 0',
                            color: '#D4AF37',
                            fontFamily: "'Crimson Text', serif",
                            letterSpacing: '-0.5px',
                            background: 'transparent',
                            border: 'none',
                            borderBottom: '2px solid transparent',
                            padding: '0 0 4px 0',
                            width: '100%',
                            transition: 'border-color 0.3s ease',
                        }}
                        onFocus={(e) => (e.currentTarget.style.borderBottomColor = '#D4AF37')}
                        onBlur={(e) => (e.currentTarget.style.borderBottomColor = 'transparent')}
                    />
                    <div
                        style={{
                            fontSize: '13px',
                            color: 'rgba(255,255,255,0.7)',
                            lineHeight: '1.4',
                        }}
                    >
                        Level {character.level} • {character.classes.map((c, i) => (
                            <span key={i}>
                                {i > 0 && '/'}
                                <TooltipTerm term={c.name as any}>{c.name}</TooltipTerm>
                            </span>
                        ))} • <TooltipTerm term={character.race as any}>{character.race}</TooltipTerm>
                    </div>
                </div>

                {/* Tab Navigation */}
                <div
                    style={{
                        display: 'flex',
                        gap: '8px',
                        marginBottom: '16px',
                        borderBottom: '1px solid rgba(255,255,255,0.1)',
                        paddingBottom: '8px',
                    }}
                >
                    {[
                        { id: 'information', label: 'Information' },
                        { id: 'skills', label: 'Skills' },
                        { id: 'biography', label: 'Biography' },
                    ].map((tab) => (
                        <button
                            key={tab.id}
                            onClick={() => setActiveTab(tab.id as CharacterTab)}
                            style={{
                                padding: '8px 16px',
                                background: activeTab === tab.id ? 'rgba(212, 175, 55, 0.2)' : 'rgba(255,255,255,0.05)',
                                border: activeTab === tab.id ? '1px solid #D4AF37' : '1px solid rgba(255,255,255,0.1)',
                                borderRadius: '8px',
                                color: activeTab === tab.id ? '#D4AF37' : 'rgba(255,255,255,0.7)',
                                fontSize: '14px',
                                fontWeight: activeTab === tab.id ? 600 : 400,
                                cursor: 'pointer',
                                transition: 'all 0.2s ease',
                            }}
                            onMouseEnter={(e) => {
                                if (activeTab !== tab.id) {
                                    e.currentTarget.style.background = 'rgba(255,255,255,0.08)';
                                }
                            }}
                            onMouseLeave={(e) => {
                                if (activeTab !== tab.id) {
                                    e.currentTarget.style.background = 'rgba(255,255,255,0.05)';
                                }
                            }}
                        >
                            {tab.label}
                        </button>
                    ))}
                </div>

                {/* Tab Content - No Scroll */}
                <div style={{ flex: 1, overflow: 'hidden' }}>
                    {activeTab === 'information' && (
                        <InformationTab
                            character={character}
                            abilities={abilities}
                            handleAbilityScoreChange={handleAbilityScoreChange}
                            handleHPChange={handleHPChange}
                            handleDeathSaveChange={handleDeathSaveChange}
                            passivePerception={passivePerception}
                            updateCharacter={updateCharacter}
                        />
                    )}
                    {activeTab === 'skills' && (
                        <SkillsTab
                            character={character}
                            skills={skills}
                            handleSkillProficiencyChange={handleSkillProficiencyChange}
                        />
                    )}
                    {activeTab === 'biography' && <BiographyTab character={character} updateCharacter={updateCharacter} />}
                </div>
            </div>
        </div>
    );
};

// Information Tab
const InformationTab: React.FC<any> = ({
    character,
    abilities,
    handleAbilityScoreChange,
    handleHPChange,
    handleDeathSaveChange,
    passivePerception,
    updateCharacter,
}) => (
    <div style={{ display: 'grid', gridTemplateColumns: '280px 1fr', gap: '20px', height: '100%' }}>
        {/* Left - Ability Scores */}
        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '8px' }}>
                {abilities.map((ability: AbilityType) => {
                    const score = character.abilityScores[ability];
                    const modifier = getAbilityModifier(score);
                    const savingThrowProficient = character.savingThrows[ability] !== 'none';
                    const savingThrowBonus = getSavingThrowBonus(character, ability);

                    return (
                        <StatBlock
                            key={ability}
                            ability={ability}
                            score={score}
                            modifier={modifier}
                            savingThrowProficient={savingThrowProficient}
                            savingThrowBonus={savingThrowBonus}
                            onScoreChange={(newScore) => handleAbilityScoreChange(ability, newScore)}
                        />
                    );
                })}
            </div>

            <div
                style={{
                    marginTop: '8px',
                    padding: '12px',
                    background: 'rgba(212, 175, 55, 0.1)',
                    borderRadius: '10px',
                    textAlign: 'center',
                }}
            >
                <div style={{ fontSize: '11px', color: '#D4AF37', marginBottom: '4px' }}>
                    <TooltipTerm term="Proficiency Bonus">Proficiency Bonus</TooltipTerm>
                </div>
                <div style={{ fontSize: '24px', fontWeight: 700, color: '#FFF' }}>
                    +{character.proficiencyBonus}
                </div>
            </div>

            {/* Passive Perception & Inspiration */}
            <div
                style={{
                    padding: '10px',
                    background: 'rgba(212, 175, 55, 0.1)',
                    borderRadius: '10px',
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                }}
            >
                <span style={{ fontSize: '12px', color: '#D4AF37' }}>
                    <TooltipTerm term="Passive Perception">Passive Perception</TooltipTerm>
                </span>
                <span style={{ fontSize: '16px', fontWeight: 700, color: '#FFF' }}>{passivePerception}</span>
            </div>

            <div
                style={{
                    padding: '10px',
                    background: 'rgba(255,255,255,0.03)',
                    borderRadius: '10px',
                }}
            >
                <label
                    style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: '8px',
                        cursor: 'pointer',
                        fontSize: '13px',
                        color: 'rgba(255, 255, 255, 0.8)',
                    }}
                >
                    <input
                        type="checkbox"
                        checked={character.inspiration}
                        onChange={(e) => updateCharacter({ inspiration: e.target.checked })}
                        style={{ width: '16px', height: '16px', cursor: 'pointer', accentColor: '#4A90E2' }}
                    />
                    <span><TooltipTerm term="Inspiration">Inspiration</TooltipTerm></span>
                </label>
            </div>
        </div>

        {/* Right - Vitals */}
        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
            {/* AC, Init, Speed */}
            <div style={{ display: 'flex', gap: '12px' }}>
                <VitalStat label="AC" value={character.armorClass} />
                <VitalStat label="INIT" value={`+${character.initiative}`} />
                <VitalStat label="SPEED" value={`${character.speed.walking} ft`} />
            </div>

            {/* HP */}
            <div style={{ padding: '16px', background: 'rgba(255,255,255,0.03)', borderRadius: '10px' }}>
                <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.7)', marginBottom: '8px' }}>
                    <TooltipTerm term="Hit Points">Hit Points</TooltipTerm>
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '8px' }}>
                    <input
                        type="number"
                        value={character.hitPoints.current}
                        onChange={(e) => handleHPChange('current', parseInt(e.target.value) || 0)}
                        style={{
                            width: '70px',
                            padding: '6px',
                            background: 'rgba(0, 0, 0, 0.4)',
                            border: '1px solid rgba(255, 255, 255, 0.2)',
                            borderRadius: '6px',
                            color: character.hitPoints.current < character.hitPoints.max * 0.25 ? '#e74c3c' : '#FFF',
                            fontFamily: "'Roboto Mono', monospace",
                            fontSize: '20px',
                            fontWeight: 700,
                            textAlign: 'center',
                        }}
                    />
                    <span style={{ fontSize: '20px', color: 'rgba(255, 255, 255, 0.3)' }}>/</span>
                    <input
                        type="number"
                        value={character.hitPoints.max}
                        onChange={(e) => handleHPChange('max', parseInt(e.target.value) || 1)}
                        style={{
                            width: '70px',
                            padding: '6px',
                            background: 'rgba(0, 0, 0, 0.4)',
                            border: '1px solid rgba(255, 255, 255, 0.2)',
                            borderRadius: '6px',
                            color: 'rgba(255, 255, 255, 0.7)',
                            fontFamily: "'Roboto Mono', monospace",
                            fontSize: '20px',
                            fontWeight: 700,
                            textAlign: 'center',
                        }}
                    />
                </div>
                <div
                    style={{
                        height: '6px',
                        background: 'rgba(0, 0, 0, 0.4)',
                        borderRadius: '3px',
                        overflow: 'hidden',
                        marginBottom: '8px',
                    }}
                >
                    <div
                        style={{
                            width: `${Math.min(100, (character.hitPoints.current / character.hitPoints.max) * 100)}%`,
                            height: '100%',
                            background: character.hitPoints.current < character.hitPoints.max * 0.25 ? '#e74c3c' : '#D4AF37',
                            transition: 'width 0.3s ease',
                        }}
                    />
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '12px' }}>
                    <label style={{ color: 'rgba(255,255,255,0.7)' }}>
                        <TooltipTerm term="Temporary Hit Points">Temp HP</TooltipTerm>:
                    </label>
                    <input
                        type="number"
                        value={character.hitPoints.temporary}
                        onChange={(e) => handleHPChange('temporary', parseInt(e.target.value) || 0)}
                        style={{
                            width: '50px',
                            padding: '4px',
                            background: 'rgba(0, 0, 0, 0.4)',
                            border: '1px solid rgba(255, 255, 255, 0.2)',
                            borderRadius: '4px',
                            color: '#FFF',
                            fontFamily: "'Roboto Mono', monospace",
                            fontSize: '13px',
                            textAlign: 'center',
                        }}
                    />
                </div>
            </div>

            {/* Hit Dice & Death Saves */}
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px' }}>
                <div style={{ padding: '12px', background: 'rgba(255,255,255,0.03)', borderRadius: '10px' }}>
                    <div style={{ fontSize: '11px', color: 'rgba(255,255,255,0.5)', marginBottom: '6px' }}>
                        <TooltipTerm term="Hit Dice">Hit Dice</TooltipTerm>
                    </div>
                    {character.hitDice.map((hd: any, index: number) => (
                        <div
                            key={index}
                            style={{
                                fontFamily: "'Roboto Mono', monospace",
                                fontSize: '14px',
                                color: '#FFF',
                                textAlign: 'center',
                            }}
                        >
                            {hd.current} / {hd.total} {hd.dieType}
                        </div>
                    ))}
                </div>

                <div style={{ padding: '12px', background: 'rgba(255,255,255,0.03)', borderRadius: '10px' }}>
                    <div style={{ fontSize: '11px', color: 'rgba(255,255,255,0.5)', marginBottom: '8px', textAlign: 'center' }}>
                        <TooltipTerm term="Death Saves">Death Saves</TooltipTerm>
                    </div>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                        <DeathSaveRow
                            label="Success"
                            count={character.deathSaves.successes}
                            onChange={(val) => handleDeathSaveChange('successes', val)}
                            isFailure={false}
                        />
                        <DeathSaveRow
                            label="Failure"
                            count={character.deathSaves.failures}
                            onChange={(val) => handleDeathSaveChange('failures', val)}
                            isFailure={true}
                        />
                    </div>
                </div>
            </div>

            {/* Attacks */}
            <AttacksSection character={character} />
        </div>
    </div>
);

// Skills Tab
const SkillsTab: React.FC<any> = ({ character, skills, handleSkillProficiencyChange }) => (
    <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '8px', height: '100%', overflow: 'hidden' }}>
        {skills.map((skill: SkillType) => {
            const bonus = getSkillBonus(character, skill);
            const proficiency = character.skills[skill];

            return (
                <SkillRow
                    key={skill}
                    skill={skill}
                    proficiency={proficiency}
                    bonus={bonus}
                    onProficiencyChange={(newProf) => handleSkillProficiencyChange(skill, newProf)}
                />
            );
        })}
    </div>
);

// Biography Tab
const BiographyTab: React.FC<any> = ({ character, updateCharacter }) => (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '12px', height: '100%' }}>
        <div style={{ padding: '16px', background: 'rgba(255,255,255,0.03)', borderRadius: '10px' }}>
            <div style={{ fontSize: '12px', color: '#D4AF37', marginBottom: '6px' }}>Background</div>
            <div style={{ fontSize: '14px', color: 'rgba(255,255,255,0.8)' }}>
                <TooltipTerm term={character.background as any}>{character.background}</TooltipTerm>
            </div>
        </div>

        <div style={{ padding: '16px', background: 'rgba(255,255,255,0.03)', borderRadius: '10px' }}>
            <div style={{ fontSize: '12px', color: '#D4AF37', marginBottom: '6px' }}>Alignment</div>
            <div style={{ fontSize: '14px', color: 'rgba(255,255,255,0.8)' }}>{character.alignment}</div>
        </div>

        <div style={{ padding: '16px', background: 'rgba(255,255,255,0.03)', borderRadius: '10px', flex: 1 }}>
            <div style={{ fontSize: '12px', color: '#D4AF37', marginBottom: '6px' }}>Notes</div>
            <textarea
                value={character.notes || ''}
                onChange={(e) => updateCharacter({ notes: e.target.value })}
                placeholder="Character notes, personality traits, bonds, flaws..."
                style={{
                    width: '100%',
                    height: 'calc(100% - 24px)',
                    background: 'rgba(0,0,0,0.3)',
                    border: '1px solid rgba(255,255,255,0.1)',
                    borderRadius: '6px',
                    padding: '8px',
                    color: 'rgba(255,255,255,0.8)',
                    fontSize: '13px',
                    fontFamily: "'Inter', sans-serif",
                    resize: 'none',
                }}
            />
        </div>
    </div>
);

// Helper Components
const VitalStat: React.FC<{ label: string; value: string | number }> = ({ label, value }) => {
    // Map short labels to full terms for tooltip lookup
    const termMap: Record<string, string> = {
        'AC': 'Armor Class',
        'INIT': 'Initiative',
        'SPEED': 'Speed'
    };

    return (
        <div style={{ flex: 1, padding: '12px', background: 'rgba(255,255,255,0.03)', borderRadius: '10px', textAlign: 'center' }}>
            <div style={{ fontSize: '18px', fontWeight: 'bold', color: '#FFF' }}>{value}</div>
            <div style={{ fontSize: '10px', color: 'rgba(255,255,255,0.5)', marginTop: '2px' }}>
                <TooltipTerm term={(termMap[label] || label) as TooltipDefinitionKey}>{label}</TooltipTerm>
            </div>
        </div>
    );
};

const DeathSaveRow: React.FC<{
    label: string;
    count: number;
    onChange: (val: number) => void;
    isFailure: boolean;
}> = ({ label, count, onChange, isFailure }) => (
    <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', fontSize: '11px' }}>
        <span style={{ color: 'rgba(255,255,255,0.6)' }}>{label}</span>
        <div style={{ display: 'flex', gap: '4px' }}>
            {[0, 1, 2].map((i) => (
                <button
                    key={i}
                    onClick={() => onChange(i < count ? i : i + 1)}
                    style={{
                        width: '18px',
                        height: '18px',
                        background: i < count ? (isFailure ? 'rgba(231, 76, 60, 0.2)' : 'rgba(74, 144, 226, 0.2)') : 'rgba(0, 0, 0, 0.4)',
                        border: `1px solid ${i < count ? (isFailure ? '#e74c3c' : '#4a90e2') : 'rgba(255, 255, 255, 0.2)'}`,
                        borderRadius: '3px',
                        color: isFailure ? '#e74c3c' : '#4a90e2',
                        fontSize: '10px',
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                    }}
                >
                    {i < count ? (isFailure ? '✕' : '✓') : ''}
                </button>
            ))}
        </div>
    </div>
);

const AttacksSection: React.FC<{ character: Character }> = ({ character }) => {
    const attacks: { name: string; bonus: number; damage: string; damageType: string; range: string; properties?: string[] }[] = [];

    // Unarmed Strike
    const strMod = getAbilityModifier(character.abilityScores.strength);
    const profBonus = character.proficiencyBonus;
    attacks.push({
        name: 'Unarmed Strike',
        bonus: strMod + profBonus,
        damage: `${1 + strMod}`,
        damageType: 'bludgeoning',
        range: '5 ft',
    });

    // Equipped Weapons (defensive checks)
    if (character.inventory?.equipped) {
        const { mainHand, offHand } = character.inventory.equipped;
        [mainHand, offHand].forEach((item) => {
            if (item && item.category === 'weapon') {
                const weapon = item as Weapon;
                const isFinesse = weapon.properties?.includes('Finesse');
                const isRanged = weapon.weaponType?.includes('ranged') || false;

                // Determine ability: Dex if Ranged, Str if Melee (unless Finesse, then max of Str/Dex)
                let ability: AbilityType = 'strength';
                if (isRanged) {
                    ability = 'dexterity';
                } else if (isFinesse) {
                    const str = character.abilityScores.strength;
                    const dex = character.abilityScores.dexterity;
                    ability = dex > str ? 'dexterity' : 'strength';
                }

                const mod = getAbilityModifier(character.abilityScores[ability]);
                // Assume proficiency for now
                const bonus = mod + profBonus;

                const damage = `${weapon.damage || '1d4'} ${mod >= 0 ? '+' : ''}${mod}`;

                attacks.push({
                    name: weapon.name,
                    bonus,
                    damage,
                    damageType: weapon.damageType || 'bludgeoning',
                    range: weapon.range ? `${weapon.range.normal}/${weapon.range.long || weapon.range.normal} ft` : '5 ft',
                    properties: weapon.properties,
                });
            }
        });
    }

    // Cantrips (simplified, defensive checks)
    if (character.spellcasting?.cantrips) {
        character.spellcasting.cantrips.forEach((spell) => {
            if (spell.damage) {
                const spellMod = getAbilityModifier(character.abilityScores[character.spellcasting!.ability]);
                const bonus = spellMod + profBonus;
                attacks.push({
                    name: spell.name,
                    bonus,
                    damage: spell.damage.base,
                    damageType: spell.damage.type,
                    range: spell.range,
                });
            }
        });
    }

    return (
        <div style={{ marginTop: '12px', padding: '12px', background: 'rgba(255,255,255,0.03)', borderRadius: '10px' }}>
            <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.5)', marginBottom: '8px', display: 'flex', justifyContent: 'space-between' }}>
                <span>Attacks & Spellcasting</span>
            </div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                {attacks.map((attack, i) => (
                    <AttackRow key={i} {...attack} />
                ))}
            </div>
        </div>
    );
};

const AttackRow: React.FC<{ name: string; bonus: number; damage: string; damageType: string; range: string; properties?: string[] }> = ({
    name,
    bonus,
    damage,
    damageType,
    range,
    properties,
}) => (
    <div
        style={{
            display: 'grid',
            gridTemplateColumns: '1fr auto auto',
            gap: '8px',
            padding: '8px',
            background: 'rgba(0,0,0,0.2)',
            borderRadius: '6px',
            alignItems: 'center',
            fontSize: '13px',
        }}
    >
        <div style={{ display: 'flex', flexDirection: 'column' }}>
            <span style={{ fontWeight: 600, color: '#FFF' }}>{name}</span>
            <span style={{ fontSize: '11px', color: 'rgba(255,255,255,0.5)' }}>
                {range} • <TooltipTerm term={damageType as TooltipDefinitionKey}>{damageType}</TooltipTerm>
                {properties && properties.length > 0 && (
                    <>
                        {' • '}
                        {properties.map((prop, i) => (
                            <span key={i}>
                                {i > 0 && ', '}
                                <TooltipTerm term={prop as TooltipDefinitionKey}>{prop}</TooltipTerm>
                            </span>
                        ))}
                    </>
                )}
            </span>
        </div>
        <div
            style={{
                padding: '2px 6px',
                background: 'rgba(255,255,255,0.1)',
                borderRadius: '4px',
                color: '#D4AF37',
                fontWeight: 600,
                fontSize: '12px',
            }}
        >
            {bonus >= 0 ? '+' : ''}{bonus}
        </div>
        <div style={{ color: 'rgba(255,255,255,0.8)', fontSize: '12px' }}>{damage}</div>
    </div>
);

export default CharacterSheetModal;
