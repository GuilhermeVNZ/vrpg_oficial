import React from 'react';
import { useModal } from '../modals/ModalContext';

interface MenuItem {
    icon: React.ReactElement;
    label: string;
    shortcut: string;
    modalType: 'character-sheet' | 'abilities' | 'inventory' | 'spells' | 'map' | 'journal' | 'compendium' | 'settings' | 'skins';
}

const CharacterSheetIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="32" height="32" fill="none" style={{ display: 'block' }}>
        <defs>
            <linearGradient id="arcaneGold" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style={{ stopColor: '#4A90E2', stopOpacity: 1 }} />
                <stop offset="100%" style={{ stopColor: '#D4AF37', stopOpacity: 1 }} />
            </linearGradient>
            <filter id="glow">
                <feGaussianBlur stdDeviation="2" result="coloredBlur" />
                <feMerge>
                    <feMergeNode in="coloredBlur" />
                    <feMergeNode in="SourceGraphic" />
                </feMerge>
            </filter>
        </defs>
        <g stroke="url(#arcaneGold)" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" filter="url(#glow)">
            <path d="M32 4 C14 4 4 14 4 32 C4 48 16 60 32 60 C48 60 60 48 60 32 C60 14 50 4 32 4 Z" opacity="0.9" />
            <path d="M32 60 V12 M12 32 H52" opacity="0.4" strokeWidth="1.5" />
            <path d="M32 16 C36.4 16 40 19.6 40 24 C40 28.4 36.4 32 32 32 C27.6 32 24 28.4 24 24 C24 19.6 27.6 16 32 16 Z" fill="url(#arcaneGold)" fillOpacity="0.3" />
            <path d="M16 48 C16 39.2 23.2 32 32 32 C40.8 32 48 39.2 48 48" />
            <path d="M32 32 V48" opacity="0.6" />
        </g>
    </svg>
);

const AbilitiesIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="32" height="32" fill="none" style={{ display: 'block' }}>
        <defs>
            <linearGradient id="a_grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#4A90E2" />
                <stop offset="100%" stopColor="#D4AF37" />
            </linearGradient>
            <filter id="a_glow">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
            </filter>
        </defs>
        <g stroke="url(#a_grad)" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" filter="url(#a_glow)">
            <path d="M32 12 L36 26 L50 32 L36 38 L32 52 L28 38 L14 32 L28 26 Z" fill="url(#a_grad)" fillOpacity="0.2" />
            <path d="M28 32 C 20 20, 8 16, 4 18 C 6 32, 16 44, 28 50" fill="none" />
            <path d="M36 32 C 44 20, 56 16, 60 18 C 58 32, 48 44, 36 50" fill="none" />
        </g>
    </svg>
);

const InventoryIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="32" height="32" fill="none" style={{ display: 'block' }}>
        <defs>
            <linearGradient id="i_grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#4A90E2" />
                <stop offset="100%" stopColor="#D4AF37" />
            </linearGradient>
            <filter id="i_glow">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
            </filter>
        </defs>
        <g stroke="url(#i_grad)" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" filter="url(#i_glow)">
            <rect x="14" y="20" width="36" height="36" rx="4" fill="url(#i_grad)" fillOpacity="0.1" />
            <path d="M14 20 L18 8 H46 L50 20 Z" fill="url(#i_grad)" fillOpacity="0.15" />
            <path d="M24 8 V56 M40 8 V56" />
            <rect x="22" y="30" width="4" height="6" rx="1" fill="none" />
            <rect x="38" y="30" width="4" height="6" rx="1" fill="none" />
        </g>
    </svg>
);

const SpellsIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="32" height="32" fill="none" style={{ display: 'block' }}>
        <defs>
            <linearGradient id="s_grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#4A90E2" />
                <stop offset="100%" stopColor="#D4AF37" />
            </linearGradient>
            <filter id="s_glow">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
            </filter>
        </defs>
        <g stroke="url(#s_grad)" strokeWidth="2.5" strokeLinecap="round" filter="url(#s_glow)">
            <path d="M32 8 C 48 8, 56 20, 56 32 C 56 48, 44 56, 32 56 C 16 56, 8 44, 8 32 C 8 16, 20 8, 32 8" strokeOpacity="0.6" />
            <path d="M32 60 C 14 54, 14 30, 32 24 C 50 18, 54 42, 32 48 C 18 52, 18 36, 32 32" fill="none" strokeWidth="3" />
            <circle cx="32" cy="32" r="4" fill="url(#s_grad)" fillOpacity="0.3" />
        </g>
    </svg>
);

const MapIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="32" height="32" fill="none" style={{ display: 'block' }}>
        <defs>
            <linearGradient id="m_grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#4A90E2" />
                <stop offset="100%" stopColor="#D4AF37" />
            </linearGradient>
            <filter id="m_glow">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
            </filter>
        </defs>
        <g stroke="url(#m_grad)" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" filter="url(#m_glow)">
            <path d="M12 8 C 8 8, 4 12, 4 16 V48 C 4 56, 12 60, 20 56 L44 48 C 52 44, 60 48, 60 56 V24 C 60 16, 52 12, 44 16 L20 8 C 16 4, 12 8, 12 8 Z" fill="url(#m_grad)" fillOpacity="0.05" />
            <path d="M12 8 V44 C 12 52, 20 52, 20 44" opacity="0.5" />
            <path d="M52 20 V52 C 52 60, 60 60, 60 52" opacity="0.5" />
            <path d="M24 28 L32 36 L44 24" strokeDasharray="4 4" />
            <path d="M40 38 L48 46 M48 38 L40 46" strokeWidth="3.5" />
        </g>
    </svg>
);

const JournalIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="32" height="32" fill="none" style={{ display: 'block' }}>
        <defs>
            <linearGradient id="j_grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#4A90E2" />
                <stop offset="100%" stopColor="#D4AF37" />
            </linearGradient>
            <filter id="j_glow">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
            </filter>
        </defs>
        <g stroke="url(#j_grad)" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" filter="url(#j_glow)">
            <rect x="14" y="12" width="36" height="44" rx="4" fill="url(#j_grad)" fillOpacity="0.1" />
            <path d="M18 12 V56 M50 12 V56" opacity="0.5" />
            <path d="M14 18 H50 M14 50 H50" opacity="0.3" />
            <path d="M48 10 C 48 10, 60 2, 60 14 C 60 22, 44 36, 36 42 L 28 50 L 34 40 C 38 36, 52 28, 54 20 C 55 16, 52 14, 48 10 Z" fill="url(#j_grad)" fillOpacity="0.15" />
            <path d="M36 42 C 42 36, 54 22, 54 16" opacity="0.5" />
        </g>
    </svg>
);

const CompendiumIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 5 64 64" width="32" height="32" fill="none" style={{ display: 'block' }}>
        <defs>
            <linearGradient id="b_grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#4A90E2" />
                <stop offset="100%" stopColor="#D4AF37" />
            </linearGradient>
            <filter id="b_glow">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
            </filter>
        </defs>
        <g stroke="url(#b_grad)" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" filter="url(#b_glow)">
            <path d="M12 48 H52 C 56 48, 58 50, 58 54 C 58 58, 56 60, 52 60 H12 C 8 60, 6 58, 6 54 C 6 50, 8 48, 12 48 Z" fill="url(#b_grad)" fillOpacity="0.05" />
            <path d="M12 54 H56" opacity="0.3" />
            <path d="M14 32 H54 C 58 32, 60 34, 60 38 C 60 42, 58 44, 54 44 H14 C 10 44, 8 42, 8 38 C 8 34, 10 32, 14 32 Z" fill="url(#b_grad)" fillOpacity="0.05" />
            <path d="M14 38 H58" opacity="0.3" />
            <path d="M16 14 H50 C 54 14, 56 16, 56 20 C 56 24, 54 28, 50 28 H16 C 12 28, 10 24, 10 20 C 10 16, 12 14, 16 14 Z" fill="url(#b_grad)" fillOpacity="0.1" />
            <path d="M16 20 H54" opacity="0.3" />
            <path d="M20 14 V28 M46 14 V28" opacity="0.5" />
        </g>
    </svg>
);

const SettingsIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="32" height="32" fill="none" style={{ display: 'block' }}>
        <defs>
            <linearGradient id="e_grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#4A90E2" />
                <stop offset="100%" stopColor="#D4AF37" />
            </linearGradient>
            <filter id="e_glow">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
            </filter>
        </defs>
        <g stroke="url(#e_grad)" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" filter="url(#e_glow)">
            <circle cx="32" cy="32" r="10" fill="url(#e_grad)" fillOpacity="0.1" />
            <path d="M60 32 C 60 36, 58 39, 55 41 L 57 49 L 49 57 L 41 55 C 39 58, 36 60, 32 60 C 28 60, 25 58, 23 55 L 15 57 L 7 49 L 9 41 C 6 39, 4 36, 4 32 C 4 28, 6 25, 9 23 L 7 15 L 15 7 L 23 9 C 25 6, 28 4, 32 4 C 36 4, 39 6, 41 9 L 49 7 L 57 15 L 55 23 C 58 25, 60 28, 60 32 Z" />
            <circle cx="32" cy="32" r="20" opacity="0.5" />
        </g>
    </svg>
);

