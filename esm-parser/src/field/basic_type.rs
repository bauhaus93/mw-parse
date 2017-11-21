use std::str;
use std::io::{ Read, Seek };

use byteorder::{ ReadBytesExt, LittleEndian };

use field::Field;
use parse_error::ParseError;

impl Field for i32 {
    fn parse_field<R: Read + Seek>(reader: &mut R, field_size: usize) -> Result<i32, ParseError> {
        assert!(field_size == 4);
        let mut field: Vec<u8> = Vec::with_capacity(field_size);
        field.resize(field_size, 0);
        reader.read_exact(&mut field)?;
        Ok((&field[..]).read_i32::<LittleEndian>()?)
    }

    fn parse_field_fixed<R: Read + Seek>(reader: &mut R) -> Result<i32, ParseError> {
        i32::parse_field(reader, 4)
    }
}

impl Field for i64 {
    fn parse_field<R: Read + Seek>(reader: &mut R, field_size: usize) -> Result<i64, ParseError> {
        assert!(field_size == 8);
        let mut field: Vec<u8> = Vec::with_capacity(field_size);
        field.resize(field_size, 0);
        reader.read_exact(&mut field)?;
        Ok((&field[..]).read_i64::<LittleEndian>()?)
    }

    fn parse_field_fixed<R: Read + Seek>(reader: &mut R) -> Result<i64, ParseError> {
        i64::parse_field(reader, 8)
    }
}

impl Field for f32 {
    fn parse_field<R: Read + Seek>(reader: &mut R, field_size: usize) -> Result<f32, ParseError> {
        assert!(field_size == 4);
        let mut field: Vec<u8> = Vec::with_capacity(field_size);
        field.resize(field_size, 0);
        reader.read_exact(&mut field)?;
        Ok((&field[..]).read_f32::<LittleEndian>()?)
    }

    fn parse_field_fixed<R: Read + Seek>(reader: &mut R) -> Result<f32, ParseError> {
        f32::parse_field(reader, 4)
    }
}


impl Field for String {
    fn parse_field<R: Read + Seek>(reader: &mut R, field_size: usize) -> Result<String, ParseError> {
        let mut field: Vec<u8> = Vec::with_capacity(field_size);
        field.resize(field_size, 0);
        reader.read_exact(&mut field)?;
        let text: String = field.iter().map(|&c| c as char).collect();
        Ok(text)
    }

    fn parse_field_fixed<R: Read + Seek>(reader: &mut R) -> Result<String, ParseError> {
        unreachable!()
    }
}
