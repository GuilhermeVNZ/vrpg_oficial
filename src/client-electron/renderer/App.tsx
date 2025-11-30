import React from 'react';
import MainLayout from './components/layout/MainLayout';
import { ModalProvider } from './components/modals/ModalContext';
import ModalManager from './components/modals/ModalManager';
import { CinematicRollProvider } from './context/CinematicRollContext';
import { CharacterProvider } from './context/CharacterContext';
import { CinematicRollOverlay } from './components/overlays/CinematicRollOverlay';
import './styles/global.css';

const App: React.FC = () => {
    return (
        <CharacterProvider>
            <CinematicRollProvider>
                <ModalProvider>
                    <MainLayout />
                    <ModalManager />
                    <CinematicRollOverlay />
                </ModalProvider>
            </CinematicRollProvider>
        </CharacterProvider>
    );
};

export default App;
