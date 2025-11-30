# Plano de Migração: Piper → Coqui XTTS

## 📋 Visão Geral

Este documento planeja a integração do **Coqui XTTS v2** como alternativa/fallback ao Piper TTS, mantendo compatibilidade total com a arquitetura atual.

## 🎯 Objetivos

1. **Drop-in Replacement**: XTTS deve funcionar como substituto direto do Piper
2. **Fallback Automático**: Sistema deve tentar Piper primeiro, usar XTTS se falhar
3. **Compatibilidade**: Manter mesma interface (`PiperAudio`, `sample_rate`, etc.)
4. **Performance**: Minimizar overhead de carregamento de modelos

## 🏗️ Arquitetura Atual

### Estrutura Atual
```
pipeline.rs
  └─> PiperModel::synthesize() → PiperAudio
       └─> SoVITS (conversão de voz)
```

### Interface Piper
```rust
pub struct PiperAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

pub trait TtsEngine {
    fn synthesize(&self, text: &str, language: &str) -> Result<PiperAudio>;
}
```

## 📌 Status Atual

**Arquivo existente**: `src/xtts.rs` 
- ✅ Estrutura básica já existe (`XttsModel`, `AudioOutput`, etc.)
- ⚠️ **Implementação atual é stub**: Gera áudio sintético, não usa modelo real
- ⚠️ **Não integrado**: Não está conectado ao pipeline principal

**Próximos passos**: Substituir implementação stub por integração real com Coqui XTTS

## 🔄 Estratégia de Implementação

### Fase 1: Atualizar Módulo XTTS Existente (Python Bridge)

**Arquivo**: `src/xtts.rs` (ATUALIZAR implementação existente)

**Mudanças necessárias**:
1. Substituir `generate_audio_basic()` por chamada real ao Coqui XTTS
2. Implementar Python bridge (similar ao Piper fallback)
3. Manter interface existente (`AudioOutput`, `SynthesisRequest`, etc.)

**Código a adicionar/modificar**:

```rust
// Manter estruturas existentes, adicionar método real:
impl XttsModel {
    // ... código existente ...

    // SUBSTITUIR generate_audio_basic() por:
    async fn synthesize_with_coqui_xtts(
        &self,
        text: &str,
        language: &str,
        speaker: Option<&str>,
    ) -> Result<AudioOutput> {
        // Python bridge para Coqui XTTS
        self.synthesize_python_bridge(text, language, speaker).await
    }

    async fn synthesize_python_bridge(
        &self,
        text: &str,
        language: &str,
        speaker: Option<&str>,
    ) -> Result<AudioOutput> {
        // Criar script Python temporário
        let temp_script = std::env::temp_dir().join(format!("xtts_synth_{}.py", std::process::id()));
        
        let python_script = format!(r#"
import sys
import json
import numpy as np
from TTS.api import TTS

# Parâmetros
text = r"{}"
language = "{}"
speaker = {}
use_gpu = True  # TODO: tornar configurável

try:
    # Carregar modelo XTTS (usa cache se já baixado)
    tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu)
    
    # Gerar áudio
    audio = tts.tts(
        text=text,
        speaker=speaker if speaker else None,
        language=language,
    )
    
    # Converter para lista JSON
    audio_list = audio.tolist() if isinstance(audio, np.ndarray) else list(audio)
    
    # Output: JSON com samples e sample_rate
    output = {{
        "samples": audio_list,
        "sample_rate": tts.synthesizer.output_sample_rate,
        "channels": 1
    }}
    
    print(json.dumps(output))
    
except Exception as e:
    print(json.dumps({{"error": str(e)}}), file=sys.stderr)
    sys.exit(1)
"#,
            text.replace('"', r#"\""#),
            language,
            if let Some(s) = speaker {
                format!(r#""{}""#, s.replace('"', r#"\""#))
            } else {
                "None".to_string()
            },
        );
        
        std::fs::write(&temp_script, python_script)
            .map_err(|e| TtsError::ModelLoad(format!("Failed to write Python script: {}", e)))?;
        
        // Executar Python
        let output = tokio::process::Command::new("python")
            .arg(&temp_script)
            .output()
            .await
            .map_err(|e| TtsError::ModelLoad(format!("Failed to run Python: {}", e)))?;
        
        // Limpar script temporário
        let _ = std::fs::remove_file(&temp_script);
        
        // Verificar erros
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            warn!("XTTS Python stderr: {}", stderr);
        }
        
        if !output.status.success() {
            return Err(TtsError::ModelLoad(format!("XTTS Python script failed: {}", stderr)));
        }
        
        // Parse output JSON
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: serde_json::Value = serde_json::from_str(stdout.trim())
            .map_err(|e| TtsError::ModelLoad(format!("Failed to parse XTTS output: {}", e)))?;
        
        if let Some(error) = result.get("error") {
            return Err(TtsError::ModelLoad(format!("XTTS error: {}", error)));
        }
        
        let samples: Vec<f32> = result["samples"]
            .as_array()
            .ok_or_else(|| TtsError::ModelLoad("Invalid samples array".to_string()))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();
        
        let sample_rate = result["sample_rate"]
            .as_u64()
            .unwrap_or(22050) as u32;
        
        Ok(AudioOutput {
            samples,
            sample_rate,
            channels: 1,
        })
    }
}
```

