import React, { useEffect } from 'react';
import TopBar from './TopBar';
import LeftSidebar from './LeftSidebar';
import RightSidebar from './RightSidebar';
import BottomBar from './BottomBar';
import CenterCanvas from './CenterCanvas';
import PushToTalk from './PushToTalk';
import { useCinematicRoll } from '../../context/CinematicRollContext';
import '../../styles/global.css';

const MainLayout: React.FC = () => {
    const { triggerRoll } = useCinematicRoll();

    useEffect(() => {
        const handleKeyPress = (e: KeyboardEvent) => {
            if (e.key.toLowerCase() === 'd' && !e.ctrlKey && !e.altKey && !e.metaKey) {
                // Prevent triggering if user is typing in an input
                const target = e.target as HTMLElement;
                if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') return;

                // Trigger a test roll
                triggerRoll({
                    title: 'DC 14 ATHLETICS CHECK',
                    subtitle: '',
                    participants: [
                        {
                            id: '1',
                            name: 'Elara',
                            portrait: '/assets-and-models/portraits/female-elf.jpg', // Placeholder
                            color: '#D4AF37',
                            bonus: 5,
                            rollType: 'normal',
                            diceType: 'd20'
                        },
                        {
                            id: '2',
                            name: 'Thorne',
                            portrait: '/assets-and-models/portraits/male-wizard.jpg', // Placeholder
                            color: '#4A90E2',
                            bonus: 3,
                            rollType: 'advantage',
                            diceType: 'd20'
                        }
                    ]
                });
            }
        };

        window.addEventListener('keydown', handleKeyPress);
        return () => window.removeEventListener('keydown', handleKeyPress);
    }, [triggerRoll]);

    return (
        <div style={{
            display: 'grid',
            gridTemplateColumns: 'auto 1fr auto',
            gridTemplateRows: 'auto 1fr auto',
            height: '100vh',
            width: '100vw',
            overflow: 'hidden',
            position: 'relative',
            backgroundColor: 'var(--vrpg-color-bg-dark)'
        }}>
            {/* Top Bar (Spans full width) */}
            <div style={{ gridColumn: '1 / -1', gridRow: '1', zIndex: 10 }}>
                <TopBar />
            </div>

            {/* Left Sidebar */}
            <div style={{ gridColumn: '1', gridRow: '2', zIndex: 10 }}>
                <LeftSidebar />
            </div>

            {/* Center Area - Canvas */}
            <div style={{ gridColumn: '2', gridRow: '2', position: 'relative', overflow: 'hidden' }}>
                <CenterCanvas />
            </div>

            {/* Right Sidebar */}
            <div style={{ gridColumn: '3', gridRow: '2', zIndex: 10 }}>
                <RightSidebar />
            </div>

            {/* Push To Talk (Bottom Left Overlay) */}
            <div style={{ position: 'absolute', bottom: 0, left: 0, zIndex: 20 }}>
                <PushToTalk />
            </div>

            {/* Bottom Bar (Spans full width) */}
            <div style={{ gridColumn: '1 / -1', gridRow: '3', zIndex: 10 }}>
                <BottomBar />
            </div>
        </div>
    );
};

export default MainLayout;
