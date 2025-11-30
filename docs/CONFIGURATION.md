# VRPG Client - Configuração

## Visão Geral

O VRPG Client utiliza um sistema de configuração hierárquico e flexível, permitindo customização completa de todos os aspectos do sistema, desde serviços backend até preferências de UI.

## Configuração Essencial

### LLM Settings

Configuração do pipeline de 2 modelos LLM (core do sistema):

```json
{
  "llm_core": {
    "pipeline_architecture": "dual_model",
    "models": {
      "qwen_1_5b": {
        "name": "Mestre Reflexo",
        "model_path": "assets-and-models/models/llm/qwen2.5-1.5b-instruct-q4_k_m.gguf",
        "max_tokens": 40,
        "gpu_layers": "auto",
        "context_size": 2048,
        "threads": 4,
        "temperature": 0.8,
        "top_p": 0.9,
        "repeat_penalty": 1.1,
        "role": "prelude",
        "latency_target_ms": 1200
      },
      "qwen_14b": {
        "name": "Mestre Real",
        "model_path": "assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf",
        "max_tokens": 2048,
        "gpu_layers": "auto",
        "context_size": 8192,
        "threads": 8,
        "temperature": 0.7,
        "top_p": 0.9,
        "repeat_penalty": 1.1,
        "role": "narration",
        "latency_target_ms": 6000
      }
    }
  }
}
```

**Arquitetura de Pipeline**: **Qwen 1.5B + Qwen 14B (Dual Model)**
- **Qwen 1.5B ("Mestre Reflexo")**: Reação humana imediata, prelúdio emocional (< 1.2s)
  - **Fast inference**: Inferência ultra-rápida para resposta imediata
  - **Emotional reaction**: Simula reação humana de mestre experiente
  - **Prevents cognitive silence**: Evita silêncio cognitivo que quebra imersão
- **Qwen 14B ("Mestre Real")**: Narrativa completa, consequências, resolução (< 6s)
  - **Strong reasoning**: Excelente capacidade de raciocínio para narrativa e NPCs
  - **Fast inference on consumer GPUs**: Inferência rápida em GPUs consumer
  - **Excellent instruction following**: Segue instruções complexas perfeitamente
  - **Supports multi-persona prompting**: Suporta mudança de persona via prompt

**Regra de Ouro**: O 1.5B sempre responde antes do 14B para evitar silêncio cognitivo.

**Ver especificações completas:**
- [QWEN_1_5B_SPEC.md](QWEN_1_5B_SPEC.md) - Especificação do Qwen-1.5B
- [QWEN_14B_SPEC.md](QWEN_14B_SPEC.md) - Especificação do Qwen-14B
- [PIPELINE_ARCHITECTURE.md](PIPELINE_ARCHITECTURE.md) - Arquitetura completa do pipeline

### Hive Integration

Configuração dos serviços Hive (opcionais, mas recomendados para funcionalidade completa):

```json
{
  "hive_integration": {
    "classify": {
      "enabled": true,
      "description": "Classificação automática de memórias e documentos"
    },
    "transmutation": {
      "enabled": true,
      "description": "Conversão de documentos (PDF, DOCX, imagens, áudio/vídeo) para Markdown"
    },
    "vectorizer": {
      "enabled": true,
      "description": "Embeddings e busca vetorial semântica para memória"
    },
    "nexus": {
      "enabled": true,
      "description": "Graph relations para relacionamentos complexos entre entidades"
    },
    "lexum": {
      "enabled": true,
      "description": "Full-text search otimizado para busca textual"
    },
    "synap": {
      "enabled": false,
      "description": "Conversação multi-modelo para diálogos complexos (opcional)"
    },
    "lesstokens": {
      "enabled": false,
      "description": "Compressão para APIs externas como fallback (opcional)"
    }
  }
}
```

