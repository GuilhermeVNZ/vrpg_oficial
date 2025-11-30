/**
 * Voice Interface HUD Component
 * 
 * HUD de interface de voz responsivo, moderno e animado, seguindo o estilo
 * visual "Glassmorphism/Arcane" do VRPG Client.
 * 
 * Estados:
 * - listening: Ouvindo entrada do usuário (azul)
 * - processing: Processando com IA (dourado)
 * - speaking: IA falando (roxo)
 */

import React, { useEffect, useState } from 'react';
import './VoiceHUD.css';

export type VoiceHUDState = 'listening' | 'processing' | 'speaking' | 'hidden';

interface VoiceHUDProps {
  state: VoiceHUDState;
  statusText?: string;
  onClose?: () => void;
  autoHideDelay?: number; // ms
}

export const VoiceHUD: React.FC<VoiceHUDProps> = ({
  state,
  statusText,
  onClose,
  autoHideDelay = 5000,
}) => {
  const [displayText, setDisplayText] = useState('');
  const [isVisible, setIsVisible] = useState(state !== 'hidden');

  useEffect(() => {
    setIsVisible(state !== 'hidden');
  }, [state]);

  useEffect(() => {
    if (state === 'speaking' && statusText) {
      // Efeito de máquina de escrever para texto da IA
      typeWriterEffect(statusText);
    } else if (statusText) {
      setDisplayText(statusText);
    } else {
      // Textos padrão por estado
      switch (state) {
        case 'listening':
          setDisplayText('Ouvindo...');
          break;
        case 'processing':
          setDisplayText('Processando...');
          break;
        case 'speaking':
          setDisplayText('IA Falando...');
          break;
        default:
          setDisplayText('');
      }
    }
  }, [state, statusText]);

  // Auto-hide após delay (exceto quando está ouvindo)
  useEffect(() => {
    if (state === 'speaking' || state === 'processing') {
      const timer = setTimeout(() => {
        if (onClose) {
          onClose();
        }
      }, autoHideDelay);
      return () => clearTimeout(timer);
    }
  }, [state, autoHideDelay, onClose]);

  const typeWriterEffect = (text: string) => {
    setDisplayText('');
    let i = 0;
    const speed = 30; // ms por caractere

    const type = () => {
      if (i < text.length) {
        setDisplayText(text.substring(0, i + 1));
        i++;
        setTimeout(type, speed);
      }
    };

    type();
  };

  if (!isVisible) {
    return null;
  }

  return (
    <div
      className={`voice-hud-container ${state}`}
      role="status"
      aria-live="polite"
      aria-label={`Estado de voz: ${state}`}
    >
      <div className="voice-visualizer" aria-hidden="true">
        <span></span>
        <span></span>
        <span></span>
        <span></span>
        <span></span>
      </div>
      <div className="voice-status-text" id="voice-status-text">
        {displayText}
      </div>
      {onClose && (
        <button
          className="voice-close-btn"
          onClick={onClose}
          aria-label="Cancelar voz"
          type="button"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      )}
    </div>
  );
};

export default VoiceHUD;









