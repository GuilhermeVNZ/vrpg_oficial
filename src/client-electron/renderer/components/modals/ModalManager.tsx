import React from 'react';
import { useModal } from './ModalContext';
import CharacterSheetModal from './CharacterSheetModal';
import AbilitiesModal from './AbilitiesModal';
import InventoryModal from './InventoryModal';
import SpellsModal from './SpellsModal';
import MapModal from './MapModal';
import JournalModal from './JournalModal';
import CompendiumModal from './CompendiumModal';
import SettingsModal from './SettingsModal';
import SkinsModal from './SkinsModal';

const ModalManager: React.FC = () => {
    const { activeModal, closeModal } = useModal();

    return (
        <>
            <CharacterSheetModal isOpen={activeModal === 'character-sheet'} onClose={closeModal} />
            <AbilitiesModal isOpen={activeModal === 'abilities'} onClose={closeModal} />
            <InventoryModal isOpen={activeModal === 'inventory'} onClose={closeModal} />
            <SpellsModal isOpen={activeModal === 'spells'} onClose={closeModal} />
            <MapModal isOpen={activeModal === 'map'} onClose={closeModal} />
            <JournalModal isOpen={activeModal === 'journal'} onClose={closeModal} />
            <CompendiumModal isOpen={activeModal === 'compendium'} onClose={closeModal} />
            <SkinsModal isOpen={activeModal === 'skins'} onClose={closeModal} />
            <SettingsModal isOpen={activeModal === 'settings'} onClose={closeModal} />
        </>
    );
};

export default ModalManager;
