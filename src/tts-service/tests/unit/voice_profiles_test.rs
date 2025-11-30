//! Unit Tests for Voice Profiles Module
//!
//! Following rulebook standards: comprehensive coverage, Given/When/Then scenarios

use tts_service::voice_profiles::{CharacterType, VoiceProfile, VoiceProfileManager};
use std::path::PathBuf;

#[tokio::test]
async fn test_voice_profile_manager_new() {
    // Given a base path
    // When creating a VoiceProfileManager
    // Then it should be created successfully
    let base_path = PathBuf::from("test_models");
    let manager = VoiceProfileManager::new(base_path.as_path());
    let profiles = manager.list_profiles().await;
    assert!(profiles.is_empty());
}

#[tokio::test]
async fn test_voice_profile_manager_load_profile() {
    // Given a VoiceProfileManager
    // When loading a profile
    // Then it should be stored and retrievable
    let base_path = PathBuf::from("test_models");
    let manager = VoiceProfileManager::new(base_path.as_path());
    
    let profile = VoiceProfile {
        character_id: "test_character".to_string(),
        name: "Test Character".to_string(),
        character_type: CharacterType::NPC,
        xtts_embedding_path: None,
        default_emotion: "neutral".to_string(),
        default_style: "neutral".to_string(),
        supported_emotions: vec!["neutral".to_string(), "happy".to_string()],
        supported_styles: vec!["neutral".to_string()],
    };
    
    manager.load_profile(profile.clone()).await.unwrap();
    
    let retrieved = manager.get_profile("test_character").await;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "Test Character");
}

#[tokio::test]
async fn test_voice_profile_manager_get_profile_not_found() {
    // Given a VoiceProfileManager
    // When getting a non-existent profile
    // Then it should return None
    let base_path = PathBuf::from("test_models");
    let manager = VoiceProfileManager::new(base_path.as_path());
    
    let profile = manager.get_profile("nonexistent").await;
    assert!(profile.is_none());
}

#[tokio::test]
async fn test_voice_profile_manager_list_profiles() {
    // Given a VoiceProfileManager with multiple profiles
    // When listing all profiles
    // Then it should return all profiles
    let base_path = PathBuf::from("test_models");
    let manager = VoiceProfileManager::new(base_path.as_path());
    
    let profile1 = VoiceProfile {
        character_id: "char1".to_string(),
        name: "Character 1".to_string(),
        character_type: CharacterType::NPC,
        xtts_embedding_path: None,
        default_emotion: "neutral".to_string(),
        default_style: "neutral".to_string(),
        supported_emotions: vec!["neutral".to_string()],
        supported_styles: vec!["neutral".to_string()],
    };
    
    let profile2 = VoiceProfile {
        character_id: "char2".to_string(),
        name: "Character 2".to_string(),
        character_type: CharacterType::Player,
        xtts_embedding_path: None,
        default_emotion: "neutral".to_string(),
        default_style: "neutral".to_string(),
        supported_emotions: vec!["neutral".to_string()],
        supported_styles: vec!["neutral".to_string()],
    };
    
    manager.load_profile(profile1).await.unwrap();
    manager.load_profile(profile2).await.unwrap();
    
    let profiles = manager.list_profiles().await;
    assert_eq!(profiles.len(), 2);
}

#[tokio::test]
async fn test_voice_profile_manager_load_default_profiles() {
    // Given a VoiceProfileManager
    // When loading default profiles
    // Then it should load the default DM profile
    let base_path = PathBuf::from("test_models");
    let manager = VoiceProfileManager::new(base_path.as_path());
    
    // This may fail if directory doesn't exist, but should not panic
    let result = manager.load_default_profiles().await;
    // Should succeed even if directory doesn't exist (auto_discover_sovits_models handles it)
    assert!(result.is_ok());
    
    let profiles = manager.list_profiles().await;
    // Should have at least the default DM profile
    assert!(!profiles.is_empty());
    
    let dm_profile = manager.get_profile("dm").await;
    assert!(dm_profile.is_some());
    assert_eq!(dm_profile.unwrap().character_type, CharacterType::DungeonMaster);
    
    // Also check for Lax Barros profile
    let lax_barros_profile = manager.get_profile("lax_barros").await;
    assert!(lax_barros_profile.is_some());
    assert_eq!(lax_barros_profile.unwrap().character_type, CharacterType::DungeonMaster);
}