**Código completo atualizado** (substituir `generate_audio_basic`):
//! Coqui XTTS Module - High-quality multilingual TTS
//! Uses Python TTS library as bridge (similar to current Piper Python fallback)

use crate::error::{Result, TtsError};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XTTSAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

// Compatibilidade com PiperAudio
impl From<XTTSAudio> for crate::piper::PiperAudio {
    fn from(xtts: XTTSAudio) -> Self {
        crate::piper::PiperAudio {
            samples: xtts.samples,
            sample_rate: xtts.sample_rate,
            channels: xtts.channels,
        }
    }
}

pub struct XTTSModel {
    inner: Option<Arc<XTTSModelInner>>,
    model_path: Option<std::path::PathBuf>,
    use_gpu: bool,
}

struct XTTSModelInner {
    model_path: std::path::PathBuf,
    config_path: std::path::PathBuf,
    sample_rate: u32,
}

impl XTTSModel {
    pub fn new() -> Self {
        Self {
            inner: None,
            model_path: None,
            use_gpu: true, // Default: usar GPU se disponível
        }
    }

    pub async fn load(&mut self, model_path: &Path) -> Result<()> {
        // Verificar se modelo existe
        let config_path = model_path.parent()
            .ok_or_else(|| TtsError::ModelLoad("Invalid model path".to_string()))?
            .join("config.json");
        
        if !model_path.exists() {
            return Err(TtsError::ModelLoad(format!("XTTS model not found: {:?}", model_path)));
        }
        
        // XTTS padrão: 22050 Hz (mesmo do Piper)
        let sample_rate = 22050u32;
        
        self.inner = Some(Arc::new(XTTSModelInner {
            model_path: model_path.to_path_buf(),
            config_path,
            sample_rate,
        }));
        
        self.model_path = Some(model_path.to_path_buf());
        
        info!("✅ XTTS model loaded: {:?}", model_path);
        Ok(())
    }

    pub async fn synthesize(
        &self,
        text: &str,
        language: &str,
        speaker: Option<&str>, // XTTS suporta speakers
    ) -> Result<XTTSAudio> {
        let inner = self.inner.as_ref()
            .ok_or_else(|| TtsError::ModelLoad("XTTS model not loaded".to_string()))?;
        
        info!("🎤 XTTS synthesizing: '{}' (lang: {}, speaker: {:?})", 
            text, language, speaker);
        
        // Usar Python bridge (similar ao Piper fallback)
        self.synthesize_python_bridge(inner, text, language, speaker).await
    }