**Status dos Serviços**:
- **Enabled**: Serviços habilitados e necessários para funcionalidade completa (documentação, memória, busca)
- **Optional**: Serviços opcionais para funcionalidades avançadas (multi-modelo, fallback API)

**Stack Hive para Memória**:
O sistema de memória utiliza o stack Hive completo:
- **Vectorizer**: Embeddings e busca vetorial semântica
- **Lexum**: Full-text search (busca textual otimizada)
- **Nexus**: Graph relations (relações de grafo entre entidades)

**Propósito**:
- **Track campaign events**: Rastrear eventos importantes da campanha
- **NPC knowledge**: Manter conhecimento sobre NPCs e suas relações
- **Player history**: Histórico de ações e decisões dos jogadores

## Estrutura de Configuração

```
vrpg-client/
├── config/
│   ├── vrpg.json              # Configuração principal
│   ├── services.json          # Configuração de serviços
│   ├── voices.json            # Configuração TTS
│   ├── mcp.json              # Endpoints MCP
│   ├── models.json           # Caminhos de modelos
│   ├── ui.json               # Preferências de interface
│   └── logging.json          # Configuração de logs
├── .env                      # Variáveis de ambiente
└── .env.example             # Template de variáveis
```

## Configuração Principal (`config/vrpg.json`)

```json
{
  "version": "1.0.0",
  "profile": "development",
  "
  
  "services": {
    "llm_core": {
      "enabled": true,
      "port": 7002,
      "host": "localhost",
      "pipeline_architecture": "dual_model",
      "models": {
        "qwen_1_5b": {
          "model_path": "assets-and-models/models/llm/qwen2.5-1.5b-instruct-q4_k_m.gguf",
          "context_size": 2048,
          "threads": 4,
          "gpu_layers": 20,
          "max_tokens": 40,
          "temperature": 0.8,
          "top_p": 0.9,
          "repeat_penalty": 1.1,
          "role": "prelude"
        },
        "qwen_14b": {
          "model_path": "assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf",
          "context_size": 8192,
          "threads": 8,
          "gpu_layers": 35,
          "max_tokens": 2048,
          "temperature": 0.7,
          "top_p": 0.9,
          "repeat_penalty": 1.1,
          "role": "narration"
        }
      }
    },
    
    "asr_service": {
      "enabled": true,
      "port": 7001,
      "host": "localhost",
      "model_path": "assets-and-models/models/asr/whisper-large-v3.bin",
      "language": "auto",
      "chunk_size_ms": 320,
      "vad_enabled": true,
      "vad_threshold": 0.5
    },
    
    "tts_service": {
      "enabled": true,
      "port": 7003,
      "host": "localhost",
      "default_voice": "dm_default",
      "sample_rate": 48000,
      "cache_enabled": true,
      "cache_size_mb": 256
    },
    
    "rules5e_service": {
      "enabled": true,
      "port": 7004,
      "host": "localhost",
      "srd_path": "assets/data/srd5e.json",
      "house_rules_enabled": false
    },
    
    "memory_service": {
      "enabled": true,
      "port": 7005,
      "host": "localhost",
      "max_memories": 10000,
      "cleanup_interval_hours": 24,
      "transmutation": {
        "enabled": true,
        "supported_formats": ["pdf", "docx", "xlsx", "pptx", "html", "xml", "txt", "csv", "jpg", "png", "mp3", "wav", "mp4"],
        "ocr_language": "eng",
        "optimize_for_llm": true,
        "max_chunk_size": 512,
        "extract_images": false,
        "audio_transcription": true
      }
    },
    
    "game_engine": {
      "auto_save_interval_minutes": 5,
      "max_session_history": 1000,
      "turn_timeout_seconds": 300
    }
  },
  
  "client": {
    "window": {
      "width": 1400,
      "height": 900,
      "min_width": 1200,
      "min_height": 700,
      "resizable": true,
      "fullscreen": false
    },
    
    "audio": {
      "input_device": "default",
      "output_device": "default",
      "input_volume": 0.8,
      "output_volume": 0.9,
      "noise_suppression": true,
      "echo_cancellation": true
    },
    
    "ui": {
      "theme": "cyberpunk",
      "animation_speed": "normal",
      "particle_density": "medium",
      "glow_intensity": 0.8,
      "font_size": "medium"
    }
  },
  
  "performance": {
    "max_cpu_usage": 0.8,
    "max_memory_mb": 8192,
    "gc_interval_minutes": 10,
    "metrics_enabled": true
  },
  
  "security": {
    "ipc_validation": true,
    "resource_limits": true,
    "audit_logging": true
  }
}
```

