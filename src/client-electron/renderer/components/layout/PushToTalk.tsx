import React, { useState } from 'react';

const PushToTalk: React.FC = () => {
    const [isTalking, setIsTalking] = useState(false);

    return (
        <div style={{
            position: 'absolute',
            bottom: '24px',
            left: '24px',
            display: 'flex',
            flexDirection: 'column',
            gap: '8px',
            zIndex: 20
        }}>
            {/* Main Mic Panel */}
            <div
                className="glass-panel"
                style={{
                    width: '100px',
                    height: '100px',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    borderRadius: '16px',
                    background: 'linear-gradient(135deg, rgba(30,30,30,0.5), rgba(10,10,10,0.6))',
                    border: '1px solid rgba(255,255,255,0.1)',
                    boxShadow: '0 8px 32px rgba(0,0,0,0.5)',
                    position: 'relative',
                    overflow: 'hidden'
                }}
            >
                {/* Active State Glow */}
                {isTalking && (
                    <div style={{
                        position: 'absolute',
                        top: 0, left: 0, right: 0, bottom: 0,
                        background: 'radial-gradient(circle, rgba(76, 175, 80, 0.2) 0%, transparent 70%)',
                        animation: 'pulse 1.5s infinite'
                    }} />
                )}

                {/* Mic Button */}
                <button
                    onMouseDown={() => setIsTalking(true)}
                    onMouseUp={() => setIsTalking(false)}
                    onMouseLeave={() => setIsTalking(false)}
                    style={{
                        width: '64px',
                        height: '64px',
                        borderRadius: '50%',
                        border: '2px solid rgba(255,255,255,0.2)',
                        background: isTalking ? 'var(--health-green)' : 'rgba(255,255,255,0.05)',
                        color: isTalking ? '#000' : '#fff',
                        fontSize: '32px',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        cursor: 'pointer',
                        transition: 'all 0.2s ease',
                        outline: 'none',
                        zIndex: 2
                    }}
                >
                    🎙️
                </button>

                {/* Side Indicator (Volume Level Placeholder) */}
                <div style={{
                    position: 'absolute',
                    right: '0',
                    top: '10px',
                    bottom: '10px',
                    width: '4px',
                    background: 'rgba(0,0,0,0.5)',
                    borderRadius: '2px 0 0 2px'
                }}>
                    <div style={{
                        width: '100%',
                        height: '60%',
                        background: 'var(--health-green)',
                        position: 'absolute',
                        bottom: 0,
                        borderRadius: '2px 0 0 2px'
                    }} />
                </div>
            </div>

            {/* Status Bar */}
            <div
                className="glass-panel"
                style={{
                    padding: '6px 12px',
                    borderRadius: '8px',
                    background: 'rgba(10,10,10,0.5)',
                    border: '1px solid rgba(255,255,255,0.1)',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    fontSize: '11px',
                    color: '#aaa',
                    width: '140px' // Slightly wider than the mic panel
                }}
            >
                <div style={{ display: 'flex', gap: '8px' }}>
                    <span>Latency: <span style={{ color: 'var(--health-green)' }}>17ms</span></span>
                </div>
                <span>▲</span>
            </div>
        </div>
    );
};

export default PushToTalk;
