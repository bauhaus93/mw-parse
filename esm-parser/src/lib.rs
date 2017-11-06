#[macro_use]
extern crate log;
extern crate byteorder;

pub mod parse_error;
pub mod tes3_header;
pub mod cell_data;
pub mod utility;
pub mod position;
pub mod subrecord;

use std::fs::File;
use std::io::BufReader;
use std::io::{ Cursor, Read, Seek, SeekFrom };
use std::fmt;

use utility::{ parse_record_header, read_data };
use parse_error::ParseError;
use tes3_header::TES3Header;
use cell_data::CellData;

pub struct GameData {
    tes3_header: TES3Header,
    cells: Vec<CellData>
}

pub fn parse_game_data(path_esm: &str) -> Result<GameData, ParseError> {
    info!("parsing {}", path_esm);

    let file = File::open(path_esm)?;
    let mut reader = BufReader::with_capacity(100, file);

    let tes3_header = read_tes3_header(&mut reader)?;
    let mut cells: Vec<CellData> = Vec::new();

    for _ in 0..tes3_header.get_num_records() {
        let header_data = read_data(&mut reader, 8)?;
        let (rec_name, rec_size) = parse_record_header(&header_data)?;

        let rec_data = read_data(&mut reader, rec_size)?;

        match rec_name.as_ref() {
            "CELL" => cells.push(CellData::new(&rec_data)?),
            _ => {}
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

fn read_tes3_header<R: Read + Seek>(reader: &mut R) -> Result<TES3Header, ParseError> {
    let data = read_data(&mut reader, 8)?;
    let (rec_name, rec_size) = parse_record_header(&data)?;
    if rec_name != "TES3" {
        return Err(ParseError::InvalidRecordName("TES3".to_owned(), rec_name));
    }
    Ok(TES3Header::new(reader)?)
}
