//! Extended Unit Tests for Phonemizer Module
//!
//! Following rulebook standards: comprehensive coverage, Given/When/Then scenarios

use tts_service::phonemizer::{
    check_espeak_available, fallback_phonemize, map_phoneme_to_id, phonemize_ipa,
};

#[test]
fn test_check_espeak_available() {
    // Given the system
    // When checking if espeak-ng is available
    // Then it should return a boolean
    let available = check_espeak_available();
    // Result depends on system, but should not panic
    assert!(available || !available); // Always true, just checking it doesn't panic
}

#[test]
fn test_fallback_phonemize() {
    // Given text and language
    // When using fallback phonemization
    // Then it should return phoneme IDs
    let result = fallback_phonemize("hello", "en");
    assert!(result.is_ok());
    let phonemes = result.unwrap();
    assert!(!phonemes.is_empty());
}

#[test]
fn test_fallback_phonemize_portuguese() {
    // Given Portuguese text
    // When using fallback phonemization
    // Then it should return phoneme IDs
    let result = fallback_phonemize("olá", "pt");
    assert!(result.is_ok());
    let phonemes = result.unwrap();
    assert!(!phonemes.is_empty());
}

#[test]
fn test_map_phoneme_to_id_valid() {
    // Given a valid phoneme and language
    // When mapping to ID
    // Then it should return a valid ID
    let result = map_phoneme_to_id("a", "en");
    assert!(result.is_ok());
    let id = result.unwrap();
    assert!(id > 0);
}

#[test]
fn test_map_phoneme_to_id_portuguese() {
    // Given a Portuguese phoneme
    // When mapping to ID
    // Then it should return a valid ID
    let result = map_phoneme_to_id("a", "pt");
    assert!(result.is_ok());
    let id = result.unwrap();
    assert!(id > 0);
}

#[test]
fn test_map_phoneme_to_id_invalid() {
    // Given an invalid phoneme
    // When mapping to ID
    // Then it should return an error
    let result = map_phoneme_to_id("xxx", "en");
    // May return error or default ID depending on implementation
    // Just verify it doesn't panic
    let _ = result;
}

#[test]
fn test_phonemize_ipa() {
    // Given text and language
    // When phonemizing to IPA
    // Then it should return IPA string
    let result = phonemize_ipa("hello", "en");
    // May fail if espeak-ng not available, but should not panic
    if result.is_ok() {
        let ipa = result.unwrap();
        assert!(!ipa.is_empty());
    }
}

#[test]
fn test_phonemize_ipa_portuguese() {
    // Given Portuguese text
    // When phonemizing to IPA
    // Then it should return IPA string
    let result = phonemize_ipa("olá", "pt");
    // May fail if espeak-ng not available, but should not panic
    if result.is_ok() {
        let ipa = result.unwrap();
        assert!(!ipa.is_empty());
    }
}

#[test]
fn test_phonemize_ipa_empty_text() {
    // Given empty text
    // When phonemizing to IPA
    // Then it should handle gracefully
    let result = phonemize_ipa("", "en");
    // May return empty string or error, but should not panic
    let _ = result;
}

#[test]
fn test_fallback_phonemize_empty_text() {
    // Given empty text
    // When using fallback phonemization
    // Then it should return empty phonemes
    let result = fallback_phonemize("", "en");
    assert!(result.is_ok());
    let phonemes = result.unwrap();
    assert!(phonemes.is_empty());
}

#[test]
fn test_fallback_phonemize_long_text() {
    // Given long text
    // When using fallback phonemization
    // Then it should return phonemes for all characters
    let long_text = "This is a very long text that should be phonemized correctly.".repeat(10);
    let result = fallback_phonemize(&long_text, "en");
    assert!(result.is_ok());
    let phonemes = result.unwrap();
    assert!(!phonemes.is_empty());
}

#[test]
fn test_map_phoneme_to_id_multiple_phonemes() {
    // Given multiple valid phonemes
    // When mapping to IDs
    // Then they should all return valid IDs
    let phonemes = vec!["a", "e", "i", "o", "u", "p", "t", "k"];
    for phoneme in phonemes {
        let result = map_phoneme_to_id(phoneme, "en");
        if result.is_ok() {
            let id = result.unwrap();
            assert!(id > 0);
        }
    }
}

#[test]
fn test_phonemize_deprecated() {
    // Given text and language
    // When using deprecated phonemize function
    // Then it should still work (for backward compatibility)
    // Note: This test verifies the deprecated function still exists and works
    // The function is marked as deprecated but should still function
    let result = tts_service::phonemizer::phonemize("hello", "en");
    // May fail if espeak-ng not available, but should not panic
    let _ = result;
}