## Configuração de Serviços (`config/services.json`)

```json
{
  "startup_order": [
    "rules5e_service",
    "memory_service", 
    "asr_service",
    "tts_service",
    "llm_core",
    "game_engine"
  ],
  
  "health_check": {
    "interval_seconds": 30,
    "timeout_seconds": 5,
    "max_failures": 3,
    "retry_delay_seconds": 10
  },
  
  "restart_policy": {
    "enabled": true,
    "max_restarts": 5,
    "restart_window_minutes": 15,
    "backoff_multiplier": 2.0
  },
  
  "resource_limits": {
    "llm_core": {
      "max_memory_mb": 6144,
      "max_cpu_percent": 80
    },
    "asr_service": {
      "max_memory_mb": 1024,
      "max_cpu_percent": 40
    },
    "tts_service": {
      "max_memory_mb": 512,
      "max_cpu_percent": 30
    }
  }
}
```

## Configuração de Vozes (`config/voices.json`)

```json
{
  "voices": {
    "dm_default": {
      "name": "Dungeon Master",
      "description": "Voz principal do Mestre",
      "piper_model": "piper-pt-br",
      "sovits_model": "assets/voices/dm_default.sovits",
      "language": "en",
      "effects": [],
      "volume": 1.0,
      "speed": 1.0
    },
    
    "dm_narrator": {
      "name": "Narrator",
      "description": "Voz narrativa cinematográfica",
      "piper_model": "piper-pt-br",
      "sovits_model": "assets/voices/narrator.sovits", 
      "speaker_embedding": "assets/voices/narrator.spk",
      "language": "en",
      "effects": ["reverb_large_hall"],
      "volume": 0.9,
      "speed": 0.9
    },
    
    "npc_old_wizard": {
      "name": "Old Wizard",
      "description": "Mago idoso sábio",
      "piper_model": "piper-pt-br",
      "sovits_model": "assets/voices/narrator.sovits",
      "speaker_embedding": "assets/voices/old_wizard.spk", 
      "language": "en",
      "effects": ["highpass_200", "reverb_small_room"],
      "volume": 0.8,
      "speed": 0.8
    },
    
    "npc_tavern_keeper": {
      "name": "Tavern Keeper",
      "description": "Taverneiro amigável",
      "base_model": "piper_en",
      "speaker_embedding": null,
      "language": "en",
      "effects": ["warmth_filter"],
      "volume": 1.0,
      "speed": 1.1
    },
    
    "monster_dragon": {
      "name": "Ancient Dragon",
      "description": "Dragão ancestral",
      "piper_model": "piper-pt-br",
      "sovits_model": "assets/voices/narrator.sovits",
      "speaker_embedding": "assets/voices/dragon.spk",
      "language": "en", 
      "effects": ["pitch_down_12", "reverb_cathedral", "distortion_heavy"],
      "volume": 1.2,
      "speed": 0.7
    },
    
    "monster_goblin": {
      "name": "Goblin",
      "description": "Goblin irritante",
      "base_model": "piper_en",
      "speaker_embedding": null,
      "language": "en",
      "effects": ["pitch_up_8", "highpass_400"],
      "volume": 0.9,
      "speed": 1.3
    }
  },
  
  "effects": {
    "reverb_small_room": {
      "type": "reverb",
      "room_size": 0.3,
      "damping": 0.5,
      "wet_level": 0.2
    },
    
    "reverb_large_hall": {
      "type": "reverb", 
      "room_size": 0.8,
      "damping": 0.3,
      "wet_level": 0.4
    },
    
    "reverb_cathedral": {
      "type": "reverb",
      "room_size": 1.0,
      "damping": 0.2,
      "wet_level": 0.6
    },
    
    "pitch_down_8": {
      "type": "pitch_shift",
      "semitones": -8
    },
    
    "pitch_down_12": {
      "type": "pitch_shift", 
      "semitones": -12
    },
    
    "pitch_up_8": {
      "type": "pitch_shift",
      "semitones": 8
    },
    
    "highpass_200": {
      "type": "highpass_filter",
      "frequency": 200
    },
    
    "highpass_400": {
      "type": "highpass_filter",
      "frequency": 400
    },
    
    "lowpass_500": {
      "type": "lowpass_filter",
      "frequency": 500
    },
    
    "distortion_light": {
      "type": "distortion",
      "drive": 0.3,
      "tone": 0.5
    },
    
    "distortion_heavy": {
      "type": "distortion",
      "drive": 0.8,
      "tone": 0.3
    },
    
    "warmth_filter": {
      "type": "eq",
      "low_gain": 2,
      "mid_gain": 0,
      "high_gain": -1
    }
  }
}
```

