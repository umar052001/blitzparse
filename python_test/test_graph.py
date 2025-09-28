import json

import blitz_parse

docx_path = "tests/assets/sample.docx"

print(f"--- Testing dynamic metadata and chunking strategies ---")

# 1. Define rich, dynamic metadata
document_metadata = {
    "title": "Interdisciplinary Studies in AI",
    "Subjects": "AI, ML, NLP, LLM",
    "author": "Dr. Kenji Tanaka",
    "journal": "Journal of Advanced Research",
    "keywords": "AI, Knowledge Graphs, RAG",
}

try:
    # 2. Select a chunking strategy
    # Using ByLine() as it's deterministic for this test
    chunk_strategy = blitz_parse.ChunkingStrategy.ByLine()
    print(f"\nUsing chunking strategy: {chunk_strategy}")

    # 3. Call the Rust function with all parameters
    graph_json_string = blitz_parse.build_graph_py(
        docx_path, document_metadata, chunk_strategy
    )
    print("✅ Success! Received JSON from Rust.")

    # 4. Parse and Validate
    graph_data = json.loads(graph_json_string)
    # print(f"\n Graph: {graph_data}")
    print(
        f"   - Graph contains {len(graph_data['nodes'])} nodes and {len(graph_data['edges'])} edges."
    )

    # --- Find nodes and edges ---
    doc_node = next(n for n in graph_data["nodes"] if n["label"] == "Document")
    journal_node = next(n for n in graph_data["nodes"] if n["label"] == "Journal")
    keyword_nodes = [n for n in graph_data["nodes"] if n["label"] == "Keywords"]

    has_journal_edge = next(
        e for e in graph_data["edges"] if e["label"] == "HAS_JOURNAL"
    )
    has_keywords_edges = [
        e for e in graph_data["edges"] if e["label"] == "HAS_KEYWORDS"
    ]

    # --- Assertions ---
    # Check if dynamic nodes were created correctly
    assert journal_node is not None, "Journal node should be created dynamically"
    assert len(keyword_nodes) == 3, "Should have created 3 keyword nodes"
    print("   - Dynamic nodes created successfully.")

    # Check if dynamic edges were created correctly
    assert has_journal_edge["source"] == doc_node["id"]
    assert has_journal_edge["target"] == journal_node["id"]
    assert len(has_keywords_edges) == 3, "Should have 3 HAS_KEYWORDS edges"
    print("   - Dynamic edges created successfully.")

    print("\n🎉 All Python-side validations passed!")

except Exception as e:
    print(f"\n❌ An error occurred during testing: {e}")
    raise
