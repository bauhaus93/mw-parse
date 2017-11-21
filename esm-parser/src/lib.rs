#[macro_use]
extern crate log;
extern crate byteorder;

pub mod parse_error;
pub mod file_type;
pub mod parse;
pub mod point;

pub mod field;
pub mod record;
pub mod subrecord;

use std::fs::File;
use std::io::{ BufReader };
use std::fmt;

use parse::Parseable;
use parse_error::ParseError;
use record::Record;
use record::tes3::TES3;

pub struct GameData {
    tes3_header: TES3
}

pub fn parse_game_data(path_esm: &str) -> Result<GameData, ParseError> {
    info!("parsing {}", path_esm);

    let file = File::open(path_esm)?;
    let mut reader = BufReader::with_capacity(1000, file);

    let tes3_header = match Record::from_stream(&mut reader)? {
        Record::Tes3(tes3) => tes3,
        _ => return Err(ParseError::InvalidRecordName("TES3".to_owned(), "<OTHER>".to_owned()))
    };


    let game_data = GameData {
        tes3_header: tes3_header
    };

    Ok(game_data)
}

impl fmt::Display for GameData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            self.tes3_header
        )
    }
}
