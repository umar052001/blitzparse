use super::chunker::{chunk_text, ChunkingStrategy};
use super::models::{Edge, KnowledgeGraph, Node};
use std::collections::HashMap;

pub struct GraphBuilder {
    graph: KnowledgeGraph,
    next_id: u64,
    // Map of: (Node Label, Node Name) -> Node ID
    // e.g., ("Author", "John Doe") -> "blitz-5"
    canonical_nodes: HashMap<(String, String), String>,
}

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            graph: KnowledgeGraph::new(),
            next_id: 0,
            canonical_nodes: HashMap::new(),
        }
    }

    pub fn with_document(
        &mut self,
        metadata: &HashMap<String, String>,
        content: &str,
        strategy: &ChunkingStrategy,
    ) -> &mut Self {
        // Define metadata keys that should remain properties, not become nodes.
        let keys_to_skip: Vec<&str> = vec!["title", "path"];

        // 1. Create the main Document node with all metadata properties.
        let doc_id = self.generate_id();
        self.add_node(Node {
            id: doc_id.clone(),
            label: "Document".to_string(),
            properties: metadata.clone(),
        });

        // 2. Dynamically create nodes and edges from metadata.
        for (key, value_str) in metadata {
            if keys_to_skip.contains(&key.as_str()) {
                continue; // Skip keys that are just properties of the document.
            }

            // The key becomes the label for the new node (e.g., "Author", "Category").
            // We capitalize it for consistency.
            let label = format!("{}{}", (&key[..1].to_uppercase()), &key[1..]);

            // The value might be a comma-separated list.
            for value in value_str.split(',').map(|s| s.trim()) {
                if value.is_empty() {
                    continue;
                }

                let entity_id = self.get_or_create_canonical_node(&label, value);

                // Create an edge with a label like "HAS_AUTHOR", "HAS_CATEGORY".
                let edge_label = format!("HAS_{}", key.to_uppercase());

                self.add_edge(Edge {
                    source: doc_id.clone(),
                    target: entity_id,
                    label: edge_label,
                    properties: HashMap::new(),
                });
            }
        }

        // 3. Process the document content into Chunk nodes.
        for chunk_text in chunk_text(content, strategy) {
            let chunk_id = self.generate_id();
            let mut chunk_properties = HashMap::new();
            chunk_properties.insert("text".to_string(), chunk_text.to_string());
            self.add_node(Node {
                id: chunk_id.clone(),
                label: "Chunk".to_string(),
                properties: chunk_properties,
            });
            self.add_edge(Edge {
                source: doc_id.clone(),
                target: chunk_id,
                label: "CONTAINS".to_string(),
                properties: HashMap::new(),
            });
        }
        self
    }

    // ... (The rest of the helper methods remain unchanged) ...
    pub fn build(self) -> KnowledgeGraph {
        self.graph
    }

    fn add_node(&mut self, node: Node) {
        self.graph.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Edge) {
        self.graph.edges.push(edge);
    }

    fn get_or_create_canonical_node(&mut self, label: &str, name: &str) -> String {
        let key = (label.to_string(), name.to_string());
        if let Some(id) = self.canonical_nodes.get(&key) {
            return id.clone();
        }

        let new_id = self.generate_id();
        let mut properties = HashMap::new();
        properties.insert("name".to_string(), name.to_string());
        self.add_node(Node {
            id: new_id.clone(),
            label: label.to_string(),
            properties,
        });

        self.canonical_nodes.insert(key, new_id.clone());
        new_id
    }

    fn generate_id(&mut self) -> String {
        self.next_id += 1;
        format!("blitz-{}", self.next_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_builder::chunker::ChunkingStrategy;

    #[test]
    fn test_dynamic_graph_builder_and_chunking() {
        let mut builder = GraphBuilder::new();
        let content = "First line of content.\nSecond line, same chunk.";

        let mut metadata = HashMap::new();
        metadata.insert("title".to_string(), "Dynamic Systems".to_string());
        metadata.insert("author".to_string(), "Ada Lovelace".to_string());
        metadata.insert(
            "subject".to_string(),
            "Computer Science, Mathematics".to_string(),
        );

        // Use a specific chunking strategy
        let strategy = ChunkingStrategy::ByLine();
        builder.with_document(&metadata, content, &strategy);
        let graph = builder.build();

        // Expected nodes: 1 Document, 2 Chunk (ByLine merges lines), 1 Author, 2 Subjects = 6 total
        assert_eq!(graph.nodes.len(), 6);

        // Find the Document node and check its properties
        let doc_node = graph.nodes.iter().find(|n| n.label == "Document").unwrap();
        assert_eq!(doc_node.properties.get("title").unwrap(), "Dynamic Systems");

        // Find the Author node (dynamically created)
        let author_node = graph.nodes.iter().find(|n| n.label == "Author").unwrap();
        assert_eq!(author_node.properties.get("name").unwrap(), "Ada Lovelace");

        // Check the dynamic edge from Document to Author
        let author_edge = graph
            .edges
            .iter()
            .find(|e| e.label == "HAS_AUTHOR")
            .unwrap();
        assert_eq!(author_edge.source, doc_node.id);
        assert_eq!(author_edge.target, author_node.id);

        // Check the dynamically created Subject nodes and edges
        let subject_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.label == "HAS_SUBJECT")
            .collect();
        assert_eq!(subject_edges.len(), 2);
        assert!(subject_edges.iter().all(|e| e.source == doc_node.id));
    }
}
