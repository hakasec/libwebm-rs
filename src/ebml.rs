use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Error as IOError};
use std::fmt::{Debug, Formatter, Error as FmtError};

macro_rules! node_type {
    ($name:ident, $base:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name($base);

        impl NodeTrait for $name {
            #[allow(dead_code)]
            fn get_element(&self) -> Element {
                self.0.element.clone()
            }

            #[allow(dead_code)]
            fn get_children(&self) -> Vec<Node> {
                self.0.children.clone()
            }
        }
    };
}

macro_rules! filter_nodes_raw {
    ($list:expr, $id:expr) => {
        $list.into_iter()
            .filter(|node| node.element.id == $id)
    }
}

macro_rules! filter_nodes {
    ($list:expr, $id:expr) => {
        filter_nodes_raw!($list, $id).collect()
    };
    ($list:expr, $nty:ident, $id:expr) => {
        filter_nodes_raw!($list, $id)
            .map(|node| $nty(node))
            .collect::<Vec<$nty>>()
    };
}

macro_rules! find_node {
    ($list:expr, $id:expr) => {
        $list.into_iter()
            .find(|node| node.element.id == $id)
    };
    ($list:expr, $nty:ident, $id:expr) => {
        match find_node!($list, $id) {
            Some(n) => Some($nty(n)),
            None => None,
        }
    };
}

macro_rules! find_node_data {
    ($list:expr, $id:expr) => {
        match find_node!($list, $id) {
            Some(n) => Some(n.element.data),
            None => None
        }
    };
    ($list:expr, $nty:ty, $id:expr) => (
        Option<$nty>(find_node_data!($list, $id))
    );
}

#[allow(dead_code)]
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

pub struct WebmReader<T: Read + Seek> {
    reader: T,
}

#[derive(Debug)]
pub struct WebmFile {
    pub header: EBMLHeaderNode,
    pub root: SegmentNode,
}

#[derive(Debug, Clone)]
pub struct Node {
    element: Element,
    children: Vec<Node>,
}

pub trait NodeTrait {
    fn get_element(&self) -> Element;
    fn get_children(&self) -> Vec<Node>;
}

// bit of a hack, but seems to work well enough
node_type!(EBMLHeaderNode, Node);
node_type!(SegmentNode, Node);
node_type!(SeekHeadNode, Node);
node_type!(SeekNode, Node);
node_type!(InfoNode, Node);
node_type!(ClusterNode, Node);
node_type!(BlockGroupNode, Node);
node_type!(SlicesNode, Node);
node_type!(TracksNode, Node);
node_type!(TrackEntryNode, Node);
node_type!(VideoNode, Node);
node_type!(ProjectionNode, Node);
node_type!(AudioNode, Node);
node_type!(ContentEncodingsNode, Node);
node_type!(ContentEncodingNode, Node);
node_type!(ContentEncryptionNode, Node);
node_type!(ContentEncAESSettingsNode, Node);
node_type!(CuesNode, Node);
node_type!(CuePointNode, Node);
node_type!(CueTrackPositionsNode, Node);
node_type!(ChaptersNode, Node);
node_type!(EditionEntryNode, Node);
node_type!(ChapterAtomNode, Node);
node_type!(ChapterDisplayNode, Node);
node_type!(TagsNode, Node);
node_type!(TagNode, Node);
node_type!(TargetsNode, Node);
node_type!(SimpleTagNode, Node);

#[derive(Clone)]
pub struct Element {
    pub id: u64,
    pub size: u64,
    pub kind: ElementKind,
    pub data: ElementData,
}

impl<T: Read + Seek> WebmReader<T> {
    pub fn new(r: T) -> WebmReader<T> {
        WebmReader {
            reader: r,
        }
    }

