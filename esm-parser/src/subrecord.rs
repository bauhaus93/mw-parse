use std::str;

use utility::{ parse_long32, parse_long64, parse_float32, parse_string, parse_position_data };
use parse_error::ParseError;
use position::PositionData;

pub struct Subrecord {
    name: String,
    content: SubrecordContent
}

#[derive(Debug)]
pub enum SubrecordContent {
    Skipped,
    Long32(i32),
    Long64(i64),
    Float32(f32),
    String(String),
    PositionData(PositionData)
}

impl Subrecord {

    pub fn new(name: String, content: SubrecordContent) -> Subrecord {

        Subrecord {
            name: name,
            content: content
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_content(&self) -> &SubrecordContent {
        &self.content
    }

    pub fn consume(self) -> (String, SubrecordContent) {
        (self.name, self.content)
    }
}

impl SubrecordContent {

    pub fn new_long32(data: &[u8]) -> Result<SubrecordContent, ParseError> {
        Ok(SubrecordContent::Long32(parse_long32(data)?))
    }

    pub fn new_long64(data: &[u8]) -> Result<SubrecordContent, ParseError> {
        assert!(data.len() == 8);
        Ok(SubrecordContent::Long64(parse_long64(data)?))
    }

    pub fn new_float32(data: &[u8]) -> Result<SubrecordContent, ParseError> {
        assert!(data.len() == 4);
        Ok(SubrecordContent::Float32(parse_float32(data)?))
    }

    pub fn new_string(data: &[u8]) -> Result<SubrecordContent, ParseError> {
        Ok(SubrecordContent::String(parse_string(data)?))
    }

    pub fn new_position_data(data: &[u8]) -> Result<SubrecordContent, ParseError> {
        Ok(SubrecordContent::PositionData(parse_position_data(data)?))
    }

    pub fn get_type_str(&self) -> &'static str {
        match *self {
            SubrecordContent::Long32(_) => "long32",
            SubrecordContent::Long64(_) => "long64",
            SubrecordContent::Float32(_) => "float32",
            SubrecordContent::String(_) => "string",
            SubrecordContent::PositionData(_) => "position_data"
        }
    }
}
