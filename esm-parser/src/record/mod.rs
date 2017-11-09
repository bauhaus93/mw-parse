pub mod record_header;
pub mod subrecord_header;

use std::io::{ Read, Seek, SeekFrom, Cursor };
use std::fmt;

use parse::Parseable;
use parse_error::ParseError;
use record::record_header::RecordHeader;
use tes3_header::TES3Header;
use cell::cell_data::CellData;

pub enum Record {
    Unhandled,
    TES3Header(TES3Header),
    Cell(CellData)
}

fn read_data<R: Read + Seek>(reader: &mut R , size: usize) -> Result<Vec<u8>, ParseError> {
    let mut data: Vec<u8> = Vec::with_capacity(size);
    data.resize(size, 0);
    reader.read_exact(&mut data)?;
    Ok(data)
}

impl Parseable<Record> for Record {

    fn parse<R: Read + Seek>(reader: &mut R) -> Result<Record, ParseError> {
        let header = RecordHeader::parse(reader)?;

        let record = match header.get_name().0.as_ref() {
            "TES3" => {
                let data = read_data(reader, header.get_size().0 as usize)?;
                let mut cursor = Cursor::new(data);
                Record::TES3Header(TES3Header::parse(&mut cursor)?)
            },
            "CELL" => {
                let data = read_data(reader, header.get_size().0 as usize)?;
                let mut cursor = Cursor::new(data);
                Record::Cell(CellData::parse(&mut cursor)?)
            },
            _unhandled => {
                reader.seek(SeekFrom::Current(header.get_size().0 as i64))?;
                Record::Unhandled
            }
        };
        Ok(record)

    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Record::TES3Header(ref tes3) => write!(f, "{}", tes3),
            Record::Cell(ref cell_data) => write!(f, "{}", cell_data),
            Record::Unhandled => write!(f, "unhandled record")
        }
    }
}
