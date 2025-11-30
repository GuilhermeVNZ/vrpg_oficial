import React from 'react';
import Modal from './Modal';

interface SettingsModalProps {
    isOpen: boolean;
    onClose: () => void;
}

const SettingsModal: React.FC<SettingsModalProps> = ({ isOpen, onClose }) => {
    return (
        <Modal isOpen={isOpen} onClose={onClose} title="" maxWidth="700px" frameless={true}>
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
                    {/* Audio Settings */}
                    <Section title="Audio">
                        <Slider label="Master Volume" value={80} />
                        <Slider label="Sound Effects" value={70} />
                        <Slider label="Music" value={60} />
                        <Slider label="Voice Chat" value={90} />
                    </Section>

                    {/* Display Settings */}
                    <Section title="Display">
                        <ToggleOption label="Fullscreen" enabled={false} />
                        <ToggleOption label="VSync" enabled={true} />
                        <DropdownOption label="UI Scale" value="100%" options={['75%', '100%', '125%', '150%']} />
                    </Section>

                    {/* Accessibility */}
                    <Section title="Accessibility">
                        <ToggleOption label="High Contrast Mode" enabled={false} />
                        <ToggleOption label="Screen Reader Support" enabled={false} />
                        <ToggleOption label="Reduce Motion" enabled={false} />
                    </Section>

                    {/* Keybindings */}
                    <Section title="Keyboard Shortcuts">
                        <KeyBinding label="Character Sheet" binding="C" />
                        <KeyBinding label="Abilities" binding="A" />
                        <KeyBinding label="Inventory" binding="I" />
                        <KeyBinding label="Spells" binding="S" />
                        <KeyBinding label="Map" binding="M" />
                        <KeyBinding label="Journal" binding="J" />
                        <KeyBinding label="Compendium" binding="B" />
                        <KeyBinding label="Dice Roll" binding="D" />
                        <KeyBinding label="Settings" binding="Esc" />
                    </Section>
                </div>
            </div>
        </Modal>
    );
};

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
    <div style={{
        background: 'rgba(0,0,0,0.4)',
        padding: '20px',
        borderRadius: '16px',
        border: '1px solid rgba(212, 175, 55, 0.3)'
    }}>
        <div style={{ fontSize: '18px', fontWeight: 'bold', color: '#D4AF37', marginBottom: '16px', fontFamily: "'Crimson Text', serif" }}>
            {title}
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
            {children}
        </div>
    </div>
);

const Slider: React.FC<{ label: string; value: number }> = ({ label, value }) => (
    <div>
        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
            <span style={{ fontSize: '14px', color: '#FFF' }}>{label}</span>
            <span style={{ fontSize: '14px', color: '#4A90E2', fontWeight: 'bold' }}>{value}%</span>
        </div>
        <input
            type="range"
            min="0"
            max="100"
            value={value}
            style={{ width: '100%', accentColor: '#D4AF37' }}
            readOnly
        />
    </div>
);

const ToggleOption: React.FC<{ label: string; enabled: boolean }> = ({ label, enabled }) => (
    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <span style={{ fontSize: '14px', color: '#FFF' }}>{label}</span>
        <div style={{
            width: '48px',
            height: '24px',
            background: enabled ? '#4A90E2' : 'rgba(255,255,255,0.2)',
            borderRadius: '12px',
            position: 'relative',
            cursor: 'pointer',
            transition: 'background 0.2s ease'
        }}>
            <div style={{
                width: '20px',
                height: '20px',
                background: '#FFF',
                borderRadius: '50%',
                position: 'absolute',
                top: '2px',
                left: enabled ? '26px' : '2px',
                transition: 'left 0.2s ease'
            }} />
        </div>
    </div>
);

const DropdownOption: React.FC<{ label: string; value: string; options: string[] }> = ({ label, value }) => (
    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <span style={{ fontSize: '14px', color: '#FFF' }}>{label}</span>
        <select
            value={value}
            style={{
                background: 'rgba(0,0,0,0.5)',
                border: '1px solid rgba(212, 175, 55, 0.3)',
                borderRadius: '8px',
                padding: '6px 12px',
                color: '#FFF',
                fontSize: '14px',
                cursor: 'pointer'
            }}
        >
            <option>{value}</option>
        </select>
    </div>
);

const KeyBinding: React.FC<{ label: string; binding: string }> = ({ label, binding }) => (
    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <span style={{ fontSize: '14px', color: '#FFF' }}>{label}</span>
        <div style={{
            background: 'rgba(0,0,0,0.5)',
            border: '1px solid rgba(212, 175, 55, 0.3)',
            borderRadius: '6px',
            padding: '6px 12px',
            fontSize: '13px',
            color: '#D4AF37',
            fontFamily: 'monospace',
            fontWeight: 'bold'
        }}>
            {binding}
        </div>
    </div>
);

export default SettingsModal;
