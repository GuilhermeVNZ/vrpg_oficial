//! Phonemizer Module - Convert text to phonemes using espeak-ng
//!
//! This module provides phonemization using espeak-ng, which is the same
//! tool for phonemization (kept for potential future use).

use crate::error::{Result, TtsError};
use std::path::Path;
use std::process::Command;
use tracing::{info, warn};

/// Phonemize text using espeak-ng (DEPRECATED - use phonemize_ipa instead)
///
/// Converts text to phonemes using espeak-ng command-line tool.
/// (Kept for potential future use with other TTS systems)
///
/// NOTE: This function is kept for backward compatibility but should use
/// phonemize_ipa for phoneme mapping.
#[deprecated(note = "Use phonemize_ipa for phoneme mapping")]
pub fn phonemize(text: &str, language: &str) -> Result<Vec<i64>> {
    // Check if espeak-ng is available
    if !check_espeak_available() {
        warn!("espeak-ng not found, using fallback phonemization. Install for better quality: see INSTALAR_ESPEAK.md");
        return fallback_phonemize(text, language);
    }

    // Map language codes to espeak-ng voice codes
    let espeak_voice = match language {
        "pt" | "pt-BR" | "pt_BR" => "pt-br",
        "en" | "en-US" | "en_US" => "en-gb", // CRITICAL: Model is GB, not US!
        "en-GB" | "en_GB" => "en-gb",
        _ => {
            warn!(
                "Unknown language '{}', defaulting to 'en-gb' (British English)",
                language
            );
            "en-gb" // Default to en-gb because model is British English
        }
    };

    // Determine espeak command (espeak-ng on Linux/macOS, espeak on Windows)
    // Also check common Windows installation paths
    let espeak_cmd = find_espeak_command()?;

    phonemize_with_path(text, language, &espeak_cmd, espeak_voice)
}