    pub fn parse(&mut self) -> Result<WebmFile, ()> {
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

        Ok(WebmFile {
            header: EBMLHeaderNode(header),
            root: SegmentNode(root),
        })
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

impl WebmFile {
    pub fn open(file: File) -> WebmFile {
        WebmReader::new(file).parse().unwrap()
    }
}

impl NodeTrait for Node {
    fn get_element(&self) -> Element {
        self.element.clone()
    }

    fn get_children(&self) -> Vec<Node> {
        self.children.clone()
    }
}

impl EBMLHeaderNode {
    pub fn get_version(&self) -> u64 {
        find_node_data!(self.get_children(), 0x4286).unwrap().into_uint()
    }

    pub fn get_read_version(&self) -> u64 {
        find_node_data!(self.get_children(), 0x42f7).unwrap().into_uint()
    }

    pub fn get_max_id_length(&self) -> u64 {
        find_node_data!(self.get_children(), 0x42f2).unwrap().into_uint()
    }

    pub fn get_max_size_length(&self) -> u64 {
        find_node_data!(self.get_children(), 0x42f3).unwrap().into_uint()
    }

    pub fn get_doc_type(&self) -> String {
        find_node_data!(self.get_children(), 0x4282).unwrap().into_string()
    }

    pub fn get_doc_type_version(&self) -> u64 {
        find_node_data!(self.get_children(), 0x4287).unwrap().into_uint()
    }

    pub fn get_doc_type_read_version(&self) -> u64 {
        find_node_data!(self.get_children(), 0x4285).unwrap().into_uint()
    }
}

impl SegmentNode {
    pub fn get_seek_head_nodes(&self) -> Vec<SeekHeadNode> {
        filter_nodes!(self.get_children(), SeekHeadNode, 0x114d9b74)
    }

    pub fn get_info_nodes(&self) -> Vec<InfoNode> {
        filter_nodes!(self.get_children(), InfoNode, 0x1549a966)
    }

    pub fn get_clusters(&self) -> Vec<ClusterNode> {
        filter_nodes!(self.get_children(), ClusterNode, 0x1F43B675)
    }

    pub fn get_tracks(&self) -> Vec<TracksNode> {
        filter_nodes!(self.get_children(), TracksNode, 0x1654ae6b)
    }

    pub fn get_cues(&self) -> Vec<CuesNode> {
        filter_nodes!(self.get_children(), CuesNode, 0x1c53bb6b)
    }

    pub fn get_chapters(&self) -> Vec<ChaptersNode> {
        filter_nodes!(self.get_children(), ChaptersNode, 0x1043a770)
    }

    pub fn get_tags(&self) -> Vec<TagsNode> {
        filter_nodes!(self.get_children(), TagsNode, 0x1254c367)
    }
}

impl SeekHeadNode {
    pub fn get_seek_nodes(&self) -> Vec<SeekNode> {
        filter_nodes!(self.get_children(), SeekNode, 0x4dbb)
    }
}

impl SeekNode {
    pub fn get_seek_id(&self) -> Vec<u8> {
        find_node_data!(self.get_children(), 0x53ab).unwrap().into_vec()
    }

    pub fn get_seek_position(&self) -> u64 {
        find_node_data!(self.get_children(), 0x53ac).unwrap().into_uint()
    }
}

impl InfoNode {
    pub fn get_timestamp_scale(&self) -> u64 {
        find_node_data!(self.get_children(), 0x2ad7b1).unwrap().into_uint()
    }

    pub fn get_duration(&self) -> Option<f64> {
        match find_node_data!(self.get_children(), 0x4489) {
            Some(d) => Some(d.into_float()),
            None => None,
        }
    }

    pub fn get_date_created(&self) -> Option<i64> {
        match find_node_data!(self.get_children(), 0x4461) {
            Some(d) => Some(d.into_int()),
            None => None,
        }
    }

    pub fn get_muxing_app(&self) -> String {
        find_node_data!(self.get_children(), 0x4d80).unwrap().into_string()
    }

    pub fn get_writing_app(&self) -> String {
        find_node_data!(self.get_children(), 0x5741).unwrap().into_string()
    }
}

impl ClusterNode {
    pub fn get_timestamp(&self) -> u64 {
        find_node_data!(self.get_children(), 0xe7).unwrap().into_uint()
    }

    pub fn get_prev_size(&self) -> Option<u64> {
        match find_node_data!(self.get_children(), 0xab) {
            Some(d) => Some(d.into_uint()),
            None => None,
        }
    }

    pub fn get_simple_blocks(&self) -> Vec<Node> {
        filter_nodes!(self.get_children(), 0xa3)
    }

    pub fn get_block_groups(&self) -> Vec<BlockGroupNode> {
        filter_nodes!(self.get_children(), BlockGroupNode, 0xa0)
    }
}

impl BlockGroupNode {
    pub fn get_block_duration(&self) -> Option<u64> {
        match find_node_data!(self.get_children(), 0x9b) {
            Some(d) => Some(d.into_uint()),
            None => None,
        }
    }

    pub fn get_reference_blocks(&self) -> Vec<i64> {
        filter_nodes_raw!(self.get_children(), 0xfb)
            .map(|node| node.element.data.into_int())
            .collect()
    }

    pub fn get_discard_padding(&self) -> Option<i64> {
        match find_node_data!(self.get_children(), 0x75a2) {
            Some(d) => Some(d.into_int()),
            None => None,
        }
    }

    pub fn get_slices(&self) -> Option<SlicesNode> {
        find_node!(self.get_children(), SlicesNode, 0x8e)
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let data_str = match self.kind {
            ElementKind::String |
            ElementKind::UTF8   => self.data.into_string(),
            ElementKind::UInt   => self.data.into_uint().to_string(),
            ElementKind::SInt |
            ElementKind::Date   => self.data.into_int().to_string(),
            _                   => format!("{:x?}", self.data.into_vec()),
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

    pub fn into_float(&self) -> f64 {
        bytes_to_float(&self.0)
    }

    pub fn into_vec(&self) -> Vec<u8> {
        self.0.clone()
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

fn bytes_to_float(bytes: &[u8]) -> f64 {
    let bits = bytes_to_uint(bytes);
    if bytes.len() > 4 {
        f64::from_bits(bits)
    } else {
        f32::from_bits(bits as u32) as f64
    }
}

fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn count_leading_zeros(mut byte: u8) -> u8 {
    if byte == 0x0 {
        8
    } else {
        let mut count = 0;
        while byte & 128 != 128 {
            byte = byte << 1;
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_int() {
        assert_eq!(bytes_to_int(&[0x7F]), 127);
        assert_eq!(bytes_to_int(&[0xFE]), -2);
        assert_eq!(bytes_to_int(&[0x00, 0x05]), 5);
    }

    #[test]
    fn test_bytes_to_uint() {
        assert_eq!(bytes_to_uint(&[0xFF]), 255);
    }

    #[test]
    fn test_count_leading_zeros() {
        assert_eq!(count_leading_zeros(0x81), 0);
        assert_eq!(count_leading_zeros(0xe), 4);
        assert_eq!(count_leading_zeros(0x0), 8);
        assert_eq!(count_leading_zeros(0x1), 7);
    }

    #[test]
    fn test_bytes_to_string() {
        assert_eq!(bytes_to_string(&[0x41, 0x42, 0x43]), "ABC");
        assert_eq!(bytes_to_string(&[0xe4, 0xbd, 0x95]), "何");
    }

    #[test]
    fn test_bytes_to_float() {
        assert_eq!(
            bytes_to_float(&[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), 
            12.5
        );
        assert_eq!(bytes_to_float(&[0x47, 0xae, 0x88, 0x80]), 89361.0);
    }
}
