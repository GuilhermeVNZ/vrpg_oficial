import React, { useState } from 'react';
import background from '../../assets/background.jpeg';
import { BattleMapContainer } from '../battlemap/BattleMapContainer';

const CenterCanvas: React.FC = () => {
    const [showBattleMap, setShowBattleMap] = useState(false);

    // Hotkey F to toggle battlemap
    React.useEffect(() => {
        const handleKeyPress = (e: KeyboardEvent) => {
            if (e.key.toLowerCase() === 'f' && !e.ctrlKey && !e.altKey && !e.metaKey) {
                const target = e.target as HTMLElement;
                if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') return;

                setShowBattleMap(prev => !prev);
                console.log(`[BattleMap] Toggled: ${!showBattleMap}`);
            }
        };

        window.addEventListener('keydown', handleKeyPress);
        return () => window.removeEventListener('keydown', handleKeyPress);
    }, [showBattleMap]);

    return (
        <div style={{ width: '100%', height: '100%', position: 'relative', overflow: 'hidden' }}>
            {/* Background estático (exploration mode) */}
            {!showBattleMap && (
                <div style={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    width: '100%',
                    height: '100%',
                    backgroundImage: `url(${background})`,
                    backgroundSize: 'cover',
                    backgroundPosition: 'center',
                    zIndex: 0
                }} />
            )}

            {/* Battlemap isométrico (combat mode) */}
            {showBattleMap && (
                <BattleMapContainer />
            )}
        </div>
    );
};

export default CenterCanvas;
