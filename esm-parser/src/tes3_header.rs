use std::io::{ Read, Seek };
use std::str;
use std::fmt;

use byteorder::{ ReadBytesExt, LittleEndian };

use parse_error::ParseError;
use utility::{ read_record_header, read_subrecord_header, read_data };

pub enum FileType {
    ESP,
    ESM,
    ESS
}

pub struct TES3Header {
    version: f32,
    file_type: FileType,
    company_name: String,
    description: String,
    num_records: i32
}

impl FileType {
    
    pub fn from_num(num: i32) -> Result<FileType, ParseError> {
        match num {
            0 => Ok(FileType::ESP),
            1 => Ok(FileType::ESM),
            32 => Ok(FileType::ESS),
            n => Err(ParseError::UnknownFileType(n))
        }
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            FileType::ESP => "ESP",
            FileType::ESM => "ESM",
            FileType::ESS => "ESS"
        }
    }
}

impl TES3Header {


    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<TES3Header, ParseError> {
        let (sub_name, sub_size) = read_subrecord_header(reader)?;

        if sub_name != "HEDR" {
           return Err(ParseError::InvalidRecordName("HEDR".to_owned(), sub_name.to_owned())); 
        }
        if sub_size != 300 {
            warn!("subrecord {} has expected size {}, but was {}",
                  sub_name,
                  300,
                  sub_size);
        }

        let buf = read_data(reader, sub_size)?;
        let mut pos = 0;

        let version = (&buf[pos..pos + 4]).read_f32::<LittleEndian>()?;
        pos += 4;
        let file_type = (&buf[pos..pos + 4]).read_i32::<LittleEndian>()?;
        pos += 4;
        let company_name = str::from_utf8(&buf[pos..pos + 32])?;
        pos += 32;
        let file_description = str::from_utf8(&buf[pos..pos + 256])?;   //TODO check how to handle äöü
        
        pos += 256;
        let num_records = (&buf[pos..pos + 4]).read_i32::<LittleEndian>()?;

        let hdr = TES3Header {
            version: version,
            file_type: FileType::from_num(file_type)?,
            company_name: company_name.to_owned(),
            description: file_description.to_owned(),
            num_records: num_records,
        };

        Ok(hdr)
    }

    pub fn get_num_records(&self) -> i32 {
        self.num_records
    }
}

impl fmt::Display for TES3Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tes3 header: version = {}, file_type = {}, company name = \"{}\", description = \"{}\", num records: {}",
               self.version,
               self.file_type.to_str(),
               self.company_name,
               self.description,
               self.num_records)
    }
}
