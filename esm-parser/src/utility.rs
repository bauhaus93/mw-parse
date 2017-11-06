use std::io::{ Read, Seek, SeekFrom };
use std::str;

use byteorder::{ ReadBytesExt, LittleEndian };

use parse_error::ParseError;
use subrecord::{ Subrecord, SubrecordContent };
use position::{ Position, Rotation, PositionData };

pub fn read_data<R: Read + Seek>(reader: &mut R , size: usize) -> Result<Vec<u8>, ParseError> {
    let mut data: Vec<u8> = Vec::with_capacity(size);
    data.resize(size, 0);
    reader.read_exact(&mut data)?;
    Ok(data)
}

pub fn parse_record_header(data: &[u8]) -> Result<(String, usize), ParseError> {
    let name = parse_string(&data[0..4])?;
    let size = parse_long32(&data[4..8])? as usize;
    info!("parsing record: name = {}, size = {}", name, size);

    Ok((name.to_owned(), size))
}

pub fn parse_subrecord_header(data: &[u8])  -> Result<(String, usize), ParseError> {
    let name = parse_string(&data[0..4])?;
    let size = parse_long32(&data[4..8])? as usize;
    info!("reading subrecord: name = {}, size = {}", name, size);
    Ok((name.to_owned(), size))
}

pub fn parse_long32(data: &[u8]) -> Result<i32, ParseError> {
    assert!(data.len() == 4);
    Ok(data.read_i32::<LittleEndian>()?)
}

pub fn parse_long64(data: &[u8]) -> Result<i64, ParseError> {
    assert!(data.len() == 8);
    Ok(data.read_i64::<LittleEndian>()?)
}

pub fn parse_float32(data: &[u8]) -> Result<f32, ParseError> {
    assert!(data.len() == 4);
    Ok(data.read_f32::<LittleEndian>()?)
}

pub fn parse_string(data: &[u8]) -> Result<String, ParseError> {
    Ok(str::from_utf8(data)?.to_owned())
}

pub fn parse_position_data(data: &[u8]) -> Result<PositionData, ParseError> {
    let position = Position::new(
        parse_float32(&data[0..4])?,
        parse_float32(&data[4..8])?,
        parse_float32(&data[8..12])?
    );
    let rotation = Rotation::new(
        parse_float32(&data[12..16])?,
        parse_float32(&data[16..20])?,
        parse_float32(&data[20..24])?
    );
    Ok(PositionData::new(position, rotation))
}

pub fn parse_cell_subrecords(data: &[u8]) -> Result<Vec<Subrecord>, ParseError> {
    let mut subrecords: Vec<Subrecord> = Vec::new();
    let mut pos = 0;
    let mut index = 0;

    while pos < data.len() {
        let (sub_name, sub_size) = parse_subrecord_header(&data[pos..pos + 8])?;
        pos += 8;

        let content = match index {
            0 | 2 => SubrecordContent::new_string(&data[pos..pos + sub_size])?,
            1 | 4 | 5 | 6 => SubrecordContent::Skipped,
            3 => SubrecordContent::new_long32(&data[pos..pos + sub_size])?
        };
        pos += sub_size;
        index += 1;
        subrecords.push(Subrecord::new(sub_name, content));
    }
    Ok(subrecords)
}
