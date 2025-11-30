import React from 'react';
import Modal from './Modal';

interface MapModalProps {
    isOpen: boolean;
    onClose: () => void;
}

const MapModal: React.FC<MapModalProps> = ({ isOpen, onClose }) => {
    return (
        <Modal isOpen={isOpen} onClose={onClose} title="" maxWidth="1100px" frameless={true}>
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
                    {/* Map Controls */}
                    <div style={{
                        background: 'rgba(0,0,0,0.4)',
                        padding: '12px 16px',
                        borderRadius: '12px',
                        border: '1px solid rgba(212, 175, 55, 0.3)',
                        display: 'flex',
                        gap: '16px',
                        alignItems: 'center'
                    }}>
                        <button style={{
                            padding: '8px 16px',
                            background: 'rgba(74, 144, 226, 0.2)',
                            border: '1px solid #4A90E2',
                            borderRadius: '8px',
                            color: '#4A90E2',
                            cursor: 'pointer',
                            fontSize: '13px',
                            fontWeight: 'bold'
                        }}>
                            🔍 Zoom In
                        </button>
                        <button style={{
                            padding: '8px 16px',
                            background: 'rgba(74, 144, 226, 0.2)',
                            border: '1px solid #4A90E2',
                            borderRadius: '8px',
                            color: '#4A90E2',
                            cursor: 'pointer',
                            fontSize: '13px',
                            fontWeight: 'bold'
                        }}>
                            🔍 Zoom Out
                        </button>
                        <button style={{
                            padding: '8px 16px',
                            background: 'rgba(212, 175, 55, 0.2)',
                            border: '1px solid #D4AF37',
                            borderRadius: '8px',
                            color: '#D4AF37',
                            cursor: 'pointer',
                            fontSize: '13px',
                            fontWeight: 'bold'
                        }}>
                            📏 Measure Distance
                        </button>
                        <div style={{ marginLeft: 'auto', fontSize: '13px', color: 'rgba(255,255,255,0.7)' }}>
                            Grid: 5ft squares
                        </div>
                    </div>

                    {/* Battle Grid */}
                    <div style={{
                        background: 'rgba(0,0,0,0.6)',
                        borderRadius: '12px',
                        border: '1px solid rgba(212, 175, 55, 0.3)',
                        padding: '20px',
                        minHeight: '500px',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        backgroundImage: `
                        linear-gradient(rgba(74, 144, 226, 0.1) 1px, transparent 1px),
                        linear-gradient(90deg, rgba(74, 144, 226, 0.1) 1px, transparent 1px)
                    `,
                        backgroundSize: '40px 40px',
                        position: 'relative'
                    }}>
                        <div style={{ textAlign: 'center', color: 'rgba(255,255,255,0.4)', fontSize: '16px' }}>
                            <div style={{ fontSize: '48px', marginBottom: '16px' }}>🗺️</div>
                            <div>Battle Grid</div>
                            <div style={{ fontSize: '13px', marginTop: '8px' }}>
                                Drag and drop tokens to move characters
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Modal>
    );
};

export default MapModal;