#[tokio::test]
async fn test_voice_profile_character_types() {
    // Given different character types
    // When creating profiles
    // Then they should be stored correctly
    let base_path = PathBuf::from("test_models");
    let manager = VoiceProfileManager::new(base_path.as_path());
    
    let dm_profile = VoiceProfile {
        character_id: "dm".to_string(),
        name: "Dungeon Master".to_string(),
        character_type: CharacterType::DungeonMaster,
        xtts_embedding_path: None,
        default_emotion: "neutral".to_string(),
        default_style: "neutral".to_string(),
        supported_emotions: vec!["neutral".to_string()],
        supported_styles: vec!["neutral".to_string()],
    };
    
    let npc_profile = VoiceProfile {
        character_id: "npc".to_string(),
        name: "NPC".to_string(),
        character_type: CharacterType::NPC,
        xtts_embedding_path: None,
        default_emotion: "neutral".to_string(),
        default_style: "neutral".to_string(),
        supported_emotions: vec!["neutral".to_string()],
        supported_styles: vec!["neutral".to_string()],
    };
    
    let player_profile = VoiceProfile {
        character_id: "player".to_string(),
        name: "Player".to_string(),
        character_type: CharacterType::Player,
        xtts_embedding_path: None,
        default_emotion: "neutral".to_string(),
        default_style: "neutral".to_string(),
        supported_emotions: vec!["neutral".to_string()],
        supported_styles: vec!["neutral".to_string()],
    };
    
    let monster_profile = VoiceProfile {
        character_id: "monster".to_string(),
        name: "Monster".to_string(),
        character_type: CharacterType::Monster,
        xtts_embedding_path: None,
        default_emotion: "neutral".to_string(),
        default_style: "neutral".to_string(),
        supported_emotions: vec!["neutral".to_string()],
        supported_styles: vec!["neutral".to_string()],
    };
    
    manager.load_profile(dm_profile).await.unwrap();
    manager.load_profile(npc_profile).await.unwrap();
    manager.load_profile(player_profile).await.unwrap();
    manager.load_profile(monster_profile).await.unwrap();
    
    let profiles = manager.list_profiles().await;
    assert_eq!(profiles.len(), 4);
}

#[tokio::test]
async fn test_voice_profile_serialization() {
    // Given a VoiceProfile
    // When serializing to JSON
    // Then it should serialize correctly
    let profile = VoiceProfile {
        character_id: "test".to_string(),
        name: "Test".to_string(),
        character_type: CharacterType::NPC,
        xtts_embedding_path: Some(PathBuf::from("test_path")),
        default_emotion: "neutral".to_string(),
        default_style: "neutral".to_string(),
        supported_emotions: vec!["neutral".to_string(), "happy".to_string()],
        supported_styles: vec!["neutral".to_string(), "dramatic".to_string()],
    };
    
    let json = serde_json::to_string(&profile).unwrap();
    let deserialized: VoiceProfile = serde_json::from_str(&json).unwrap();
    
    assert_eq!(profile.character_id, deserialized.character_id);
    assert_eq!(profile.name, deserialized.name);
    assert_eq!(profile.character_type, deserialized.character_type);
    assert_eq!(profile.default_emotion, deserialized.default_emotion);
    assert_eq!(profile.default_style, deserialized.default_style);
    assert_eq!(profile.supported_emotions, deserialized.supported_emotions);
    assert_eq!(profile.supported_styles, deserialized.supported_styles);
}

#[tokio::test]
async fn test_voice_profile_update_existing() {
    // Given a VoiceProfileManager with an existing profile
    // When loading a profile with the same character_id
    // Then it should update the existing profile
    let base_path = PathBuf::from("test_models");
    let manager = VoiceProfileManager::new(base_path.as_path());
    
    let profile1 = VoiceProfile {
        character_id: "test".to_string(),
        name: "Original Name".to_string(),
        character_type: CharacterType::NPC,
        xtts_embedding_path: None,
        default_emotion: "neutral".to_string(),
        default_style: "neutral".to_string(),
        supported_emotions: vec!["neutral".to_string()],
        supported_styles: vec!["neutral".to_string()],
    };
    
    manager.load_profile(profile1).await.unwrap();
    
    let profile2 = VoiceProfile {
        character_id: "test".to_string(),
        name: "Updated Name".to_string(),
        character_type: CharacterType::Player,
        xtts_embedding_path: None,
        default_emotion: "happy".to_string(),
        default_style: "dramatic".to_string(),
        supported_emotions: vec!["happy".to_string()],
        supported_styles: vec!["dramatic".to_string()],
    };
    
    manager.load_profile(profile2).await.unwrap();
    
    let retrieved = manager.get_profile("test").await.unwrap();
    assert_eq!(retrieved.name, "Updated Name");
    assert_eq!(retrieved.character_type, CharacterType::Player);
    assert_eq!(retrieved.default_emotion, "happy");
}

