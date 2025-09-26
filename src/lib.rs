use pyo3::prelude::*;
pub mod errors;
pub mod parsers;

use errors::BlitzParseError;
use std::path::Path;

pub fn extract_text(path: &str) -> Result<String, BlitzParseError> {
    let extension = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| BlitzParseError::UnsupportedFileType("No extension".to_string()))?;

    match extension.to_lowercase().as_str() {
        "pdf" => parsers::pdf::parse_pdf(path),
        "docx" => parsers::docx::parse_docx(path),
        "txt" => parsers::txt::parse_txt(path),
        _ => Err(BlitzParseError::UnsupportedFileType(extension.to_string())),
    }
}

// This `#[pyfunction]` attribute exposes our Rust function to Python.
// We also handle the error conversion here, turning a Rust `Result` into a Python exception.
#[pyfunction]
fn extract_text_py(path: &str) -> PyResult<String> {
    match extract_text(path) {
        Ok(text) => Ok(text),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
    }
}

// This `#[pymodule]` block creates the Python module.
// Python will be able to `import blitzparse` because of this.
#[pymodule]
fn blitz_parse(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_text_py, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn it_extracts_text_from_txt_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        let expected_text = "Hello, blitzparse!\nThis is a test.";
        writeln!(file, "{}", expected_text).unwrap();

        let result = extract_text(file_path.to_str().unwrap());

        assert!(result.is_ok());
        let extracted_text = result.unwrap();
        assert_eq!(extracted_text.trim(), expected_text);
    }
}
