import React, { useState, useRef, useCallback, useEffect } from 'react';
import ReactDOM from 'react-dom';
import './Tooltip.css';

export interface TooltipProps {
    content: React.ReactNode;
    children: React.ReactNode;
    position?: 'top' | 'bottom' | 'left' | 'right';
    delay?: number;
    maxWidth?: number;
}

export const Tooltip: React.FC<TooltipProps> = ({
    content,
    children,
    position = 'top',
    delay = 500,
    maxWidth = 320,
}) => {
    const [isVisible, setIsVisible] = useState(false);
    const [isPinned, setIsPinned] = useState(false);
    const [coords, setCoords] = useState({ x: 0, y: 0 });
    const [adjustedPosition, setAdjustedPosition] = useState(position);
    const timeoutRef = useRef<NodeJS.Timeout | null>(null);
    const triggerRef = useRef<HTMLDivElement>(null);
    const tooltipRef = useRef<HTMLDivElement>(null);

    const calculatePosition = useCallback(() => {
        if (!triggerRef.current || !tooltipRef.current) return;

        const trigger = triggerRef.current.getBoundingClientRect();
        const tooltip = tooltipRef.current.getBoundingClientRect();
        const viewportWidth = window.innerWidth;
        const viewportHeight = window.innerHeight;
        const padding = 10;

        let finalX = 0;
        let finalY = 0;
        let finalPosition = position;

        // Calculate position based on preference
        if (position === 'top' || position === 'bottom') {
            finalX = trigger.left + trigger.width / 2;

            if (position === 'top') {
                finalY = trigger.top - tooltip.height - padding;
                // If doesn't fit on top, flip to bottom
                if (finalY < padding) {
                    finalPosition = 'bottom';
                    finalY = trigger.bottom + padding;
                }
            } else { // position === 'bottom'
                finalY = trigger.bottom + padding;
                // If doesn't fit on bottom, flip to top
                if (finalY + tooltip.height > viewportHeight - padding) {
                    finalPosition = 'top';
                    finalY = trigger.top - tooltip.height - padding;
                }
            }

            // Adjust horizontal to stay in viewport
            const halfWidth = tooltip.width / 2;
            if (finalX - halfWidth < padding) {
                finalX = halfWidth + padding;
            } else if (finalX + halfWidth > viewportWidth - padding) {
                finalX = viewportWidth - halfWidth - padding;
            }
        }
        // TODO: Add logic for 'left' and 'right' positions

        // Final vertical bounds check
        if (finalY < padding) finalY = padding;
        if (finalY + tooltip.height > viewportHeight - padding) {
            finalY = viewportHeight - tooltip.height - padding;
        }

        setCoords({ x: finalX, y: finalY });
        setAdjustedPosition(finalPosition);
    }, [position]);

    const handleMouseEnter = () => {
        if (isPinned) return;

        timeoutRef.current = setTimeout(() => {
            // Calculate position before showing
            // We need to render it first to get dimensions, so we might need a two-pass approach
            // or just render it hidden first.
            setIsVisible(true);
            // Use requestAnimationFrame to wait for render then calculate
            requestAnimationFrame(() => {
                calculatePosition();
            });
        }, delay);
    };

    const handleMouseLeave = () => {
        if (isPinned) return;

        if (timeoutRef.current) {
            clearTimeout(timeoutRef.current);
        }
        setIsVisible(false);
    };

    const handleClick = (e: React.MouseEvent) => {
        e.stopPropagation(); // Prevent immediate closing
        const newPinned = !isPinned;
        setIsPinned(newPinned);

        if (newPinned) {
            setIsVisible(true);
            requestAnimationFrame(() => calculatePosition());
        } else {
            // If unpinning, we might want to keep it visible if mouse is still hovering
            // But simple behavior is to close or revert to hover state.
            // Let's just keep it visible if hovering, but for now strict toggle is safer.
            // Actually, if we unpin, we should probably check hover state? 
            // For simplicity, let's just leave it visible but unpinned (so mouseleave will close it)
            // or close it immediately. User said "until user click outside".
            // If they click trigger again, it should probably close.
            setIsVisible(false);
        }
    };

    // Handle click outside to unpin
    useEffect(() => {
        if (!isPinned) return;

        const handleClickOutside = (e: MouseEvent) => {
            if (
                triggerRef.current &&
                !triggerRef.current.contains(e.target as Node) &&
                tooltipRef.current &&
                !tooltipRef.current.contains(e.target as Node)
            ) {
                setIsPinned(false);
                setIsVisible(false);
            }
        };

        // Use capture phase to detect clicks even if stopPropagation is called by modals
        document.addEventListener('mousedown', handleClickOutside, true);
        return () => document.removeEventListener('mousedown', handleClickOutside, true);
    }, [isPinned]);

    // Recalculate on scroll or resize
    useEffect(() => {
        if (!isVisible) return;

        const handleUpdate = () => calculatePosition();
        window.addEventListener('scroll', handleUpdate, true);
        window.addEventListener('resize', handleUpdate);

        return () => {
            window.removeEventListener('scroll', handleUpdate, true);
            window.removeEventListener('resize', handleUpdate);
        };
    }, [isVisible, calculatePosition]);

    const tooltipElement = (
        <div
            ref={tooltipRef}
            className={`tooltip tooltip-${adjustedPosition}`}
            style={{
                left: coords.x,
                top: coords.y,
                maxWidth,
                transform: adjustedPosition === 'top' || adjustedPosition === 'bottom'
                    ? 'translateX(-50%)'
                    : 'translateY(-50%)',
                position: 'fixed',
                zIndex: 10000,
                opacity: isVisible ? 1 : 0,
                // Enable pointer events when visible (hover or pinned) so user can scroll
                pointerEvents: isVisible ? 'auto' : 'none',
                transition: 'opacity 0.15s ease',
                visibility: isVisible ? 'visible' : 'hidden', // Ensure it doesn't block clicks when "hidden"
            }}
            role="tooltip"
            aria-live="polite"
            onClick={(e) => e.stopPropagation()} // Prevent clicks inside tooltip from closing it
            onMouseEnter={handleMouseEnter} // Keep tooltip visible when hovering over it
            onMouseLeave={handleMouseLeave} // Hide when leaving tooltip
        >
            <div className="tooltip-content">
                {content}
            </div>
        </div>
    );

    return (
        <div
            ref={triggerRef}
            className="tooltip-trigger"
            onMouseEnter={handleMouseEnter}
            onMouseLeave={handleMouseLeave}
            onClick={handleClick}
            style={{ display: 'inline-block', cursor: 'pointer' }}
        >
            {children}
            {ReactDOM.createPortal(tooltipElement, document.body)}
        </div>
    );
};
