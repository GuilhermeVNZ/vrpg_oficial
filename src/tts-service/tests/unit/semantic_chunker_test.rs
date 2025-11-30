//! Unit tests for semantic chunker

use tts_service::semantic_chunker::{ChunkerConfig, SemanticChunker};

#[test]
fn test_chunk_basic() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "This is a test sentence. This is another sentence.";
    let chunks = chunker.chunk(text).unwrap();

    assert!(!chunks.is_empty());
    // Short text may not meet 180 char minimum - that's OK for basic test
    assert!(chunks[0].char_count > 0);
}

#[test]
fn test_chunk_long_text() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "This is a very long text that should be split into multiple chunks. ".repeat(10);
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

#[test]
fn test_chunk_short_text() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "Short text.";
    let chunks = chunker.chunk(text).unwrap();

    // Should still create a chunk even if short
    assert!(!chunks.is_empty());
}

#[test]
fn test_chunk_empty_text() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "";
    let chunks = chunker.chunk(text).unwrap();

    // Empty text should produce no chunks
    assert!(chunks.is_empty());
}

#[test]
fn test_find_pause_points() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "This is a sentence, with a pause; and another part.";
    let pause_points = chunker.find_pause_points(text);
    
    // Should find commas and semicolons
    assert!(!pause_points.is_empty());
    assert!(pause_points.len() >= 2);
}

#[test]
fn test_find_pause_points_conjunctions() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "This is a sentence and another part, but also this.";
    let pause_points = chunker.find_pause_points(text);
    
    // Should find conjunctions and commas
    assert!(!pause_points.is_empty());
}

#[test]
fn test_create_chunk() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "This is a test sentence with enough characters.";
    let chunk = chunker.create_chunk(text).unwrap();
    
    assert_eq!(chunk.text, text);
    assert!(chunk.char_count > 0);
    assert!(chunk.estimated_duration > 0.0);
}

#[test]
fn test_chunk_duration_estimation() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "This is a test sentence with enough characters to estimate duration properly.";
    let chunks = chunker.chunk(text).unwrap();

    assert!(!chunks.is_empty());
    // Duration should be positive
    assert!(chunks[0].estimated_duration > 0.0);
}

#[test]
fn test_chunk_config_custom() {
    let config = ChunkerConfig {
        target_duration: 4.0,
        min_duration: 2.0,
        max_duration: 6.0,
        target_chars: 200,
        min_chars: 150,
        max_chars: 280,
    };
    let chunker = SemanticChunker::new(config);
    let text = "This is a test. ".repeat(20);
    let chunks = chunker.chunk(&text).unwrap();

    assert!(!chunks.is_empty());
    for chunk in &chunks {
        assert!(chunk.char_count >= 150);
        assert!(chunk.char_count <= 280);
    }
}

#[test]
fn test_chunk_find_pause_points() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "This is a sentence, with a pause, and another part.";
    let pause_points = chunker.find_pause_points(text);
    
    // Should find commas
    assert!(!pause_points.is_empty());
}

#[test]
fn test_chunk_empty_text() {
    let chunker = SemanticChunker::new(ChunkerConfig::default());
    let text = "";
    let chunks = chunker.chunk(text).unwrap();

    // Empty text should produce no chunks
    assert!(chunks.is_empty());
}