## Configuração de Transmutation (`config/transmutation.json`)

```json
{
  "enabled": true,
  "conversion_options": {
    "optimize_for_llm": true,
    "max_chunk_size": 512,
    "normalize_whitespace": true,
    "remove_headers_footers": false,
    "extract_images": false,
    "extract_tables": true,
    "preserve_layout": false
  },
  
  "ocr_settings": {
    "enabled": true,
    "language": "eng",
    "dpi": 300,
    "image_quality": "high"
  },
  
  "audio_settings": {
    "enabled": true,
    "whisper_model": "base",
    "language": "auto"
  },
  
  "supported_formats": {
    "documents": ["pdf", "docx", "xlsx", "pptx", "html", "xml", "txt", "csv", "rtf", "odt"],
    "images": ["jpg", "jpeg", "png", "tiff", "bmp", "gif", "webp"],
    "audio": ["mp3", "wav", "m4a", "flac", "ogg"],
    "video": ["mp4", "avi", "mkv", "mov", "webm"],
    "archives": ["zip", "tar", "gz", "7z"]
  },
  
  "performance": {
    "parallel_conversions": 4,
    "max_file_size_mb": 100,
    "timeout_seconds": 300,
    "cache_enabled": true,
    "cache_size_mb": 256
  }
}
```

## Configuração MCP (`config/mcp.json`)

```json
{
  "enabled": true,
  "timeout_seconds": 10,
  "retry_attempts": 3,
  "retry_delay_ms": 1000,
  
  "services": {
    "synap": {
      "enabled": true,
      "endpoint": "http://localhost:8001",
      "api_key": "${SYNAP_API_KEY}",
      "max_participants": 8,
      "conversation_memory": 50,
      "timeout_seconds": 15
    },
    
    "classify": {
      "enabled": true,
      "endpoint": "http://localhost:8003",
      "api_key": "${CLASSIFY_API_KEY}",
      "batch_size": 10,
      "confidence_threshold": 0.8,
      "timeout_seconds": 5
    },
    
    "nexus": {
      "enabled": true,
      "endpoint": "http://localhost:8004",
      "api_key": "${NEXUS_API_KEY}",
      "query_expansion": true,
      "spell_check": true,
      "timeout_seconds": 3
    },
    
    "lexum": {
      "enabled": true,
      "endpoint": "http://localhost:8005",
      "api_key": "${LEXUM_API_KEY}",
      "fulltext_optimization": true,
      "index_optimization": "high",
      "timeout_seconds": 5
    },
    
    "vectorizer": {
      "enabled": true,
      "endpoint": "http://localhost:8002",
      "api_key": "${VECTORIZER_API_KEY}",
      "index_name": "vrpg_memories",
      "embedding_model": "all-MiniLM-L6-v2",
      "max_results": 20,
      "timeout_seconds": 8
    }
  },
  
  "fallback": {
    "enabled": true,
    "local_memory_only": true,
    "simple_search": true,
    "basic_classification": true
  }
}
```

