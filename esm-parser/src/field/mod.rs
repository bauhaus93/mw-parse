use std::io::{ Read, Seek };

use byteorder::{ LittleEndian, ReadBytesExt };

use parse::Parseable;
use parse_error::ParseError;

impl Parseable for i32 {
    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(reader.read_i32::<LittleEndian>()?)
    }
}

impl Parseable for u32 {
    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }
}

impl Parseable for f32 {
    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(reader.read_f32::<LittleEndian>()?)
    }
}

impl Parseable for i64 {
    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(reader.read_i64::<LittleEndian>()?)
    }
}

impl Parseable for String {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let text: String = buf.iter().map(|&c| c as char).collect();
        Ok(text)
    }

}
