#[derive(Debug)]
pub enum RecycleBinParserErrorType {
    GeneralParsingError,
    UnknownVersion,
}

#[derive(Debug)]
pub struct RecycleBinParserError {
    error_type: RecycleBinParserErrorType,
    error_msg: String,
}

impl RecycleBinParserError {
    pub fn unknown_version(msg: &str) -> Self {
        Self {
            error_type: RecycleBinParserErrorType::UnknownVersion,
            error_msg: msg.to_string(),
        }
    }
    pub fn parsing_error(msg: &str) -> Self {
        Self {
            error_type: RecycleBinParserErrorType::GeneralParsingError,
            error_msg: msg.to_string(),
        }
    }
}