## Configuração de Modelos (`config/models.json`)

```json
{
  "models": {
    "llm": {
      "pipeline_architecture": "dual_model",
      "qwen_1_5b": {
        "name": "Qwen2.5-1.5B-Instruct",
        "role": "Mestre Reflexo",
        "path": "assets-and-models/models/llm/qwen2.5-1.5b-instruct-q4_k_m.gguf",
        "size_gb": 1.0,
        "context_size": 2048,
        "max_tokens": 40,
        "latency_target_ms": 1200,
        "checksum": "sha256:abc123...",
        "download_url": "https://huggingface.co/..."
      },
      "qwen_14b": {
        "name": "Qwen2.5-14B-Instruct",
        "role": "Mestre Real",
        "path": "assets-and-models/models/llm/qwen2.5-14b-instruct-q4_k_m.gguf",
        "size_gb": 8.2,
        "context_size": 8192,
        "max_tokens": 2048,
        "latency_target_ms": 6000,
        "checksum": "sha256:def456...",
        "download_url": "https://huggingface.co/..."
      },
      "fallback": {
        "name": "Llama-3.1-8B-Instruct", 
        "path": "assets-and-models/models/llm/llama3_1_8b.q4_k_m.gguf",
        "size_gb": 4.6,
        "context_size": 4096,
        "checksum": "sha256:ghi789...",
        "download_url": "https://huggingface.co/..."
      }
    },
    
    "asr": {
      "primary": {
        "name": "Whisper Large v3",
        "path": "assets-and-models/models/asr/whisper-large-v3.bin",
        "size_gb": 2.9,
        "languages": ["en", "pt", "es", "fr"],
        "checksum": "sha256:ghi789...",
        "download_url": "https://huggingface.co/..."
      },
      
      "fallback": {
        "name": "Whisper Medium",
        "path": "assets-and-models/models/asr/whisper-medium.bin", 
        "size_gb": 1.5,
        "languages": ["en", "pt"],
        "checksum": "sha256:jkl012...",
        "download_url": "https://huggingface.co/..."
      }
    },
    
    "tts": {
      "piper": {
        "name": "Piper TTS",
        "path_pt": "assets-and-models/models/tts/piper-pt-br.onnx",
        "path_en": "assets-and-models/models/tts/piper-en-us.onnx",
        "size_gb": 1.8,
        "languages": ["en", "pt", "es", "fr"],
        "voice_cloning": true,
        "checksum": "sha256:mno345...",
        "download_url": "https://huggingface.co/..."
      },
      
      "sovits": {
        "name": "SoVITS Voice Conversion",
        "path": "assets-and-models/models/tts/sovits/",
        "description": "Modelos SoVITS por personagem",
        "models": {
          "narrator_default": {
            "path": "narrator_default.sovits",
            "size_mb": 150,
            "emotions": ["neutral", "calm", "dramatic"]
          },
          "npc_guard": {
            "path": "npc_guard.sovits",
            "size_mb": 150,
            "emotions": ["authoritative", "skeptic", "angry"]
          },
          "npc_barkeep": {
            "path": "npc_barkeep.sovits",
            "size_mb": 150,
            "emotions": ["friendly", "warm", "amused"]
          }
        },
        "checksum": "sha256:pqr678...",
        "download_url": "https://huggingface.co/..."
      }
    }
  },
  
  "auto_download": {
    "enabled": true,
    "check_checksums": true,
    "parallel_downloads": 2,
    "retry_attempts": 3
  }
}
```

