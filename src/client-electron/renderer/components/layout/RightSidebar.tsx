import React, { useState, useEffect, useRef } from 'react';
import StatusCard, { StatusEffectProps } from './StatusCard';

// Mock D&D 5e Status Effects
const activeStatuses: StatusEffectProps[] = [
    {
        name: 'Peaceful Crane Stance',
        duration: '1 min',
        type: 'buff',
        source: 'Monk Feature',
        effect: 'Advantage on Athletics checks',
        description: 'The solar has perfect balance. She can stand on things too narrow or weak to expect the way (any wire, crumbling succents, your hand will reading to roll Athletics.'
    },
    {
        name: 'Poisoned',
        duration: '1 hr',
        type: 'condition',
        effect: 'Disadvantage on Attack Rolls & Ability Checks',
        description: 'A poisoned creature has disadvantage on attack rolls and ability checks.'
    },
    {
        name: 'Haste',
        duration: '9 rounds',
        type: 'buff',
        source: 'Spell',
        effect: '+2 AC, Double Speed, Extra Action',
        description: 'Choose a willing creature that you can see within range. Until the spell ends, the target\'s speed is doubled, it gains a +2 bonus to AC, it has advantage on Dexterity saving throws, and it gains an additional action on each of its turns.'
    }
];

interface ChatMessage {
    id: string;
    sender: string;
    text: string;
    timestamp: string;
    isSystem?: boolean;
}

