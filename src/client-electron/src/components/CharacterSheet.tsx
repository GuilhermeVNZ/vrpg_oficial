/**
 * Character Sheet Component - D&D 5e (Updated Layout)
 * 
 * Ficha de personagem completa com glassmorphism, seguindo o Design System
 * do VRPG Client. Layout atualizado baseado na imagem de referência com
 * retrato circular, barra de atributos horizontal e grid de 3 colunas.
 */

import React, { useState, useEffect } from 'react';
import './CharacterSheet.css';

export interface CharacterData {
  name: string;
  level: number;
  class: string;
  subclass?: string;
  race: string;
  background: string;
  alignment?: string;
  xp?: {
    current: number;
    max: number;
  };
  portrait?: string; // URL ou caminho para imagem do retrato
  abilityScores: {
    strength: number;
    dexterity: number;
    constitution: number;
    intelligence: number;
    wisdom: number;
    charisma: number;
  };
  hp: {
    current: number;
    max: number;
  };
  ac: number;
  initiative: number;
  speed: number;
  hitDice?: string; // Ex: "5d10"
  savingThrows?: Array<{
    ability: string;
    modifier: number;
    proficient: boolean;
  }>;
  skills: Array<{
    name: string;
    ability: string;
    proficient: boolean;
    modifier: number;
  }>;
  proficiencies?: string[]; // Lista de proficiências (armas, armaduras, etc.)
  actions: Array<{
    name: string;
    bonus: number;
    damage?: string;
    range?: string;
    type: string;
    versatile?: string; // Para armas versáteis
  }>;
  maneuverDC?: number; // Para Battle Master
  features: Array<{
    name: string;
    source: string;
    description: string;
    uses?: string; // Ex: "1/SR"
  }>;
  spells?: {
    spellSaveDC: number;
    spellAttackBonus: number;
    slots: {
      [level: number]: { used: number; total: number };
    };
    known: {
      [level: number]: string[];
    };
  };
  inventory: {
    currency: {
      pp: number;
      gp: number;
      ep: number;
      sp: number;
      cp: number;
    };
    items: Array<{
      name: string;
      quantity: number;
    }>;
  };
  personality?: {
    traits?: string;
    ideals?: string;
    bonds?: string;
    flaws?: string;
  };
}

interface CharacterSheetProps {
  character: CharacterData;
  isOpen: boolean;
  onClose: () => void;
}

type TabType = 'main' | 'spells' | 'inventory' | 'features';