## Configuração de UI (`config/ui.json`)

```json
{
  "theme": {
    "name": "cyberpunk",
    "colors": {
      "primary": "#00E5FF",
      "secondary": "#7B4CFF", 
      "accent": "#FF2EFF",
      "success": "#39FF14",
      "warning": "#FF6B35",
      "background": "#000000"
    },
    
    "animations": {
      "enabled": true,
      "speed": "normal",
      "particle_effects": true,
      "glow_effects": true,
      "transitions": "smooth"
    },
    
    "layout": {
      "sidebar_width": 200,
      "scene_notes_width": 180,
      "top_bar_height": 60,
      "voice_bar_height": 80,
      "panel_border_radius": 8
    }
  },
  
  "accessibility": {
    "high_contrast": false,
    "reduced_motion": false,
    "font_scaling": 1.0,
    "keyboard_navigation": true,
    "screen_reader": true
  },
  
  "hotkeys": {
    "talk": "Space",
    "mute": "M",
    "next_turn": "Enter",
    "open_character_sheet": "C",
    "open_inventory": "I",
    "open_log": "L",
    "toggle_fullscreen": "F11",
    "quit": "Ctrl+Q"
  },
  
  "notifications": {
    "enabled": true,
    "position": "top-right",
    "duration_seconds": 5,
    "sound_enabled": true
  }
}
```

## Configuração de Logging (`config/logging.json`)

```json
{
  "level": "info",
  "format": "json",
  "output": "file",
  
  "file": {
    "path": "logs/vrpg.log",
    "max_size_mb": 100,
    "max_files": 10,
    "rotation": "daily"
  },
  
  "console": {
    "enabled": true,
    "level": "warn",
    "colors": true
  },
  
  "modules": {
    "llm_core": "debug",
    "asr_service": "info", 
    "tts_service": "info",
    "rules5e_service": "warn",
    "memory_service": "info",
    "game_engine": "debug",
    "client_electron": "info",
    "mcp_integration": "debug"
  },
  
  "structured_logging": {
    "enabled": true,
    "include_timestamps": true,
    "include_thread_id": true,
    "include_module": true
  },
  
  "metrics": {
    "enabled": true,
    "interval_seconds": 60,
    "file": "logs/metrics.json"
  }
}
```

## Variáveis de Ambiente (`.env`)

```bash
# Configuração de Ambiente
NODE_ENV=development
RUST_LOG=info
VRPG_PROFILE=development

# Caminhos
VRPG_CONFIG_DIR=./config
VRPG_ASSETS_DIR=./assets
VRPG_LOGS_DIR=./logs
VRPG_DATA_DIR=./data

# APIs Externas (Fallback)
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
GOOGLE_API_KEY=...

# MCP Services
SYNAP_API_KEY=synap_...
CLASSIFY_API_KEY=classify_...
NEXUS_API_KEY=nexus_...
LEXUM_API_KEY=lexum_...
VECTORIZER_API_KEY=vectorizer_...

# MCP Endpoints
SYNAP_ENDPOINT=http://localhost:8001
CLASSIFY_ENDPOINT=http://localhost:8003
NEXUS_ENDPOINT=http://localhost:8004
LEXUM_ENDPOINT=http://localhost:8005
VECTORIZER_ENDPOINT=http://localhost:8002

# Performance
VRPG_MAX_MEMORY_MB=8192
VRPG_MAX_CPU_PERCENT=80
VRPG_THREAD_COUNT=8

# Segurança
VRPG_ENABLE_AUDIT=true
VRPG_ENABLE_ENCRYPTION=true
VRPG_SECRET_KEY=...

# Debug
VRPG_DEBUG_MODE=false
VRPG_VERBOSE_LOGGING=false
VRPG_PROFILE_PERFORMANCE=false
```

