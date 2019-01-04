use std::io::{Read, Seek, SeekFrom, Error as IOError};
use std::fmt::{Debug, Formatter, Error as FmtError};

const MAGIC_NUMBER: [u8; 4] = [
    0x1a,
    0x45,
    0xdf,
    0xa3
]; 

#[derive(Debug, Clone, PartialEq)]
pub enum ElementKind {
    Unknown,
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

pub struct EBMLParser<T: Read + Seek> {
    reader: T,
}

#[derive(Debug)]
pub struct Document {
    pub header: Node,
    pub root: Node,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub element: Element,
    pub children: Vec<Node>,
}

#[derive(Clone)]
pub struct Element {
    pub id: u64,
    pub size: u64,
    pub kind: ElementKind,
    pub data: ElementData,
}

impl<T: Read + Seek> EBMLParser<T> {
    pub fn new(r: T) -> EBMLParser<T> {
        EBMLParser {
            reader: r,
        }
    }

    pub fn parse(&mut self) -> Document {
        // check magic number
        match self.check_magic_number() {
            Ok(v) => {
                if !v {
                    panic!("incorrect magic number")
                }
            },
            Err(e) => panic!(e), 
        }
        
        // seek back to beginning
        self.reader.seek(SeekFrom::Start(0)).unwrap();

        // parse master element
        let header = self.build_node_tree();
        // parse segment
        let root = self.build_node_tree();

        Document {
            header: header,
            root: root,
        }
    }

    fn build_node_tree(&mut self) -> Node {
        let elem = self.parse_element();
        let mut children: Vec<Node> = Vec::new();
        
        if elem.kind == ElementKind::Master {
            let start = self.reader.seek(SeekFrom::Current(0)).unwrap();
            let mut offset = start;

            while offset < start + elem.size {
                children.push(self.build_node_tree());
                offset = self.reader.seek(SeekFrom::Current(0)).unwrap();
            }    
        }

        Node {
            element: elem,
            children: children,
        }
    }

    fn parse_element(&mut self) -> Element {
        let id_size = count_leading_zeros(read_bytes(&mut self.reader, 1)[0]) + 1;
        self.reader.seek(SeekFrom::Current(-1)).unwrap();

        let id = bytes_to_uint(&read_bytes(&mut self.reader, id_size as usize));
        let size = read_vint(&mut self.reader);

        let kind = match id {
            0xe7 | 0xab | 0xcc |
            0xd7 | 0x83 | 0xb9 |
            0x88 | 0x9c | 0x9a |
            0xb0 | 0xba | 0x9f |
            0xb3 | 0xf1 | 0xf7 |
            0x4286 | 0x42f7 | 0x42f2 |
            0x42f3 | 0x4287 | 0x4285 |
            0x53ac | 0x73c5 | 0x55aa |
            0x56aa | 0x56bb | 0x53b8 |
            0x53c0 |
            0x2ad7b1 | 0x23e383         => ElementKind::UInt,

            0xfb |
            0x75a2                      => ElementKind::SInt,

            0xb5 |
            0x4489                      => ElementKind::Float,

            0x4461                      => ElementKind::Date,

            0x86 |
            0x4282 |
            0x22b59c                    => ElementKind::String,

            0x9b |
            0x4d80 | 0x5741 | 0x536e |
            0x258688                    => ElementKind::UTF8,

            0xa3 | 0xa1 |
            0xec | 0xbf |
            0x53ab | 0x63a2             => ElementKind::Binary,

            0xa0 | 0x8e | 0xe8 |
            0xae | 0xe0 | 0xe1 |
            0xbb | 0xb7 |
            0x4dbb |
            0x1a45dfa3 | 0x18538067 |
            0x114d9b74 | 0x1549a966 |
            0x1f43b675 | 0x1654ae6b |
            0x1c53bb6b                  => ElementKind::Master,
            
            _                           => ElementKind::Unknown,
        };

        let data = if kind == ElementKind::Master {
            ElementData(Vec::new())
        } else {
            ElementData(read_bytes(&mut self.reader, size as usize))
        };


        Element {
            id: id,
            size: size,
            kind: kind,
            data: data,
        }
    }

    fn check_magic_number(&mut self) -> Result<bool, IOError> {
        let mut buf: [u8; 4] = [0; 4];
        match self.reader.read(&mut buf) {
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
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let data_str = match self.kind {
            ElementKind::String |
            ElementKind::UTF8   => self.data.into_string(),
            ElementKind::UInt   => self.data.into_uint().to_string(),
            ElementKind::SInt   => self.data.into_int().to_string(),
            _                   => String::from("[]"),
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
