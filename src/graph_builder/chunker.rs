use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use pyo3::prelude::*;

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ChunkingStrategy {
    ByLine(),
    RecursiveCharacter { chunk_size: usize, overlap: usize },
}

pub fn chunk_text(text: &str, strategy: &ChunkingStrategy) -> Vec<String> {
    match strategy {
        ChunkingStrategy::ByLine() => text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(String::from)
            .collect(),
        ChunkingStrategy::RecursiveCharacter {
            chunk_size,
            overlap,
        } => recursive_character_split(text, *chunk_size, *overlap),
    }
}

fn recursive_character_split(text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let graphemes: Vec<&str> = text.graphemes(true).collect();
    if graphemes.len() <= chunk_size {
        chunks.push(text.to_string());
        return chunks;
    }

    let mut start = 0;
    while start < graphemes.len() {
        let end = std::cmp::min(start + chunk_size, graphemes.len());
        let chunk_str: String = graphemes[start..end].concat();
        chunks.push(chunk_str);
        if end == graphemes.len() {
            break;
        }
        start += chunk_size - overlap;
    }
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_split() {
        let text = "This is a sentence. This is another sentence.";
        let chunks = recursive_character_split(text, 20, 5);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], "This is a sentence. ");
        assert_eq!(chunks[1], "nce. This is another");
        assert_eq!(chunks[2], "other sentence.");
    }
}
