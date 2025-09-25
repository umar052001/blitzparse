use crate::errors::BlitzParseError;

pub fn parse_txt(path: &str) -> Result<String, BlitzParseError> {
    std::fs::read_to_string(path).map_err(BlitzParseError::Io)
}
