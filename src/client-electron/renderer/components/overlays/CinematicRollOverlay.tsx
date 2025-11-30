import React, { useEffect, useRef, useState } from 'react';
import { useCinematicRoll } from '../../context/CinematicRollContext';
import { DiceEngine, DiceEngineHandle } from '../../dice/engine/DiceEngine';
import { DiceRollResult } from '../../dice/DiceBoxWrapper'; // Keep interface for now or move it

// Audio asset paths (adjust if needed)
const INTRO_AUDIO_PATH = '/assets-and-models/songs/Rolling Thunder.mp3';
const DICE_ROLL_AUDIO_PATH = '/assets-and-models/SFX/RollingDiceSound.mp3';
const RESULT_AUDIO_PATH = '/assets-and-models/songs/EndResultRoll.mp3';

export const CinematicRollOverlay: React.FC = () => {
    const { rollRequest, triggerRoll, clearRoll } = useCinematicRoll();
    const [phase, setPhase] = useState<'idle' | 'intro' | 'participants' | 'rolling' | 'result'>('idle');
    const [rollResults, setRollResults] = useState<DiceRollResult | null>(null);
    const [isDragging, setIsDragging] = useState(false);
    const [dragStart, setDragStart] = useState<{ x: number, y: number } | null>(null);
    const [dragCurrent, setDragCurrent] = useState<{ x: number, y: number } | null>(null);

    const audioRef = useRef<HTMLAudioElement | null>(null);
    const diceRef = useRef<DiceEngineHandle | null>(null);

    // Initialise intro audio once
    useEffect(() => {
        audioRef.current = new Audio(INTRO_AUDIO_PATH);
        audioRef.current.volume = 0.5;
    }, []);

    // React to roll request changes
    useEffect(() => {
        if (rollRequest) {
            startSequence();
        } else {
            setPhase('idle');
            setRollResults(null);
            diceRef.current?.clear();
        }
    }, [rollRequest]);

    const startSequence = async () => {
        if (!rollRequest) return;
        setPhase('intro');
        audioRef.current?.play().catch(e => console.warn('Intro audio failed', e));
        await new Promise(res => setTimeout(res, 2000)); // intro duration
        setPhase('participants');
    };

    // Dev helper to create a roll request
    const handleRollButtonClick = () => {
        if (!rollRequest) {
            triggerRoll({
                title: 'Test Roll',
                subtitle: 'Against DC 15',
                participants: [
                    {
                        id: 'dev1',
                        name: 'Dev Tester',
                        portrait: '/assets-and-models/portraits/victor.png',
                        color: '#ff0000',
                        bonus: 2,
                        rollType: 'normal',
                        diceType: 'd20'
                    }
                ]
            });
        }
    };

    const performRoll = async (throwVector?: { x: number, y: number }) => {
        if (!rollRequest || !diceRef.current) {
            console.error('Missing rollRequest or dice component');
            return;
        }
        const rollAudio = new Audio(DICE_ROLL_AUDIO_PATH);
        rollAudio.volume = 0.7;
        rollAudio.play().catch(e => console.warn('Roll audio failed', e));

        const primary = rollRequest.participants[0];
        if (!primary) {
            console.error('No participants');
            clearRoll();
            return;
        }
        const notation = `1${primary.diceType}${primary.bonus >= 0 ? '+' + primary.bonus : primary.bonus}`;
        try {
            const totalValue = await diceRef.current.roll(notation, throwVector);
            const result: DiceRollResult = {
                total: totalValue + primary.bonus,
                rolls: [{ dice: primary.diceType, value: totalValue }]
            };
            setRollResults(result);
            await new Promise(res => setTimeout(res, 500)); // small pause

            // Stop intro music before playing result music
            if (audioRef.current) {
                audioRef.current.pause();
                audioRef.current.currentTime = 0;
            }

            const resultAudio = new Audio(RESULT_AUDIO_PATH);
            resultAudio.volume = 0.6;
            resultAudio.play().catch(e => console.warn('Result audio failed', e));
            setPhase('result');
            if (rollRequest.onComplete) {
                rollRequest.onComplete([
                    {
                        dice: result.rolls.map(r => ({ value: r.value, faceIndex: r.value - 1 })),
                        final: result.total,
                        modifier: primary.bonus
                    } as any
                ]);
            }
            setTimeout(() => clearRoll(), 4000);
        } catch (err) {
            console.error('Roll error', err);
            clearRoll();
        }
    };

    // Listen for 'D' key to trigger roll (dev helper)
    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key === 'd' || e.key === 'D') {
                if (rollRequest) {
                    clearRoll();
                } else {
                    handleRollButtonClick();
                }
            }
        };
        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [rollRequest, clearRoll]);

    // Idle state – render nothing
    if (phase === 'idle' && !rollRequest) {
        return null;
    }

    // Main overlay for cinematic phases
    return (
        <div
            style={{
                position: 'fixed',
                inset: 0,
                zIndex: 9999,
                pointerEvents: phase === 'rolling' ? 'none' : 'auto',
                display: 'flex',
                flexDirection: 'column',
                justifyContent: 'center',
                alignItems: 'center',
                fontFamily: "'Cinzel', serif"
            }}
        >
            {/* Dark overlay */}
            <div
                style={{
                    position: 'absolute',
                    inset: 0,
                    background: 'rgba(0,0,0,0.6)',
                    backdropFilter: 'blur(8px)',
                    zIndex: 0
                }}
            />

            {/* Dice Engine */}
            <DiceEngine ref={diceRef} />

            {/* UI strip */}
            <div
                style={{
                    position: 'absolute',
                    width: '100%',
                    height: '250px',
                    background: 'rgba(20,20,20,0.6)',
                    backdropFilter: 'blur(24px)',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    boxShadow: '0 8px 32px rgba(0,0,0,0.4), inset 0 1px 0 rgba(255,255,255,0.1)',
                    borderTop: '1px solid rgba(255,255,255,0.1)',
                    borderBottom: '1px solid rgba(255,255,255,0.1)',
                    zIndex: 10,
                    overflow: 'hidden'
                }}
            >
                <div
                    style={{
                        width: '100%',
                        maxWidth: '1200px',
                        display: 'flex',
                        flexDirection: 'column',
                        justifyContent: 'center',
                        alignItems: 'center',
                        color: 'white',
                        textShadow: '0 2px 4px rgba(0,0,0,0.8)'
                    }}
                >
                    {/* Intro */}
                    {phase === 'intro' && rollRequest && (
                        <div style={{ textAlign: 'center', animation: 'fadeIn 0.5s' }}>
                            <h1 style={{ fontSize: '3rem', margin: 0, letterSpacing: '0.1em' }}>{rollRequest.title}</h1>
                            <h2 style={{ fontSize: '1.5rem', margin: '10px 0 0', color: '#D4AF37' }}>{rollRequest.subtitle}</h2>
                        </div>
                    )}

                    {/* Participants */}
                    {phase === 'participants' && rollRequest && (
                        <div style={{ display: 'flex', gap: '40px', animation: 'fadeIn 0.5s' }}>
                            {rollRequest.participants.map(p => (
                                <div key={p.id} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
                                    <div
                                        style={{
                                            width: '120px',
                                            height: '120px',
                                            borderRadius: '50%',
                                            backgroundImage: `url(${p.portrait})`,
                                            backgroundSize: 'cover',
                                            backgroundPosition: 'center',
                                            border: '3px solid #D4AF37',
                                            boxShadow: '0 5px 15px rgba(0,0,0,0.5)',
                                            marginBottom: '10px'
                                        }}
                                    />
                                    <div style={{ fontSize: '1.2rem', fontWeight: 'bold' }}>{p.name}</div>
                                    <div style={{ color: '#D4AF37', fontSize: '1rem' }}>
                                        {p.bonus >= 0 ? `+${p.bonus}` : p.bonus}
                                        {p.rollType !== 'normal' && (
                                            <span style={{ marginLeft: '5px', fontSize: '0.8em' }}>({p.rollType.toUpperCase()})</span>
                                        )}
                                    </div>
                                </div>
                            ))}
                        </div>
                    )}

                    {/* Rolling phase - show participants without button */}
                    {phase === 'rolling' && rollRequest && (
                        <div style={{ display: 'flex', gap: '40px', animation: 'fadeIn 0.5s' }}>
                            {rollRequest.participants.map(p => (
                                <div key={p.id} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
                                    <div
                                        style={{
                                            width: '120px',
                                            height: '120px',
                                            borderRadius: '50%',
                                            backgroundImage: `url(${p.portrait})`,
                                            backgroundSize: 'cover',
                                            backgroundPosition: 'center',
                                            border: '3px solid #D4AF37',
                                            boxShadow: '0 5px 15px rgba(0,0,0,0.5)',
                                            marginBottom: '10px'
                                        }}
                                    />
                                    <div style={{ fontSize: '1.2rem', fontWeight: 'bold' }}>{p.name}</div>
                                    <div style={{ color: '#D4AF37', fontSize: '1rem' }}>
                                        {p.bonus >= 0 ? `+${p.bonus}` : p.bonus}
                                        {p.rollType !== 'normal' && (
                                            <span style={{ marginLeft: '5px', fontSize: '0.8em' }}>({p.rollType.toUpperCase()})</span>
                                        )}
                                    </div>
                                </div>
                            ))}
                        </div>
                    )}

                    {/* Result */}
                    {phase === 'result' && rollResults && rollRequest && (
                        <div style={{ textAlign: 'center', animation: 'zoomIn 0.3s' }}>
                            <h1 style={{ fontSize: '4rem', margin: 0, color: '#fff', textTransform: 'uppercase' }}>
                                {rollResults.total >= (rollRequest.dc || 15) ? 'SUCCESS!' : 'FAILED!'}
                            </h1>
                            <div style={{ fontSize: '2rem', color: '#D4AF37', marginTop: '10px' }}>
                                TOTAL: {rollResults.total}
                            </div>
                        </div>
                    )}
                </div>
            </div>

            {/* Hand gesture tutorial - positioned at bottom */}
            {phase === 'participants' && (
                <>
                    {/* Subtle hand gesture animation */}
                    {!isDragging && (
                        <div
                            style={{
                                position: 'absolute',
                                bottom: '100px',
                                left: '50%',
                                transform: 'translateX(-50%)',
                                display: 'flex',
                                flexDirection: 'column',
                                alignItems: 'center',
                                gap: '15px',
                                animation: 'fadeIn 0.5s, pulse 2s infinite',
                                opacity: 0.7,
                                zIndex: 20,
                                pointerEvents: 'none'
                            }}
                        >
                            {/* Hand icon with drag motion */}
                            <svg width="80" height="80" viewBox="0 0 100 100" style={{ filter: 'drop-shadow(0 2px 8px rgba(212, 175, 55, 0.3))' }}>
                                <defs>
                                    <linearGradient id="handGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                                        <stop offset="0%" style={{ stopColor: '#F2C94C', stopOpacity: 1 }} />
                                        <stop offset="100%" style={{ stopColor: '#D4AF37', stopOpacity: 1 }} />
                                    </linearGradient>
                                </defs>
                                {/* Hand outline */}
                                <path d="M 50 30 Q 45 25 40 30 L 40 60 Q 40 70 50 70 Q 60 70 60 60 L 60 30 Q 55 25 50 30 Z"
                                    fill="url(#handGradient)" stroke="#D4AF37" strokeWidth="2" />
                                {/* Finger */}
                                <circle cx="50" cy="25" r="5" fill="url(#handGradient)" stroke="#D4AF37" strokeWidth="2" />
                                {/* Drag arrow */}
                                <path d="M 70 50 Q 80 50 85 55 L 82 52 M 85 55 L 82 58"
                                    stroke="#D4AF37" strokeWidth="2" fill="none" strokeLinecap="round"
                                    style={{ animation: 'slideArrow 1.5s ease-in-out infinite' }} />
                            </svg>
                            <div style={{
                                color: '#D4AF37',
                                fontSize: '0.9rem',
                                fontWeight: 600,
                                textTransform: 'uppercase',
                                letterSpacing: '0.1em',
                                textShadow: '0 2px 4px rgba(0,0,0,0.8)'
                            }}>
                                Drag para lançar
                            </div>
                        </div>
                    )}

                    {/* Interactive overlay for dragging */}
                    <div
                        style={{
                            position: 'absolute',
                            inset: 0,
                            cursor: isDragging ? 'grabbing' : 'grab',
                            zIndex: 15
                        }}
                        onMouseDown={(e) => {
                            const start = {
                                x: e.clientX,
                                y: e.clientY
                            };
                            setDragStart(start);
                            setDragCurrent(start);
                            setIsDragging(true);
                        }}
                    />

                    {/* Improved drag arrow visualization - High Visibility Glass/Neon */}
                    {isDragging && dragStart && dragCurrent && (() => {
                        const dx = dragCurrent.x - dragStart.x;
                        const dy = dragCurrent.y - dragStart.y;
                        const distance = Math.sqrt(dx * dx + dy * dy);
                        const angle = Math.atan2(dy, dx) * 180 / Math.PI;

                        // Calculate end point for the line (stop before the tip)
                        // The tip is approx 20px long, so stop 20px early
                        const stopDistance = Math.max(0, distance - 25);
                        const ratio = distance > 0 ? stopDistance / distance : 0;
                        const lineEnd = {
                            x: dragStart.x + dx * ratio,
                            y: dragStart.y + dy * ratio
                        };

                        return (
                            <svg
                                style={{
                                    position: 'absolute',
                                    top: 0,
                                    left: 0,
                                    width: '100%',
                                    height: '100%',
                                    pointerEvents: 'none',
                                    zIndex: 19,
                                    overflow: 'visible'
                                }}
                            >
                                <defs>
                                    {/* Strong Neon Glow */}
                                    <filter id="neonGlow" x="-50%" y="-50%" width="200%" height="200%">
                                        <feGaussianBlur in="SourceGraphic" stdDeviation="2" result="blur" />
                                        <feDropShadow dx="0" dy="0" stdDeviation="4" floodColor="#D4AF37" floodOpacity="0.8" />
                                        <feDropShadow dx="0" dy="0" stdDeviation="8" floodColor="#F2C94C" floodOpacity="0.4" />
                                        <feComposite in="SourceGraphic" in2="blur" operator="over" />
                                    </filter>
                                </defs>

                                {/* Anchor Point - Pulsing Target */}
                                <g transform={`translate(${dragStart.x}, ${dragStart.y})`}>
                                    {/* Outer Ring */}
                                    <circle r="15" fill="none" stroke="#D4AF37" strokeWidth="1" opacity="0.3">
                                        <animate attributeName="r" values="15;20;15" dur="2s" repeatCount="indefinite" />
                                        <animate attributeName="opacity" values="0.3;0;0.3" dur="2s" repeatCount="indefinite" />
                                    </circle>
                                    {/* Inner Dot */}
                                    <circle r="4" fill="#FFF" filter="url(#neonGlow)" />
                                </g>

                                {/* The Line - Solid Core with Glow */}
                                <line
                                    x1={dragStart.x}
                                    y1={dragStart.y}
                                    x2={lineEnd.x}
                                    y2={lineEnd.y}
                                    stroke="#FFF"
                                    strokeWidth="2"
                                    strokeLinecap="round"
                                    filter="url(#neonGlow)"
                                    opacity={0.8}
                                />

                                {/* Secondary Glow Line for Thickness */}
                                <line
                                    x1={dragStart.x}
                                    y1={dragStart.y}
                                    x2={lineEnd.x}
                                    y2={lineEnd.y}
                                    stroke="#D4AF37"
                                    strokeWidth={Math.min(distance / 20, 6)} // Dynamic thickness
                                    strokeLinecap="round"
                                    opacity="0.4"
                                    filter="url(#neonGlow)"
                                />

                                {/* The Arrowhead - Sharp Chevron */}
                                <g transform={`translate(${dragCurrent.x}, ${dragCurrent.y}) rotate(${angle})`}>
                                    {/* Glowy backing */}
                                    <path
                                        d="M -20 -12 L 0 0 L -20 12"
                                        fill="none"
                                        stroke="#D4AF37"
                                        strokeWidth="4"
                                        strokeLinecap="round"
                                        strokeLinejoin="round"
                                        opacity="0.5"
                                        filter="url(#neonGlow)"
                                    />
                                    {/* Sharp white core */}
                                    <path
                                        d="M -20 -12 L 0 0 L -20 12"
                                        fill="none"
                                        stroke="#FFF"
                                        strokeWidth="2"
                                        strokeLinecap="round"
                                        strokeLinejoin="round"
                                        filter="url(#neonGlow)"
                                    />
                                </g>
                            </svg>
                        );
                    })()}
                </>
            )}

            {/* Global mouse move/up handlers for dragging */}
            {isDragging && phase === 'participants' && (
                <div
                    style={{
                        position: 'fixed',
                        inset: 0,
                        zIndex: 25,
                        cursor: 'grabbing'
                    }}
                    onMouseMove={(e) => {
                        setDragCurrent({ x: e.clientX, y: e.clientY });
                    }}
                    onMouseUp={(e) => {
                        if (!dragStart) {
                            setIsDragging(false);
                            return;
                        }

                        const dragVector = {
                            x: e.clientX - dragStart.x,
                            y: e.clientY - dragStart.y
                        };

                        const magnitude = Math.sqrt(dragVector.x ** 2 + dragVector.y ** 2);
                        const MIN_DRAG_DISTANCE = 50; // Increased minimum for better UX

                        if (magnitude >= MIN_DRAG_DISTANCE) {
                            // Roll with custom throw vector
                            setPhase('rolling');
                            performRoll(dragVector);
                        } else {
                            // Cancel: show brief feedback
                            console.log('[CinematicRoll] Drag too short, cancelled');
                        }

                        // Reset drag state
                        setIsDragging(false);
                        setDragStart(null);
                        setDragCurrent(null);
                    }}
                    onMouseLeave={() => {
                        // Cancel drag if mouse leaves window
                        setIsDragging(false);
                        setDragStart(null);
                        setDragCurrent(null);
                    }}
                />
            )}
        </div>
    );
};