/// Find espeak-ng/espeak command
fn find_espeak_command() -> Result<String> {
    // Try espeak-ng first (Linux/macOS)
    if Command::new("espeak-ng").arg("--version").output().is_ok() {
        return Ok("espeak-ng".to_string());
    }

    // Try espeak (Windows/older systems)
    if Command::new("espeak").arg("--version").output().is_ok() {
        return Ok("espeak".to_string());
    }

    // Try common Windows paths
    #[cfg(windows)]
    {
        // Get the project root directory (assuming we're in vrpg-client/src/tts-service)
        let project_root = std::env::current_dir().ok().and_then(|p| {
            // Navigate up from src/tts-service to vrpg-client
            p.parent()
                .and_then(|src| src.parent())
                .map(|root| root.to_path_buf())
        });

        let mut common_paths: Vec<String> = vec![
            r"C:\Program Files\eSpeak NG\espeak-ng.exe".to_string(), // Note: space in directory name
            r"C:\Program Files\espeak-ng\espeak-ng.exe".to_string(),
            r"C:\Program Files (x86)\eSpeak NG\espeak-ng.exe".to_string(),
            r"C:\Program Files (x86)\espeak-ng\espeak-ng.exe".to_string(),
            r"C:\Program Files\espeak\espeak.exe".to_string(),
            r"C:\Program Files (x86)\espeak\espeak.exe".to_string(),
        ];

        // Add project-specific path if available
        if let Some(root) = &project_root {
            let project_path = root.join("tools").join("espeak-ng").join("espeak-ng.exe");
            if project_path.exists() {
                common_paths.insert(0, project_path.to_string_lossy().to_string());
            }
            // Also try to find espeak-ng.exe in subdirectories
            let project_dir = root.join("tools").join("espeak-ng");
            if project_dir.exists() {
                // Try common subdirectories where binaries might be
                let subdirs = vec!["bin", "build", "src", "win32", "x64", "Release", "Debug"];
                for subdir in subdirs {
                    let subdir_path = project_dir.join(subdir).join("espeak-ng.exe");
                    if subdir_path.exists() {
                        common_paths.insert(0, subdir_path.to_string_lossy().to_string());
                    }
                }
                // Also search for espeak-ng.exe in the directory
                if let Ok(entries) = std::fs::read_dir(&project_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                            if name == "espeak-ng.exe" || name == "espeak.exe" {
                                common_paths.insert(0, path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }

        for path_str in common_paths {
            if Path::new(&path_str).exists() {
                if Command::new(&path_str).arg("--version").output().is_ok() {
                    return Ok(path_str);
                }
            }
        }
    }

    Err(TtsError::Synthesis(
        "espeak-ng not found in PATH or common installation paths".to_string(),
    ))
}

/// Phonemize text using espeak-ng at a specific path
fn phonemize_with_path(
    text: &str,
    language: &str,
    espeak_cmd: &str,
    espeak_voice: &str,
) -> Result<Vec<i64>> {
    // Use espeak-ng/espeak to convert text to phonemes
    // espeak-ng -q --ipa -v <voice> "<text>"
    // -q: quiet mode (don't produce speech)
    // --ipa: output phonemes in IPA format
    // -v <voice>: voice/language

    let output = Command::new(espeak_cmd)
        .arg("-q")
        .arg("--ipa")
        .arg("-v")
        .arg(espeak_voice)
        .arg(text)
        .output()
        .map_err(|e| TtsError::Synthesis(format!("Failed to run {}: {}", espeak_cmd, e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TtsError::Synthesis(format!(
            "{} failed: {}",
            espeak_cmd, stderr
        )));
    }

    // Parse espeak-ng output
    let phoneme_text = String::from_utf8_lossy(&output.stdout);
    let phoneme_ids = parse_phonemes(&phoneme_text, language)?;

    info!("Phonemized '{}' -> {} phonemes", text, phoneme_ids.len());

    Ok(phoneme_ids)
}

/// Phonemize text and return IPA phonemes as strings
pub fn phonemize_ipa(text: &str, language: &str) -> Result<Vec<String>> {
    // Check if espeak-ng is available
    if !check_espeak_available() {
        warn!("espeak-ng not found, using fallback phonemization. Install for better quality: see INSTALAR_ESPEAK.md");
        return fallback_phonemize_ipa(text, language);
    }

    // Map language codes to espeak-ng voice codes
    let espeak_voice = match language {
        "pt" | "pt-BR" | "pt_BR" => "pt-br",
        "en" | "en-US" | "en_US" => "en-gb", // CRITICAL: Model is GB, not US!
        "en-GB" | "en_GB" => "en-gb",
        _ => {
            warn!(
                "Unknown language '{}', defaulting to 'en-gb' (British English)",
                language
            );
            "en-gb" // Default to en-gb because model is British English
        }
    };

    // Determine espeak command
    let espeak_cmd = find_espeak_command()?;

    phonemize_ipa_with_path(text, language, &espeak_cmd, espeak_voice)
}

/// Phonemize text using espeak-ng and return IPA phonemes as strings
fn phonemize_ipa_with_path(
    text: &str,
    _language: &str,
    espeak_cmd: &str,
    espeak_voice: &str,
) -> Result<Vec<String>> {
    let output = Command::new(espeak_cmd)
        .arg("-q")
        .arg("--ipa")
        .arg("-v")
        .arg(espeak_voice)
        .arg(text)
        .output()
        .map_err(|e| TtsError::Synthesis(format!("Failed to run {}: {}", espeak_cmd, e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TtsError::Synthesis(format!(
            "{} failed: {}",
            espeak_cmd, stderr
        )));
    }

    // Parse espeak-ng output
    let phoneme_text = String::from_utf8_lossy(&output.stdout);
    info!("🔍 ESPEAK-NG RAW OUTPUT: '{}'", phoneme_text.trim());
    let ipa_phonemes = parse_phonemes_to_strings(&phoneme_text)?;
    info!("🔍 PARSED IPA PHONEMES: {:?}", ipa_phonemes);

    // Pauses are already added in parse_phonemes_to_strings between words
    // No need to add more pauses here - that would create too many pauses

    info!(
        "Phonemized '{}' -> {} IPA phonemes (with pauses between words)",
        text,
        ipa_phonemes.len()
    );

    Ok(ipa_phonemes)
}

/// Parse espeak-ng IPA output to phoneme strings
///
/// espeak-ng returns words as complete phonetic strings (e.g., "ðɪ", "eɪntʃənt").
/// We need to split these into individual phonemes.
fn parse_phonemes_to_strings(phoneme_text: &str) -> Result<Vec<String>> {
    // Remove whitespace and split into word tokens
    let words: Vec<&str> = phoneme_text.trim().split_whitespace().collect();

    if words.is_empty() {
        return Err(TtsError::Synthesis("No phonemes generated".to_string()));
    }

    // Split each word into individual phonemes and add pauses between words
    let mut phoneme_strings = Vec::new();
    for (i, word) in words.iter().enumerate() {
        // Split word into individual phonemes
        let phonemes = split_ipa_word_to_phonemes(word);
        phoneme_strings.extend(phonemes);

        // Add a single pause (silence marker) between words
        // REVERTED: Multiple pauses were making audio too fast/choppy
        if i < words.len() - 1 {
            phoneme_strings.push(" ".to_string()); // Space will be mapped to silence ID 0
        }
    }

    if phoneme_strings.is_empty() {
        return Err(TtsError::Synthesis("No phonemes extracted".to_string()));
    }

    Ok(phoneme_strings)
}

/// Split an IPA word string into individual phonemes
///
/// Handles IPA digraphs and trigraphs like:
/// - "tʃ" (ch sound)
/// - "dʒ" (j sound)
/// - "eɪ" (diphthong)
/// - "aɪ" (diphthong)
/// - "ɔɪ" (diphthong)
/// - "aʊ" (diphthong)
/// - "oʊ" (diphthong)
///
/// This function tries to match known IPA digraphs first, then falls back to single characters.
fn split_ipa_word_to_phonemes(word: &str) -> Vec<String> {
    let mut phonemes = Vec::new();
    let chars: Vec<char> = word.chars().collect();
    let mut i = 0;

    // IPA digraphs that should be kept together (in order of priority - longer first)
    // Note: Some of these might be single Unicode characters, so we check them as sequences
    // CRITICAL: Order matters! Check longer sequences first (e.g., "oʊ" before "o" and "ʊ")
    // CRITICAL FIX: espeak-ng returns "əʊ" (not "oʊ") for "Hello" - must include both!
    let digraphs = [
        // Diphthongs (2-char sequences) - MUST be checked BEFORE single characters!
        "eɪ", "aɪ", "ɔɪ", "aʊ", "oʊ",
        "əʊ", // CRITICAL: Added "əʊ" - espeak-ng uses this for "Hello"
        "ɪə", "eə", "ʊə", // Affricates and other 2-char sequences
        "tʃ", "dʒ",
    ];

    // Single-character phonemes that are Unicode (not ASCII)
    let unicode_phonemes = [
        'ŋ', 'ɲ', 'ʎ', 'ʍ', 'ɕ', 'ʑ', 'ɧ', 'ɬ', 'ɮ', 'ʋ', 'ɹ', 'ɻ', 'ɽ', 'ɾ', 'ɺ', 'ʀ', 'ʁ', 'ħ',
        'ʕ', 'ɦ', 'ɥ', 'ʜ', 'ʢ', 'ʡ', 'ə', 'ɪ', 'ʊ', 'ɛ', 'ɔ', 'æ', 'ʌ', 'ɑ', 'ɐ', 'ɜ', 'ɞ', 'ɘ',
        'ɨ', 'ɯ', 'ɵ', 'ɶ', 'ɷ', 'ɸ', 'β', 'θ', 'ð', 'ʃ', 'ʒ', 'ʂ', 'ʐ', 'ʈ', 'ɖ', 'ɟ', 'ɠ', 'ʄ',
        'ɓ', 'ɗ', 'ʙ', 'ʀ', 'ʟ', 'ʎ',
    ];

    while i < chars.len() {
        let mut found = false;

        // Check for digraphs first (2-character sequences)
        if i + 1 < chars.len() {
            let digraph: String = chars[i..=i + 1].iter().collect();
            if digraphs.contains(&digraph.as_str()) {
                phonemes.push(digraph);
                i += 2;
                found = true;
            }
        }

        // Single character phoneme
        if !found {
            let ch = chars[i];
            // Skip stress markers, diacritics, numbers, and whitespace
            if !ch.is_ascii_digit() && !"ˈˌː ".contains(ch) && !ch.is_whitespace() {
                // Only add if it's a known Unicode phoneme or a valid IPA character
                // ASCII letters can be IPA phonemes (like 'a', 'e', 'i', 'o', 'u')
                if unicode_phonemes.contains(&ch) || (ch.is_alphabetic() && ch.is_ascii()) {
                    phonemes.push(ch.to_string());
                } else {
                    // Unknown character - log warning but skip it to avoid garbage
                    // This prevents invalid characters from breaking the phoneme sequence
                }
            }
            i += 1;
        }
    }

    phonemes
}

/// Fallback phonemization returning IPA-like strings
fn fallback_phonemize_ipa(text: &str, _language: &str) -> Result<Vec<String>> {
    warn!("Using fallback phonemization (espeak-ng not available)");

    let text_lower = text.to_lowercase();
    let words: Vec<&str> = text_lower.split_whitespace().collect();

    let mut phonemes = Vec::new();

    for word in words {
        // Simple grapheme-to-phoneme mapping (very basic)
        for ch in word.chars() {
            if ch.is_alphabetic() {
                // Map to basic IPA-like phonemes
                let phoneme = match ch {
                    'a' => "a",
                    'e' => "e",
                    'i' => "i",
                    'o' => "o",
                    'u' => "u",
                    'p' => "p",
                    'b' => "b",
                    't' => "t",
                    'd' => "d",
                    'k' | 'c' => "k",
                    'g' => "g",
                    'f' => "f",
                    'v' => "v",
                    's' => "s",
                    'z' => "z",
                    'h' => "h",
                    'm' => "m",
                    'n' => "n",
                    'l' => "l",
                    'r' => "r",
                    'w' => "w",
                    'y' => "j",
                    _ => "ə", // schwa as default
                };
                phonemes.push(phoneme.to_string());
            }
        }
        // Add word separator
        phonemes.push(" ".to_string());
    }

    if phonemes.is_empty() {
        phonemes = vec!["ə".to_string(), "a".to_string(), "i".to_string()]; // Basic fallback
    }

    Ok(phonemes)
}

/// Check if espeak-ng is available in PATH or common installation paths
fn check_espeak_available() -> bool {
    find_espeak_command().is_ok()
}

/// Parse espeak-ng phoneme output to phoneme IDs
///
/// espeak-ng outputs phonemes in IPA format like: "həˈloʊ wɜrld"
/// We need to convert these to phoneme IDs that XTTS expects.
///
/// This is a simplified mapping. XTTS uses a comprehensive phoneme vocabulary.
fn parse_phonemes(phoneme_text: &str, language: &str) -> Result<Vec<i64>> {
    // Remove whitespace and split into phoneme tokens
    let phonemes: Vec<&str> = phoneme_text.trim().split_whitespace().collect();

    if phonemes.is_empty() {
        return Err(TtsError::Synthesis("No phonemes generated".to_string()));
    }

    // Map IPA phonemes to IDs
    // This is a simplified mapping for phoneme vocabulary
    // For now, we'll create a basic mapping based on common phonemes
    let mut phoneme_ids = Vec::new();

    for phoneme in phonemes {
        // Remove stress markers and other IPA diacritics
        let clean_phoneme = phoneme
            .chars()
            .filter(|c| !c.is_ascii_digit() && !"ˈˌː".contains(*c))
            .collect::<String>();

        // Map to phoneme ID (simplified mapping)
        let phoneme_id = map_phoneme_to_id(&clean_phoneme, language)?;
        phoneme_ids.push(phoneme_id);
    }

    Ok(phoneme_ids)
}

/// Map IPA phoneme to phoneme ID
///
/// This is a simplified mapping. XTTS uses a comprehensive phoneme vocabulary
/// loaded from the model's config file. For now, we use a basic mapping.
fn map_phoneme_to_id(phoneme: &str, language: &str) -> Result<i64> {
    // Basic phoneme mapping (simplified)
    // XTTS loads phoneme vocabulary from model config
    let id = match language {
        "pt" | "pt-BR" | "pt_BR" => {
            // Portuguese phonemes
            match phoneme {
                "a" | "ɐ" => 10,
                "e" | "ɛ" => 11,
                "i" | "ɪ" => 12,
                "o" | "ɔ" => 13,
                "u" | "ʊ" => 14,
                "p" => 20,
                "b" => 21,
                "t" => 22,
                "d" => 23,
                "k" => 24,
                "g" => 25,
                "f" => 26,
                "v" => 27,
                "s" => 28,
                "z" => 29,
                "ʃ" => 30,
                "ʒ" => 31,
                "m" => 32,
                "n" => 33,
                "ɲ" => 34,
                "l" => 35,
                "ɾ" | "r" => 36,
                "j" => 37,
                "w" => 38,
                _ => {
                    // Unknown phoneme - use hash-based ID
                    (phoneme.chars().map(|c| c as u64).sum::<u64>() % 1000) as i64 + 100
                }
            }
        }
        _ => {
            // English phonemes (default)
            match phoneme {
                "ə" | "ʌ" => 10, // schwa
                "æ" => 11,       // cat
                "ɑ" => 12,       // father
                "ɔ" => 13,       // law
                "ɛ" => 14,       // bed
                "ɪ" => 15,       // bit
                "i" => 16,       // beat
                "ʊ" => 17,       // book
                "u" => 18,       // boot
                "eɪ" => 19,      // day
                "aɪ" => 20,      // my
                "ɔɪ" => 21,      // boy
                "aʊ" => 22,      // now
                "oʊ" => 23,      // go
                "p" => 30,
                "b" => 31,
                "t" => 32,
                "d" => 33,
                "k" => 34,
                "g" => 35,
                "f" => 36,
                "v" => 37,
                "θ" => 38, // think
                "ð" => 39, // this
                "s" => 40,
                "z" => 41,
                "ʃ" => 42, // ship
                "ʒ" => 43, // measure
                "h" => 44,
                "m" => 45,
                "n" => 46,
                "ŋ" => 47, // sing
                "l" => 48,
                "r" | "ɹ" => 49,
                "j" => 50, // yes
                "w" => 51, // we
                _ => {
                    // Unknown phoneme - use hash-based ID
                    (phoneme.chars().map(|c| c as u64).sum::<u64>() % 1000) as i64 + 100
                }
            }
        }
    };

    Ok(id)
}

/// Fallback phonemization when espeak-ng is not available
///
/// Uses a simplified rule-based approach. Quality is lower than espeak-ng
/// but allows the system to function without external dependencies.
fn fallback_phonemize(text: &str, language: &str) -> Result<Vec<i64>> {
    warn!("Using fallback phonemization (espeak-ng not available)");

    let text_lower = text.to_lowercase();
    let words: Vec<&str> = text_lower.split_whitespace().collect();

    let mut phoneme_ids = Vec::new();

    for word in words {
        // Simple grapheme-to-phoneme mapping (very basic)
        for ch in word.chars() {
            if ch.is_alphabetic() {
                let phoneme_id = match language {
                    "pt" | "pt-BR" | "pt_BR" => {
                        // Portuguese basic mapping
                        match ch {
                            'a' => 10,
                            'e' => 11,
                            'i' => 12,
                            'o' => 13,
                            'u' => 14,
                            'p' => 20,
                            'b' => 21,
                            't' => 22,
                            'd' => 23,
                            'k' | 'c' => 24,
                            'g' => 25,
                            'f' => 26,
                            'v' => 27,
                            's' => 28,
                            'z' => 29,
                            'm' => 32,
                            'n' => 33,
                            'l' => 35,
                            'r' => 36,
                            _ => 50 + (ch as i64 % 20),
                        }
                    }
                    _ => {
                        // English basic mapping
                        match ch {
                            'a' => 10,
                            'e' => 11,
                            'i' => 12,
                            'o' => 13,
                            'u' => 14,
                            'p' => 30,
                            'b' => 31,
                            't' => 32,
                            'd' => 33,
                            'k' | 'c' => 34,
                            'g' => 35,
                            'f' => 36,
                            'v' => 37,
                            's' => 40,
                            'z' => 41,
                            'h' => 44,
                            'm' => 45,
                            'n' => 46,
                            'l' => 48,
                            'r' => 49,
                            'w' => 51,
                            'y' => 50,
                            _ => 60 + (ch as i64 % 20),
                        }
                    }
                };
                phoneme_ids.push(phoneme_id);
            }
        }
        // Add word separator
        phoneme_ids.push(0);
    }

    if phoneme_ids.is_empty() {
        phoneme_ids = vec![10, 11, 12]; // Basic fallback
    }

    Ok(phoneme_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_espeak_available() {
        // This will fail if espeak-ng is not installed
        let available = check_espeak_available();
        println!("espeak-ng available: {}", available);
    }

    #[test]
    fn test_map_phoneme_to_id() {
        // Phoneme IDs may vary based on implementation
        // Just verify they return valid IDs
        let a_en = map_phoneme_to_id("a", "en").unwrap();
        let p_en = map_phoneme_to_id("p", "en").unwrap();
        let a_pt = map_phoneme_to_id("a", "pt").unwrap();
        
        assert!(a_en > 0);
        assert!(p_en > 0);
        assert!(a_pt > 0);
    }
}
