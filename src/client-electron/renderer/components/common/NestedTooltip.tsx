import React, { useRef, useEffect } from 'react';
import ReactDOM from 'react-dom';
import { TooltipDefinition } from '../../data/tooltipDefinitions';
import './NestedTooltip.css';

export interface NestedTooltipProps {
    definition: TooltipDefinition;
    position: { x: number; y: number };
    onClose: () => void;
}

export const NestedTooltip: React.FC<NestedTooltipProps> = ({
    definition,
    position,
    onClose,
}) => {
    const tooltipRef = useRef<HTMLDivElement>(null);

    // Adjust position to stay on screen
    useEffect(() => {
        if (!tooltipRef.current) return;

        const tooltip = tooltipRef.current;
        const rect = tooltip.getBoundingClientRect();
        const viewportWidth = window.innerWidth;
        const viewportHeight = window.innerHeight;

        let adjustedX = position.x;
        let adjustedY = position.y;

        // Keep tooltip on screen horizontally
        if (rect.right > viewportWidth) {
            adjustedX = viewportWidth - rect.width - 10;
        } else if (rect.left < 0) {
            adjustedX = 10;
        }

        // Keep tooltip on screen vertically
        if (rect.bottom > viewportHeight) {
            adjustedY = position.y - rect.height - 10;
        }

        if (adjustedX !== position.x || adjustedY !== position.y) {
            tooltip.style.left = `${adjustedX}px`;
            tooltip.style.top = `${adjustedY}px`;
        }
    }, [position]);

    const tooltipElement = (
        <div
            ref={tooltipRef}
            className="nested-tooltip"
            style={{
                left: position.x,
                top: position.y,
                transform: 'translateX(-50%)',
                position: 'fixed',
                zIndex: 10001, // Higher than regular tooltip (10000)
            }}
            role="tooltip"
            aria-live="polite"
            onClick={(e) => e.stopPropagation()}
        >
            <div className="nested-tooltip-header">
                <span className="nested-tooltip-title">{definition.title}</span>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    {definition.type && (
                        <span className="nested-tooltip-type">{definition.type}</span>
                    )}
                    <button
                        onClick={(e) => {
                            e.stopPropagation();
                            onClose();
                        }}
                        style={{
                            background: 'none',
                            border: 'none',
                            color: 'rgba(255,255,255,0.5)',
                            cursor: 'pointer',
                            padding: '0',
                            fontSize: '14px',
                            lineHeight: 1,
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center'
                        }}
                        aria-label="Close tooltip"
                    >
                        ✕
                    </button>
                </div>
            </div>

            <div className="nested-tooltip-content">
                {typeof definition.content === 'string'
                    ? definition.content
                    : definition.content}
            </div>
        </div>
    );

    return ReactDOM.createPortal(tooltipElement, document.body);
};
