#[macro_use]
extern crate log;
extern crate byteorder;

pub mod parse_error;
pub mod tes3_header;
pub mod record;
pub mod subrecord;
pub mod file_type;
pub mod basic_type;
pub mod parse;
pub mod point;

use std::fs::File;
use std::io::{ BufReader };
use std::fmt;

use parse_error::ParseError;
use tes3_header::TES3Header;
use record::Record;
use parse::Parseable;
use record::cell::Cell;

pub struct GameData {
    tes3_header: TES3Header,
    cell_data: Vec<Cell>
}

pub fn parse_game_data(path_esm: &str) -> Result<GameData, ParseError> {
    info!("parsing {}", path_esm);

    let file = File::open(path_esm)?;
    let mut reader = BufReader::with_capacity(100, file);

    let tes3_header = match Record::parse(&mut reader)? {
        Record::TES3Header(tes3) => tes3,
        _invalid_record => return Err(ParseError::InvalidRecordName("TES3".to_owned(), "<other>".to_owned()))
    };

    let mut cells: Vec<Cell> = Vec::new();

    for _ in 0..tes3_header.get_hedr().get_num_records() {
        let record = match Record::parse(&mut reader) {
            Ok(r) => r,
            Err(e) => {
                info!("{}", e);
                continue;
            }
        };

        match record {
            Record::Cell(cell_data) => {
                if cells.len() % 500 == 0 {
                    cells.reserve(500);
                }
                info!("cells: {}", cells.len());
                cells.push(cell_data);
            },
            _ => {}
        }
    }

    for cd in &cells {
        info!("{}", cd);
    }

    let game_data = GameData {
        tes3_header: tes3_header,
        cell_data: cells
    };

    Ok(game_data)
}

impl fmt::Display for GameData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tes3_header)
    }
}
