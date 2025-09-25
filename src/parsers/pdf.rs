use crate::errors::BlitzParseError;

pub fn parse_pdf(path: &str) -> Result<String, BlitzParseError> {
    let bytes = std::fs::read(path)?;
    pdf_extract::extract_text_from_mem(&bytes).map_err(|e| BlitzParseError::Pdf(e.to_string()))
}