    async fn synthesize_python_bridge(
        &self,
        inner: &XTTSModelInner,
        text: &str,
        language: &str,
        speaker: Option<&str>,
    ) -> Result<XTTSAudio> {
        // Criar script Python temporário
        let temp_script = std::env::temp_dir().join(format!("xtts_synth_{}.py", std::process::id()));
        
        let python_script = format!(r#"
import sys
import json
import numpy as np
from TTS.api import TTS

# Parâmetros
text = r"{}"
language = "{}"
speaker = {}
model_path = r"{}"
use_gpu = {}

try:
    # Carregar modelo XTTS
    if model_path:
        tts = TTS(
            model_path=model_path,
            config_path=r"{}",
            progress_bar=False,
            gpu=use_gpu
        )
    else:
        # Usar modelo pré-baixado do cache
        tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=use_gpu)
    
    # Gerar áudio
    # XTTS retorna numpy array diretamente
    audio = tts.tts(
        text=text,
        speaker=speaker if speaker else None,
        language=language,
    )
    
    # Converter para lista JSON
    audio_list = audio.tolist() if isinstance(audio, np.ndarray) else list(audio)
    
    # Output: JSON com samples e sample_rate
    output = {{
        "samples": audio_list,
        "sample_rate": tts.synthesizer.output_sample_rate,
        "channels": 1
    }}
    
    print(json.dumps(output))
    
except Exception as e:
    print(json.dumps({{"error": str(e)}}), file=sys.stderr)
    sys.exit(1)
"#,
            text.replace('"', r#"\""#),
            language,
            if let Some(s) = speaker {
                format!(r#""{}""#, s.replace('"', r#"\""#))
            } else {
                "None".to_string()
            },
            inner.model_path.to_string_lossy().replace('\\', "\\\\"),
            self.use_gpu,
            inner.config_path.to_string_lossy().replace('\\', "\\\\"),
        );
        
        std::fs::write(&temp_script, python_script)
            .map_err(|e| TtsError::ModelLoad(format!("Failed to write Python script: {}", e)))?;
        
        // Executar Python
        let output = tokio::process::Command::new("python")
            .arg(&temp_script)
            .output()
            .await
            .map_err(|e| TtsError::ModelLoad(format!("Failed to run Python: {}", e)))?;
        
        // Limpar script temporário
        let _ = std::fs::remove_file(&temp_script);
        
        // Verificar erros
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            warn!("XTTS Python stderr: {}", stderr);
        }
        
        if !output.status.success() {
            return Err(TtsError::ModelLoad(format!("XTTS Python script failed: {}", stderr)));
        }
        
        // Parse output JSON
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: serde_json::Value = serde_json::from_str(stdout.trim())
            .map_err(|e| TtsError::ModelLoad(format!("Failed to parse XTTS output: {}", e)))?;
        
        if let Some(error) = result.get("error") {
            return Err(TtsError::ModelLoad(format!("XTTS error: {}", error)));
        }
        
        let samples: Vec<f32> = result["samples"]
            .as_array()
            .ok_or_else(|| TtsError::ModelLoad("Invalid samples array".to_string()))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();
        
        let sample_rate = result["sample_rate"]
            .as_u64()
            .unwrap_or(22050) as u32;
        
        let channels = result["channels"]
            .as_u64()
            .unwrap_or(1) as u16;
        
        info!("✅ XTTS generated {} samples at {} Hz", samples.len(), sample_rate);
        
        Ok(XTTSAudio {
            samples,
            sample_rate,
            channels,
        })
    }
}

pub type SharedXTTSModel = Arc<RwLock<XTTSModel>>;
```

### Fase 2: Atualizar Pipeline para Suportar Fallback

**Arquivo**: `src/pipeline.rs` (modificações)

```rust
use crate::xtts::{XTTSModel, SharedXTTSModel, XTTSAudio};

pub struct TtsPipeline {
    piper_pt: SharedPiperModel,
    piper_en: SharedPiperModel,
    xtts: Option<SharedXTTSModel>, // Opcional: XTTS como fallback
    sovits: SharedSoVITSModel,
    // ...
    use_xtts_fallback: bool, // Flag para habilitar fallback XTTS
}

impl TtsPipeline {
    pub async fn synthesize(&self, text: &str, language: &str) -> Result<PiperAudio> {
        // Tentar Piper primeiro
        let piper_result = match language {
            "pt" | "pt-BR" => self.piper_pt.read().await.synthesize(text, language).await,
            "en" | "en-US" | "en-GB" => self.piper_en.read().await.synthesize(text, language).await,
            _ => self.piper_en.read().await.synthesize(text, "en").await,
        };
        
        // Se Piper falhar E XTTS estiver habilitado, usar XTTS
        if piper_result.is_err() && self.use_xtts_fallback {
            if let Some(xtts) = &self.xtts {
                warn!("⚠️ Piper failed, falling back to XTTS");
                let xtts_audio = xtts.read().await.synthesize(text, language, None).await?;
                return Ok(xtts_audio.into()); // Converter XTTSAudio → PiperAudio
            }
        }
        
        piper_result
    }
}
```

### Fase 3: Configuração e Setup

**Arquivo**: `src/bin/tts-server.rs` (modificações)

```rust
// Adicionar flag de configuração
struct Config {
    // ... existing config
    pub use_xtts: bool,
    pub xtts_model_path: Option<PathBuf>,
}

