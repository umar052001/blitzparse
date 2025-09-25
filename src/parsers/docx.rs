use crate::errors::BlitzParseError;

pub fn parse_docx(path: &str) -> Result<String, BlitzParseError> {
    let content = std::fs::read(path)?;
    let docx = docx_rs::read_docx(&content)
        .map_err(|_| BlitzParseError::Docx("Failed to read DOCX file".to_string()))?;

    // Iterate through the document's children (paragraphs, tables, etc.)
    let full_text = docx
        .document
        .children
        .iter()
        .filter_map(|child| {
            // We only care about Paragraph nodes for now.
            if let docx_rs::DocumentChild::Paragraph(p) = child {
                // A paragraph is made of 'Runs'. We collect the text from each.
                let paragraph_text = p
                    .children
                    .iter()
                    .filter_map(|p_child| {
                        // A paragraph child can be a 'Run', 'Insert', etc.
                        if let docx_rs::ParagraphChild::Run(r) = p_child {
                            // A 'Run' has children too, one of which is 'Text'.
                            let run_text = r
                                .children
                                .iter()
                                .filter_map(|r_child| {
                                    if let docx_rs::RunChild::Text(t) = r_child {
                                        Some(t.text.as_str())
                                    } else {
                                        None
                                    }
                                })
                                .collect::<String>();
                            Some(run_text)
                        } else {
                            None
                        }
                    })
                    .collect::<String>();
                Some(paragraph_text)
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(full_text)
}
