//! Voice Profiles Module - Character voice profile management
//!
//! This module manages voice profiles for different characters (DM, NPCs, players, monsters).

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceProfile {
    pub character_id: String,
    pub name: String,
    pub character_type: CharacterType,
    pub xtts_embedding_path: Option<PathBuf>,
    pub default_emotion: String,
    pub default_style: String,
    pub supported_emotions: Vec<String>,
    pub supported_styles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CharacterType {
    #[serde(rename = "dm")]
    DungeonMaster,
    #[serde(rename = "npc")]
    NPC,
    #[serde(rename = "player")]
    Player,
    #[serde(rename = "monster")]
    Monster,
}

pub struct VoiceProfileManager {
    profiles: Arc<RwLock<HashMap<String, VoiceProfile>>>,
    base_path: PathBuf,
}

impl VoiceProfileManager {
    pub fn new(base_path: &Path) -> Self {
        Self {
            profiles: Arc::new(RwLock::new(HashMap::new())),
            base_path: base_path.to_path_buf(),
        }
    }

    pub async fn load_profile(&self, profile: VoiceProfile) -> Result<()> {
        let mut profiles = self.profiles.write().await;
        profiles.insert(profile.character_id.clone(), profile);
        Ok(())
    }

    pub async fn get_profile(&self, character_id: &str) -> Option<VoiceProfile> {
        let profiles = self.profiles.read().await;
        profiles.get(character_id).cloned()
    }

    pub async fn list_profiles(&self) -> Vec<VoiceProfile> {
        let profiles = self.profiles.read().await;
        profiles.values().cloned().collect()
    }

    pub async fn load_default_profiles(&self) -> Result<()> {
        // Load default DM profile - usando Ana Florence (voz original do XTTS)
        let dm_profile = VoiceProfile {
            character_id: "dm".to_string(),
            name: "Mestre".to_string(),
            character_type: CharacterType::DungeonMaster,
            xtts_embedding_path: None, // Usa voz original Ana Florence (sem embedding customizado)
            default_emotion: "neutral".to_string(),
            default_style: "neutral".to_string(),
            supported_emotions: vec![
                "neutral".to_string(),
                "calm".to_string(),
                "authoritative".to_string(),
                "dramatic".to_string(),
            ],
            supported_styles: vec![
                "neutral".to_string(),
                "narrative".to_string(),
                "dramatic".to_string(),
            ],
        };
        self.load_profile(dm_profile).await?;
        
        // Load Lax Barros profile (dublador - voz customizada)
        let lax_barros_profile = VoiceProfile {
            character_id: "lax_barros".to_string(),
            name: "Lax Barros".to_string(),
            character_type: CharacterType::DungeonMaster,
            xtts_embedding_path: Some(self.base_path.join("narrator_default_xtts_reference_clean.wav")),
            default_emotion: "neutral".to_string(),
            default_style: "neutral".to_string(),
            supported_emotions: vec![
                "neutral".to_string(),
                "calm".to_string(),
                "authoritative".to_string(),
                "dramatic".to_string(),
            ],
            supported_styles: vec![
                "neutral".to_string(),
                "narrative".to_string(),
                "dramatic".to_string(),
            ],
        };
        self.load_profile(lax_barros_profile).await?;

        // Try to auto-discover XTTS embeddings
        self.auto_discover_xtts_embeddings().await?;

        Ok(())
    }

    pub async fn auto_discover_xtts_embeddings(&self) -> Result<()> {
        let embeddings_dir = self.base_path.join("xtts_embeddings");

        if !embeddings_dir.exists() {
            tracing::warn!("XTTS embeddings directory does not exist: {:?}", embeddings_dir);
            return Ok(());
        }

        let mut profiles = self.profiles.write().await;

        // Scan for .wav files (XTTS embeddings)
        match std::fs::read_dir(&embeddings_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("wav") {
                            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                                // Extract character_id from filename (e.g., "narrator_default_xtts_reference_clean" -> "lax_barros")
                                let mut character_id = file_stem
                                    .replace("_xtts_reference_clean", "")
                                    .replace("_xtts_reference", "")
                                    .to_string();
                                
                                // Map legacy "narrator_default" to "lax_barros"
                                if character_id == "narrator_default" {
                                    character_id = "lax_barros".to_string();
                                }

                                // Create or update profile
                                if let Some(profile) = profiles.get_mut(&character_id) {
                                    profile.xtts_embedding_path = Some(path.clone());
                                } else {
                                    // Create new profile for discovered embedding
                                    let new_profile = VoiceProfile {
                                        character_id: character_id.clone(),
                                        name: character_id.replace("_", " "),
                                        character_type: CharacterType::NPC,
                                        xtts_embedding_path: Some(path),
                                        default_emotion: "neutral".to_string(),
                                        default_style: "neutral".to_string(),
                                        supported_emotions: vec![
                                            "neutral".to_string(),
                                            "calm".to_string(),
                                            "angry".to_string(),
                                            "fear".to_string(),
                                            "joy".to_string(),
                                            "sad".to_string(),
                                        ],
                                        supported_styles: vec![
                                            "neutral".to_string(),
                                            "dry".to_string(),
                                            "warm".to_string(),
                                        ],
                                    };
                                    profiles.insert(character_id, new_profile);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to read XTTS embeddings directory: {}", e);
            }
        }

        Ok(())
    }

    pub async fn get_xtts_embedding_path(&self, character_id: &str) -> Option<PathBuf> {
        let profiles = self.profiles.read().await;
        profiles.get(character_id)?.xtts_embedding_path.clone()
    }
}

pub type SharedVoiceProfileManager = Arc<VoiceProfileManager>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_voice_profile_manager() {
        let manager = VoiceProfileManager::new(Path::new("test_models"));

        let profile = VoiceProfile {
            character_id: "test_character".to_string(),
            name: "Test Character".to_string(),
            character_type: CharacterType::NPC,
            xtts_embedding_path: None,
            default_emotion: "neutral".to_string(),
            default_style: "neutral".to_string(),
            supported_emotions: vec!["neutral".to_string()],
            supported_styles: vec!["neutral".to_string()],
        };

        manager.load_profile(profile).await.unwrap();
        let loaded = manager.get_profile("test_character").await;
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "Test Character");
    }
}
