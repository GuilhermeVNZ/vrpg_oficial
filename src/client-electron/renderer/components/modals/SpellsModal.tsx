import React, { useState } from 'react';
import { useCharacter } from '../../context/CharacterContext';
import { Tooltip } from '../common/Tooltip';
import { Spell } from '../../types/Character';
import './InventoryModal.css';

interface SpellsModalProps {
    isOpen: boolean;
    onClose: () => void;
}

type SpellLevel = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9;

const SpellsModal: React.FC<SpellsModalProps> = ({ isOpen, onClose }) => {
    const { character, updateCharacter } = useCharacter();
    const [activeLevel, setActiveLevel] = useState<SpellLevel>(1);
    const [selectedSpell, setSelectedSpell] = useState<Spell | null>(null);

    if (!isOpen || !character) return null;

    // --- Helpers ---

    const getSpellsByLevel = (level: number) => {
        const allSpells = [...(character.spellcasting?.knownSpells || []), ...(character.spellcasting?.cantrips || [])];
        return allSpells.filter(s => s.level === level);
    };

    const getPreparedSpellsByLevel = (level: number) => {
        if (level === 0) return character.spellcasting?.cantrips || [];
        const preparedIds = character.spellcasting?.preparedSpells || [];
        const allSpells = character.spellcasting?.knownSpells || [];
        return allSpells.filter(s => s.level === level && preparedIds.includes(s.id));
    };

    const getMaxPreparedSpells = () => {
        return 4;
    };

    // --- Action Handlers ---

    const handlePrepareSpell = (spell: Spell) => {
        if (!character.spellcasting || spell.level === 0) return;

        const currentPrepared = character.spellcasting.preparedSpells || [];
        const preparedAtLevel = getPreparedSpellsByLevel(spell.level).length;

        if (currentPrepared.includes(spell.id)) return;

        if (preparedAtLevel >= getMaxPreparedSpells()) {
            console.log('Maximum prepared spells reached for this level');
            return;
        }

        const updatedCharacter = {
            ...character,
            spellcasting: {
                ...character.spellcasting,
                preparedSpells: [...currentPrepared, spell.id]
            }
        };
        updateCharacter(updatedCharacter);
        setSelectedSpell(null);
    };

    const handleUnprepareSpell = (spell: Spell) => {
        if (!character.spellcasting) return;

        const currentPrepared = character.spellcasting.preparedSpells || [];
        const updatedCharacter = {
            ...character,
            spellcasting: {
                ...character.spellcasting,
                preparedSpells: currentPrepared.filter(id => id !== spell.id)
            }
        };
        updateCharacter(updatedCharacter);
        setSelectedSpell(null);
    };

    // --- Renderers ---

    const renderSpellTooltip = (spell: Spell) => (
        <div>
            <div style={{ fontSize: '16px', fontWeight: 'bold', color: '#D4AF37', marginBottom: '4px', fontFamily: "'Cinzel', serif" }}>
                {spell.name}
            </div>
            <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.6)', marginBottom: '8px', fontStyle: 'italic' }}>
                {spell.school} • {spell.level === 0 ? 'Cantrip' : `Level ${spell.level}`}
            </div>
            <div style={{ fontSize: '12px', marginBottom: '8px', display: 'grid', gridTemplateColumns: 'auto 1fr', gap: '4px 12px' }}>
                <span style={{ color: 'rgba(255,255,255,0.5)' }}>Casting Time:</span> <span>{spell.castingTime}</span>
                <span style={{ color: 'rgba(255,255,255,0.5)' }}>Range:</span> <span>{spell.range}</span>
                <span style={{ color: 'rgba(255,255,255,0.5)' }}>Duration:</span> <span>{spell.duration}</span>
                <span style={{ color: 'rgba(255,255,255,0.5)' }}>Components:</span>
                <span>
                    {[
                        spell.components.verbal && 'V',
                        spell.components.somatic && 'S',
                        spell.components.material && `M (${spell.components.materialDescription || ''})`
                    ].filter(Boolean).join(', ')}
                </span>
            </div>
            <div style={{ fontSize: '13px', color: 'rgba(255,255,255,0.9)', lineHeight: '1.4', whiteSpace: 'pre-wrap' }}>
                {spell.description}
            </div>
            {spell.atHigherLevels && (
                <div style={{ marginTop: '8px', fontSize: '12px', color: '#aaffaa' }}>
                    <strong>At Higher Levels:</strong> {spell.atHigherLevels}
                </div>
            )}
        </div>
    );

    const renderSpellCard = (spell: Spell, isPrepared: boolean = false, onClick?: () => void) => (
        <Tooltip content={renderSpellTooltip(spell)}>
            <div
                onClick={onClick}
                style={{
                    width: '100%',
                    minHeight: '60px',
                    padding: '12px 16px',
                    background: isPrepared ? 'rgba(212, 175, 55, 0.1)' : 'rgba(0, 0, 0, 0.4)',
                    border: selectedSpell?.id === spell.id ? '2px solid #D4AF37' : '1px solid rgba(212, 175, 55, 0.2)',
                    borderRadius: '6px',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '12px',
                    cursor: onClick ? 'pointer' : 'default',
                    transition: 'all 0.2s ease',
                }}
            >
                <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '4px' }}>
                    <div style={{ fontSize: '15px', fontWeight: 600, color: isPrepared ? '#D4AF37' : '#FFF' }}>
                        {spell.name}
                    </div>
                    <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.5)', textTransform: 'capitalize' }}>
                        {spell.school}
                    </div>
                </div>

                <div style={{ display: 'flex', gap: '6px' }}>
                    {spell.concentration && (
                        <div style={{
                            fontSize: '10px',
                            color: '#3498db',
                            border: '1px solid #3498db',
                            padding: '2px 6px',
                            borderRadius: '4px'
                        }}>
                            C
                        </div>
                    )}
                    {spell.ritual && (
                        <div style={{
                            fontSize: '10px',
                            color: '#2ecc71',
                            border: '1px solid #2ecc71',
                            padding: '2px 6px',
                            borderRadius: '4px'
                        }}>
                            R
                        </div>
                    )}
                </div>
            </div>
        </Tooltip>
    );

    const knownSpells = getSpellsByLevel(activeLevel);
    const preparedSpells = getPreparedSpellsByLevel(activeLevel);
    const preparedIds = character.spellcasting?.preparedSpells || [];

    return (
        <div
            className="modal-overlay"
            onClick={onClose}
            style={{
                background: 'rgba(0, 0, 0, 0.7)',
                backdropFilter: 'blur(4px)'
            }}
        >
            <div
                onClick={(e) => e.stopPropagation()}
                style={{
                    width: '100%',
                    maxWidth: '1000px',
                    height: '820px',
                    background: '#000000',
                    borderRadius: '12px',
                    boxShadow: '0 20px 60px rgba(0, 0, 0, 0.8)',
                    display: 'flex',
                    overflow: 'hidden',
                }}
            >
                {/* Main Content Area */}
                <div style={{ flex: 1, display: 'flex', flexDirection: 'column', padding: '32px', gap: '24px' }}>

                    {/* Header */}
                    <div style={{
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                        paddingBottom: '16px',
                        borderBottom: '1px solid rgba(212, 175, 55, 0.2)',
                    }}>
                        <div style={{ fontFamily: "'Cinzel', serif", fontSize: '28px', color: '#D4AF37', fontWeight: 'bold' }}>
                            Spellbook
                        </div>
                        {character.spellcasting ? (
                            <div style={{ display: 'flex', gap: '32px' }}>
                                <div style={{ textAlign: 'center' }}>
                                    <div style={{ fontSize: '11px', color: 'rgba(255,255,255,0.5)', textTransform: 'uppercase' }}>Spell Save DC</div>
                                    <div style={{ fontSize: '20px', fontWeight: 'bold', color: '#FFF' }}>
                                        {8 + (character.proficiencyBonus || 2) + Math.floor(((character.abilityScores[character.spellcasting.ability] || 10) - 10) / 2)}
                                    </div>
                                </div>
                                <div style={{ textAlign: 'center' }}>
                                    <div style={{ fontSize: '11px', color: 'rgba(255,255,255,0.5)', textTransform: 'uppercase' }}>Attack Bonus</div>
                                    <div style={{ fontSize: '20px', fontWeight: 'bold', color: '#FFF' }}>
                                        +{(character.proficiencyBonus || 2) + Math.floor(((character.abilityScores[character.spellcasting.ability] || 10) - 10) / 2)}
                                    </div>
                                </div>
                                <div style={{ textAlign: 'center' }}>
                                    <div style={{ fontSize: '11px', color: 'rgba(255,255,255,0.5)', textTransform: 'uppercase' }}>Ability</div>
                                    <div style={{ fontSize: '20px', fontWeight: 'bold', color: '#D4AF37', textTransform: 'capitalize' }}>
                                        {character.spellcasting.ability}
                                    </div>
                                </div>
                            </div>
                        ) : (
                            <div style={{ fontSize: '14px', color: 'rgba(255,255,255,0.5)', fontStyle: 'italic' }}>
                                This character does not have spellcasting abilities.
                            </div>
                        )}
                    </div>

                    {/* Two Column Layout */}
                    <div style={{ flex: 1, display: 'flex', gap: '16px', overflow: 'hidden' }}>

                        {/* Left: Known Spells */}
                        <div style={{ minWidth: '370px', flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
                            <div style={{
                                fontSize: '18px',
                                color: 'rgba(212, 175, 55, 0.9)',
                                marginBottom: '12px',
                                fontFamily: "'Cinzel', serif",
                                fontWeight: 600,
                                textAlign: 'center',
                                borderBottom: '1px solid rgba(212, 175, 55, 0.2)',
                                paddingBottom: '8px'
                            }}>
                                Known Spells {activeLevel === 0 ? '(Cantrips)' : `(Level ${activeLevel})`}
                            </div>

                            <div style={{
                                flex: 1,
                                overflowY: 'auto',
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '8px'
                            }}>
                                {knownSpells.map((spell) => {
                                    const isPrepared = preparedIds.includes(spell.id);
                                    return (
                                        <div
                                            key={spell.id}
                                            style={{ opacity: isPrepared ? 0.4 : 1, width: '100%' }}
                                        >
                                            {renderSpellCard(spell, false, () => !isPrepared && setSelectedSpell(spell))}
                                        </div>
                                    );
                                })}
                                {knownSpells.length === 0 && (
                                    <div style={{ textAlign: 'center', padding: '20px', color: 'rgba(255,255,255,0.3)' }}>
                                        No spells known at this level.
                                    </div>
                                )}
                            </div>
                        </div>

                        {/* Center: Transfer Buttons */}
                        <div style={{
                            display: 'flex',
                            flexDirection: 'column',
                            justifyContent: 'center',
                            gap: '24px',
                            padding: '0 12px'
                        }}>
                            <button
                                onClick={() => selectedSpell && handlePrepareSpell(selectedSpell)}
                                disabled={!selectedSpell || activeLevel === 0}
                                style={{
                                    width: '50px',
                                    height: '50px',
                                    borderRadius: '6px',
                                    border: '2px solid rgba(212, 175, 55, 0.4)',
                                    background: selectedSpell && activeLevel > 0 ? 'rgba(212, 175, 55, 0.15)' : 'rgba(0,0,0,0.3)',
                                    color: selectedSpell && activeLevel > 0 ? '#FFFFFF' : 'rgba(255,255,255,0.2)',
                                    fontSize: '28px',
                                    fontWeight: 'bold',
                                    cursor: selectedSpell && activeLevel > 0 ? 'pointer' : 'default',
                                    transition: 'all 0.2s ease',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'center',
                                }}
                            >
                                ▶
                            </button>
                            <button
                                onClick={() => selectedSpell && handleUnprepareSpell(selectedSpell)}
                                disabled={!selectedSpell || activeLevel === 0}
                                style={{
                                    width: '50px',
                                    height: '50px',
                                    borderRadius: '6px',
                                    border: '2px solid rgba(212, 175, 55, 0.4)',
                                    background: selectedSpell && activeLevel > 0 ? 'rgba(212, 175, 55, 0.15)' : 'rgba(0,0,0,0.3)',
                                    color: selectedSpell && activeLevel > 0 ? '#FFFFFF' : 'rgba(255,255,255,0.2)',
                                    fontSize: '28px',
                                    fontWeight: 'bold',
                                    cursor: selectedSpell && activeLevel > 0 ? 'pointer' : 'default',
                                    transition: 'all 0.2s ease',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'center',
                                }}
                            >
                                ◀
                            </button>
                        </div>

                        {/* Right: Prepared Spells */}
                        <div style={{ minWidth: '370px', flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
                            <div style={{
                                fontSize: '18px',
                                color: 'rgba(212, 175, 55, 0.9)',
                                marginBottom: '12px',
                                fontFamily: "'Cinzel', serif",
                                fontWeight: 600,
                                display: 'flex',
                                justifyContent: 'space-between',
                                borderBottom: '1px solid rgba(212, 175, 55, 0.2)',
                                paddingBottom: '8px'
                            }}>
                                <span>Prepared Spells</span>
                                {activeLevel > 0 && <span>{preparedSpells.length} / {getMaxPreparedSpells()}</span>}
                            </div>

                            <div style={{
                                flex: 1,
                                overflowY: 'auto',
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '8px',
                                background: 'rgba(255, 255, 255, 0.02)',
                                borderRadius: '8px',
                                padding: '8px'
                            }}>
                                {preparedSpells.map((spell) => (
                                    <div key={spell.id} style={{ width: '100%' }}>
                                        {renderSpellCard(spell, true, () => setSelectedSpell(spell))}
                                    </div>
                                ))}
                                {preparedSpells.length === 0 && (
                                    <div style={{ textAlign: 'center', padding: '20px', color: 'rgba(255,255,255,0.3)', fontStyle: 'italic' }}>
                                        {activeLevel === 0 ? 'Cantrips are always prepared.' : 'No spells prepared.'}
                                    </div>
                                )}
                            </div>
                        </div>

                    </div>

                </div>

                {/* Right Sidebar: Level Tabs */}
                <div style={{
                    width: '60px',
                    background: 'rgba(255, 255, 255, 0.03)',
                    borderLeft: '1px solid rgba(212, 175, 55, 0.2)',
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    paddingTop: '20px',
                    gap: '8px',
                }}>
                    {[0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map((level) => (
                        <button
                            key={level}
                            onClick={() => setActiveLevel(level as SpellLevel)}
                            style={{
                                width: '40px',
                                height: '40px',
                                borderRadius: '4px',
                                border: activeLevel === level ? '1px solid #D4AF37' : '1px solid transparent',
                                background: activeLevel === level ? 'rgba(212, 175, 55, 0.2)' : 'transparent',
                                color: activeLevel === level ? '#D4AF37' : 'rgba(255, 255, 255, 0.5)',
                                fontSize: '14px',
                                fontWeight: 'bold',
                                fontFamily: "'Cinzel', serif",
                                cursor: 'pointer',
                                transition: 'all 0.2s ease',
                                display: 'flex',
                                alignItems: 'center',
                                justifyContent: 'center',
                            }}
                            title={level === 0 ? 'Cantrips' : `Level ${level}`}
                        >
                            {level === 0 ? 'C' : level}
                        </button>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default SpellsModal;
