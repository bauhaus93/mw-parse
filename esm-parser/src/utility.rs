use std::io::{ Read, Seek };
use std::str;

use byteorder::{ ReadBytesExt, LittleEndian };

use parse_error::ParseError;

pub fn read_record_header<R: Read + Seek>(reader: &mut R) -> Result<(String, usize), ParseError> {
    let mut buf: [u8; 16] = [0; 16];

    reader.read_exact(&mut buf)?;

    let name = str::from_utf8(&buf[..4])?;
    let size = (&buf[4..8]).read_i32::<LittleEndian>()? as usize;
    info!("reading record: name = {}, size = {}", name, size);
    
    Ok((name.to_owned(), size))
}

pub fn read_subrecord_header<R: Read + Seek>(reader: &mut R)  -> Result<(String, usize), ParseError> {
    let mut buf: [u8; 8] = [0; 8];

    reader.read_exact(&mut buf)?;

    let name = str::from_utf8(&buf[..4])?;
    let size = (&buf[4..]).read_i32::<LittleEndian>()? as usize;
    info!("reading subrecord: name = {}, size = {}", name, size);
    Ok((name.to_owned(), size))
}

pub fn read_data<R: Read + Seek>(reader: &mut R , size: usize) -> Result<Vec<u8>, ParseError> {
    let mut data: Vec<u8> = Vec::with_capacity(size);
    data.resize(size, 0);
    reader.read_exact(&mut data)?;
    Ok(data)
}

 
