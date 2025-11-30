import React, { useState, useEffect, useRef } from 'react';
import { IsometricRenderer } from './IsometricRenderer';

export const BattleMapContainer: React.FC = () => {
    const containerRef = useRef<HTMLDivElement>(null);
    const [dimensions, setDimensions] = useState({
        width: 0,
        height: 0
    });

    useEffect(() => {
        if (!containerRef.current) return;

        const updateDimensions = () => {
            if (containerRef.current) {
                setDimensions({
                    width: containerRef.current.clientWidth,
                    height: containerRef.current.clientHeight
                });
            }
        };

        // Initial size
        updateDimensions();

        const observer = new ResizeObserver(() => {
            updateDimensions();
        });

        observer.observe(containerRef.current);

        return () => observer.disconnect();
    }, []);

    // Don't render renderer until we have dimensions
    if (dimensions.width === 0) return <div ref={containerRef} style={{ width: '100%', height: '100%' }} />;

    return (
        <div ref={containerRef} style={{
            position: 'absolute',
            top: 0,
            left: 0,
            width: '100%',
            height: '100%',
            zIndex: 0
        }}>
            <IsometricRenderer
                width={dimensions.width}
                height={dimensions.height}
                gridWidth={200}
                gridHeight={200}
            />
        </div>
    );
};
