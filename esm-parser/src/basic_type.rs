use std::str;
use std::io::{ Read, Seek };

use byteorder::{ ReadBytesExt, LittleEndian };

use parse::{ Parseable, ParseableExact };
use parse_error::ParseError;

impl Parseable for i32 {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<i32, ParseError> {
        i32::parse_exact(reader, 4)
    }
}

impl Parseable for i64 {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<i64, ParseError> {
        i64::parse_exact(reader, 8)
    }
}

impl Parseable for f32 {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<f32, ParseError> {
        f32::parse_exact(reader, 4)
    }
}

impl ParseableExact for i32 {
    fn parse_exact<R: Read + Seek>(reader: &mut R, size: usize) -> Result<i32, ParseError> {
        assert!(size == 4);
        let mut field: Vec<u8> = Vec::with_capacity(size);
        field.resize(size, 0);
        reader.read_exact(&mut field)?;
        Ok((&field[..]).read_i32::<LittleEndian>()?)
    }
}

impl ParseableExact for i64 {
    fn parse_exact<R: Read + Seek>(reader: &mut R, size: usize) -> Result<i64, ParseError> {
        assert!(size == 8);
        let mut field: Vec<u8> = Vec::with_capacity(size);
        field.resize(size, 0);
        reader.read_exact(&mut field)?;
        Ok((&field[..]).read_i64::<LittleEndian>()?)
    }
}

impl ParseableExact for f32 {
    fn parse_exact<R: Read + Seek>(reader: &mut R, size: usize) -> Result<f32, ParseError> {
        assert!(size == 4);
        let mut field: Vec<u8> = Vec::with_capacity(size);
        field.resize(size, 0);
        reader.read_exact(&mut field)?;
        Ok((&field[..]).read_f32::<LittleEndian>()?)
    }
}

impl ParseableExact for String {
    fn parse_exact<R: Read + Seek>(reader: &mut R, size: usize) -> Result<String, ParseError> {
        let mut field: Vec<u8> = Vec::with_capacity(size);
        field.resize(size, 0);
        reader.read_exact(&mut field)?;
        let text: String = field.iter().map(|&c| c as char).collect();
        Ok(text)
    }
}
