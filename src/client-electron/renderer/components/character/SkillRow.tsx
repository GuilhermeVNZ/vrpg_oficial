import React from 'react';
import { SkillType, ProficiencyLevel } from '../../types/Character';
import { TooltipTerm } from '../common/TooltipTerm';
import './SkillRow.css';

interface SkillRowProps {
    skill: SkillType;
    proficiency: ProficiencyLevel;
    bonus: number;
    onProficiencyChange?: (newProficiency: ProficiencyLevel) => void;
}

const SKILL_NAMES: Record<SkillType, string> = {
    acrobatics: 'Acrobatics',
    animalHandling: 'Animal Handling',
    arcana: 'Arcana',
    athletics: 'Athletics',
    deception: 'Deception',
    history: 'History',
    insight: 'Insight',
    intimidation: 'Intimidation',
    investigation: 'Investigation',
    medicine: 'Medicine',
    nature: 'Nature',
    perception: 'Perception',
    performance: 'Performance',
    persuasion: 'Persuasion',
    religion: 'Religion',
    sleightOfHand: 'Sleight of Hand',
    stealth: 'Stealth',
    survival: 'Survival',
};

const SKILL_ABILITIES: Record<SkillType, string> = {
    acrobatics: 'Dex',
    animalHandling: 'Wis',
    arcana: 'Int',
    athletics: 'Str',
    deception: 'Cha',
    history: 'Int',
    insight: 'Wis',
    intimidation: 'Cha',
    investigation: 'Int',
    medicine: 'Wis',
    nature: 'Int',
    perception: 'Wis',
    performance: 'Cha',
    persuasion: 'Cha',
    religion: 'Int',
    sleightOfHand: 'Dex',
    stealth: 'Dex',
    survival: 'Wis',
};

export const SkillRow: React.FC<SkillRowProps> = ({
    skill,
    proficiency,
    bonus,
    onProficiencyChange,
}) => {
    const skillName = SKILL_NAMES[skill];
    const ability = SKILL_ABILITIES[skill];

    const cycleProficiency = () => {
        if (!onProficiencyChange) return;

        const cycle: ProficiencyLevel[] = ['none', 'proficient', 'expertise'];
        const currentIndex = cycle.indexOf(proficiency);
        const nextIndex = (currentIndex + 1) % cycle.length;
        onProficiencyChange(cycle[nextIndex] as ProficiencyLevel);
    };

    const getProficiencyIcon = () => {
        switch (proficiency) {
            case 'none':
                return '○';
            case 'proficient':
                return '⦿';
            case 'expertise':
                return '⦿⦿';
            default:
                return '○';
        }
    };

    return (
        <div className="skill-row">
            <button
                className="skill-proficiency-button"
                onClick={cycleProficiency}
                disabled={!onProficiencyChange}
                aria-label={`Toggle proficiency for ${skillName}`}
            >
                {getProficiencyIcon()}
            </button>

            <div className="skill-name">
                <TooltipTerm term={skillName as any}>
                    {skillName}
                </TooltipTerm>
                <span className="skill-ability">({ability})</span>
            </div>

            <div className="skill-bonus">
                {bonus >= 0 ? '+' : ''}{bonus}
            </div>
        </div>
    );
};