const SkinsIcon: React.FC = () => (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="32" height="32" fill="none" style={{ display: 'block' }}>
        <defs>
            <linearGradient id="k_grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#4A90E2" />
                <stop offset="100%" stopColor="#D4AF37" />
            </linearGradient>
            <filter id="k_glow">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
            </filter>
        </defs>
        <g stroke="url(#k_grad)" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" filter="url(#k_glow)">
            {/* Outer Hexagon */}
            <path d="M32 4 L58 19 L58 45 L32 60 L6 45 L6 19 Z" fill="url(#k_grad)" fillOpacity="0.05" />

            {/* Inner Triangle (The Face) */}
            <path d="M32 16 L46 40 L18 40 Z" fill="url(#k_grad)" fillOpacity="0.1" />

            {/* Connecting Lines */}
            <path d="M32 16 L32 4" />
            <path d="M32 16 L58 19" />
            <path d="M32 16 L6 19" />

            <path d="M46 40 L58 19" />
            <path d="M46 40 L58 45" />
            <path d="M46 40 L32 60" />

            <path d="M18 40 L6 19" />
            <path d="M18 40 L6 45" />
            <path d="M18 40 L32 60" />
        </g>

        {/* Number 20 */}
        <text
            x="32"
            y="34"
            textAnchor="middle"
            fontSize="16"
            fontFamily="Arial, sans-serif"
            fontWeight="bold"
            fill="url(#k_grad)"
            stroke="none"
            filter="url(#k_glow)"
            style={{ pointerEvents: 'none' }}
        >
            20
        </text>
    </svg>
);

const LeftSidebar: React.FC = () => {
    const { openModal } = useModal();

    const menuItems: MenuItem[] = [
        { icon: <CharacterSheetIcon />, label: 'Character Sheet', shortcut: 'C', modalType: 'character-sheet' },
        { icon: <AbilitiesIcon />, label: 'Abilities', shortcut: 'A', modalType: 'abilities' },
        { icon: <InventoryIcon />, label: 'Inventory', shortcut: 'I', modalType: 'inventory' },
        { icon: <SpellsIcon />, label: 'Spells', shortcut: 'S', modalType: 'spells' },
        { icon: <MapIcon />, label: 'Map', shortcut: 'M', modalType: 'map' },
        { icon: <JournalIcon />, label: 'Journal', shortcut: 'J', modalType: 'journal' },
        { icon: <CompendiumIcon />, label: 'Compendium', shortcut: 'B', modalType: 'compendium' },
        { icon: <SkinsIcon />, label: 'Skins', shortcut: 'K', modalType: 'skins' },
        { icon: <SettingsIcon />, label: 'Settings', shortcut: 'Esc', modalType: 'settings' },
    ];

    return (
        <aside style={{
            position: 'absolute',
            top: '100px',
            left: '24px',
            display: 'flex',
            flexDirection: 'column',
            gap: '8px',
            zIndex: 20
        }}>
            {menuItems.map((item, index) => (
                <button
                    key={index}
                    onClick={() => openModal(item.modalType)}
                    title={`${item.label} (${item.shortcut})`}
                    className="glass-interactive"
                    style={{
                        width: '80px',
                        height: '50px',
                        borderRadius: '8px',
                        border: '1px solid rgba(255, 255, 255, 0.3)',
                        background: 'linear-gradient(180deg, rgba(30,30,30,0.65) 0%, rgba(20,20,20,0.55) 100%)',
                        backdropFilter: 'blur(16px)',
                        boxShadow: '0 8px 16px rgba(0,0,0,0.6), 0 2px 4px rgba(0,0,0,0.4)',
                        position: 'relative',
                        overflow: 'hidden',
                        cursor: 'pointer',
                        transition: 'all 0.3s ease',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        fontSize: '28px',
                        color: '#fff'
                    }}
                >
                    <div style={{ position: 'relative', zIndex: 2 }}>
                        {item.icon}
                    </div>
                    {/* Hover glow */}
                    <div style={{
                        position: 'absolute',
                        top: 0, left: 0, right: 0, bottom: 0,
                        background: 'radial-gradient(circle at center, rgba(212, 175, 55, 0.2), transparent)',
                        opacity: 0,
                        transition: 'opacity 0.3s ease',
                        pointerEvents: 'none'
                    }} className="hover-glow" />
                </button>
            ))}
        </aside>
    );
};

export default LeftSidebar;
