import React, { createContext, useContext, useState, ReactNode } from 'react';

export type ModalType =
    | 'character-sheet'
    | 'abilities'
    | 'inventory'
    | 'spells'
    | 'map'
    | 'journal'
    | 'compendium'
    | 'settings'
    | 'dice-roll'
    | 'skins'
    | null;

interface ModalContextType {
    activeModal: ModalType;
    openModal: (modal: ModalType) => void;
    closeModal: () => void;
}

const ModalContext = createContext<ModalContextType | undefined>(undefined);

export const useModal = () => {
    const context = useContext(ModalContext);
    if (!context) {
        throw new Error('useModal must be used within ModalProvider');
    }
    return context;
};

interface ModalProviderProps {
    children: ReactNode;
}

export const ModalProvider: React.FC<ModalProviderProps> = ({ children }) => {
    const [activeModal, setActiveModal] = useState<ModalType>(null);

    const openModal = (modal: ModalType) => {
        setActiveModal(modal);
    };

    const closeModal = () => {
        setActiveModal(null);
    };

    // Keyboard shortcut handling
    React.useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            // Close modal on Escape
            if (e.key === 'Escape') {
                if (activeModal === 'settings') {
                    closeModal();
                } else if (activeModal) {
                    closeModal();
                } else {
                    // Esc opens settings when no modal is open
                    openModal('settings');
                }
                return;
            }

            // Don't trigger shortcuts if a modal is already open (except Esc)
            if (activeModal) return;

            // Don't trigger shortcuts if user is typing in an input or textarea
            const target = e.target as HTMLElement;
            if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) {
                return;
            }

            // Modal shortcuts
            switch (e.key.toLowerCase()) {
                case 'c':
                    openModal('character-sheet');
                    break;
                case 'a':
                    openModal('abilities');
                    break;
                case 'i':
                    openModal('inventory');
                    break;
                case 's':
                    openModal('spells');
                    break;
                case 'm':
                    openModal('map');
                    break;
                case 'j':
                    openModal('journal');
                    break;
                case 'b':
                    openModal('compendium');
                    break;
                case 'd':
                    openModal('dice-roll');
                    break;
                case 'k':
                    openModal('skins');
                    break;
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [activeModal]);

    return (
        <ModalContext.Provider value={{ activeModal, openModal, closeModal }}>
            {children}
        </ModalContext.Provider>
    );
};
