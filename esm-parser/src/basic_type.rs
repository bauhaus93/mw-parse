use std::str;
use std::io::{ Read, Seek };

use byteorder::{ ReadBytesExt, LittleEndian };

use parse::{ Parseable, ParseableWithSize };
use parse_error::ParseError;

pub struct Long32(pub i32);
pub struct Long64(pub i64);
pub struct Float32(pub f32);
pub struct Text(pub String);

impl Parseable<Long32> for Long32 {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<Long32, ParseError> {
        Ok(Long32(reader.read_i32::<LittleEndian>()?))
    }
}

impl Parseable<Long64> for Long64 {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<Long64, ParseError> {
        Ok(Long64(reader.read_i64::<LittleEndian>()?))
    }
}

impl Parseable<Float32> for Float32 {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<Float32, ParseError> {
        Ok(Float32(reader.read_f32::<LittleEndian>()?))
    }
}

impl ParseableWithSize<Text> for Text {
    fn parse<R: Read + Seek>(reader: &mut R, size: usize) -> Result<Text, ParseError> {
        let mut field: Vec<u8> = Vec::with_capacity(size);
        field.resize(size, 0);
        reader.read_exact(&mut field)?;
        let text: String = field.iter().map(|&c| c as char).collect();
        Ok(Text(text))
    }
}
