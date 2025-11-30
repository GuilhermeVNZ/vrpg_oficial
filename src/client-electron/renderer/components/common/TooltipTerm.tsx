import React from 'react';
import { TOOLTIP_DEFINITIONS, TooltipDefinitionKey } from '../../data/tooltipDefinitions';
import { Tooltip } from './Tooltip';
import './TooltipTerm.css';

export interface TooltipTermProps {
    term: TooltipDefinitionKey;
    children: React.ReactNode;
}

export const TooltipTerm: React.FC<TooltipTermProps> = ({ term, children }) => {
    const definition = TOOLTIP_DEFINITIONS[term];

    const tooltipContent = definition ? (
        <div>
            <div style={{ fontWeight: 'bold', marginBottom: '4px', color: '#D4AF37' }}>
                {definition.title}
            </div>
            <div style={{ whiteSpace: 'pre-wrap' }}>
                {definition.content}
            </div>
            {/* Removed "Ctrl+Click" hint since it's now click-to-pin or hover */}
        </div>
    ) : null;

    return (
        <Tooltip content={tooltipContent}>
            <span
                className="tooltip-term"
                role="button"
                tabIndex={0}
                aria-label={`${children}. Click to pin details`}
            >
                {children}
            </span>
        </Tooltip>
    );
};
