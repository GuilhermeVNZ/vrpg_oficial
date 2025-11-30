import React from 'react';
import { useCharacter } from '../../context/CharacterContext';
import { Tooltip } from '../common/Tooltip';

interface AbilitiesModalProps {
    isOpen: boolean;
    onClose: () => void;
}

const AbilitiesModal: React.FC<AbilitiesModalProps> = ({ isOpen, onClose }) => {
    const { character } = useCharacter();

    if (!isOpen || !character) return null;

    return (
        <div className="modal-overlay" onClick={onClose}>
            <div
                className="abilities-modal"
                onClick={(e) => e.stopPropagation()}
                style={{
                    background: 'rgba(20, 20, 20, 0.6)',
                    backdropFilter: 'blur(24px)',
                    padding: '24px',
                    borderRadius: '24px',
                    border: 'none',
                    boxShadow: '0 20px 50px rgba(0,0,0,0.3)',
                    maxWidth: '800px',
                    width: '100%',
                    height: '70vh',
                    display: 'flex',
                    flexDirection: 'column',
                    position: 'relative',
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
                <h2
                    style={{
                        fontSize: '28px',
                        fontWeight: 'bold',
                        margin: '0 0 16px 0',
                        color: '#D4AF37',
                        fontFamily: "'Crimson Text', serif",
                    }}
                >
                    Features & Abilities
                </h2>

                {/* Content - Scrollable */}
                <div style={{ flex: 1, overflow: 'auto', paddingRight: '8px' }}>
                    {/* Features */}
                    <div style={{ marginBottom: '24px' }}>
                        <div
                            style={{
                                fontSize: '18px',
                                fontWeight: 600,
                                color: '#FFF',
                                marginBottom: '12px',
                                fontFamily: "'Crimson Text', serif",
                                display: 'flex',
                                alignItems: 'center',
                                gap: '12px',
                            }}
                        >
                            Features
                            <div style={{ flex: 1, height: '1px', background: 'rgba(255,255,255,0.1)' }} />
                        </div>

                        <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                            {character.features.map((feature) => {
                                // Determine recharge type display
                                let rechargeType = 'Passive';
                                if (feature.uses) {
                                    const recharge = feature.uses.recharge;
                                    if (recharge === 'short-rest') rechargeType = 'Short Rest';
                                    else if (recharge === 'long-rest') rechargeType = 'Long Rest';
                                    else if (recharge === 'dawn') rechargeType = 'Dawn';
                                    else rechargeType = 'Limited';
                                }

                                // Determine uses/value display
                                let usesDisplay = '—';
                                if (feature.uses) {
                                    usesDisplay = `${feature.uses.current}/${feature.uses.max}`;
                                } else if (feature.name === 'Darkvision') {
                                    usesDisplay = '60 ft';
                                } else {
                                    usesDisplay = '∞';
                                }

                                return (
                                    <Tooltip
                                        key={feature.id}
                                        content={
                                            <div style={{ fontSize: '13px', lineHeight: '1.6' }}>
                                                <div style={{ fontWeight: 600, marginBottom: '6px', color: '#D4AF37' }}>
                                                    {feature.name}
                                                </div>
                                                {feature.description}
                                                {feature.uses && (
                                                    <div style={{ marginTop: '8px', fontSize: '11px', opacity: 0.7 }}>
                                                        Recharge: {rechargeType}
                                                    </div>
                                                )}
                                            </div>
                                        }
                                        delay={300}
                                    >
                                        <div
                                            style={{
                                                display: 'grid',
                                                gridTemplateColumns: '1fr auto auto',
                                                gap: '12px',
                                                padding: '10px 12px',
                                                background: 'rgba(255,255,255,0.03)',
                                                border: '1px solid rgba(255,255,255,0.08)',
                                                borderRadius: '8px',
                                                alignItems: 'center',
                                                cursor: 'help',
                                                transition: 'all 0.2s ease',
                                            }}
                                            onMouseEnter={(e) => {
                                                e.currentTarget.style.background = 'rgba(255,255,255,0.05)';
                                                e.currentTarget.style.borderColor = 'rgba(212, 175, 55, 0.3)';
                                            }}
                                            onMouseLeave={(e) => {
                                                e.currentTarget.style.background = 'rgba(255,255,255,0.03)';
                                                e.currentTarget.style.borderColor = 'rgba(255,255,255,0.08)';
                                            }}
                                        >
                                            {/* Column 1: Name + Source */}
                                            <div>
                                                <div style={{ fontSize: '14px', fontWeight: 600, color: '#FFF' }}>
                                                    {feature.name}
                                                </div>
                                                <div
                                                    style={{
                                                        fontSize: '10px',
                                                        color: '#D4AF37',
                                                        textTransform: 'uppercase',
                                                        letterSpacing: '0.5px',
                                                        marginTop: '2px',
                                                    }}
                                                >
                                                    {feature.source}
                                                </div>
                                            </div>

                                            {/* Column 2: Recharge Type */}
                                            <div
                                                style={{
                                                    fontSize: '11px',
                                                    color: 'rgba(255,255,255,0.6)',
                                                    textAlign: 'right',
                                                    whiteSpace: 'nowrap',
                                                }}
                                            >
                                                {rechargeType}
                                            </div>

                                            {/* Column 3: Uses/Value */}
                                            <div
                                                style={{
                                                    padding: '4px 10px',
                                                    background: feature.uses
                                                        ? 'rgba(212, 175, 55, 0.15)'
                                                        : 'rgba(255,255,255,0.05)',
                                                    border: feature.uses
                                                        ? '1px solid rgba(212, 175, 55, 0.3)'
                                                        : '1px solid rgba(255,255,255,0.1)',
                                                    borderRadius: '6px',
                                                    fontSize: feature.uses ? '12px' : '20px',
                                                    fontWeight: 600,
                                                    color: feature.uses ? '#D4AF37' : 'rgba(255,255,255,0.5)',
                                                    minWidth: '50px',
                                                    textAlign: 'center',
                                                    fontFamily: "'Roboto Mono', monospace",
                                                }}
                                            >
                                                {usesDisplay}
                                            </div>
                                        </div>
                                    </Tooltip>
                                );
                            })}
                        </div>
                    </div>

                    {/* Proficiencies */}
                    <div>
                        <div
                            style={{
                                fontSize: '18px',
                                fontWeight: 600,
                                color: '#FFF',
                                marginBottom: '12px',
                                fontFamily: "'Crimson Text', serif",
                                display: 'flex',
                                alignItems: 'center',
                                gap: '12px',
                            }}
                        >
                            Proficiencies
                            <div style={{ flex: 1, height: '1px', background: 'rgba(255,255,255,0.1)' }} />
                        </div>

                        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '16px' }}>
                            {/* Languages */}
                            <div
                                style={{
                                    padding: '16px',
                                    background: 'rgba(255,255,255,0.03)',
                                    border: '1px solid rgba(255,255,255,0.1)',
                                    borderRadius: '10px',
                                }}
                            >
                                <div
                                    style={{
                                        fontSize: '13px',
                                        color: '#D4AF37',
                                        textTransform: 'uppercase',
                                        letterSpacing: '0.5px',
                                        marginBottom: '8px',
                                    }}
                                >
                                    Languages
                                </div>
                                <div style={{ fontSize: '14px', color: 'rgba(255,255,255,0.8)' }}>
                                    {character.proficiencies.languages.join(', ')}
                                </div>
                            </div>

                            {/* Weapons */}
                            <div
                                style={{
                                    padding: '16px',
                                    background: 'rgba(255,255,255,0.03)',
                                    border: '1px solid rgba(255,255,255,0.1)',
                                    borderRadius: '10px',
                                }}
                            >
                                <div
                                    style={{
                                        fontSize: '13px',
                                        color: '#D4AF37',
                                        textTransform: 'uppercase',
                                        letterSpacing: '0.5px',
                                        marginBottom: '8px',
                                    }}
                                >
                                    Weapons
                                </div>
                                <div style={{ fontSize: '14px', color: 'rgba(255,255,255,0.8)' }}>
                                    {character.proficiencies.weapons.join(', ')}
                                </div>
                            </div>

                            {/* Armor */}
                            {character.proficiencies.armor.length > 0 && (
                                <div
                                    style={{
                                        padding: '16px',
                                        background: 'rgba(255,255,255,0.03)',
                                        border: '1px solid rgba(255,255,255,0.1)',
                                        borderRadius: '10px',
                                    }}
                                >
                                    <div
                                        style={{
                                            fontSize: '13px',
                                            color: '#D4AF37',
                                            textTransform: 'uppercase',
                                            letterSpacing: '0.5px',
                                            marginBottom: '8px',
                                        }}
                                    >
                                        Armor
                                    </div>
                                    <div style={{ fontSize: '14px', color: 'rgba(255,255,255,0.8)' }}>
                                        {character.proficiencies.armor.join(', ')}
                                    </div>
                                </div>
                            )}

                            {/* Tools */}
                            {character.proficiencies.tools.length > 0 && (
                                <div
                                    style={{
                                        padding: '16px',
                                        background: 'rgba(255,255,255,0.03)',
                                        border: '1px solid rgba(255,255,255,0.1)',
                                        borderRadius: '10px',
                                    }}
                                >
                                    <div
                                        style={{
                                            fontSize: '13px',
                                            color: '#D4AF37',
                                            textTransform: 'uppercase',
                                            letterSpacing: '0.5px',
                                            marginBottom: '8px',
                                        }}
                                    >
                                        Tools
                                    </div>
                                    <div style={{ fontSize: '14px', color: 'rgba(255,255,255,0.8)' }}>
                                        {character.proficiencies.tools.join(', ')}
                                    </div>
                                </div>
                            )}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default AbilitiesModal;
