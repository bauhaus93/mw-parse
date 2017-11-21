pub mod tes3;
pub mod cell;
mod header;

use std::io::Read;
use std::io;

use parse::Parseable;
use parse_error::ParseError;

use record::header::Header;
use record::tes3::TES3;
use record::cell::Cell;

pub enum Record {
    Unhandled(String),
    Tes3(TES3),
    Cell(Cell)
}

impl Parseable for Record {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let header = Header::from_stream(reader)?;
        trace!("curr record: name = {}, size = {}", header.get_name(), header.get_size());
        let mut ref_reader = reader.by_ref().take(header.get_size() as u64);

        let record = match header.get_name() {
            "TES3" => Record::Tes3(TES3::from_stream(&mut ref_reader)?),
            "CELL" => Record::Cell(Cell::from_stream(&mut ref_reader)?),
            _ => Record::Unhandled(header.get_name().to_owned())
        };
        io::copy(&mut ref_reader, &mut io::sink())?;    //Reads any unread data from copied reader

        Ok(record)
    }
}
