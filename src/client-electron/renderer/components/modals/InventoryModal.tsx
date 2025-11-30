import React, { useState } from 'react';
import { useCharacter } from '../../context/CharacterContext';
import { TooltipTerm } from '../common/TooltipTerm';
import { Tooltip } from '../common/Tooltip';
import { TooltipDefinitionKey } from '../../data/tooltipDefinitions';
import './InventoryModal.css';
import paperdollBg from '../../assets/paperdoll_bg.png';
import smallSlotImage from '../../assets/small_item_slot.png';
import bigSlotImage from '../../assets/big_item_slot.png';
import inventoryBg from '../../assets/inventory_bg.png';
import landBg from '../../assets/land_bg.png';
import petBg from '../../assets/pet_bg.png';
import storageBg from '../../assets/storage_bg.png';

import quarterstaffImg from '../../assets/quarterstaff.png';
import robeImg from '../../assets/robe.png';

interface InventoryModalProps {
    isOpen: boolean;
    onClose: () => void;
}

type InventoryTab = 'equipped' | 'attuned' | 'backpack' | 'bank' | 'landholdings' | 'companions';

const InventoryModal: React.FC<InventoryModalProps> = ({ isOpen, onClose }) => {
    const { character } = useCharacter();
    const [activeTab, setActiveTab] = useState<InventoryTab>('equipped');

    if (!isOpen || !character) return null;

    return (
        <div
            className="modal-overlay"
            onClick={onClose}
            style={{
                background: 'rgba(0, 0, 0, 0.7)',
                backdropFilter: 'blur(4px)'
            }}
        >
            <div
                onClick={(e) => e.stopPropagation()}
                style={{
                    width: '100%',
                    maxWidth: '1000px',
                    height: '820px', // Fixed height to prevent resizing
                    background: '#000000',
                    // border: '1px solid rgba(212, 175, 55, 0.3)', // Removed as requested
                    borderRadius: '12px',
                    boxShadow: '0 20px 60px rgba(0, 0, 0, 0.8)',
                    padding: '0',
                    display: 'flex',
                    flexDirection: 'column',
                    overflow: 'hidden', // Ensure content scrolls inside
                }}
            >
                {/* Tab Navigation */}
                <div style={{
                    display: 'flex',
                    borderBottom: '1px solid rgba(212, 175, 55, 0.2)',
                    padding: '0 32px',
                    gap: '8px',
                }}>
                    {[
                        { id: 'equipped', label: 'Equipped' },
                        { id: 'attuned', label: 'Attuned' },
                        { id: 'backpack', label: 'Backpack' },
                        { id: 'bank', label: 'Bank' },
                        { id: 'landholdings', label: 'Landholdings' },
                        { id: 'companions', label: 'Companions' },
                    ].map((tab) => (
                        <button
                            key={tab.id}
                            onClick={() => setActiveTab(tab.id as InventoryTab)}
                            style={{
                                padding: '16px 24px',
                                background: activeTab === tab.id ? 'rgba(212, 175, 55, 0.1)' : 'transparent',
                                border: 'none',
                                borderBottom: activeTab === tab.id ? '2px solid rgba(212, 175, 55, 0.8)' : '2px solid transparent',
                                color: activeTab === tab.id ? 'rgba(212, 175, 55, 1)' : 'rgba(255, 255, 255, 0.6)',
                                fontFamily: "'Cinzel', serif",
                                fontSize: '14px',
                                fontWeight: 600,
                                cursor: 'pointer',
                                transition: 'all 0.2s ease',
                                letterSpacing: '0.5px',
                            }}
                            onMouseEnter={(e) => {
                                if (activeTab !== tab.id) {
                                    e.currentTarget.style.color = 'rgba(212, 175, 55, 0.8)';
                                }
                            }}
                            onMouseLeave={(e) => {
                                if (activeTab !== tab.id) {
                                    e.currentTarget.style.color = 'rgba(255, 255, 255, 0.6)';
                                }
                            }}
                        >
                            {tab.label}
                        </button>
                    ))}
                </div>

                {/* Content */}
                <div style={{ flex: 1, overflow: 'hidden', position: 'relative', padding: '32px' }}>
                    {activeTab === 'equipped' && <EquippedTab character={character} />}
                    {activeTab === 'attuned' && <AttunedTab character={character} />}
                    {activeTab === 'backpack' && <BackpackTab character={character} />}
                    {activeTab === 'bank' && <BankTab character={character} />}
                    {activeTab === 'landholdings' && <LandholdingsTab />}
                    {activeTab === 'companions' && <CompanionsTab />}
                </div>
            </div>
        </div>
    );
};

