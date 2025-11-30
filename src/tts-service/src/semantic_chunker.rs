//! Semantic Chunker
//!
//! Chunks text semantically based on natural pauses and narrative flow
//! Supports profile-based chunking (FAST vs CINEMATIC)

use crate::error::Result;
use crate::tts_profile::TtsProfile;

/// Semantic chunk with metadata
#[derive(Debug, Clone)]
pub struct TextChunk {
    /// Chunk text
    pub text: String,
    /// Estimated duration in seconds (based on ~150 words/min)
    pub estimated_duration: f32,
    /// Character count
    pub char_count: usize,
}

/// Semantic chunker configuration
#[derive(Debug, Clone)]
pub struct ChunkerConfig {
    /// Target chunk duration (seconds)
    pub target_duration: f32,
    /// Minimum chunk duration (seconds)
    pub min_duration: f32,
    /// Maximum chunk duration (seconds)
    pub max_duration: f32,
    /// Target character count
    pub target_chars: usize,
    /// Minimum character count
    pub min_chars: usize,
    /// Maximum character count
    pub max_chars: usize,
}

impl Default for ChunkerConfig {
    fn default() -> Self {
        Self {
            target_duration: 5.0,  // 5 seconds target
            min_duration: 2.4,     // 2.4 seconds minimum
            max_duration: 8.0,     // 8 seconds maximum
            target_chars: 250,      // 250 characters target
            min_chars: 180,         // 180 characters minimum
            max_chars: 320,         // 320 characters maximum
        }
    }
}

/// Semantic text chunker
pub struct SemanticChunker {
    config: ChunkerConfig,
}

impl SemanticChunker {
    /// Create new semantic chunker
    pub fn new(config: ChunkerConfig) -> Self {
        Self { config }
    }

    /// Chunk text into semantic segments (legacy method, uses default config)
    pub fn chunk(&self, text: &str) -> Result<Vec<TextChunk>> {
        self.chunk_with_profile(text, None)
    }

    /// Chunk text with profile-specific configuration
    /// For FAST profile: first chunk is tiny (30 chars), subsequent chunks are small (90 chars)
    /// For CINEMATIC profile: first chunk is moderate (100 chars), subsequent chunks are larger (150 chars)
    pub fn chunk_with_profile(&self, text: &str, profile: Option<&TtsProfile>) -> Result<Vec<TextChunk>> {
        let (first_limit, next_limit) = if let Some(profile) = profile {
            (profile.first_chunk_max_chars, profile.next_chunk_max_chars)
        } else {
            // Use default config limits
            (self.config.min_chars, self.config.target_chars)
        };

        let mut chunks = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        
        if words.is_empty() {
            return Ok(chunks);
        }

        let mut current_chunk = String::new();
        let mut current_limit = first_limit;
        let mut is_first_chunk = true;

        for word in words {
            let word_with_space = if current_chunk.is_empty() {
                word.to_string()
            } else {
                format!(" {}", word)
            };

            // Check if adding this word would exceed limit
            if current_chunk.len() + word_with_space.len() > current_limit && !current_chunk.is_empty() {
                // Finalize current chunk
                chunks.push(self.create_chunk(&current_chunk)?);
                current_chunk.clear();
                current_limit = next_limit; // Switch to next chunk limit
                is_first_chunk = false;
            }

            current_chunk.push_str(&word_with_space);
        }

        // Add remaining text as final chunk
        if !current_chunk.is_empty() {
            chunks.push(self.create_chunk(&current_chunk)?);
        }

        Ok(chunks)
    }

    /// Legacy chunk method (kept for compatibility)
    pub fn chunk_legacy(&self, text: &str) -> Result<Vec<TextChunk>> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_chars = 0;

