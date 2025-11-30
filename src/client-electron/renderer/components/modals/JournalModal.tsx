import React from 'react';
import Modal from './Modal';

interface JournalModalProps {
    isOpen: boolean;
    onClose: () => void;
}

const mockJournal = {
    activeQuests: [
        { name: 'Defeat the Ancient Dragon', description: 'The dragon Vexrathrax threatens the kingdom', progress: '3/5 objectives' },
        { name: 'Retrieve the Arcane Tome', description: 'Find the lost spellbook in the ruins', progress: '2/3 objectives' }
    ],
    notes: [
        { title: 'Session 12 - Nov 20', content: 'We entered the dragon\'s lair and discovered the hidden treasure room...' },
        { title: 'NPC: Elara Moonwhisper', content: 'High Elf merchant in Silverlight. Owes us a favor.' }
    ]
};

const JournalModal: React.FC<JournalModalProps> = ({ isOpen, onClose }) => {
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

                <div style={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
                    {/* Active Quests */}
                    <div style={{
                        background: 'rgba(0,0,0,0.4)',
                        padding: '20px',
                        borderRadius: '16px',
                        border: '1px solid rgba(212, 175, 55, 0.3)'
                    }}>
                        <div style={{ fontSize: '18px', fontWeight: 'bold', color: '#D4AF37', marginBottom: '16px', fontFamily: "'Crimson Text', serif" }}>
                            Active Quests
                        </div>
                        {mockJournal.activeQuests.map((quest, i) => (
                            <div key={i} style={{
                                padding: '16px',
                                background: 'rgba(0,0,0,0.5)',
                                borderRadius: '12px',
                                border: '1px solid rgba(212, 175, 55, 0.3)',
                                marginBottom: '12px'
                            }}>
                                <div style={{ fontSize: '16px', fontWeight: 'bold', color: '#FFF', marginBottom: '8px' }}>{quest.name}</div>
                                <div style={{ fontSize: '14px', color: 'rgba(255,255,255,0.8)', marginBottom: '8px' }}>{quest.description}</div>
                                <div style={{ fontSize: '12px', color: '#4A90E2' }}>{quest.progress}</div>
                            </div>
                        ))}
                    </div>

                    {/* Notes */}
                    <div style={{
                        background: 'rgba(0,0,0,0.4)',
                        padding: '20px',
                        borderRadius: '16px',
                        border: '1px solid rgba(212, 175, 55, 0.3)'
                    }}>
                        <div style={{ fontSize: '18px', fontWeight: 'bold', color: '#D4AF37', marginBottom: '16px', fontFamily: "'Crimson Text', serif" }}>
                            Notes
                        </div>
                        {mockJournal.notes.map((note, i) => (
                            <div key={i} style={{
                                padding: '16px',
                                background: 'rgba(0,0,0,0.5)',
                                borderRadius: '12px',
                                border: '1px solid rgba(255,255,255,0.1)',
                                marginBottom: '12px'
                            }}>
                                <div style={{ fontSize: '14px', fontWeight: 'bold', color: '#4A90E2', marginBottom: '8px' }}>{note.title}</div>
                                <div style={{ fontSize: '13px', color: 'rgba(255,255,255,0.8)', lineHeight: '1.6' }}>{note.content}</div>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </Modal>
    );
};

export default JournalModal;
