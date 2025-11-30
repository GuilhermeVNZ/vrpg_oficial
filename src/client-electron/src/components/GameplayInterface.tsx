/**
 * Gameplay Interface Component
 * 
 * Interface principal do jogo durante a sessão, com:
 * - Background da cena (gerado pelo difusor)
 * - Overlay da UI com glassmorphism
 * - Top-left: Nível e XP
 * - Sidebar: Menu de botões
 * - Top-right: Chat e notificações
 * - Footer: Push-to-talk, Party frame, Action bar
 * - Botão de toggle UI (screenshot mode)
 */

import React, { useState, useEffect } from 'react';
import './GameplayInterface.css';

export interface PartyMember {
  id: string;
  name: string;
  portrait: string;
  hp: {
    current: number;
    max: number;
  };
}

export interface ActionSlot {
  id: number;
  icon?: string;
  name?: string;
  hotkey: string;
}

export interface ChatMessage {
  id: string;
  author: string;
  isGM: boolean;
  content: string;
  timestamp: number;
}

export interface AbilityCard {
  id: string;
  title: string;
  type: string;
  duration?: string;
  description: string;
  timestamp: number;
}

interface GameplayInterfaceProps {
  sceneBackground?: string; // URL da imagem de fundo
  level?: number;
  xp?: {
    current: number;
    max: number;
  };
  partyMembers?: PartyMember[];
  actionSlots?: ActionSlot[];
  chatMessages?: ChatMessage[];
  abilityCards?: AbilityCard[];
  latency?: number;
  fps?: number;
  onMenuClick?: (menu: string) => void;
  onActionClick?: (slotId: number) => void;
  onPartyMemberClick?: (memberId: string) => void;
  onPushToTalk?: (pressed: boolean) => void;
  onChatSend?: (message: string) => void;
}

