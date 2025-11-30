import React, { useState } from 'react';

export interface StatusEffectProps {
    name: string;
    duration: string;
    type: 'condition' | 'buff' | 'debuff' | 'environmental';
    description: string;
    effect: string;
    source?: string;
}

const StatusCard: React.FC<StatusEffectProps> = ({ name, duration, type, description, effect, source }) => {
    const [isHovered, setIsHovered] = useState(false);

    // Color mapping based on type
    const getBorderColor = () => {
        switch (type) {
            case 'condition': return '#FF5252'; // Red for negative conditions
            case 'buff': return '#D4AF37'; // Gold for buffs
            case 'debuff': return '#AB47BC'; // Purple for magical debuffs
            case 'environmental': return '#4CAF50'; // Green for environmental
            default: return '#D4AF37';
        }
    };

    const borderColor = getBorderColor();

    return (
        <div
            className="glass-panel"
            style={{
                padding: '12px 16px',
                borderLeft: `4px solid ${borderColor}`,
                background: 'rgba(15, 15, 15, 0.8)', // Slightly darker for better readability
                transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                cursor: 'help',
                overflow: 'hidden',
                maxHeight: isHovered ? '300px' : '60px', // Animate height
                marginBottom: '8px',
                backdropFilter: 'blur(12px)',
                boxShadow: isHovered ? '0 8px 32px rgba(0,0,0,0.5)' : '0 4px 12px rgba(0,0,0,0.2)',
                position: 'relative',
                zIndex: isHovered ? 10 : 1
            }}
            onMouseEnter={() => setIsHovered(true)}
            onMouseLeave={() => setIsHovered(false)}
        >
            {/* Header (Always Visible) */}
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', height: '36px' }}>
                <h4 style={{
                    fontSize: '15px',
                    color: borderColor,
                    margin: 0,
                    fontWeight: 600,
                    whiteSpace: 'nowrap',
                    overflow: 'hidden',
                    textOverflow: 'ellipsis',
                    maxWidth: '70%'
                }}>
                    {name}
                </h4>
                <span style={{ fontSize: '11px', color: 'rgba(255,255,255,0.5)', fontFamily: "'JetBrains Mono', monospace" }}>
                    {duration}
                </span>
            </div>

            {/* Expanded Content */}
            <div style={{
                opacity: isHovered ? 1 : 0,
                transition: 'opacity 0.2s ease-in-out',
                transitionDelay: isHovered ? '0.1s' : '0s', // Delay fade in slightly
                marginTop: '8px',
                pointerEvents: isHovered ? 'auto' : 'none'
            }}>
                <p style={{
                    fontSize: '10px',
                    color: 'rgba(255,255,255,0.5)',
                    textTransform: 'uppercase',
                    marginBottom: '8px',
                    letterSpacing: '0.5px'
                }}>
                    {type.toUpperCase()} {source ? `| ${source}` : ''}
                </p>

                <div style={{ fontSize: '13px', color: 'rgba(255,255,255,0.9)', lineHeight: '1.5' }}>
                    <p style={{ marginBottom: '8px', fontWeight: 500, color: '#FFF' }}>
                        Effect: <span style={{ color: 'rgba(255,255,255,0.8)', fontWeight: 400 }}>{effect}</span>
                    </p>
                    <p style={{ fontStyle: 'italic', color: 'rgba(255,255,255,0.6)', fontSize: '12px' }}>
                        {description}
                    </p>
                </div>
            </div>
        </div>
    );
};

export default StatusCard;
