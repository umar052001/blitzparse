use std::fmt;

// A custom error enum for our library. This gives us precise,
// informative errors for different failure scenarios.
#[derive(Debug)]
pub enum BlitzParseError {
    Io(std::io::Error),
    Pdf(String),
    Docx(String),
    UnsupportedFileType(String),
}

// This allows our error type to be displayed nicely.
impl fmt::Display for BlitzParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlitzParseError::Io(e) => write!(f, "I/O Error: {}", e),
            BlitzParseError::Pdf(e) => write!(f, "PDF Parsing Error: {}", e),
            BlitzParseError::Docx(e) => write!(f, "DOCX Parsing Error: {}", e),
            BlitzParseError::UnsupportedFileType(ext) => {
                write!(f, "Unsupported file type: {}", ext)
            }
        }
    }
}

// This is crucial for allowing our error to work with Rust's `?` operator.
impl std::error::Error for BlitzParseError {}

// Helper implementations to convert underlying library errors into our custom error type.
impl From<std::io::Error> for BlitzParseError {
    fn from(err: std::io::Error) -> Self {
        BlitzParseError::Io(err)
    }
}