const RightSidebar: React.FC = () => {
    const [messages, setMessages] = useState<ChatMessage[]>([
        { id: '1', sender: 'Gamemaster', text: 'The dragon roars, shaking the cavern walls!', timestamp: '5m ago', isSystem: true },
        { id: '2', sender: 'Victor', text: 'I draw my sword and prepare to strike!', timestamp: '4m ago' },
        { id: '3', sender: 'Shinta', text: 'Wait! I try to reason with it first.', timestamp: '3m ago' }
    ]);
    const [inputValue, setInputValue] = useState('');
    const chatContainerRef = useRef<HTMLDivElement>(null);

    // Auto-scroll to bottom on new message
    useEffect(() => {
        if (chatContainerRef.current) {
            chatContainerRef.current.scrollTop = chatContainerRef.current.scrollHeight;
        }
    }, [messages]);

    const handleSendMessage = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && inputValue.trim()) {
            const newMessage: ChatMessage = {
                id: Date.now().toString(),
                sender: 'Me',
                text: inputValue,
                timestamp: 'Just now'
            };
            setMessages(prev => [...prev, newMessage]);
            setInputValue('');
        }
    };

    return (
        <aside style={{
            position: 'absolute',
            top: '24px',
            right: '24px',
            bottom: '24px',
            width: '320px',
            display: 'flex',
            flexDirection: 'column',
            gap: '16px',
            zIndex: 20,
            pointerEvents: 'none' // Allow clicking through empty space
        }}>
            {/* Status Effects Section - Pointer events enabled for interaction */}
            <div style={{ pointerEvents: 'auto', display: 'flex', flexDirection: 'column', gap: '4px' }}>
                {activeStatuses.map((status, index) => (
                    <StatusCard key={index} {...status} />
                ))}
            </div>

            {/* Spacer */}
            <div style={{ flex: 1 }} />

            {/* Chat Area - Pointer events enabled */}
            <div style={{
                pointerEvents: 'auto',
                display: 'flex',
                flexDirection: 'column',
                gap: '12px',
                maxHeight: '50vh' // Limit height to allow scrolling
            }}>
                {/* Messages Container */}
                <div
                    ref={chatContainerRef}
                    style={{
                        display: 'flex',
                        flexDirection: 'column',
                        gap: '8px',
                        overflowY: 'auto',
                        paddingRight: '4px',
                        scrollbarWidth: 'none', // Firefox
                        msOverflowStyle: 'none', // IE/Edge
                    }}
                    className="chat-scroll-container"
                >
                    <style>{`
                        .chat-scroll-container::-webkit-scrollbar {
                            display: none;
                        }
                        @keyframes slideInLeft {
                            from {
                                opacity: 0;
                                transform: translateX(-20px);
                            }
                            to {
                                opacity: 1;
                                transform: translateX(0);
                            }
                        }
                    `}</style>

                    {messages.map((msg) => (
                        <div
                            key={msg.id}
                            className="glass-panel"
                            style={{
                                padding: '12px',
                                background: msg.isSystem ? 'rgba(212, 175, 55, 0.1)' : 'rgba(0,0,0,0.6)',
                                borderRadius: '12px',
                                borderLeft: msg.isSystem ? '2px solid #D4AF37' : 'none',
                                animation: 'slideInLeft 0.3s ease-out forwards'
                            }}
                        >
                            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                                <span style={{
                                    color: msg.isSystem ? '#D4AF37' : 'rgba(255,255,255,0.9)',
                                    fontSize: '12px',
                                    fontWeight: 600
                                }}>
                                    {msg.sender}
                                </span>
                                <span style={{ fontSize: '10px', opacity: 0.5 }}>{msg.timestamp}</span>
                            </div>
                            <div style={{ fontSize: '13px', color: 'rgba(255,255,255,0.8)', lineHeight: '1.4' }}>
                                {msg.text}
                            </div>
                        </div>
                    ))}
                </div>

                {/* Text Input */}
                <div style={{ marginTop: 'auto', marginBottom: '8px' }}>
                    <input
                        type="text"
                        value={inputValue}
                        onChange={(e) => setInputValue(e.target.value)}
                        onKeyDown={handleSendMessage}
                        placeholder="Enter message..."
                        style={{
                            width: '100%',
                            background: 'transparent',
                            border: 'none',
                            color: 'rgba(255,255,255,0.9)',
                            fontSize: '14px',
                            fontStyle: 'italic',
                            padding: '8px',
                            outline: 'none',
                            textAlign: 'right',
                            textShadow: '0 2px 4px rgba(0,0,0,0.5)'
                        }}
                    />
                </div>

                {/* Voice Activity Indicator */}
                <div className="glass-panel" style={{
                    padding: '12px 16px',
                    borderRadius: '24px',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '12px',
                    background: 'rgba(20, 20, 25, 0.9)',
                    border: '1px solid rgba(255,255,255,0.1)',
                }}>
                    {/* Animated Audio Orb */}
                    <div style={{
                        width: '40px',
                        height: '40px',
                        borderRadius: '50%',
                        position: 'relative',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        flexShrink: 0
                    }}>
                        {/* Outer pulse rings */}
                        <div style={{
                            position: 'absolute',
                            width: '100%',
                            height: '100%',
                            borderRadius: '50%',
                            border: '2px solid rgba(98, 0, 234, 0.4)',
                            animation: 'audioRipple 2s ease-out infinite'
                        }} />
                        <div style={{
                            position: 'absolute',
                            width: '100%',
                            height: '100%',
                            borderRadius: '50%',
                            border: '2px solid rgba(179, 136, 255, 0.3)',
                            animation: 'audioRipple 2s ease-out infinite 0.5s'
                        }} />

                        {/* Core orb */}
                        <div style={{
                            width: '24px',
                            height: '24px',
                            borderRadius: '50%',
                            background: 'linear-gradient(135deg, #6200ea 0%, #b388ff 100%)',
                            boxShadow: '0 0 20px rgba(98, 0, 234, 0.6), inset 0 0 10px rgba(255,255,255,0.3)',
                            animation: 'orbPulse 1.5s ease-in-out infinite',
                            position: 'relative',
                            zIndex: 2
                        }}>
                            {/* Inner glow */}
                            <div style={{
                                position: 'absolute',
                                top: '50%',
                                left: '50%',
                                transform: 'translate(-50%, -50%)',
                                width: '60%',
                                height: '60%',
                                borderRadius: '50%',
                                background: 'rgba(255,255,255,0.6)',
                                filter: 'blur(4px)'
                            }} />
                        </div>
                    </div>

                    {/* Status Text */}
                    <div style={{
                        flex: 1,
                        fontSize: '12px',
                        color: 'rgba(255,255,255,0.9)',
                        fontWeight: 500
                    }}>
                        Listening... | <span style={{ color: '#b388ff' }}>Gamemaster</span>
                    </div>

                    <button style={{
                        background: 'none',
                        border: 'none',
                        color: 'rgba(255,255,255,0.5)',
                        cursor: 'pointer',
                        fontSize: '16px',
                        padding: '4px'
                    }}>✕</button>
                </div>

                {/* CSS Animations */}
                <style>{`
                    @keyframes audioRipple {
                        0% {
                            transform: scale(1);
                            opacity: 0.6;
                        }
                        100% {
                            transform: scale(1.8);
                            opacity: 0;
                        }
                    }
                    
                    @keyframes orbPulse {
                        0%, 100% {
                            transform: scale(1);
                            box-shadow: 0 0 20px rgba(98, 0, 234, 0.6), inset 0 0 10px rgba(255,255,255,0.3);
                        }
                        50% {
                            transform: scale(1.1);
                            box-shadow: 0 0 30px rgba(98, 0, 234, 0.9), inset 0 0 15px rgba(255,255,255,0.5);
                        }
                    }
                `}</style>
            </div>
        </aside>
    );
};

export default RightSidebar;
