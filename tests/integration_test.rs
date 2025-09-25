use blitz_parse::extract_text;

#[test]
fn test_docx_extraction_from_public_api() {
    let result = extract_text("tests/assets/sample.docx");
    assert!(result.is_ok(), "DOCX parsing should succeed");

    let text = result.unwrap();

    assert!(
        text.contains(
            "Vestibulum neque massa, scelerisque sit amet ligula eu, congue molestie mi."
        ),
        "Failed to find paragraph text."
    );

    assert!(
        text.contains("In eleifend velit vitae libero sollicitudin euismod."),
        "Failed to find table text."
    );
}

#[test]
fn test_pdf_extraction_from_public_api() {
    let result = extract_text("tests/assets/sample.pdf");
    assert!(result.is_ok(), "PDF parsing should succeed");

    let text = result.unwrap();

    assert!(
        text.contains(
            "Vestibulum neque massa, scelerisque sit amet ligula eu, congue molestie mi."
        ),
        "Failed to find paragraph text in PDF."
    );
}

#[test]
fn test_unsupported_file_type() {
    let result = extract_text("tests/assets/sample.png");
    assert!(
        result.is_err(),
        "Should return an error for unsupported file types"
    );
}