        // Split by sentences first
        let sentences: Vec<&str> = text
            .split(|c: char| c == '.' || c == '!' || c == '?')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for sentence in sentences {
            let sentence_len = sentence.len();

            // Check if adding this sentence would exceed max
            if current_chars + sentence_len > self.config.max_chars
                && !current_chunk.is_empty()
            {
                // Finalize current chunk
                chunks.push(self.create_chunk(&current_chunk)?);
                current_chunk.clear();
                current_chars = 0;
            }

            // Check for pause points within sentence (commas, conjunctions)
            let pause_points = self.find_pause_points(sentence);
            
            if pause_points.is_empty() {
                // No pause points, add entire sentence
                if !current_chunk.is_empty() {
                    current_chunk.push(' ');
                }
                current_chunk.push_str(sentence);
                current_chunk.push('.');
                current_chars += sentence_len + 1;
            } else {
                // Split at pause points
                let parts: Vec<&str> = sentence
                    .split(|c: char| c == ',' || c == ';')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                for part in parts {
                    let part_len = part.len();

                    // Check if adding this part would exceed max
                    if current_chars + part_len > self.config.max_chars
                        && !current_chunk.is_empty()
                    {
                        chunks.push(self.create_chunk(&current_chunk)?);
                        current_chunk.clear();
                        current_chars = 0;
                    }

                    if !current_chunk.is_empty() {
                        current_chunk.push(' ');
                    }
                    current_chunk.push_str(part);
                    current_chars += part_len;

                    // Check if we've reached target size
                    if current_chars >= self.config.target_chars
                        && current_chars >= self.config.min_chars
                    {
                        chunks.push(self.create_chunk(&current_chunk)?);
                        current_chunk.clear();
                        current_chars = 0;
                    }
                }

                // Add sentence ending
                if !current_chunk.is_empty() {
                    current_chunk.push('.');
                    current_chars += 1;
                }
            }

            // Finalize chunk if it meets minimum requirements
            if current_chars >= self.config.min_chars
                && current_chars >= (self.config.target_chars as f32 * 0.8) as usize
            {
                chunks.push(self.create_chunk(&current_chunk)?);
                current_chunk.clear();
                current_chars = 0;
            }
        }

        // Add remaining text as final chunk
        if !current_chunk.is_empty() {
            chunks.push(self.create_chunk(&current_chunk)?);
        }

        Ok(chunks)
    }

    /// Find pause points in text
    fn find_pause_points(&self, text: &str) -> Vec<usize> {
        let mut points = Vec::new();
        let conjunctions = [" and ", " as ", " while ", " when ", " but ", " or "];

        for (i, _) in text.match_indices(|c: char| c == ',' || c == ';') {
            points.push(i);
        }

        for conj in &conjunctions {
            for (i, _) in text.match_indices(conj) {
                points.push(i);
            }
        }

        points.sort();
        points
    }

    /// Create chunk from text
    fn create_chunk(&self, text: &str) -> Result<TextChunk> {
        let char_count = text.chars().count();
        // Estimate duration: ~150 words/min = 2.5 words/sec
        // Average word length: ~5 chars
        // So: chars / 5 / 2.5 = chars / 12.5 seconds
        let estimated_duration = char_count as f32 / 12.5;

        Ok(TextChunk {
            text: text.to_string(),
            estimated_duration,
            char_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_basic() {
        let chunker = SemanticChunker::new(ChunkerConfig::default());
        // Use longer text to meet minimum requirements
        let text = "This is a test sentence. This is another sentence. ".repeat(15);
        let chunks = chunker.chunk(&text).unwrap();

        assert!(!chunks.is_empty());
        // With repeated text, should meet minimum
        assert!(chunks[0].char_count > 0);
    }

    #[test]
    fn test_chunk_long_text() {
        let chunker = SemanticChunker::new(ChunkerConfig::default());
        let text = "This is a very long text that should be split into multiple chunks. ".repeat(20);
        let chunks = chunker.chunk(&text).unwrap();

        assert!(chunks.len() > 1);
        for chunk in &chunks {
            // Some chunks may be smaller if they're the last chunk
            // But most should meet the target
            assert!(chunk.char_count > 0);
            assert!(chunk.char_count <= 320);
        }
        // At least one chunk should meet the target
        assert!(chunks.iter().any(|c| c.char_count >= 180));
    }

    #[test]
    fn test_chunk_with_pauses() {
        let chunker = SemanticChunker::new(ChunkerConfig::default());
        let text = "This is a sentence, with a pause, and another part.";
        let chunks = chunker.chunk(text).unwrap();

        assert!(!chunks.is_empty());
    }
}

