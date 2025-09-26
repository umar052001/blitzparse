use crate::errors::BlitzParseError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::Cursor;
use std::io::Read;

pub fn parse_docx(path: &str) -> Result<String, BlitzParseError> {
    // Try the ultra-fast ZIP extraction approach first
    match parse_docx_zip_direct(path) {
        Ok(text) => Ok(text),
        Err(_) => {
            // Fallback to docx-rs if ZIP approach fails
            parse_docx_fallback(path)
        }
    }
}

/// Ultra-fast DOCX parser using direct ZIP extraction
fn parse_docx_zip_direct(path: &str) -> Result<String, BlitzParseError> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut archive = zip::ZipArchive::new(reader)
        .map_err(|e| BlitzParseError::Docx(format!("Failed to open ZIP: {}", e)))?;

    // Extract document.xml directly - this is where the text lives
    let mut document_xml = archive
        .by_name("word/document.xml")
        .map_err(|e| BlitzParseError::Docx(format!("No document.xml found: {}", e)))?;

    let mut xml_content = String::with_capacity(document_xml.size() as usize);
    document_xml.read_to_string(&mut xml_content)?;

    // Parse XML and extract text using fast string operations
    extract_text_from_xml(&xml_content)
}

/// Extract text from document.xml using simple string operations
pub fn extract_text_from_xml(xml: &str) -> Result<String, BlitzParseError> {
    let mut reader = Reader::from_reader(Cursor::new(xml));
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut result = String::new();

    let mut in_paragraph = false;
    let mut in_table = false;
    let mut in_row = false;
    let mut in_cell = false;

    loop {
        match reader.read_event_into(&mut buf) {
            // --- Structural elements ---
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"w:p" => {
                    in_paragraph = true;
                }
                b"w:tbl" => {
                    in_table = true;
                    result.push('\n'); // separate from body text
                }
                b"w:tr" => {
                    in_row = true;
                    result.push('|'); // start markdown row
                }
                b"w:tc" => {
                    in_cell = true;
                    result.push(' ');
                }
                _ => {}
            },

            Ok(Event::End(e)) => match e.name().as_ref() {
                b"w:p" => {
                    if in_paragraph && !in_table {
                        result.push('\n'); // normal paragraph newline
                    }
                    in_paragraph = false;
                }
                b"w:tbl" => {
                    in_table = false;
                    result.push('\n'); // table ends
                }
                b"w:tr" => {
                    if in_row {
                        result.push_str("|\n"); // close markdown row
                    }
                    in_row = false;
                }
                b"w:tc" => {
                    if in_cell {
                        result.push(' ');
                        result.push('|'); // close cell
                    }
                    in_cell = false;
                }
                _ => {}
            },

            // --- Text extraction ---
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default();
                result.push_str(&text);
                if in_table && in_cell {
                    result.push(' '); // spacing inside cell
                } else {
                    result.push(' ');
                }
            }

            Ok(Event::Eof) => break,

            Err(e) => return Err(BlitzParseError::Docx(format!("XML error: {e}"))),
            _ => {}
        }

        buf.clear();
    }

    // Normalize whitespace
    let result = result
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(result)
}

/// Fallback using docx-rs (much slower but more robust)
fn parse_docx_fallback(path: &str) -> Result<String, BlitzParseError> {
    let content = std::fs::read(path)?;
    let docx = docx_rs::read_docx(&content)
        .map_err(|_| BlitzParseError::Docx("Failed to read DOCX file".to_string()))?;

    // Simplified, fast extraction
    let mut result = String::with_capacity(content.len() / 8);

    for child in &docx.document.children {
        if let docx_rs::DocumentChild::Paragraph(p) = child {
            let mut paragraph_text = String::new();

            for p_child in &p.children {
                if let docx_rs::ParagraphChild::Run(r) = p_child {
                    for r_child in &r.children {
                        if let docx_rs::RunChild::Text(t) = r_child {
                            paragraph_text.push_str(&t.text);
                        }
                    }
                }
            }

            if !paragraph_text.is_empty() {
                result.push_str(&paragraph_text);
                result.push('\n');
            }
        }
    }

    // Remove trailing newline
    if result.ends_with('\n') {
        result.pop();
    }

    Ok(result)
}