export const GameplayInterface: React.FC<GameplayInterfaceProps> = ({
  sceneBackground,
  level = 5,
  xp = { current: 12500, max: 20000 },
  partyMembers = [],
  actionSlots = [],
  chatMessages = [],
  abilityCards = [],
  latency = 17,
  fps = 61,
  onMenuClick,
  onActionClick,
  onPartyMemberClick,
  onPushToTalk,
  onChatSend,
}) => {
  const [uiHidden, setUiHidden] = useState(false);
  const [chatInput, setChatInput] = useState('');
  const [isPushingToTalk, setIsPushingToTalk] = useState(false);

  // Toggle UI visibility
  const toggleUI = () => {
    setUiHidden(!uiHidden);
  };

  // Handle push to talk
  const handlePushToTalkStart = () => {
    setIsPushingToTalk(true);
    onPushToTalk?.(true);
  };

  const handlePushToTalkEnd = () => {
    setIsPushingToTalk(false);
    onPushToTalk?.(false);
  };

  // Handle chat send
  const handleChatSend = (e: React.FormEvent) => {
    e.preventDefault();
    if (chatInput.trim() && onChatSend) {
      onChatSend(chatInput);
      setChatInput('');
    }
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Toggle UI with 'H'
      if ((e.key === 'h' || e.key === 'H') && 
          e.target instanceof HTMLElement && 
          e.target.tagName !== 'INPUT' && 
          e.target.tagName !== 'TEXTAREA') {
        toggleUI();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [uiHidden]);

  const xpPercentage = (xp.current / xp.max) * 100;

  // Default action slots (1-10)
  const defaultSlots: ActionSlot[] = Array.from({ length: 10 }, (_, i) => ({
    id: i + 1,
    hotkey: i === 9 ? '0' : String(i + 1),
  }));

  const slots = actionSlots.length > 0 ? actionSlots : defaultSlots;

  // Default party members (4 placeholders)
  const defaultMembers: PartyMember[] = [
    { id: '1', name: 'Player 1', portrait: '', hp: { current: 90, max: 100 } },
    { id: '2', name: 'Player 2', portrait: '', hp: { current: 75, max: 100 } },
    { id: '3', name: 'Player 3', portrait: '', hp: { current: 25, max: 100 } },
    { id: '4', name: 'Player 4', portrait: '', hp: { current: 100, max: 100 } },
  ];

  const members = partyMembers.length > 0 ? partyMembers : defaultMembers;

  return (
    <div className="gameplay-container">
      {/* Background da Cena */}
      <div
        className="scene-background"
        style={sceneBackground ? { backgroundImage: `url(${sceneBackground})` } : {}}
      />

      {/* Overlay da UI */}
      <div className={`vrpg-ui-overlay ${uiHidden ? 'ui-hidden' : ''}`} id="uiOverlay">
        {/* Top Left: Level & XP */}
        <div className="top-left-area">
          <div className="level-indicator glass-panel">{level}</div>
          <div className="xp-bar-container">
            <span className="xp-text">
              XP: {xp.current.toLocaleString()} / {xp.max.toLocaleString()}
            </span>
            <div className="xp-progress-track glass-panel">
              <div
                className="xp-progress-fill"
                style={{ width: `${xpPercentage}%` }}
              />
            </div>
          </div>
        </div>

        {/* Sidebar: Menu Buttons */}
        <aside className="sidebar-left">
          <button
            className="menu-btn glass-panel glass-interactive"
            onClick={() => onMenuClick?.('character')}
            title="Ficha do Personagem"
            type="button"
          >
            👤
          </button>
          <button
            className="menu-btn glass-panel glass-interactive"
            onClick={() => onMenuClick?.('inventory')}
            title="Inventário"
            type="button"
          >
            🎒
          </button>
          <button
            className="menu-btn glass-panel glass-interactive"
            onClick={() => onMenuClick?.('abilities')}
            title="Habilidades & Magias"
            type="button"
          >
            ✨
          </button>
          <button
            className="menu-btn glass-panel glass-interactive"
            onClick={() => onMenuClick?.('journal')}
            title="Diário & Missões"
            type="button"
          >
            📜
          </button>
          <button
            className="menu-btn glass-panel glass-interactive"
            onClick={() => onMenuClick?.('map')}
            title="Mapa Mundi"
            type="button"
          >
            🗺️
          </button>
          <div style={{ flexGrow: 1 }} />
          <button
            className="menu-btn glass-panel glass-interactive"
            onClick={() => onMenuClick?.('settings')}
            title="Configurações"
            type="button"
          >
            ⚙️
          </button>
        </aside>

        {/* Top Right: Screenshot Toggle & Chat */}
        <div className="top-right-area">
          <button
            className={`screenshot-toggle-btn glass-panel glass-interactive ${uiHidden ? 'ui-visible' : ''}`}
            onClick={toggleUI}
            title={uiHidden ? 'Mostrar Interface' : 'Modo Screenshot (Esconder Interface)'}
            type="button"
          >
            {uiHidden ? '👁️' : '📷'}
          </button>

          <section className="chat-panel-container glass-panel">
            <div className="chat-history">
              {/* Ability Cards */}
              {abilityCards.map((card) => (
                <div key={card.id} className="floating-card glass-panel">
                  <div className="card-title">{card.title}</div>
                  <div className="card-meta">
                    {card.type} {card.duration && `| ${card.duration}`}
                  </div>
                  <div className="card-body">{card.description}</div>
                </div>
              ))}

              {/* Chat Messages */}
              {chatMessages.map((msg) => (
                <div key={msg.id} className="chat-message">
                  <span className={`chat-author ${msg.isGM ? 'gm-author' : ''}`}>
                    {msg.author}:
                  </span>{' '}
                  {msg.content}
                </div>
              ))}
            </div>

            {/* Chat Input Area */}
            <div className="chat-input-area">
              <div className="audio-visualizer-placeholder" title="Indicador de quem está falando" />
              <form onSubmit={handleChatSend} className="chat-input-form">
                <input
                  type="text"
                  className="chat-input"
                  placeholder="Enter message..."
                  value={chatInput}
                  onChange={(e) => setChatInput(e.target.value)}
                />
                <button type="submit" className="chat-send-btn" aria-label="Enviar mensagem">
                  ✨
                </button>
              </form>
            </div>
          </section>
        </div>

        {/* Footer: Push to Talk, Party, Actions */}
        <footer className="footer-area">
          {/* Push to Talk (Left) */}
          <div
            className={`push-to-talk-container glass-panel glass-interactive ${isPushingToTalk ? 'active' : ''}`}
            onMouseDown={handlePushToTalkStart}
            onMouseUp={handlePushToTalkEnd}
            onMouseLeave={handlePushToTalkEnd}
            onTouchStart={handlePushToTalkStart}
            onTouchEnd={handlePushToTalkEnd}
            role="button"
            tabIndex={0}
            aria-label="Push to Talk"
          >
            <span className="mic-icon">🎙️</span>
            <div className="connection-stats">
              <span>
                Latency: <span className="stat-good">{latency}ms</span>
              </span>
              <span>
                FPS: <span className="stat-good">{fps}</span>
              </span>
            </div>
          </div>

          {/* Party Frame & Action Bar (Center) */}
          <div className="party-action-container">
            <div className="party-frame">
              {members.map((member) => {
                const hpPercentage = (member.hp.current / member.hp.max) * 100;
                const isCritical = hpPercentage < 30;
                return (
                  <div
                    key={member.id}
                    className="party-member-portrait"
                    onClick={() => onPartyMemberClick?.(member.id)}
                    role="button"
                    tabIndex={0}
                    style={
                      member.portrait
                        ? { backgroundImage: `url(${member.portrait})` }
                        : {}
                    }
                  >
                    <div className="hp-bar-mini">
                      <div
                        className={`hp-fill-mini ${isCritical ? 'hp-fill-critical' : ''}`}
                        style={{ width: `${hpPercentage}%` }}
                      />
                    </div>
                  </div>
                );
              })}
            </div>

            <div className="action-bar glass-panel">
              {slots.map((slot) => (
                <div
                  key={slot.id}
                  className={`action-slot glass-interactive ${slot.icon ? 'slot-filled' : ''}`}
                  onClick={() => onActionClick?.(slot.id)}
                  role="button"
                  tabIndex={0}
                  style={slot.icon ? { backgroundImage: `url(${slot.icon})` } : {}}
                >
                  <span className="slot-hotkey">{slot.hotkey}</span>
                  {slot.id === 10 && <span className="slot-label">TALK</span>}
                </div>
              ))}
            </div>
          </div>

          {/* Spacer (Right) */}
          <div style={{ width: '200px' }} />
        </footer>
      </div>
    </div>
  );
};

export default GameplayInterface;









