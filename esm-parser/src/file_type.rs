
use parse_error::ParseError;

#[derive(Debug)]
pub enum FileType {
    ESP,
    ESM,
    ESS
}

impl FileType {

    pub fn from_num(num: i32) -> Result<FileType, ParseError> {
        match num {
            0 => Ok(FileType::ESP),
            1 => Ok(FileType::ESM),
            32 => Ok(FileType::ESS),
            n => Err(ParseError::UnknownFileType(n))
        }
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            FileType::ESP => "ESP",
            FileType::ESM => "ESM",
            FileType::ESS => "ESS"
        }
    }
}
