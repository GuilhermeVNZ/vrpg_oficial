/**
 * Hook para gerenciar o estado do Voice HUD
 * 
 * Facilita a integração do VoiceHUD com o sistema de voz do VRPG Client.
 */

import { useState, useCallback, useEffect } from 'react';
import { VoiceHUDState } from '../components/VoiceHUD';

export interface UseVoiceHUDReturn {
  state: VoiceHUDState;
  statusText: string;
  startListening: () => void;
  startProcessing: () => void;
  startSpeaking: (text?: string) => void;
  showError: (message?: string) => void;
  hide: () => void;
}

export const useVoiceHUD = (
  autoHideDelay: number = 5000
): UseVoiceHUDReturn => {
  const [state, setState] = useState<VoiceHUDState>('hidden');
  const [statusText, setStatusText] = useState('');

  const startListening = useCallback(() => {
    setState('listening');
    setStatusText('Ouvindo...');
  }, []);

  const startProcessing = useCallback(() => {
    setState('processing');
    setStatusText('Processando...');
  }, []);

  const startSpeaking = useCallback((text?: string) => {
    setState('speaking');
    if (text) {
      setStatusText(text);
    } else {
      setStatusText('IA Falando...');
    }
  }, []);

  const showError = useCallback((message: string = 'Não entendi. Tente novamente.') => {
    setState('speaking'); // Usa estado speaking para mostrar erro
    setStatusText(message);
    // Auto-hide mais rápido em caso de erro
    setTimeout(() => {
      setState('hidden');
    }, 3000);
  }, []);

  const hide = useCallback(() => {
    setState('hidden');
    setStatusText('');
  }, []);

  // Auto-hide após delay (exceto quando está ouvindo)
  useEffect(() => {
    if (state === 'speaking' || state === 'processing') {
      const timer = setTimeout(() => {
        hide();
      }, autoHideDelay);
      return () => clearTimeout(timer);
    }
  }, [state, autoHideDelay, hide]);

  return {
    state,
    statusText,
    startListening,
    startProcessing,
    startSpeaking,
    showError,
    hide,
  };
};









