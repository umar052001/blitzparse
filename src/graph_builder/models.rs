use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single entity or concept in the knowledge graph.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: String,
    pub label: String,                       // e.g., "Person", "Paper", "Concept"
    pub properties: HashMap<String, String>, // Flexible key-value metadata
}

/// Represents a directed relationship between two nodes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    pub source: String, // ID of the source node
    pub target: String, // ID of the target node
    pub label: String,  // e.g., "CITES", "AUTHORED_BY"
    pub properties: HashMap<String, String>,
}

/// The main container for the knowledge graph.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl KnowledgeGraph {
    /// Creates a new, empty knowledge graph.
    pub fn new() -> Self {
        KnowledgeGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}
