use std::io::{ Read, Seek };
use std::str;
use std::fmt;

use parse_error::ParseError;
use position::PositionData;
use subrecord::{ Subrecord, SubrecordContent };
use utility::{ parse_cell_subrecords };

pub struct CellData {
    name: String,
    region_name: String,
    objects: Vec<CellObject>
}

pub struct CellObject {
    index: i32,
    name: String,
    scale: f32,
    position_data: PositionData,
}

impl CellData {

    pub fn new(data: &[u8]) -> Result<CellData, ParseError> {
        let mut subrecords = parse_cell_subrecords(data)?;

        let mut name = String::new();
        let mut region_name = String::new();
        let mut objects: Vec<CellObject> = Vec::new();

        let cell_object_subrecords = match subrecords.len() {
            n  if n >= 7 => subrecords.split_off(7),
            _ => Vec::new()
        };

        for sub in subrecords {
            let (sub_name, sub_content) = sub.consume();
            match sub_name.as_ref() {
                "NAME" => name = match sub_content {
                    SubrecordContent::String(s) => s,
                    other => return Err(ParseError::InvalidSubrecordType(sub_name, other))
                },
                "RGNN" => region_name = match sub_content {
                    SubrecordContent::String(s) => s,
                    other => return Err(ParseError::InvalidSubrecordType(sub_name, other))
                }
            }
        }

        assert!(cell_object_subrecords.len() % 15 == 0);
        while let cell_object_sub = cell_object_subrecords.split_off(15) {
            let cell_data = CellObject::new(cell_object_sub)?;
        }

        let cell_data = CellData {
            name: name,
            region_name: region_name,
            objects: objects
        };

        Ok(cell_data)
    }
}

impl CellObject {

    pub fn new(subrecords: Vec<Subrecord>) -> Result<CellObject, ParseError> {

        let mut index = None;
        let mut name = None;
        let mut scale = None;
        let mut position_data = None;

        for sub in subrecords {
            let (sub_name, sub_content) = sub.consume();
            match sub_name.as_ref() {
                "FRMR" => index = match sub_content {
                    SubrecordContent::Long32(v) => Some(v),
                    other => return Err(ParseError::InvalidSubrecordType(sub_name, other))
                },
                "NAME" => name = match sub_content {
                    SubrecordContent::String(s) => Some(s),
                    other => return Err(ParseError::InvalidSubrecordType(sub_name, other))
                },
                "XSCL" => scale = match sub_content {
                    SubrecordContent::Float32(v) => Some(v),
                    other => return Err(ParseError::InvalidSubrecordType(sub_name, other))
                },
                "DATA" => position_data = match sub_content {
                    SubrecordContent::PositionData(pd) => Some(pd),
                    other => return Err(ParseError::InvalidSubrecordType(sub_name, other))
                }
            }
        }

        let cell_object = match (index, name, scale, position_data) {
            (Some(i), Some(n), Some(s), Some(pd)) => CellObject {
                index: i,
                name: n,
                scale: s,
                position_data: pd
            },
            _ => unreachable!()
        };

        Ok(cell_object)
    }

}
