// src/client-electron/renderer/components/modals/DiceRollModal.tsx
import React, { useEffect, useRef, useState } from 'react';
import Modal from './Modal';
import * as THREE from 'three';
import { RollManager } from '../../dice/RollManager';

interface DiceRollModalProps {
    isOpen: boolean;
    onClose: () => void;
}

const DiceRollModal: React.FC<DiceRollModalProps> = ({ isOpen, onClose }) => {
    const mountRef = useRef<HTMLDivElement>(null);
    const [result, setResult] = useState<number | null>(null);
    const [isRolling, setIsRolling] = useState(false);
    const rollManagerRef = useRef<RollManager | null>(null);

    const rollRequest = {
        modifier: 5,
        reason: 'Strength Modifier (+3)\nProficiency (+2)'
    };

    const handleRoll = async () => {
        if (isRolling || !rollManagerRef.current) return;
        setIsRolling(true);
        setResult(null);
        const rollResult = await rollManagerRef.current.roll('2d20', { advantage: true });
        if (rollResult) {
            setResult(rollResult.final);
        }
        setIsRolling(false);
    };

    useEffect(() => {
        if (!isOpen || !mountRef.current) return;

        const width = 700;
        const height = 450;

        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x0a0f19);

        const camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000);
        camera.position.set(0, 8, 12);
        camera.lookAt(0, 0, 0);

        const renderer = new THREE.WebGLRenderer({ antialias: true });
        renderer.setSize(width, height);
        renderer.shadowMap.enabled = true;
        renderer.shadowMap.type = THREE.PCFSoftShadowMap;

        if (mountRef.current) {
            mountRef.current.innerHTML = '';
            mountRef.current.appendChild(renderer.domElement);
        }

        // Lighting - reduced to ~10% of original
        const ambientLight = new THREE.AmbientLight(0xffffff, 0.1);
        scene.add(ambientLight);
        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.2);
        directionalLight.position.set(10, 20, 10);
        directionalLight.castShadow = true;
        scene.add(directionalLight);
        const pointLight1 = new THREE.PointLight(0xffd700, 0.25, 25);
        pointLight1.position.set(-8, 6, -8);
        scene.add(pointLight1);
        const pointLight2 = new THREE.PointLight(0x4dd0e1, 0.2, 20);
        pointLight2.position.set(8, 6, 8);
        scene.add(pointLight2);

        // Initialize RollManager
        const manager = new RollManager(scene);
        manager.addGround();
        rollManagerRef.current = manager;

        const animate = () => {
            requestAnimationFrame(animate);
            renderer.render(scene, camera);
        };
        animate();

        return () => {
            renderer.dispose();
            scene.clear();
            if (mountRef.current?.contains(renderer.domElement)) {
                mountRef.current.removeChild(renderer.domElement);
            }
        };
    }, [isOpen]);

    const totalResult = result !== null ? result + rollRequest.modifier : null;

    return (
        <Modal isOpen={isOpen} onClose={onClose} title="" maxWidth="900px" frameless={true}>
            <div style={{
                background: 'linear-gradient(135deg, rgba(15, 20, 30, 0.98) 0%, rgba(10, 15, 25, 0.99) 100%)',
                backdropFilter: 'blur(30px)',
                borderRadius: '12px',
                position: 'relative',
                border: '2px solid rgba(139, 115, 85, 0.8)',
                boxShadow: '0 0 40px rgba(77, 208, 225, 0.4), inset 0 0 30px rgba(0,0,0,0.6)',
                overflow: 'hidden'
            }}>
                {/* Decorative corners */}
                {[{ top: '-8px', left: '-8px' }, { top: '-8px', right: '-8px' }, { bottom: '-8px', left: '-8px' }, { bottom: '-8px', right: '-8px' }].map((pos, i) => (
                    <div key={`corner-${i}`} style={{
                        position: 'absolute',
                        ...pos,
                        width: '16px',
                        height: '16px',
                        background: 'radial-gradient(circle, #4dd0e1 20%, #2196f3 100%)',
                        borderRadius: '50%',
                        boxShadow: '0 0 20px #4dd0e1',
                        border: '2px solid rgba(139, 115, 85, 0.9)',
                        zIndex: 10
                    }} />
                ))}
                {/* Top/bottom gems */}
                {[{ top: '-10px', left: '50%' }, { bottom: '-10px', left: '50%' }].map((pos, i) => (
                    <div key={`gem-${i}`} style={{
                        position: 'absolute',
                        ...pos,
                        transform: 'translateX(-50%)',
                        width: '20px',
                        height: '20px',
                        background: 'radial-gradient(circle, #4dd0e1 10%, #2196f3 100%)',
                        borderRadius: '50%',
                        boxShadow: '0 0 25px #4dd0e1',
                        border: '2px solid rgba(139, 115, 85, 0.9)',
                        zIndex: 10
                    }} />
                ))}
                <div style={{ padding: '32px', position: 'relative' }}>
                    {/* Modifier - top left */}
                    <div style={{ position: 'absolute', top: '40px', left: '40px', zIndex: 5 }}>
                        <div style={{
                            fontSize: '72px',
                            fontWeight: 'bold',
                            color: '#ffffff',
                            textShadow: '0 4px 12px rgba(0,0,0,0.9), 0 0 30px rgba(77, 208, 225, 0.5)',
                            fontFamily: "'Cinzel', serif",
                            marginBottom: '12px',
                            lineHeight: '1'
                        }}>{`+${rollRequest.modifier}`}</div>
                        <div style={{
                            fontSize: '14px',
                            color: 'rgba(255,255,255,0.8)',
                            lineHeight: '1.5',
                            whiteSpace: 'pre-line',
                            textShadow: '0 2px 4px rgba(0,0,0,0.8)'
                        }}>{rollRequest.reason}</div>
                    </div>
                    {/* Dice area */}
                    <div style={{ width: '700px', height: '450px', margin: '0 auto', position: 'relative' }}>
                        <div ref={mountRef} style={{ width: '100%', height: '100%' }} />
                    </div>
                    {/* Result */}
                    {!isRolling && result !== null && (
                        <div style={{
                            position: 'absolute',
                            top: '50%',
                            right: '40px',
                            transform: 'translateY(-50%)',
                            textAlign: 'center',
                            animation: 'resultAppear 0.6s cubic-bezier(0.34, 1.56, 0.64, 1)'
                        }}>
                            <div style={{
                                fontSize: '16px',
                                color: 'rgba(255,255,255,0.6)',
                                marginBottom: '16px',
                                textTransform: 'uppercase',
                                letterSpacing: '3px',
                                fontWeight: 'bold'
                            }}>TOTAL</div>
                            <div style={{
                                fontSize: '108px',
                                fontWeight: 'bold',
                                color: '#FFD700',
                                textShadow: '0 6px 24px rgba(255, 215, 0, 0.9), 0 0 40px rgba(255, 215, 0, 0.5)',
                                fontFamily: "'Cinzel', serif",
                                lineHeight: '1'
                            }}>{totalResult}</div>
                            <div style={{
                                fontSize: '13px',
                                color: 'rgba(255,255,255,0.5)',
                                marginTop: '12px',
                                fontWeight: '500'
                            }}>{`(${result} + ${rollRequest.modifier})`}</div>
                        </div>
                    )}
                </div>
                {/* Buttons */}
                <div style={{ display: 'flex', justifyContent: 'center', gap: '32px', padding: '0 32px 32px 32px' }}>
                    <button
                        onClick={handleRoll}
                        disabled={isRolling}
                        style={{
                            padding: '20px 64px',
                            background: isRolling
                                ? 'rgba(77, 208, 225, 0.15)'
                                : 'linear-gradient(135deg, rgba(77, 208, 225, 0.6) 0%, rgba(33, 150, 243, 0.6) 100%)',
                            border: '2px solid #4dd0e1',
                            borderRadius: '10px',
                            color: '#4dd0e1',
                            fontSize: '22px',
                            fontWeight: 'bold',
                            cursor: isRolling ? 'not-allowed' : 'pointer',
                            textTransform: 'uppercase',
                            letterSpacing: '3px',
                            fontFamily: "'Cinzel', serif",
                            transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                            opacity: isRolling ? 0.4 : 1,
                            boxShadow: isRolling ? 'none' : '0 6px 24px rgba(77, 208, 225, 0.6)',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '16px'
                        }}
                        onMouseEnter={e => {
                            if (!isRolling) {
                                e.currentTarget.style.transform = 'translateY(-3px)';
                                e.currentTarget.style.boxShadow = '0 8px 32px rgba(77, 208, 225, 0.8)';
                            }
                        }}
                        onMouseLeave={e => {
                            e.currentTarget.style.transform = 'translateY(0)';
                            e.currentTarget.style.boxShadow = '0 6px 24px rgba(77, 208, 225, 0.6)';
                        }}
                    >
                        <span style={{ fontSize: '32px' }}>🎲</span>
                        ROLL
                    </button>
                    <button
                        onClick={onClose}
                        style={{
                            padding: '20px 64px',
                            background: 'rgba(139, 115, 85, 0.4)',
                            border: '2px solid rgba(139, 115, 85, 0.8)',
                            borderRadius: '10px',
                            color: '#d4af37',
                            fontSize: '22px',
                            fontWeight: 'bold',
                            cursor: 'pointer',
                            textTransform: 'uppercase',
                            letterSpacing: '3px',
                            fontFamily: "'Cinzel', serif",
                            transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '16px'
                        }}
                        onMouseEnter={e => {
                            e.currentTarget.style.background = 'rgba(139, 115, 85, 0.6)';
                            e.currentTarget.style.transform = 'translateY(-3px)';
                        }}
                        onMouseLeave={e => {
                            e.currentTarget.style.background = 'rgba(139, 115, 85, 0.4)';
                            e.currentTarget.style.transform = 'translateY(0)';
                        }}
                    >
                        <span style={{ fontSize: '28px' }}>✕</span>
                        CANCEL
                    </button>
                </div>
                <style>{`
          @keyframes resultAppear {
            0% { opacity: 0; transform: translateY(-50%) scale(0.5) rotate(-10deg); }
            60% { transform: translateY(-50%) scale(1.1) rotate(5deg); }
            100% { opacity: 1; transform: translateY(-50%) scale(1) rotate(0deg); }
          }
        `}</style>
            </div>
        </Modal>
    );
};

export default DiceRollModal;