// Helper to render item tooltip content
const renderItemTooltip = (item: any) => (
    <div>
        <div style={{ fontSize: '16px', fontWeight: 'bold', color: '#D4AF37', marginBottom: '4px', fontFamily: "'Cinzel', serif" }}>
            {item.name}
        </div>
        <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.6)', marginBottom: '8px', fontStyle: 'italic' }}>
            {item.rarity || 'Common'} • {item.type || 'Item'} • {item.weight || 0} lbs
        </div>
        <div style={{ fontSize: '13px', color: 'rgba(255,255,255,0.9)', lineHeight: '1.4', whiteSpace: 'pre-wrap' }}>
            {item.description || "No description available."}
        </div>
        {item.value && (
            <div style={{ marginTop: '8px', fontSize: '12px', color: '#D4AF37' }}>
                Value: {item.value} gp
            </div>
        )}
    </div>
);

const EquippedTab: React.FC<{ character: any }> = ({ character }) => {
    const { equipped } = character.inventory;

    const SlotBox: React.FC<{ label: string; item?: any; isLarge?: boolean }> = ({
        label, item, isLarge = false
    }) => {
        const getItemIcon = (name: string) => {
            if (name.toLowerCase().includes('quarterstaff')) return quarterstaffImg;
            if (name.toLowerCase().includes('robe')) return robeImg;
            return null;
        };

        const icon = item ? getItemIcon(item.name) : null;

        const slotContent = (
            <div style={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                gap: '1px',
                position: 'relative',
            }}>
                {/* Teal glow background */}
                <div style={{
                    position: 'absolute',
                    width: isLarge ? '120px' : '90px',
                    height: isLarge ? '180px' : '90px',
                    top: '10px',
                    background: 'radial-gradient(circle, rgba(0, 200, 200, 0.12) 0%, transparent 70%)',
                    filter: 'blur(15px)',
                    pointerEvents: 'none',
                    zIndex: 0,
                }} />

                <div style={{
                    fontFamily: "'Cinzel', 'Times New Roman', serif",
                    fontSize: '12px',
                    fontWeight: 600,
                    color: 'rgba(212, 175, 55, 0.9)',
                    textTransform: 'uppercase',
                    letterSpacing: '0.3px',
                    whiteSpace: 'nowrap',
                    zIndex: 1,
                    lineHeight: '1',
                }}>
                    {label}
                </div>
                <div style={{
                    width: isLarge ? '120px' : '90px',
                    height: isLarge ? '180px' : '90px',
                    position: 'relative',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    zIndex: 1,
                }}>
                    {/* 1. Glow Points Layer (Behind Slot Image) */}
                    {(() => {
                        const offset = isLarge ? '9px' : '14px'; // Increased by 4px (5->9, 10->14)
                        return [
                            { top: offset, left: '50%', transform: 'translate(-50%, -50%)' }, // Top
                            { bottom: offset, left: '50%', transform: 'translate(-50%, 50%)' }, // Bottom
                            { top: '50%', left: offset, transform: 'translate(-50%, -50%)' }, // Left
                            { top: '50%', right: offset, transform: 'translate(50%, -50%)' }  // Right
                        ].map((pos, i) => (
                            <div key={i} style={{
                                position: 'absolute',
                                width: '12px',
                                height: '12px',
                                background: 'radial-gradient(circle, rgba(0, 255, 255, 0.9) 0%, transparent 70%)',
                                filter: 'blur(3px)',
                                pointerEvents: 'none',
                                zIndex: 1, // Behind Slot Image
                                ...pos
                            }} />
                        ));
                    })()}

                    {/* 2. Slot Image Layer */}
                    <div style={{
                        position: 'absolute',
                        inset: 0,
                        backgroundImage: `url(${isLarge ? bigSlotImage : smallSlotImage})`,
                        backgroundSize: '100% 100%',
                        backgroundPosition: 'center',
                        backgroundRepeat: 'no-repeat',
                        zIndex: 2, // In front of glows
                        pointerEvents: 'none',
                    }} />

                    {/* 3. Item Content Layer */}
                    {item && (
                        <div style={{
                            position: 'absolute',
                            inset: '15px',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            fontSize: '9px',
                            fontWeight: 600,
                            color: '#FFFFFF',
                            textAlign: 'center',
                            textShadow: '0 2px 4px rgba(0,0,0,0.8)',
                            padding: '4px',
                            zIndex: 3, // On top of everything
                        }}>
                            {icon ? (
                                <img src={icon} alt={item.name} style={{ width: '100%', height: '100%', objectFit: 'contain', filter: 'drop-shadow(0 0 4px rgba(0,0,0,0.8))' }} />
                            ) : (
                                item.name
                            )}
                        </div>
                    )}
                </div>
            </div>
        );

        return item ? (
            <Tooltip content={renderItemTooltip(item)}>
                {slotContent}
            </Tooltip>
        ) : slotContent;
    };

    return (
        <div style={{
            display: 'flex',
            flexDirection: 'column',
            height: '100%',
        }}>
            {/* Main content area */}
            <div style={{
                display: 'flex',
                gap: '40px',
                flex: 1,
            }}>
                {/* Left side: Paperdoll silhouette */}
                <div style={{
                    flex: '0 0 400px',
                    position: 'relative',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    marginTop: '-100px',
                    marginLeft: '20px',
                }}>
                    <div style={{
                        width: '100%',
                        height: '100%',
                        backgroundImage: `url(${paperdollBg})`,
                        backgroundSize: 'contain',
                        backgroundPosition: 'center',
                        backgroundRepeat: 'no-repeat',
                    }} />
                </div>

                {/* Right side: Equipment slots grid */}
                <div style={{
                    flex: 1,
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '10px',
                }}>
                    {/* Small slots grid - 3 columns, 4 rows */}
                    <div style={{
                        display: 'grid',
                        gridTemplateColumns: 'repeat(3, 1fr)',
                        gap: '10px',
                    }}>
                        <SlotBox label="HELMET" item={equipped.head} />
                        <SlotBox label="AMULET" item={equipped.neck} />
                        <SlotBox label="CLOAK" item={equipped.back} />

                        <SlotBox label="GLOVES" item={equipped.hands} />
                        <SlotBox label="ARMOR" item={equipped.chest} />
                        <SlotBox label="BRACERS" item={equipped.wrists} />

                        <SlotBox label="RING 1" item={equipped.ring1} />
                        <SlotBox label="RING 2" item={equipped.ring2} />
                        <SlotBox label="BOOTS" item={equipped.feet} />

                        <SlotBox label="MISC 1" item={equipped.misc1} />
                        <SlotBox label="MISC 2" item={equipped.misc2} />
                        <SlotBox label="MISC 3" item={equipped.misc3} />
                    </div>

                    {/* Large weapon slots - 2 columns */}
                    <div style={{
                        display: 'grid',
                        gridTemplateColumns: 'repeat(2, 1fr)',
                        gap: '10px',
                        marginTop: '4px',
                    }}>
                        <SlotBox label="MAIN HAND" item={equipped.mainHand} isLarge />
                        <SlotBox label="OFF HAND" item={equipped.offHand} isLarge />
                    </div>
                </div>
            </div>

            {/* Footer - Full width horizontal layout */}
            <div style={{
                marginTop: '16px',
                padding: '16px 24px',
                background: 'rgba(255, 255, 255, 0.03)',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                borderRadius: '8px',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                gap: '32px',
            }}>
                {/* Left section: Weight & Status */}
                <div style={{
                    display: 'flex',
                    gap: '32px',
                    alignItems: 'center',
                }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                        <TooltipTerm term="Weight">
                            <div style={{ fontSize: '13px', color: 'rgba(255, 255, 255, 0.7)' }}>Weight:</div>
                        </TooltipTerm>
                        <div style={{ fontSize: '14px', fontWeight: 600, color: '#E8D4A0', fontFamily: "'Cinzel', serif" }}>
                            {character.inventory.totalWeight || 0} / {(character.abilities?.strength || 10) * 15} lbs
                        </div>
                    </div>

                    <div style={{
                        width: '1px',
                        height: '24px',
                        background: 'rgba(255, 255, 255, 0.1)',
                    }} />

                    <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                        <TooltipTerm term="Encumbrance">
                            <div style={{ fontSize: '13px', color: 'rgba(255, 255, 255, 0.7)' }}>Status:</div>
                        </TooltipTerm>
                        <div style={{
                            fontSize: '14px',
                            fontWeight: 600,
                            color: (() => {
                                const weight = character.inventory.totalWeight || 0;
                                const str = character.abilities?.strength || 10;
                                if (weight > str * 10) return '#e74c3c';
                                if (weight > str * 5) return '#f39c12';
                                return '#2ecc71';
                            })(),
                            fontFamily: "'Cinzel', serif",
                        }}>
                            {(() => {
                                const weight = character.inventory.totalWeight || 0;
                                const str = character.abilities?.strength || 10;
                                if (weight > str * 10) return 'Heavily Encumbered';
                                if (weight > str * 5) return 'Encumbered';
                                return 'Normal';
                            })()}
                        </div>
                    </div>
                </div>

                {/* Right section: Currency */}
                <div style={{
                    display: 'flex',
                    gap: '16px',
                    alignItems: 'center',
                }}>
                    <TooltipTerm term="Currency">
                        <div style={{ fontSize: '13px', color: 'rgba(255,255,255,0.5)', marginRight: '8px' }}>Funds:</div>
                    </TooltipTerm>
                    {Object.entries(character.inventory.currency || {}).map(([type, value]) => (
                        <div key={type} style={{
                            display: 'flex',
                            alignItems: 'center',
                            gap: '6px',
                        }}>
                            <div style={{
                                fontSize: '16px',
                                fontWeight: 'bold',
                                color: '#D4AF37',
                                fontFamily: "'Cinzel', serif",
                            }}>
                                {value as number}
                            </div>
                            <div style={{
                                fontSize: '10px',
                                color: 'rgba(255,255,255,0.5)',
                                textTransform: 'uppercase',
                                letterSpacing: '0.5px',
                            }}>
                                {type}
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

const AttunedTab: React.FC<{ character: any }> = ({ character }) => {
    const attunedItems = character.inventory.attuned;
    const maxAttunement = 3;

    return (
        <div style={{ height: '100%', overflowY: 'auto', paddingRight: '8px' }}>
            <div style={{ marginBottom: '16px', fontSize: '13px', color: 'rgba(255,255,255,0.7)' }}>
                <TooltipTerm term="Attunement">Attuned Items</TooltipTerm>: {attunedItems.length} / {maxAttunement}
            </div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                {[...Array(maxAttunement)].map((_, index) => {
                    const item = attunedItems[index];
                    const content = (
                        <div key={index} style={{
                            display: 'flex',
                            gap: '20px',
                            padding: '16px',
                            background: 'rgba(255, 255, 255, 0.03)',
                            border: '1px solid rgba(212, 175, 55, 0.2)',
                            borderRadius: '8px',
                            alignItems: 'center'
                        }}>
                            {/* Slot Image */}
                            <div style={{
                                width: '90px',
                                height: '135px', // Scaled down big slot
                                backgroundImage: `url(${bigSlotImage})`,
                                backgroundSize: '100% 100%',
                                display: 'flex',
                                alignItems: 'center',
                                justifyContent: 'center',
                                flexShrink: 0,
                                position: 'relative',
                            }}>
                                {item && (
                                    <div style={{
                                        position: 'absolute',
                                        inset: '10px',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'center',
                                        textAlign: 'center',
                                        fontSize: '10px',
                                        fontWeight: 600,
                                        color: '#FFF',
                                        textShadow: '0 2px 4px rgba(0,0,0,0.8)',
                                    }}>
                                        {item.name}
                                    </div>
                                )}
                            </div>

                            {/* Item Details */}
                            <div style={{ flex: 1 }}>
                                {item ? (
                                    <>
                                        <div style={{ fontSize: '18px', fontWeight: 600, color: '#D4AF37', marginBottom: '4px', fontFamily: "'Cinzel', serif" }}>
                                            {item.name}
                                        </div>
                                        <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.6)', marginBottom: '8px', fontStyle: 'italic' }}>
                                            {item.rarity || 'Wondrous Item'} • {item.type || 'Equipment'}
                                        </div>
                                        <div style={{ fontSize: '13px', color: 'rgba(255,255,255,0.8)', lineHeight: '1.4' }}>
                                            {item.description || "This item pulses with magical energy. Its true properties are revealed only to those who attune to it."}
                                        </div>
                                    </>
                                ) : (
                                    <div style={{ color: 'rgba(255,255,255,0.3)', fontStyle: 'italic' }}>
                                        Empty Attunement Slot
                                    </div>
                                )}
                            </div>
                        </div>
                    );

                    return item ? <Tooltip key={index} content={renderItemTooltip(item)}>{content}</Tooltip> : content;
                })}
            </div>
        </div>
    );
};

const SplitLayout: React.FC<{
    image: string;
    children: React.ReactNode;
    title?: string;
    tooltipTerm?: string;
}> = ({ image, children, title, tooltipTerm }) => (
    <div style={{ display: 'flex', height: '100%', gap: '32px' }}>
        {/* Left Column - Image */}
        <div style={{
            flex: '0 0 400px', // Matches Equipped tab paperdoll width
            backgroundImage: `url(${image})`,
            backgroundSize: 'contain', // Ensure full image is visible
            backgroundPosition: 'center',
            backgroundRepeat: 'no-repeat',
            borderRadius: '8px',
            border: '1px solid rgba(212, 175, 55, 0.2)',
            boxShadow: 'inset 0 0 40px rgba(0,0,0,0.8)',
        }} />

        {/* Right Column - Content */}
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
            {title && (
                <div style={{
                    fontSize: '20px',
                    fontFamily: "'Cinzel', serif",
                    color: '#D4AF37',
                    marginBottom: '16px',
                    borderBottom: '1px solid rgba(212, 175, 55, 0.2)',
                    paddingBottom: '8px'
                }}>
                    {tooltipTerm ? (
                        <TooltipTerm term={tooltipTerm as TooltipDefinitionKey}>{title}</TooltipTerm>
                    ) : title}
                </div>
            )}
            <div style={{ flex: 1, overflowY: 'auto', paddingRight: '8px' }}>
                {children}
            </div>
        </div>
    </div>
);

const BackpackTab: React.FC<{ character: any }> = ({ character }) => {
    const items = character.inventory.backpack;

    return (
        <SplitLayout image={inventoryBg} title="Backpack Contents">
            <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                {items.length === 0 ? (
                    <div style={{ textAlign: 'center', padding: '40px', color: 'rgba(255,255,255,0.4)' }}>Your backpack is empty</div>
                ) : (
                    items.map((item: any, index: number) => (
                        <Tooltip key={index} content={renderItemTooltip(item)}>
                            <div style={{ padding: '10px 12px', background: 'rgba(0,0,0,0.4)', border: '1px solid rgba(212, 175, 55, 0.2)', borderRadius: '8px', display: 'grid', gridTemplateColumns: '1fr auto auto', gap: '12px', alignItems: 'center' }}>
                                <div style={{ fontSize: '14px', fontWeight: 600, color: '#FFF' }}>
                                    {item.name}
                                </div>
                                <div style={{ fontSize: '11px', color: 'rgba(255,255,255,0.6)' }}>Qty: {item.quantity}</div>
                                <div style={{ fontSize: '11px', color: 'rgba(255,255,255,0.5)' }}>{item.weight * item.quantity} lbs</div>
                            </div>
                        </Tooltip>
                    ))
                )}
            </div>
        </SplitLayout>
    );
};

const BankTab: React.FC<{ character: any }> = ({ character }) => {
    // Placeholder for bank items since they might not exist in the current character structure yet
    // Assuming a similar structure to backpack for now, or empty
    const items = character.inventory.bank || [];

    return (
        <SplitLayout image={storageBg} title="Bank Storage">
            <div style={{ marginBottom: '12px', fontSize: '12px', color: 'rgba(255,255,255,0.5)', fontStyle: 'italic' }}>
                Items stored here do not count towards your carry weight.
            </div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                {items.length === 0 ? (
                    <div style={{ textAlign: 'center', padding: '40px', color: 'rgba(255,255,255,0.4)' }}>Your bank storage is empty</div>
                ) : (
                    items.map((item: any, index: number) => (
                        <Tooltip key={index} content={renderItemTooltip(item)}>
                            <div style={{ padding: '10px 12px', background: 'rgba(0,0,0,0.4)', border: '1px solid rgba(212, 175, 55, 0.2)', borderRadius: '8px', display: 'grid', gridTemplateColumns: '1fr auto', gap: '12px', alignItems: 'center' }}>
                                <div style={{ fontSize: '14px', fontWeight: 600, color: '#FFF' }}>
                                    {item.name}
                                </div>
                                <div style={{ fontSize: '11px', color: 'rgba(255,255,255,0.6)' }}>Qty: {item.quantity}</div>
                            </div>
                        </Tooltip>
                    ))
                )}
            </div>
        </SplitLayout>
    );
};

const renderPropertyTooltip = (prop: any) => (
    <div>
        <div style={{ fontSize: '16px', fontWeight: 'bold', color: '#D4AF37', marginBottom: '4px', fontFamily: "'Cinzel', serif" }}>
            {prop.name}
        </div>
        <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.6)', marginBottom: '8px' }}>
            {prop.location} • {prop.type}
        </div>
        <div style={{ display: 'grid', gridTemplateColumns: 'auto 1fr', gap: '8px 16px', fontSize: '12px', color: 'rgba(255,255,255,0.8)' }}>
            <div style={{ color: 'rgba(255,255,255,0.5)' }}>Status:</div>
            <div style={{ color: '#2ecc71' }}>{prop.status}</div>
            <div style={{ color: 'rgba(255,255,255,0.5)' }}>Income:</div>
            <div>{prop.income}</div>
            <div style={{ color: 'rgba(255,255,255,0.5)' }}>Maintenance:</div>
            <div>{prop.maintenance}</div>
            <div style={{ color: 'rgba(255,255,255,0.5)' }}>Staff:</div>
            <div>{prop.staff}</div>
        </div>
        <div style={{ marginTop: '8px', fontSize: '12px', fontStyle: 'italic', color: 'rgba(255,255,255,0.5)' }}>
            {prop.description}
        </div>
    </div>
);

const LandholdingsTab: React.FC = () => {
    // Placeholder data with extended stats for tooltip
    const properties = [
        {
            name: "Small Cottage",
            location: "Phandalin",
            type: "Residential",
            status: "Owned",
            income: "-",
            maintenance: "1 gp / day",
            staff: "None",
            description: "A modest home on the outskirts of town. Needs some roof repairs."
        },
        {
            name: "Abandoned Mine",
            location: "Sword Mountains",
            type: "Business",
            status: "Cleared",
            income: "Potential: 50 gp / week",
            maintenance: "5 gp / week (Guards)",
            staff: "2 Guards",
            description: "An old silver mine, recently cleared of goblins. Requires investment to restart operations."
        }
    ];

    return (
        <SplitLayout image={landBg} title="Landholdings & Property" tooltipTerm="Property">
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                {properties.map((prop, index) => (
                    <Tooltip key={index} content={renderPropertyTooltip(prop)}>
                        <div style={{ padding: '16px', background: 'rgba(0,0,0,0.4)', border: '1px solid rgba(212, 175, 55, 0.2)', borderRadius: '8px' }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                                <div style={{ fontSize: '15px', fontWeight: 600, color: '#D4AF37' }}>{prop.name}</div>
                                <div style={{ fontSize: '12px', color: '#2ecc71', fontWeight: 600 }}>{prop.status}</div>
                            </div>
                            <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.7)' }}>{prop.location} • {prop.type}</div>
                        </div>
                    </Tooltip>
                ))}
            </div>
        </SplitLayout>
    );
};

const renderCompanionTooltip = (comp: any) => (
    <div>
        <div style={{ fontSize: '16px', fontWeight: 'bold', color: '#D4AF37', marginBottom: '4px', fontFamily: "'Cinzel', serif" }}>
            {comp.name}
        </div>
        <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.6)', marginBottom: '8px' }}>
            {comp.type} • {comp.size} {comp.race}
        </div>

        {/* Stats Grid */}
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(6, 1fr)', gap: '4px', marginBottom: '12px', textAlign: 'center' }}>
            {Object.entries(comp.stats).map(([stat, val]) => (
                <div key={stat} style={{ background: 'rgba(255,255,255,0.1)', borderRadius: '4px', padding: '2px' }}>
                    <div style={{ fontSize: '9px', color: 'rgba(255,255,255,0.5)' }}>{stat}</div>
                    <div style={{ fontSize: '11px', fontWeight: 'bold' }}>{val as number}</div>
                </div>
            ))}
        </div>

        <div style={{ fontSize: '12px', marginBottom: '8px' }}>
            <span style={{ color: 'rgba(255,255,255,0.5)' }}>Armor Class:</span> {comp.ac}<br />
            <span style={{ color: 'rgba(255,255,255,0.5)' }}>Hit Points:</span> {comp.hp}<br />
            <span style={{ color: 'rgba(255,255,255,0.5)' }}>Speed:</span> {comp.speed}
        </div>

        {comp.features && (
            <div style={{ fontSize: '11px', color: 'rgba(255,255,255,0.8)' }}>
                {comp.features.map((feat: string, i: number) => (
                    <div key={i}>• {feat}</div>
                ))}
            </div>
        )}
    </div>
);

const CompanionsTab: React.FC = () => {
    // Placeholder data based on PHB/MM stats
    const companions = [
        {
            name: "Roach",
            type: "Riding Horse",
            size: "Large",
            race: "Beast",
            hp: "13 (2d10 + 2)",
            speed: "60 ft",
            ac: 10,
            stats: { STR: 16, DEX: 10, CON: 12, INT: 2, WIS: 11, CHA: 7 },
            features: ["Hooves: +5 to hit, 2d4+3 bludgeoning damage."]
        },
        {
            name: "Hooty",
            type: "Owl (Familiar)",
            size: "Tiny",
            race: "Celestial",
            hp: "1 (1d4 - 1)",
            speed: "5 ft, fly 60 ft",
            ac: 11,
            stats: { STR: 3, DEX: 13, CON: 8, INT: 2, WIS: 12, CHA: 7 },
            features: [
                "Flyby: Doesn't provoke opportunity attacks when flying out of reach.",
                "Keen Hearing and Sight: Advantage on Perception checks."
            ]
        }
    ];

    return (
        <SplitLayout image={petBg} title="Companions & Mounts" tooltipTerm="Companion">
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                {companions.map((comp, index) => (
                    <Tooltip key={index} content={renderCompanionTooltip(comp)}>
                        <div style={{ padding: '16px', background: 'rgba(0,0,0,0.4)', border: '1px solid rgba(212, 175, 55, 0.2)', borderRadius: '8px' }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                                <div style={{ fontSize: '15px', fontWeight: 600, color: '#D4AF37' }}>{comp.name}</div>
                                <div style={{ fontSize: '12px', color: 'rgba(255,255,255,0.5)' }}>{comp.type}</div>
                            </div>
                            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '8px', fontSize: '12px', color: 'rgba(255,255,255,0.8)' }}>
                                <div style={{ background: 'rgba(255,255,255,0.05)', padding: '4px 8px', borderRadius: '4px', textAlign: 'center' }}>
                                    <span style={{ color: 'rgba(255,255,255,0.5)' }}>HP:</span> {comp.hp.split(' ')[0]}
                                </div>
                                <div style={{ background: 'rgba(255,255,255,0.05)', padding: '4px 8px', borderRadius: '4px', textAlign: 'center' }}>
                                    <span style={{ color: 'rgba(255,255,255,0.5)' }}>AC:</span> {comp.ac}
                                </div>
                                <div style={{ background: 'rgba(255,255,255,0.05)', padding: '4px 8px', borderRadius: '4px', textAlign: 'center' }}>
                                    <span style={{ color: 'rgba(255,255,255,0.5)' }}>SPD:</span> {comp.speed.split(',')[0]}
                                </div>
                            </div>
                        </div>
                    </Tooltip>
                ))}
            </div>
        </SplitLayout>
    );
};

export default InventoryModal;
