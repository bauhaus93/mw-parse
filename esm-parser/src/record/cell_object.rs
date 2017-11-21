use std::io::{ Read, Seek };

use parse::Parseable;
use parse_error::ParseError;
use point::{ Position, Rotation };
use subrecord::tuple::{ Sextuple };
use subrecord::Subrecord;
use field::Field;

pub struct CellObject {
    index: u32,
    name: String,
    scale: f32,
    door: Option<Door>
}

pub struct Door {
    exit_name: String,
    lock_level: Option<f32>,
    key_name: Option<String>,
    trap_name: Option<String>
}

impl Parseable for CellObject {

    fn parse<R: Read + Seek>(reader: &mut R) -> Result<CellObject, ParseError> {
        let object_index = i32::parse_subrecord(reader, "FRMR")?;
        let object_id = String::parse_subrecord(reader, "NAME")?;
        let scale = f32::parse_subrecord(reader, "XSCL")?;

        Ok(CellObject {
            index: object_index as u32,
            name: object_id,
            scale: scale,
            door: None
        })

    }
}

impl Parseable for Door {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<Door, ParseError> {
        let position_rotation = match Sextuple::<f32>::parse_optional_subrecord(reader, "DODT")? {
            Some(v) => v,
            None => return Err(ParseError::InvalidSubrecordName("DNAM".to_owned(), "NO_DOOR".to_owned()))
        };

        let door_name = String::parse_subrecord(reader, "DNAM")?;


        let lock_level = f32::parse_optional_subrecord(reader, "FLTV")?;
        let key_name = String::parse_optional_subrecord(reader, "KNAM")?;
        let trap_name = String::parse_optional_subrecord(reader, "TNAM")?;

        let door = Door {
            exit_name: door_name,
            lock_level: lock_level,
            key_name: key_name,
            trap_name: trap_name
        };

        Ok(door)
    }
}