export const CharacterSheet: React.FC<CharacterSheetProps> = ({
  character,
  isOpen,
  onClose,
}) => {
  const [activeTab, setActiveTab] = useState<TabType>('main');

  // Fechar com ESC
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
      document.body.style.overflow = 'hidden';
    }

    return () => {
      document.removeEventListener('keydown', handleEscape);
      document.body.style.overflow = '';
    };
  }, [isOpen, onClose]);

  if (!isOpen) {
    return null;
  }

  // Calcular modificadores de atributos
  const getAbilityModifier = (score: number): number => {
    return Math.floor((score - 10) / 2);
  };

  const formatModifier = (mod: number): string => {
    return mod >= 0 ? `+${mod}` : `${mod}`;
  };

  const hpPercentage = (character.hp.current / character.hp.max) * 100;

  // Ícones para atributos
  const abilityIcons: { [key: string]: string } = {
    strength: '⚔️',
    dexterity: '🦵',
    constitution: '🛡️',
    intelligence: '🧠',
    wisdom: '🦉',
    charisma: '🎭',
  };

  const abilityLabels: { [key: string]: string } = {
    strength: 'STR',
    dexterity: 'DEX',
    constitution: 'CON',
    intelligence: 'INT',
    wisdom: 'WIS',
    charisma: 'CHA',
  };

  return (
    <div
      className="sheet-modal-overlay active"
      onClick={(e) => {
        if (e.target === e.currentTarget) {
          onClose();
        }
      }}
      role="dialog"
      aria-modal="true"
      aria-labelledby="character-sheet-title"
    >
      <div className="sheet-container glass-panel-ornate">
        <button
          className="sheet-close-btn"
          onClick={onClose}
          aria-label="Fechar Ficha"
          type="button"
        >
          ✕
        </button>

        <header className="sheet-header">
          <div className="header-left">
            {character.portrait && (
              <div className="char-portrait-container">
                <img
                  src={character.portrait}
                  alt={`Retrato de ${character.name}`}
                  className="char-portrait"
                />
              </div>
            )}
            <div className="char-identity">
              <h1 id="character-sheet-title" className="char-name">
                {character.name.toUpperCase()}
              </h1>
              <div className="char-details">
                Level {character.level} {character.race} {character.class}
                {character.subclass && ` (${character.subclass})`}
                <br />
                Background: {character.background}
                {character.alignment && (
                  <>
                    <br />
                    Alignment: {character.alignment}
                  </>
                )}
                {character.xp && (
                  <>
                    <br />
                    XP: {character.xp.current} / {character.xp.max}
                  </>
                )}
              </div>
            </div>
          </div>

          <div className="header-right glass-sub-panel vital-panel">
            <h3 className="panel-title-sm title-accent">Combat Status</h3>
            <div className="vital-grid">
              <div className="vital-item">
                <span className="vital-label">Armor Class:</span>
                <span className="vital-value">{character.ac}</span>
              </div>
              <div className="vital-item">
                <span className="vital-label">Initiative:</span>
                <span className="vital-value title-accent">
                  {formatModifier(character.initiative)}
                </span>
              </div>
              <div className="vital-item">
                <span className="vital-label">Speed:</span>
                <span className="vital-value">{character.speed} ft.</span>
              </div>
              <div className="vital-item">
                <span className="vital-label">Hit Points:</span>
                <span className="vital-value health-accent">
                  {character.hp.current} / {character.hp.max}
                </span>
              </div>
              {character.hitDice && (
                <div className="vital-item">
                  <span className="vital-label">Hit Dice:</span>
                  <span className="vital-value">{character.hitDice}</span>
                </div>
              )}
            </div>
          </div>
        </header>

        <section className="attributes-bar">
          <h3 className="section-title-center title-accent">Attributes</h3>
          <div className="attributes-container">
            {[
              { key: 'strength', value: character.abilityScores.strength },
              { key: 'dexterity', value: character.abilityScores.dexterity },
              { key: 'constitution', value: character.abilityScores.constitution },
              { key: 'intelligence', value: character.abilityScores.intelligence },
              { key: 'wisdom', value: character.abilityScores.wisdom },
              { key: 'charisma', value: character.abilityScores.charisma },
            ].map((ability) => {
              const mod = getAbilityModifier(ability.value);
              return (
                <div key={ability.key} className="attribute-column">
                  <span className="attr-label">{abilityLabels[ability.key]}</span>
                  <span className="attr-score">{ability.value}</span>
                  <span className={`attr-mod ${mod >= 0 ? 'title-accent' : ''}`}>
                    {formatModifier(mod)}
                  </span>
                  <span className="attr-icon">{abilityIcons[ability.key]}</span>
                </div>
              );
            })}
          </div>
        </section>

        <nav className="sheet-tabs-nav" role="tablist">
          {[
            { id: 'main', label: 'Principal' },
            { id: 'spells', label: 'Magias' },
            { id: 'inventory', label: 'Inventário' },
            { id: 'features', label: 'Talentos & Traits' },
          ].map((tab) => (
            <button
              key={tab.id}
              className={`tab-btn ${activeTab === tab.id ? 'active' : ''}`}
              onClick={() => setActiveTab(tab.id as TabType)}
              role="tab"
              aria-selected={activeTab === tab.id}
              aria-controls={`tab-${tab.id}`}
              type="button"
            >
              {tab.label}
            </button>
          ))}
        </nav>

        <main className="sheet-content-area">
          {/* Tab: Main */}
          {activeTab === 'main' && (
            <div id="tab-main" className="tab-panel active main-grid-layout" role="tabpanel">
              <div className="left-column">
                <div className="glass-sub-panel content-panel">
                  <h3 className="panel-title-sm title-accent">Skills & Saves</h3>
                  <div className="skills-saves-grid">
                    {character.savingThrows && character.savingThrows.length > 0 && (
                      <div className="saves-list">
                        <h4 className="sub-title">Saving Throws:</h4>
                        <ul>
                          {character.savingThrows.map((save, idx) => (
                            <li key={idx}>
                              <span>{save.ability}</span>
                              <span className="val">
                                ({formatModifier(save.modifier)}, {save.proficient ? 'P' : '-'})
                              </span>
                            </li>
                          ))}
                        </ul>
                      </div>
                    )}
                    <div className="skills-list">
                      <h4 className="sub-title">Skills:</h4>
                      <ul>
                        {character.skills.map((skill, idx) => (
                          <li key={idx}>
                            <span>{skill.name}</span>
                            <span className="val">
                              ({formatModifier(skill.modifier)}, {skill.proficient ? 'P' : '-'})
                            </span>
                          </li>
                        ))}
                      </ul>
                    </div>
                  </div>
                </div>

                {character.proficiencies && character.proficiencies.length > 0 && (
                  <div className="glass-sub-panel content-panel proficiencies-panel">
                    <h3 className="panel-title-sm title-accent">Proficiencies</h3>
                    <div className="prof-icons">
                      {character.proficiencies.map((prof, idx) => (
                        <span key={idx} className="prof-icon" title={prof}>
                          {prof}
                        </span>
                      ))}
                    </div>
                  </div>
                )}
              </div>

              <div className="center-column">
                <div className="glass-sub-panel content-panel">
                  <h3 className="panel-title-sm title-accent">Attacks & Spellcasting</h3>
                  <ul className="attacks-list">
                    {character.actions.map((action, idx) => (
                      <li key={idx}>
                        <span className="attack-name">{action.name}:</span>
                        <span className="attack-detail">
                          {formatModifier(action.bonus)} to hit
                          {action.damage && `, ${action.damage} ${action.type}`}
                          {action.versatile && ` (Versatile ${action.versatile})`}
                          {action.range && ` (Range ${action.range})`}
                        </span>
                      </li>
                    ))}
                    {character.maneuverDC && (
                      <li className="maneuver-dc">
                        <span className="attack-name title-accent">Maneuver DC:</span>
                        <span className="attack-detail title-accent">{character.maneuverDC}</span>
                      </li>
                    )}
                  </ul>
                </div>

                <div className="glass-sub-panel content-panel">
                  <h3 className="panel-title-sm title-accent">Features & Traits</h3>
                  <ul className="features-list-summary">
                    {character.features.map((feature, idx) => (
                      <li key={idx}>
                        <strong className="title-accent">{feature.name}</strong>
                        {feature.uses && ` (${feature.uses})`}
                        {feature.description && `: ${feature.description}`}
                      </li>
                    ))}
                  </ul>
                </div>
              </div>

              <div className="right-column">
                <div className="glass-sub-panel content-panel equipment-panel">
                  <h3 className="panel-title-sm title-accent">Equipment</h3>
                  <ul className="equip-list-summary">
                    {character.inventory.items.map((item, idx) => (
                      <li key={idx}>
                        {item.name} {item.quantity > 1 && `(${item.quantity})`}
                      </li>
                    ))}
                  </ul>
                  <div className="currency-row">
                    <span>
                      <strong className="title-accent">CP:</strong> {character.inventory.currency.cp}
                    </span>
                    <span>
                      <strong className="title-accent">SP:</strong> {character.inventory.currency.sp}
                    </span>
                    <span>
                      <strong className="title-accent">EP:</strong> {character.inventory.currency.ep}
                    </span>
                    <span>
                      <strong className="title-accent">GP:</strong> {character.inventory.currency.gp}
                    </span>
                    <span>
                      <strong className="title-accent">PP:</strong> {character.inventory.currency.pp}
                    </span>
                  </div>
                </div>

                {character.personality && (
                  <div className="glass-sub-panel content-panel">
                    <h3 className="panel-title-sm title-accent">Personality</h3>
                    {character.personality.traits && (
                      <div className="personality-block">
                        <h4 className="title-accent">Personality Traits:</h4>
                        <p>{character.personality.traits}</p>
                      </div>
                    )}
                    {character.personality.ideals && (
                      <div className="personality-block">
                        <h4 className="title-accent">Ideals:</h4>
                        <p>{character.personality.ideals}</p>
                      </div>
                    )}
                    {character.personality.bonds && (
                      <div className="personality-block">
                        <h4 className="title-accent">Bonds:</h4>
                        <p>{character.personality.bonds}</p>
                      </div>
                    )}
                    {character.personality.flaws && (
                      <div className="personality-block">
                        <h4 className="title-accent">Flaws:</h4>
                        <p>{character.personality.flaws}</p>
                      </div>
                    )}
                  </div>
                )}
              </div>
            </div>
          )}

          {/* Tab: Spells */}
          {activeTab === 'spells' && character.spells && (
            <div id="tab-spells" className="tab-panel active glass-sub-panel content-panel" role="tabpanel">
              <h3 className="panel-title-sm title-accent">Grimório</h3>
              <div className="spell-header-stats">
                <div>
                  CD de Resistência:{' '}
                  <span className="title-accent">{character.spells.spellSaveDC}</span>
                </div>
                <div>
                  Bônus de Ataque:{' '}
                  <span className="title-accent">{formatModifier(character.spells.spellAttackBonus)}</span>
                </div>
              </div>

              {Object.entries(character.spells.known).map(([level, spells]) => {
                const levelNum = parseInt(level);
                const slots = character.spells.slots[levelNum];
                return (
                  <div key={level}>
                    <h4 className="spell-level-title">
                      {levelNum === 0 ? 'Truques (0)' : `Nível ${levelNum}`}
                      {slots && (
                        <span className="spell-slots">
                          (Slots: {slots.used}/{slots.total})
                        </span>
                      )}
                    </h4>
                    <ul className="data-list spell-list">
                      {spells.map((spell, idx) => (
                        <li key={idx}>{spell}</li>
                      ))}
                    </ul>
                  </div>
                );
              })}
            </div>
          )}

          {/* Tab: Inventory */}
          {activeTab === 'inventory' && (
            <div id="tab-inventory" className="tab-panel active glass-sub-panel content-panel" role="tabpanel">
              <h3 className="panel-title-sm title-accent">Inventário Detalhado</h3>
              <div className="currency-bar glass-sub-panel">
                <div>
                  <span className="title-accent">{character.inventory.currency.gp}</span> PO
                </div>
                <div>
                  <span>{character.inventory.currency.pp}</span> PP
                </div>
                <div>
                  <span>{character.inventory.currency.cp}</span> PC
                </div>
              </div>
              <ul className="data-list inventory-list">
                <li className="inv-header">
                  <span>Item</span> <span>Qtd.</span>
                </li>
                {character.inventory.items.map((item, idx) => (
                  <li key={idx}>
                    <span>{item.name}</span> <span>{item.quantity}</span>
                  </li>
                ))}
              </ul>
            </div>
          )}

          {/* Tab: Features */}
          {activeTab === 'features' && (
            <div id="tab-features" className="tab-panel active glass-sub-panel content-panel" role="tabpanel">
              <h3 className="panel-title-sm title-accent">Talentos & Características</h3>
              {character.features.map((feature, idx) => (
                <div key={idx} className="feature-block">
                  <h4 className="feature-name">
                    {feature.name} ({feature.source})
                  </h4>
                  <p>{feature.description}</p>
                  {feature.uses && (
                    <p className="feature-uses">
                      <strong>Usos:</strong> {feature.uses}
                    </p>
                  )}
                </div>
              ))}
            </div>
          )}
        </main>
      </div>
    </div>
  );
};

export default CharacterSheet;