## Perfis de Configuração

### Desenvolvimento (`profiles/development.json`)
```json
{
  "extends": "base",
  "overrides": {
    "logging.level": "debug",
    "logging.console.enabled": true,
    "performance.metrics_enabled": true,
    "client.window.fullscreen": false,
    "services.llm_core.gpu_layers": 0,
    "mcp.timeout_seconds": 30
  }
}
```

### Produção (`profiles/production.json`)
```json
{
  "extends": "base", 
  "overrides": {
    "logging.level": "warn",
    "logging.console.enabled": false,
    "performance.max_cpu_usage": 0.6,
    "security.audit_logging": true,
    "services.llm_core.gpu_layers": 35,
    "mcp.timeout_seconds": 5
  }
}
```

### Performance (`profiles/performance.json`)
```json
{
  "extends": "base",
  "overrides": {
    "services.llm_core.threads": 16,
    "services.llm_core.gpu_layers": 50,
    "services.asr_service.chunk_size_ms": 160,
    "services.tts_service.cache_size_mb": 512,
    "client.ui.animation_speed": "fast",
    "client.ui.particle_density": "low"
  }
}
```

## Validação de Configuração

### Schema Validation
```rust
use serde::{Deserialize, Serialize};
use jsonschema::JSONSchema;

#[derive(Deserialize, Serialize)]
pub struct VrpgConfig {
    pub version: String,
    pub profile: String,
    pub services: ServicesConfig,
    pub client: ClientConfig,
    pub performance: PerformanceConfig,
    pub security: SecurityConfig,
}

impl VrpgConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validar portas não conflitantes
        let ports = self.services.get_all_ports();
        if ports.len() != ports.iter().collect::<std::collections::HashSet<_>>().len() {
            return Err(ConfigError::DuplicatePorts);
        }
        
        // Validar caminhos de modelos
        for model_path in self.services.get_model_paths() {
            if !std::path::Path::new(&model_path).exists() {
                return Err(ConfigError::ModelNotFound(model_path));
            }
        }
        
        // Validar limites de recursos
        if self.performance.max_memory_mb < 2048 {
            return Err(ConfigError::InsufficientMemory);
        }
        
        Ok(())
    }
}
```

### Runtime Configuration Reload
```rust
pub struct ConfigManager {
    config: Arc<RwLock<VrpgConfig>>,
    watchers: Vec<FileWatcher>,
}

impl ConfigManager {
    pub async fn reload_config(&self) -> Result<()> {
        let new_config = VrpgConfig::load_from_files()?;
        new_config.validate()?;
        
        let mut config = self.config.write().await;
        *config = new_config;
        
        // Notificar serviços sobre mudanças
        self.notify_services_config_changed().await?;
        
        Ok(())
    }
}
```

## Comandos de Configuração

### CLI Configuration Tool
```bash
# Visualizar configuração atual
vrpg-client config show

# Validar configuração
vrpg-client config validate

# Alterar configuração específica
vrpg-client config set services.llm_core.temperature 0.8

# Resetar para padrões
vrpg-client config reset

# Exportar configuração
vrpg-client config export --format json --output config-backup.json

# Importar configuração
vrpg-client config import --file config-backup.json

# Listar perfis disponíveis
vrpg-client config profiles list

# Aplicar perfil
vrpg-client config profiles apply production
```

### Environment Setup
```bash
# Setup inicial
npm run setup:config

# Gerar configuração de exemplo
npm run config:generate-example

# Validar configuração
npm run config:validate

# Aplicar configuração de desenvolvimento
npm run config:dev

# Aplicar configuração de produção  
npm run config:prod
```

---

Este sistema de configuração oferece flexibilidade máxima mantendo simplicidade de uso, permitindo desde configurações básicas até customizações avançadas para diferentes ambientes e casos de uso.
