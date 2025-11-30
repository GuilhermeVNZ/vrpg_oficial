import React, { useState } from 'react';
import Modal from './Modal';

interface CompendiumModalProps {
    isOpen: boolean;
    onClose: () => void;
}

const CompendiumModal: React.FC<CompendiumModalProps> = ({ isOpen, onClose }) => {
    const [activeTab, setActiveTab] = useState<'rules' | 'monsters' | 'items' | 'spells'>('rules');

    return (
        <Modal isOpen={isOpen} onClose={onClose} title="" frameless={true}>
            <div style={{
                background: 'rgba(20, 20, 20, 0.6)',
                backdropFilter: 'blur(24px)',
                padding: '24px',
                borderRadius: '24px',
                color: '#FFF',
                fontFamily: "'Inter', sans-serif",
                position: 'relative'
            }}>
                {/* Custom Close Button */}
                <button
                    onClick={onClose}
                    style={{
                        position: 'absolute',
                        top: '20px',
                        right: '20px',
                        background: 'rgba(255,255,255,0.1)',
                        border: 'none',
                        color: '#FFF',
                        fontSize: '24px',
                        width: '36px',
                        height: '36px',
                        borderRadius: '50%',
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        transition: 'all 0.2s ease',
                        zIndex: 10
                    }}
                    onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.2)'}
                    onMouseLeave={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.1)'}
                >
                    ✕
                </button>

                <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
                    {/* Tabs */}
                    <div style={{
                        display: 'flex',
                        gap: '8px',
                        borderBottom: '1px solid rgba(212, 175, 55, 0.3)',
                        paddingBottom: '12px'
                    }}>
                        {(['rules', 'monsters', 'items', 'spells'] as const).map((tab) => (
                            <button
                                key={tab}
                                onClick={() => setActiveTab(tab)}
                                style={{
                                    padding: '10px 20px',
                                    background: activeTab === tab ? 'rgba(212, 175, 55, 0.3)' : 'rgba(0,0,0,0.4)',
                                    border: activeTab === tab ? '1px solid #D4AF37' : '1px solid rgba(255,255,255,0.1)',
                                    borderRadius: '8px',
                                    color: activeTab === tab ? '#D4AF37' : 'rgba(255,255,255,0.7)',
                                    cursor: 'pointer',
                                    fontSize: '14px',
                                    fontWeight: activeTab === tab ? 'bold' : 'normal',
                                    textTransform: 'capitalize',
                                    transition: 'all 0.2s ease'
                                }}
                            >
                                {tab}
                            </button>
                        ))}
                    </div>

                    {/* Content */}
                    <div style={{
                        background: 'rgba(0,0,0,0.4)',
                        padding: '20px',
                        borderRadius: '16px',
                        border: '1px solid rgba(212, 175, 55, 0.3)',
                        minHeight: '400px'
                    }}>
                        {activeTab === 'rules' && <RulesTab />}
                        {activeTab === 'monsters' && <MonstersTab />}
                        {activeTab === 'items' && <ItemsTab />}
                        {activeTab === 'spells' && <SpellsTab />}
                    </div>
                </div>
            </div>
        </Modal>
    );
};

const RulesTab = () => (
    <div>
        <h3 style={{ color: '#D4AF37', marginBottom: '16px' }}>Core Rules Reference</h3>
        {['Combat Actions', 'Conditions', 'Ability Checks', 'Saving Throws', 'Death & Dying'].map((rule, i) => (
            <div key={i} style={{
                padding: '12px',
                background: 'rgba(0,0,0,0.5)',
                borderRadius: '8px',
                border: '1px solid rgba(255,255,255,0.1)',
                marginBottom: '8px',
                cursor: 'pointer',
                color: '#FFF'
            }}>
                {rule}
            </div>
        ))}
    </div>
);

const MonstersTab = () => (
    <div>
        <h3 style={{ color: '#D4AF37', marginBottom: '16px' }}>Monster Manual</h3>
        {['Ancient Red Dragon (CR 24)', 'Beholder (CR 13)', 'Goblin (CR 1/4)', 'Orc (CR 1/2)'].map((monster, i) => (
            <div key={i} style={{
                padding: '12px',
                background: 'rgba(0,0,0,0.5)',
                borderRadius: '8px',
                border: '1px solid rgba(255,255,255,0.1)',
                marginBottom: '8px',
                cursor: 'pointer',
                color: '#FFF'
            }}>
                {monster}
            </div>
        ))}
    </div>
);

const ItemsTab = () => (
    <div>
        <h3 style={{ color: '#D4AF37', marginBottom: '16px' }}>Magic Items</h3>
        {['Bag of Holding', 'Ring of Protection', 'Vorpal Sword', 'Cloak of Invisibility'].map((item, i) => (
            <div key={i} style={{
                padding: '12px',
                background: 'rgba(0,0,0,0.5)',
                borderRadius: '8px',
                border: '1px solid rgba(255,255,255,0.1)',
                marginBottom: '8px',
                cursor: 'pointer',
                color: '#FFF'
            }}>
                {item}
            </div>
        ))}
    </div>
);

const SpellsTab = () => (
    <div>
        <h3 style={{ color: '#D4AF37', marginBottom: '16px' }}>Spell Database</h3>
        {['Fireball (3rd)', 'Magic Missile (1st)', 'Wish (9th)', 'Eldritch Blast (Cantrip)'].map((spell, i) => (
            <div key={i} style={{
                padding: '12px',
                background: 'rgba(0,0,0,0.5)',
                borderRadius: '8px',
                border: '1px solid rgba(255,255,255,0.1)',
                marginBottom: '8px',
                cursor: 'pointer',
                color: '#FFF'
            }}>
                {spell}
            </div>
        ))}
    </div>
);

export default CompendiumModal;
