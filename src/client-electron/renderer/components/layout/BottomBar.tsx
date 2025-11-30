import React from 'react';

const BottomBar: React.FC = () => {
    const party = [
        { name: 'Aramil', hp: 90, maxHp: 100, img: 'https://i.pravatar.cc/150?u=1' },
        { name: 'Lyra', hp: 75, maxHp: 100, img: 'https://i.pravatar.cc/150?u=2' },
        { name: 'Thorn', hp: 100, maxHp: 100, img: 'https://i.pravatar.cc/150?u=3' },
        { name: 'Elara', hp: 30, maxHp: 100, img: 'https://i.pravatar.cc/150?u=4' },
    ];

    return (
        <footer style={{
            position: 'absolute',
            bottom: '24px',
            left: '50%',
            transform: 'translateX(-50%)',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            gap: '12px',
            pointerEvents: 'none'
        }}>
            {/* Party Portraits (Above the bar) */}
            <div style={{
                pointerEvents: 'auto',
                display: 'flex',
                gap: '24px',
                zIndex: 10,
                marginBottom: '-25px'
            }}>
                {party.map((member, i) => (
                    <div key={i} style={{ position: 'relative', width: '110px', height: '110px' }}>
                        {/* Avatar Circle with Glass Border */}
                        <div style={{
                            width: '100%',
                            height: '100%',
                            borderRadius: '50%',
                            border: '2px solid rgba(255, 255, 255, 0.15)',
                            background: `url(${member.img}) center/cover`,
                            boxShadow: '0 8px 20px rgba(0,0,0,0.6), inset 0 0 20px rgba(0,0,0,0.8)',
                            position: 'relative',
                            zIndex: 2
                        }}>
                            {/* Inner Glass Shine */}
                            <div style={{
                                position: 'absolute',
                                top: 0, left: 0, right: 0, bottom: 0,
                                borderRadius: '50%',
                                background: 'radial-gradient(circle at 30% 30%, rgba(255,255,255,0.1) 0%, transparent 60%)',
                                pointerEvents: 'none'
                            }} />
                        </div>

                        {/* HP Arc Overlay */}
                        <div style={{ position: 'absolute', top: '-4px', left: '-4px', right: '-4px', bottom: '-4px', zIndex: 3, pointerEvents: 'none' }}>
                            <svg width="100%" height="100%" viewBox="0 0 100 100" style={{ transform: 'rotate(0deg)' }}>
                                <circle cx="50" cy="50" r="46" fill="none" stroke="rgba(0,0,0,0.8)" strokeWidth="5" strokeDasharray={`${46 * 2 * Math.PI / 2} ${46 * 2 * Math.PI}`} strokeLinecap="round" />
                                <circle cx="50" cy="50" r="46" fill="none" stroke="var(--health-green)" strokeWidth="5" strokeDasharray={`${(46 * 2 * Math.PI / 2) * (member.hp / member.maxHp)} ${46 * 2 * Math.PI}`} strokeLinecap="round" style={{ filter: 'drop-shadow(0 0 4px var(--health-green))' }} />
                            </svg>
                        </div>

                        {/* HP Text Label */}
                        <div style={{
                            position: 'absolute',
                            bottom: '-10px',
                            left: '50%',
                            transform: 'translateX(-50%)',
                            zIndex: 4,
                            fontSize: '12px',
                            fontWeight: 'bold',
                            color: '#fff',
                            textShadow: '0 2px 4px #000',
                            background: 'rgba(0,0,0,0.6)',
                            padding: '2px 6px',
                            borderRadius: '8px',
                            border: '1px solid rgba(255,255,255,0.1)'
                        }}>HP</div>
                    </div>
                ))}
            </div>

            {/* Gamemaster Narrative Panel */}
            <div style={{
                pointerEvents: 'none',
                maxWidth: '600px',
                width: '100%'
            }}>
                <div style={{
                    padding: '20px 24px',
                    background: 'rgba(15, 15, 15, 0.7)',
                    borderRadius: '16px',
                    border: 'none',
                    boxShadow: 'inset 0 0 20px rgba(74, 144, 226, 0.15), 0 4px 12px rgba(0,0,0,0.4)',
                    position: 'relative'
                }}>
                    {/* Inner glow */}
                    <div style={{
                        position: 'absolute',
                        top: 0, left: 0, right: 0, bottom: 0,
                        borderRadius: '14px',
                        background: 'radial-gradient(circle at 50% 0%, rgba(212, 175, 55, 0.08) 0%, transparent 70%)',
                        pointerEvents: 'none'
                    }} />

                    <div style={{ position: 'relative', zIndex: 1 }}>
                        <div style={{
                            fontSize: '16px',
                            fontWeight: 'bold',
                            color: '#D4AF37',
                            marginBottom: '8px',
                            fontFamily: "'Crimson Text', serif",
                            textShadow: '0 2px 8px rgba(212, 175, 55, 0.6)'
                        }}>Gamemaster:</div>
                        <div style={{
                            fontSize: '15px',
                            lineHeight: '1.6',
                            color: '#FFFFFF',
                            fontFamily: "'Inter', sans-serif",
                            textShadow: '0 2px 4px rgba(0,0,0,0.8)'
                        }}>
                            The dragon is recovering from its breath weapon. Declare your main action, bonus action, and movement.
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    );
};

export default BottomBar;
