import React, { useEffect, ReactNode } from 'react';


interface ModalProps {
    isOpen: boolean;
    onClose: () => void;
    title: string;
    children: ReactNode;
    maxWidth?: string;
    frameless?: boolean;
}

const Modal: React.FC<ModalProps> = ({ isOpen, onClose, title, children, maxWidth = '900px', frameless = false }) => {
    // Close on click outside
    const handleBackdropClick = (e: React.MouseEvent) => {
        if (e.target === e.currentTarget) {
            onClose();
        }
    };

    // Prevent body scroll when modal is open
    useEffect(() => {
        if (isOpen) {
            document.body.style.overflow = 'hidden';
        } else {
            document.body.style.overflow = 'unset';
        }
        return () => {
            document.body.style.overflow = 'unset';
        };
    }, [isOpen]);

    if (!isOpen) return null;

    return (
        <div
            onClick={handleBackdropClick}
            style={{
                position: 'fixed',
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
                background: 'rgba(0, 0, 0, 0.7)',
                backdropFilter: 'blur(8px)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                zIndex: 1000,
                padding: '20px',
                animation: 'fadeIn 0.2s ease-out'
            }}
        >
            <div
                style={frameless ? {
                    width: '100%',
                    maxWidth,
                    maxHeight: '90vh',
                    display: 'flex',
                    flexDirection: 'column',
                    position: 'relative',
                    animation: 'modalSlideIn 0.3s ease-out',
                    background: 'transparent',
                    boxShadow: 'none',
                    border: 'none'
                } : {
                    background: 'rgba(15, 15, 15, 0.95)',
                    backdropFilter: 'blur(20px)',
                    border: '2px solid rgba(212, 175, 55, 0.5)',
                    borderRadius: '24px',
                    boxShadow: '0 16px 48px rgba(0,0,0,0.8), inset 0 0 40px rgba(74, 144, 226, 0.1)',
                    width: '100%',
                    maxWidth,
                    maxHeight: '90vh',
                    overflow: 'hidden',
                    display: 'flex',
                    flexDirection: 'column',
                    position: 'relative',
                    animation: 'modalSlideIn 0.3s ease-out'
                }}
            >
                {/* Header - Only show if not frameless */}
                {!frameless && (
                    <div
                        style={{
                            padding: '24px 32px',
                            borderBottom: '1px solid rgba(212, 175, 55, 0.3)',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'space-between',
                            background: 'linear-gradient(180deg, rgba(212, 175, 55, 0.08) 0%, transparent 100%)'
                        }}
                    >
                        <h2
                            style={{
                                margin: 0,
                                fontSize: '28px',
                                fontWeight: 'bold',
                                fontFamily: "'Crimson Text', serif",
                                color: '#D4AF37',
                                textShadow: '0 2px 8px rgba(212, 175, 55, 0.6)'
                            }}
                        >
                            {title}
                        </h2>

                        {/* Close button */}
                        <button
                            onClick={onClose}
                            style={{
                                background: 'rgba(0,0,0,0.5)',
                                border: '1px solid rgba(212, 175, 55, 0.4)',
                                borderRadius: '8px',
                                width: '36px',
                                height: '36px',
                                display: 'flex',
                                alignItems: 'center',
                                justifyContent: 'center',
                                cursor: 'pointer',
                                color: '#D4AF37',
                                fontSize: '20px',
                                transition: 'all 0.2s ease',
                                fontFamily: 'monospace'
                            }}
                            onMouseEnter={(e) => {
                                e.currentTarget.style.background = 'rgba(212, 175, 55, 0.2)';
                                e.currentTarget.style.borderColor = '#D4AF37';
                            }}
                            onMouseLeave={(e) => {
                                e.currentTarget.style.background = 'rgba(0,0,0,0.5)';
                                e.currentTarget.style.borderColor = 'rgba(212, 175, 55, 0.4)';
                            }}
                        >
                            ✕
                        </button>
                    </div>
                )}

                {/* Content with custom scrollbar */}
                <div
                    style={{
                        flex: 1,
                        overflow: 'auto',
                        padding: frameless ? 0 : '32px',
                    }}
                    className="modal-content-scroll"
                >
                    {children}
                </div>

                {/* Golden glow effect - Only if not frameless */}
                {!frameless && (
                    <div
                        style={{
                            position: 'absolute',
                            top: 0,
                            left: 0,
                            right: 0,
                            height: '4px',
                            background: 'linear-gradient(90deg, transparent, #D4AF37, transparent)',
                            opacity: 0.6,
                            pointerEvents: 'none'
                        }}
                    />
                )}
            </div>

            <style>{`
                @keyframes fadeIn {
                    from { opacity: 0; }
                    to { opacity: 1; }
                }

                @keyframes modalSlideIn {
                    from {
                        opacity: 0;
                        transform: translateY(-20px) scale(0.95);
                    }
                    to {
                        opacity: 1;
                        transform: translateY(0) scale(1);
                    }
                }

                .modal-content-scroll {
                    scrollbar-width: thin;
                    scrollbar-color: rgba(212, 175, 55, 0.5) rgba(0,0,0,0.3);
                }

                .modal-content-scroll::-webkit-scrollbar {
                    width: 8px;
                }

                .modal-content-scroll::-webkit-scrollbar-track {
                    background: rgba(0,0,0,0.3);
                    border-radius: 4px;
                }

                .modal-content-scroll::-webkit-scrollbar-thumb {
                    background: rgba(212, 175, 55, 0.5);
                    border-radius: 4px;
                }

                .modal-content-scroll::-webkit-scrollbar-thumb:hover {
                    background: rgba(212, 175, 55, 0.7);
                }
            `}</style>
        </div>
    );
};

export default Modal;
