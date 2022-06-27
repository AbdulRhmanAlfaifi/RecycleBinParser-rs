use byteorder::{LittleEndian, ReadBytesExt};
use errors::RecycleBinParserError;
use serde::Serialize;
use std::io::Read;
use std::result::Result;
use winparsingtools::{date_time::FileTime, utils::read_utf16_string};

mod errors;
mod tests;
#[derive(Debug, Serialize)]
pub struct RecycleBinParser {
    pub version: u64,
    pub file_size: u64,
    pub deletion_time: FileTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename_length: Option<u32>,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
}

impl RecycleBinParser {
    pub fn from_reader<R: Read>(r: &mut R) -> Result<Self, RecycleBinParserError> {
        let version = r.read_u64::<LittleEndian>().map_err(|e| {
            RecycleBinParserError::parsing_error(&format!(
                "Unable to parser the 'version' field, ERROR: {}",
                e
            ))
        })?;
        let file_size = r.read_u64::<LittleEndian>().map_err(|e| {
            RecycleBinParserError::parsing_error(&format!(
                "Unable to parser the 'file_size' field, ERROR: {}",
                e
            ))
        })?;
        let deletion_time = FileTime::new(r.read_u64::<LittleEndian>().map_err(|e| {
            RecycleBinParserError::parsing_error(&format!(
                "Unable to parser the 'deletion_time' field, ERROR: {}",
                e
            ))
        })?);
        let filename_length = match version {
            1 => None,
            2 => Some(r.read_u32::<LittleEndian>().map_err(|e| {
                RecycleBinParserError::parsing_error(&format!(
                    "Unable to parser the 'filename_length' field, ERROR: {}",
                    e
                ))
            })?),
            _ => {
                return Err(RecycleBinParserError::unknown_version(&format!(
                    "Unknown version detected, version number '{}'",
                    version
                )));
            }
        };
        let filename = read_utf16_string(r, None).map_err(|e| {
            RecycleBinParserError::parsing_error(&format!(
                "Unable to parser the 'filename' field, ERROR: {}",
                e
            ))
        })?;

        Ok(Self {
            version,
            file_size,
            deletion_time,
            filename_length,
            filename,
            sid: None,
        })
    }

    pub fn set_sid(&mut self, sid: String) {
        self.sid = Some(sid);
    }
}
