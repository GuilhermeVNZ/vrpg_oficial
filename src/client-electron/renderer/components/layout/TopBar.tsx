import React from 'react';

const TopBar: React.FC = () => {
    return (
        <div style={{
            position: 'absolute',
            top: '24px',
            left: '24px',
            display: 'flex',
            alignItems: 'center',
            gap: '12px',
            zIndex: 20
        }}>
            {/* Level Circle */}
            <div style={{
                width: '64px',
                height: '64px',
                borderRadius: '50%',
                background: 'linear-gradient(135deg, #1a1a1a 0%, #0f0f0f 100%)',
                border: '2px solid var(--gold-frost)',
                boxShadow: '0 0 15px rgba(212, 175, 55, 0.5)',
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                justifyContent: 'center',
                zIndex: 2
            }}>
                <span style={{
                    fontSize: '10px',
                    color: 'rgba(255,255,255,0.6)',
                    textTransform: 'uppercase',
                    letterSpacing: '1px'
                }}>LEVEL</span>
                <span style={{
                    fontSize: '28px',
                    fontWeight: 'bold',
                    color: 'var(--gold-frost)',
                    lineHeight: '1'
                }}>5</span>
            </div>

            {/* XP Bar Container */}
            <div className="glass-panel" style={{
                height: '48px',
                padding: '0 24px 0 40px', // Padding left to accommodate circle overlap
                marginLeft: '-32px', // Overlap with circle
                display: 'flex',
                alignItems: 'center',
                borderRadius: '0 24px 24px 0',
                borderLeft: 'none',
                background: 'rgba(15, 15, 15, 0.8)',
                minWidth: '300px'
            }}>
                <div style={{ width: '100%', display: 'flex', flexDirection: 'column', gap: '4px' }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '12px', color: 'rgba(255,255,255,0.8)' }}>
                        <span style={{ color: 'var(--gold-frost)' }}>XP: 12,500 / 20,000</span>
                        <span style={{ opacity: 0.5 }}>Next Level</span>
                    </div>
                    <div style={{
                        width: '100%',
                        height: '8px',
                        background: 'rgba(0,0,0,0.5)',
                        borderRadius: '4px',
                        overflow: 'hidden',
                        border: '1px solid rgba(255,255,255,0.1)'
                    }}>
                        <div style={{
                            width: '62.5%',
                            height: '100%',
                            background: 'linear-gradient(90deg, var(--vrpg-color-arcane-blue) 0%, #64b5f6 100%)',
                            boxShadow: '0 0 10px var(--vrpg-color-arcane-glow)'
                        }} />
                    </div>
                </div>
            </div>
        </div >
    );
};

export default TopBar;
