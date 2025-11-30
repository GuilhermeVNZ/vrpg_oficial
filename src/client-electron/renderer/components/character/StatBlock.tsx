import React from 'react';
import { AbilityType } from '../../types/Character';
import { Tooltip } from '../common/Tooltip';
import { TooltipTerm } from '../common/TooltipTerm';
import './StatBlock.css';

interface StatBlockProps {
    ability: AbilityType;
    score: number;
    modifier: number;
    savingThrowProficient: boolean;
    savingThrowBonus: number;
    onScoreChange?: (newScore: number) => void;
}

const ABILITY_NAMES: Record<AbilityType, string> = {
    strength: 'Strength',
    dexterity: 'Dexterity',
    constitution: 'Constitution',
    intelligence: 'Intelligence',
    wisdom: 'Wisdom',
    charisma: 'Charisma',
};

const ABILITY_DESCRIPTIONS: Record<AbilityType, string> = {
    strength: `Measures: Physical power

Used for:
• Melee attack & damage rolls
• Athletics skill
• Carrying capacity
• Strength saving throws`,

    dexterity: `Measures: Agility, reflexes

Used for:
• AC (if light/medium armor)
• Initiative (turn order)
• Ranged & Finesse weapon attacks
• Acrobatics, Sleight of Hand, Stealth
• Dex saving throws`,

    constitution: `Measures: Endurance, health

Used for:
• Hit Points: +Con mod per level
• Concentration checks
• Con saving throws

Every character benefits from high Con.`,

    intelligence: `Measures: Reasoning, knowledge

Used for:
• Wizard spellcasting
• Arcana, History, Investigation, Nature, Religion
• Int saving throws`,

    wisdom: `Measures: Awareness, intuition

Used for:
• Cleric/Druid spellcasting
• Perception, Insight, Medicine, Survival
• Wis saving throws

Important for everyone (Perception).`,

    charisma: `Measures: Force of personality

Used for:
• Bard/Sorcerer/Warlock/Paladin spellcasting
• Deception, Intimidation, Performance, Persuasion
• Cha saving throws`,
};

export const StatBlock: React.FC<StatBlockProps> = ({
    ability,
    score,
    modifier,
    savingThrowProficient,
    savingThrowBonus,
    onScoreChange,
}) => {
    const abilityName = ABILITY_NAMES[ability];
    const description = ABILITY_DESCRIPTIONS[ability];

    const tooltipContent = (
        <div>
            <div className="tooltip-header">{abilityName}</div>
            <div className="tooltip-section">{description}</div>
            <hr className="tooltip-divider" />
            <div className="tooltip-formula">
                Your value: {score} ({modifier >= 0 ? '+' : ''}{modifier} modifier)
                <br />
                Modifier = (Score - 10) ÷ 2 (round down)
                <br />
                {score} - 10 = {score - 10}, {score - 10} ÷ 2 = {modifier >= 0 ? '+' : ''}{modifier}
            </div>
            {savingThrowProficient && (
                <div className="tooltip-section">
                    <strong>Saving Throw:</strong> {savingThrowBonus >= 0 ? '+' : ''}{savingThrowBonus}
                    <br />
                    (Proficient ✓)
                </div>
            )}
        </div>
    );

    return (
        <Tooltip content={tooltipContent}>
            <div className="stat-block">
                <div className="stat-block-name">{abilityName.substring(0, 3).toUpperCase()}</div>

                <div className="stat-block-modifier">
                    {modifier >= 0 ? '+' : ''}{modifier}
                </div>

                <div className="stat-block-score">
                    {onScoreChange ? (
                        <input
                            type="number"
                            value={score}
                            onChange={(e) => onScoreChange(parseInt(e.target.value) || 10)}
                            min={1}
                            max={30}
                            className="stat-block-input"
                        />
                    ) : (
                        score
                    )}
                </div>

                {savingThrowProficient && (
                    <div className="stat-block-save-indicator" title="Proficient in saving throw">
                        ⦿
                    </div>
                )}
            </div>
        </Tooltip>
    );
};
