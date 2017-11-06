#[macro_use]
extern crate log;
extern crate byteorder;

pub mod parse_error;
pub mod tes3_header;
pub mod cell;
pub mod utility;

use std::fs::File;
use std::io::BufReader;
use std::io::{ Read, Seek };
use std::io::Cursor;
use std::fmt;


use utility::read_record_header;
use parse_error::ParseError;
use tes3_header::TES3Header;
use cell::Cell;

pub struct GameData {
    tes3_header: TES3Header,
    cells: Vec<Cell>
}

pub fn parse_game_data(path_esm: &str) -> Result<GameData, ParseError> {
    info!("parsing {}", path_esm);

    let file = File::open(path_esm)?;
    let reader = BufReader::new(file);
    let mut cursor = Cursor::new(reader);

    let tes3_header = parse_tes3_header(&mut cursor)?;
    let cells: Vec<Cell> = Vec::new();

    for _ in tes3_header.get_num_records() {
        let (rec_name, rec_size) = read_record_header(&mut cursor)?;
        match rec_name {
           
            unhandled => {
                
            }
        }

    }

    let game_data = GameData {
        tes3_header: tes3_header,
        cells: cells
    };

    Ok(game_data)
}

impl fmt::Display for GameData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tes3_header)
    }
}

fn parse_tes3_header<R: Read + Seek>(reader: &mut R) -> Result<TES3Header, ParseError> {
    let (rec_name, rec_size) = read_record_header(reader)?;
    if rec_name != "TES3" {
        return Err(ParseError::InvalidRecordName("TES3".to_owned(), rec_name));
    }
    Ok(TES3Header::new(reader)?)
}


