use std::io::{Read, Seek, SeekFrom, Error as IOError};
use std::fmt::{Debug, Formatter, Error as FmtError};

const MAGIC_NUMBER: [u8; 4] = [
    0x1a,
    0x45,
    0xdf,
    0xa3
]; 

#[derive(Debug, Clone)]
pub enum ElementKind {
    Master,
    UInt,
    SInt,
    Float,
    String,
    UTF8,
    Date,
    Binary,
}

#[derive(Clone)]
pub struct ElementData(Vec<u8>);

pub struct EBMLParser;

#[derive(Debug)]
pub struct Document {
    pub header: MasterElement,
    pub root: MasterElement,
}

#[derive(Debug, Clone)]
pub struct MasterElement {
    pub master: Element,
    pub elements: Vec<Element>,    
}

#[derive(Clone)]
pub struct Element {
    pub id: u64,
    pub size: u64,
    pub kind: ElementKind,
    pub data: ElementData,
}

impl EBMLParser {
    pub fn parse(mut r: impl Read + Seek) -> Document {
        // check magic number
        match check_magic_number(&mut r) {
            Ok(v) => {
                if !v {
                    panic!("incorrect magic number")
                }
            },
            Err(e) => panic!(e),
        }
        
        // seek back to beginning
        r.seek(SeekFrom::Start(0)).unwrap();

        // parse master element
        let header = EBMLParser::parse_master_element(&mut r);
        // parse segment
        let root = EBMLParser::parse_master_element(&mut r);

        Document {
            header: header,
            root: root,
        }
    }

    fn parse_master_element(mut r: impl Read + Seek) -> MasterElement {
        let start = r.seek(SeekFrom::Current(0)).unwrap();
        let master = EBMLParser::parse_element(&mut r);
        
        r.seek(SeekFrom::Start(start)).unwrap();
        read_vint(&mut r);
        read_vint(&mut r);
        
        let mut elements: Vec<Element> = Vec::new();
        let data_start = r.seek(SeekFrom::Current(0)).unwrap();
        let mut offset = start;

        while offset < data_start + master.size {
            let elem = EBMLParser::parse_element(&mut r);
            elements.push(elem);

            offset = r.seek(SeekFrom::Current(0)).unwrap();
        }

        MasterElement {
            master: master,
            elements: elements,
        }
    }

    fn parse_element(mut r: impl Read + Seek) -> Element {
        let id_size = count_leading_zeros(read_bytes(&mut r, 1)[0]) + 1;
        r.seek(SeekFrom::Current(-1)).unwrap();

        let id = bytes_to_uint(&read_bytes(&mut r, id_size as usize));
        let size = read_vint(&mut r);
        let data = ElementData(read_bytes(r, size as usize));

        let kind = match id {
            0x4286 | 0x42f7 | 0x42f2 |
            0x42f3 | 0x4287 | 0x4285    => ElementKind::UInt,
            0x4282                      => ElementKind::String,
            _                           => ElementKind::Master,
        };

        Element {
            id: id,
            size: size,
            kind: kind,
            data: data,
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let data_str = match self.kind {
            ElementKind::String |
            ElementKind::UTF8   => self.data.into_string(),
            ElementKind::UInt   => self.data.into_uint().to_string(),
            ElementKind::SInt   => self.data.into_int().to_string(),
            _                   => String::from("[SubElements]"),
        };
        write!(
            f, 
            "(id: 0x{:x}, size: {}, kind: {:?}, data: {})", 
            self.id, 
            self.size,
            self.kind,
            data_str,
        )
    }
}

impl ElementData {
    pub fn into_string(&self) -> String {
        bytes_to_string(&self.0)
    }

    pub fn into_uint(&self) -> u64 {
        bytes_to_uint(&self.0)
    }

    pub fn into_int(&self) -> i64 {
        bytes_to_int(&self.0)
    }
}

fn check_magic_number(mut r: impl Read) -> Result<bool, IOError> {
    let mut buf: [u8; 4] = [0; 4];
    match r.read(&mut buf) {
        Ok(size) => {
            if size != 4 {
                Ok(false)
            } else if buf != MAGIC_NUMBER {
                Ok(false)
            } else {
                Ok(true)
            }
        },
        Err(e) => Err(e),
    }
}

fn read_vint(mut r: impl Read) -> u64 {
    let mut buf = vec![0; 1];
    r.read_exact(&mut buf).unwrap();
    let count = 
        (count_leading_zeros(buf[0] as u8) + 1) as usize;

    if count > 1 {
        let mut tmp = vec![0; count - 1];
        r.read_exact(&mut tmp).unwrap();

        buf.append(&mut tmp);
    }

    let bitmask = 2u8.pow(8 - count as u32) - 1;
    buf[0] &= bitmask;

    bytes_to_uint(&buf)
}

fn read_bytes(mut r: impl Read, num: usize) -> Vec<u8> {
    let mut buf = vec![0; num];
    r.read_exact(&mut buf).unwrap();
    buf
}

fn bytes_to_uint(bytes: &[u8]) -> u64 {
    let mut result: u64 = 0;
    for b in bytes.iter() {
        result = (result << 8) | (*b as u64);
    }
    result
}

fn bytes_to_int(bytes: &[u8]) -> i64 {
    let mut result: i64 = if bytes[0] & 128 == 128 {
        0x7FFFFFFFFFFFFFFF
    } else {
        0
    };
    for b in bytes.iter() {
        result = (result << 8) | (*b as i64);
    }
    result
}

fn bytes_to_string(bytes: &[u8]) -> String {
    let mut s = String::new();
    for c in bytes.iter() {
        s.push(*c as char);
    }
    s
}

fn count_leading_zeros(mut byte: u8) -> u8 {
    let mut count = 0;
    while byte & 128 != 128 && byte != 0 {
        byte = byte << 1;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_to_int_test() {
        assert_eq!(bytes_to_int(&[0x7F]), 127);
        assert_eq!(bytes_to_int(&[0xFE]), -2);
        assert_eq!(bytes_to_int(&[0x00, 0x05]), 5);
    }

    #[test]
    fn bytes_to_uint_test() {
        assert_eq!(bytes_to_uint(&[0xFF]), 255);
    }
}
