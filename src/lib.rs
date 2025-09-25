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
