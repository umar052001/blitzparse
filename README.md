# BlitzParse 🚀

**High-performance, Rust-powered document parsing for Python.**

**BlitzParse** is a native Python module engineered in Rust to provide an order-of-magnitude performance increase for text extraction from common document formats. By leveraging Rust's performance, memory safety, and powerful ecosystem, BlitzParse offers a seamless and high-speed alternative to pure Python libraries, making it ideal for data processing pipelines, backend services, and any application that handles documents at scale.

## Performance

The primary goal of BlitzParse is to be exceptionally fast. In benchmark tests against the popular `python-docx` library, BlitzParse demonstrates a significant performance advantage.

| Library | Average Time (ms) | Performance |
| :--- | :--- | :--- |
| **BlitzParse (Rust)** | **\~0.3 ms** | **55.9x Faster** |
| `python-docx` | \~18.2 ms | Baseline |

*Benchmarks were conducted on a complex `.docx` file containing paragraphs, tables, headers, and footers.*

## Features

  * **Blazing Fast:** Built in Rust for maximum performance and efficiency.
  * **Native Python Module:** Installs easily and can be imported just like any other Python package.
  * **Multi-Format Support:** Robust text extraction for `.docx`, `.pdf`, and `.txt` files.
  * **Rich Content Parsing:** Extracts text from paragraphs, tables, and other structural elements.
  * **Production Ready:** Thoroughly tested with both unit and integration tests, and validated with a CI pipeline.

## Installation
With blitz_parse now published on [PyPI](https://pypi.org/project/blitz-parse/), installation is as simple as:

`pip install blitz-parse`

BlitzParse is built as a native Python module using `maturin`.

1.  **Create a virtual environment:**

    ```bash
    uv venv .venv
    source .venv/bin/activate
    ```

2.  **Install `maturin`:**

    ```bash
    uv pip install maturin
    ```

3.  **Build and install `blitz_parse`:**
    From the root of the project directory, run:

    ```bash
    maturin develop --release
    ```

    This command compiles the Rust code in release mode (fully optimized) and installs it into your active virtual environment.

## Usage

Using BlitzParse in your Python code is straightforward.

```python
import blitz_parse

# Path to your document
file_path = "path/to/your/document.docx"

try:
    # Call the high-performance Rust function
    extracted_text = blitz_parse.extract_text_py(file_path)
    print("--- Extraction Successful ---")
    print(extracted_text[:500] + "...")

except ValueError as e:
    print(f"An error occurred: {e}")
```

## Development and Testing

Contributions are welcome. To set up a development environment:

1.  **Install Rust:** Follow the official instructions at [rust-lang.org](https://rust-lang.org/).
2.  **Clone the repository:**
    ```bash
    git clone https://github.com/umar052001/blitz_parse.git
    cd blitz_parse
    ```
3.  **Run the test suite:**
    ```bash
    cargo test
    ```
    The project includes a comprehensive suite of unit tests for internal logic and integration tests for the public API.

## Roadmap

BlitzParse is the foundational layer of a larger vision for high-performance document intelligence. Future development will focus on:

  * **Embedding Service:** An integrated, high-speed service for creating vector embeddings from document text, preparing data for LLMs.
  * **Graph-Based RAG:** A system where documents are nodes in a knowledge graph, allowing an LLM agent to traverse relationships and synthesize answers from multiple sources.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