// No main():
let mut pipeline = TtsPipeline::new(...);

if config.use_xtts {
    let mut xtts = XTTSModel::new();
    if let Some(path) = &config.xtts_model_path {
        xtts.load(path).await?;
    } else {
        // Usar modelo do cache do Coqui TTS
        info!("📦 XTTS will use cached model from Coqui TTS");
    }
    pipeline.set_xtts_fallback(Arc::new(RwLock::new(xtts))).await;
}
```

## 📦 Dependências Python

### requirements.txt (novo arquivo)
```
TTS>=0.20.0
torch>=2.0.0
torchaudio>=2.0.0
```

### Instalação
```bash
pip install TTS
# Ou com GPU:
pip install TTS torch torchaudio --index-url https://download.pytorch.org/whl/cu118
```

## 📁 Estrutura de Arquivos

```
vrpg-client/src/tts-service/
├── src/
│   ├── piper.rs          # (mantido)
│   ├── xtts.rs           # (NOVO)
│   ├── pipeline.rs       # (modificado: adicionar fallback)
│   └── ...
├── assets-and-models/models/tts/
│   ├── piper-en-us.onnx  # (mantido)
│   ├── piper-pt-br.onnx  # (mantido)
│   └── xtts_v2/          # (NOVO - opcional, se baixar manualmente)
│       ├── model.pth
│       ├── config.json
│       ├── vocab.json
│       └── speakers_xtts.pth
└── PLANO_XTTS_MIGRATION.md  # Este arquivo
```

## 🔧 Configuração

### config.toml (exemplo)
```toml
[tts]
# Usar XTTS como fallback se Piper falhar
use_xtts_fallback = true

# Caminho para modelo XTTS (opcional)
# Se None, usa modelo do cache do Coqui TTS
xtts_model_path = null  # ou "assets-and-models/models/tts/xtts_v2"

# Usar GPU para XTTS
xtts_use_gpu = true
```

## 🚀 Plano de Implementação

### Etapa 1: Setup Inicial (1-2 horas)
- [ ] Criar `src/xtts.rs` com estrutura básica
- [ ] Adicionar dependências Python (`TTS`)
- [ ] Testar instalação do Coqui TTS
- [ ] Criar script Python de teste standalone

### Etapa 2: Implementação Core (2-3 horas)
- [ ] Implementar `XTTSModel::load()`
- [ ] Implementar `XTTSModel::synthesize()` com Python bridge
- [ ] Testar geração de áudio básica
- [ ] Validar formato de saída (compatibilidade com `PiperAudio`)

### Etapa 3: Integração Pipeline (1-2 horas)
- [ ] Modificar `TtsPipeline` para suportar XTTS
- [ ] Implementar lógica de fallback
- [ ] Adicionar configuração (flags, paths)
- [ ] Testar fallback Piper → XTTS

### Etapa 4: Testes e Refinamento (1-2 horas)
- [ ] Testar com diferentes idiomas (PT, EN)
- [ ] Testar com diferentes speakers (se aplicável)
- [ ] Validar qualidade de áudio
- [ ] Comparar performance Piper vs XTTS
- [ ] Documentar uso

### Etapa 5: Otimizações (opcional, 1-2 horas)
- [ ] Cache de modelo XTTS (evitar recarregar)
- [ ] Pool de workers Python (se necessário)
- [ ] Métricas de performance
- [ ] Logs detalhados

## ⚠️ Considerações Importantes

### Vantagens XTTS
- ✅ **Multilíngue nativo**: Suporta PT, EN e muitos outros
- ✅ **Alta qualidade**: Melhor que Piper em muitos casos
- ✅ **Speakers**: Suporta diferentes vozes/speakers
- ✅ **Ativo**: Projeto mantido ativamente

### Desvantagens/Desafios
- ⚠️ **Python dependency**: Requer Python + TTS library
- ⚠️ **Tamanho do modelo**: ~1.5GB (vs ~50MB do Piper)
- ⚠️ **Performance**: Mais lento que Piper (especialmente CPU)
- ⚠️ **GPU recomendado**: Funciona melhor com GPU

### Compatibilidade
- ✅ **Interface**: Mesma interface que Piper (`PiperAudio`)
- ✅ **Sample rate**: 22050 Hz (mesmo do Piper)
- ✅ **Formato**: Float32 samples (compatível)

## 🧪 Testes

### Teste Básico
```python
# test_xtts_standalone.py
from TTS.api import TTS

