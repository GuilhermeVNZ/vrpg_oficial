use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub confidence: f32,
    pub language: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioChunk {
    pub data: Vec<f32>,
    pub sample_rate: u32,
    pub timestamp_ms: u64,
}

pub struct WhisperModel {
    model_loaded: bool,
    use_gpu: bool,
    model_size: String,
    script_path: std::path::PathBuf,
}

impl Default for WhisperModel {
    fn default() -> Self {
        Self::new()
    }
}

impl WhisperModel {
    pub fn new() -> Self {
        let script_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| {
                p.join("src")
                    .join("asr-service")
                    .join("scripts")
                    .join("whisper_transcribe.py")
            })
            .unwrap_or_else(|| std::path::PathBuf::from("scripts/whisper_transcribe.py"));

        Self {
            model_loaded: false,
            use_gpu: std::env::var("VRPG_ASR_USE_GPU")
                .unwrap_or_default()
                .to_lowercase()
                == "true",
            model_size: "large-v3".to_string(),
            script_path,
        }
    }

    pub fn new_with_options(use_gpu: bool, model_size: Option<String>) -> Self {
        let script_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| {
                p.join("src")
                    .join("asr-service")
                    .join("scripts")
                    .join("whisper_transcribe.py")
            })
            .unwrap_or_else(|| std::path::PathBuf::from("scripts/whisper_transcribe.py"));

        Self {
            model_loaded: false,
            use_gpu,
            model_size: model_size.unwrap_or_else(|| "large-v3".to_string()),
            script_path,
        }
    }

    pub async fn load(&mut self, _model_path: &str) -> Result<()> {
        // Verificar se o script Python existe
        if !self.script_path.exists() {
            warn!("Whisper Python script not found at: {:?}", self.script_path);
            // Tentar caminho alternativo
            let alt_path =
                std::path::PathBuf::from("src/asr-service/scripts/whisper_transcribe.py");
            if alt_path.exists() {
                self.script_path = alt_path;
            } else {
                return Err(crate::error::AsrError::ModelLoad(format!(
                    "Whisper Python script not found at: {:?}",
                    self.script_path
                )));
            }
        }

        self.model_loaded = true;
        info!(
            "Whisper model loaded (GPU: {}, Model: {})",
            self.use_gpu, self.model_size
        );
        Ok(())
    }

    pub async fn transcribe(&self, chunk: &AudioChunk) -> Result<TranscriptionResult> {
        use crate::error::AsrError;
        if !self.model_loaded {
            return Err(AsrError::ModelLoad("Model not loaded".to_string()));
        }

        info!(
            "🎤 Whisper transcribing audio chunk ({} samples, {} Hz)",
            chunk.data.len(),
            chunk.sample_rate
        );

        // Chamar Python bridge para transcrição real
        self.transcribe_with_faster_whisper(chunk).await
    }

    /// Transcreve usando faster-whisper via Python bridge
    async fn transcribe_with_faster_whisper(
        &self,
        chunk: &AudioChunk,
    ) -> Result<TranscriptionResult> {
        use crate::error::AsrError;
        use serde_json::json;

        // Criar JSON de entrada
        let input_data = json!({
            "audio_data": chunk.data,
            "sample_rate": chunk.sample_rate,
            "language": "auto",
            "use_gpu": self.use_gpu,
            "model_size": self.model_size,
        });

        // Executar script Python com stdin redirecionado
        let mut child = tokio::process::Command::new("python")
            .arg(&self.script_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| AsrError::ModelLoad(format!("Failed to spawn Whisper Python: {}", e)))?;

        // Escrever input JSON no stdin
        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            let input_json = serde_json::to_string(&input_data)?;
            stdin
                .write_all(input_json.as_bytes())
                .await
                .map_err(|e| AsrError::ModelLoad(format!("Failed to write to stdin: {}", e)))?;
            stdin
                .shutdown()
                .await
                .map_err(|e| AsrError::ModelLoad(format!("Failed to shutdown stdin: {}", e)))?;
        }

        // Aguardar output
        let output = child
            .wait_with_output()
            .await
            .map_err(|e| AsrError::ModelLoad(format!("Failed to get Whisper output: {}", e)))?;

        // Verificar stderr para logs
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            info!("Whisper Python stderr: {}", stderr);
        }

        if !output.status.success() {
            return Err(AsrError::ModelLoad(format!(
                "Whisper Python script failed: {}",
                stderr
            )));
        }

        // Parse JSON output
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            return Err(AsrError::ModelLoad(
                "Whisper Python script returned empty output".to_string(),
            ));
        }

        let result: serde_json::Value = serde_json::from_str(stdout.trim()).map_err(|e| {
            AsrError::ModelLoad(format!(
                "Failed to parse Whisper output: {} (stdout: {})",
                e,
                stdout.len()
            ))
        })?;

        if let Some(error) = result.get("error") {
            return Err(AsrError::ModelLoad(format!("Whisper error: {}", error)));
        }

        // Extrair resultado
        let text = result["text"]
            .as_str()
            .ok_or_else(|| AsrError::ModelLoad("Invalid text in Whisper output".to_string()))?
            .to_string();

        let confidence = result["confidence"].as_f64().ok_or_else(|| {
            AsrError::ModelLoad("Invalid confidence in Whisper output".to_string())
        })? as f32;

        let language = result["language"]
            .as_str()
            .ok_or_else(|| AsrError::ModelLoad("Invalid language in Whisper output".to_string()))?
            .to_string();

        let duration_ms = result["duration_ms"].as_u64().ok_or_else(|| {
            AsrError::ModelLoad("Invalid duration_ms in Whisper output".to_string())
        })?;

        info!(
            "✅ Whisper transcription complete: '{}' (lang: {}, confidence: {:.2})",
            text.chars().take(50).collect::<String>(),
            language,
            confidence
        );

        Ok(TranscriptionResult {
            text,
            confidence,
            language,
            duration_ms,
        })
    }

    pub fn is_loaded(&self) -> bool {
        self.model_loaded
    }
}

pub type SharedWhisperModel = Arc<RwLock<WhisperModel>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_whisper_model_loading() {
        let mut model = WhisperModel::new();
        assert!(!model.is_loaded());

        model.load("test_model").await.unwrap();
        assert!(model.is_loaded());
    }

    #[tokio::test]
    async fn test_whisper_transcribe() {
        let mut model = WhisperModel::new();
        model.load("test_model").await.unwrap();

        let chunk = AudioChunk {
            data: vec![0.0; 16000], // 1 second at 16kHz
            sample_rate: 16000,
            timestamp_ms: 0,
        };

        let result = model.transcribe(&chunk).await.unwrap();
        assert!(!result.text.is_empty());
        assert!(result.confidence > 0.0);
    }

    #[tokio::test]
    async fn test_whisper_transcribe_not_loaded() {
        let model = WhisperModel::new();
        let chunk = AudioChunk {
            data: vec![0.0; 16000],
            sample_rate: 16000,
            timestamp_ms: 0,
        };

        let result = model.transcribe(&chunk).await;
        assert!(result.is_err());
    }
}