tts = TTS("tts_models/multilingual/multi-dataset/xtts_v2", gpu=True)
tts.tts_to_file(
    text="Hello, this is a test of XTTS.",
    file_path="test_xtts.wav",
    speaker=None,  # Voz padrão
    language="en",
)
```

### Teste Integrado (Rust)
```rust
// Teste via pipeline
let pipeline = TtsPipeline::new(...);
let audio = pipeline.synthesize("Hello World", "en").await?;
// Salvar e verificar qualidade
```

## 📝 Notas de Implementação

1. **Python Bridge**: Similar ao fallback Python atual do Piper, mas usando Coqui TTS
2. **Modelo Cache**: Coqui TTS baixa modelos automaticamente para `~/.local/share/tts/`
3. **GPU Support**: Verificar disponibilidade de GPU antes de habilitar
4. **Error Handling**: Fallback deve ser silencioso (não quebrar pipeline se XTTS falhar)

## 🔗 Referências

- [Coqui TTS Documentation](https://github.com/coqui-ai/TTS)
- [XTTS v2 Model Card](https://huggingface.co/coqui/XTTS-v2)
- [Coqui TTS API Reference](https://github.com/coqui-ai/TTS/wiki/API-Reference)

## ✅ Checklist Final

- [ ] XTTS module atualizado (substituir stub por Coqui XTTS real)
- [ ] Módulo exportado em `lib.rs`
- [ ] Pipeline atualizado com fallback
- [ ] Configuração adicionada
- [ ] Testes básicos passando
- [ ] Documentação atualizada
- [ ] Performance validada
- [ ] Deploy/teste em produção

## 🔧 Modificações Necessárias em Arquivos Existentes

### 1. `src/lib.rs`
```rust
// ADICIONAR:
pub mod xtts;
pub use xtts::{XttsModel, AudioOutput, SynthesisRequest, SharedXttsModel};
```

### 2. `src/xtts.rs`
- [ ] Substituir `generate_audio_basic()` por `synthesize_with_coqui_xtts()`
- [ ] Adicionar método `synthesize_python_bridge()`
- [ ] Atualizar `synthesize()` para usar Coqui XTTS real
- [ ] Manter compatibilidade com interface existente

### 3. `src/pipeline.rs`
- [ ] Adicionar `xtts: Option<SharedXttsModel>`
- [ ] Implementar lógica de fallback Piper → XTTS
- [ ] Adicionar flag `use_xtts_fallback`

### 4. `src/bin/tts-server.rs`
- [ ] Adicionar configuração para XTTS
- [ ] Inicializar XTTS se habilitado
- [ ] Passar XTTS para pipeline

## 📝 Resumo Executivo

**Situação Atual**:
- ✅ Estrutura XTTS já existe (`src/xtts.rs`)
- ⚠️ Implementação é stub (não usa modelo real)
- ⚠️ Não integrado ao pipeline

**Ação Necessária**:
1. **Substituir stub por Coqui XTTS real** (Python bridge)
2. **Integrar ao pipeline** como fallback opcional
3. **Testar e validar** qualidade/performance

**Tempo Estimado**: 4-6 horas de desenvolvimento

**Risco**: Baixo (fallback opcional, não quebra funcionalidade existente)

---

**Status**: 📋 Planejamento Completo - Pronto para implementação

**Próximo Passo**: Implementar `synthesize_python_bridge()` em `src/xtts.rs`

