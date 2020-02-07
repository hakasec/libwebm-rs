use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Error as IOError};
use std::fmt::{Debug, Formatter, Error as FmtError};

// Generate a node type from some base node
macro_rules! node_type {
    ($name:ident, $base:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name($base);

        impl $name {
            #[allow(dead_code)]
            pub fn get_element(&self) -> Element {
                self.0.element.clone()
            }

            #[allow(dead_code)]
            pub fn get_children(&self) -> Vec<Node> {
                self.0.children.clone()
            }
        }
    };
}

// Filter nodes by ID from list and don't collect
macro_rules! filter_nodes_raw {
    ($list:expr, $id:expr) => {
        $list.into_iter()
            .filter(|node| node.element.id == $id)
    }
}

// Filter nodes and collect from lists
macro_rules! filter_nodes {
    ($list:expr, $id:expr) => {
        filter_nodes_raw!($list, $id).collect()
    };
    // Convert to another node type before collection
    ($list:expr, $nty:ident, $id:expr) => {
        filter_nodes_raw!($list, $id)
            .map(|node| $nty(node))
            .collect::<Vec<$nty>>()
    };
}

// Find a single node in a list by it's ID
macro_rules! find_node {
    ($list:expr, $id:expr) => {
        $list.into_iter()
            .find(|node| node.element.id == $id)
    };
    // Find and convert to a given node type
    ($list:expr, $nty:ident, $id:expr) => {
        match find_node!($list, $id) {
            Some(n) => Some($nty(n)),
            None => None,
        }
    };
}

// Return the data from a node in list
macro_rules! find_node_data {
    ($list:expr, $id:expr) => {
        match find_node!($list, $id) {
            Some(n) => Some(n.element.data),
            None => None
        }
    };
}

// Return a node's data and call into to convert type. Wrap result in Option
macro_rules! find_node_data_opt {
    ($list:expr, $id:expr) => {
        match find_node_data!($list, $id) {
            Some(d) => Some(d.into()),
            None => None,
        }
    };
}

// Return a node's data, unwrap, and convert
macro_rules! find_node_data_mand {
    ($list:expr, $id:expr) => {
        find_node_data!($list, $id).unwrap().into()
    };
}

// Magic number for webm files
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

pub struct NodeInfo<'a> {
    id: u64,
    name: &'a str,
}

const NODE_INFOS: [NodeInfo<'static>; 122] = [
    NodeInfo { id: 0x1a45dfa3, name: "EBMLHeaderNode" },
    NodeInfo { id: 0x18538067, name: "SegmentNode" },
    NodeInfo { id: 0x114d9b74, name: "SeekHeadNode" },
    NodeInfo { id: 0x4dbb, name: "SeekNode" },
    NodeInfo { id: 0x1549a966, name: "InfoNode" },
    NodeInfo { id: 0x1f43b675, name: "ClusterNode" },
    NodeInfo { id: 0xa0, name: "BlockGroupNode" },
    NodeInfo { id: 0x8e, name: "SlicesNode" },
    NodeInfo { id: 0x1654ae6b, name: "TracksNode" },
    NodeInfo { id: 0xae, name: "TrackEntryNode" },
    NodeInfo { id: 0xe0, name: "VideoNode" },
    NodeInfo { id: 0xe1, name: "AudioNode" },
    NodeInfo { id: 0x6d80, name: "ContentEncodingsNode" },
    NodeInfo { id: 0x6240, name: "ContentEncodingNode" },
    NodeInfo { id: 0x5035, name: "ContentEncryptionNode" },
    NodeInfo { id: 0x47e7, name: "ContentEncAESSettingsNode" },
    NodeInfo { id: 0x1c53bb6b, name: "CuesNode" },
    NodeInfo { id: 0xbb, name: "CuePointNode" },
    NodeInfo { id: 0xb7, name: "CueTrackPositionsNode" },
    NodeInfo { id: 0x1043a770, name: "ChaptersNode" },
    NodeInfo { id: 0x45b9, name: "EditionEntryNode" },
    NodeInfo { id: 0xb6, name: "ChapterAtomNode" },
    NodeInfo { id: 0x80, name: "ChapterDisplayNode" },
    NodeInfo { id: 0x1254c367, name: "TagsNode" },
    NodeInfo { id: 0x7373, name: "TagNode" },
    NodeInfo { id: 0x63c0, name: "TargetsNode" },
    NodeInfo { id: 0x67c8, name: "SimpleTagNode" },

    // non-master nodes
    // ebml header
    NodeInfo { id: 0x4286, name: "EBMLVersion" },
    NodeInfo { id: 0x42f7, name: "EBMLReadVersion" },
    NodeInfo { id: 0x42f2, name: "EBMLMaxIDLength" },
    NodeInfo { id: 0x42f3, name: "EBMLMaxSizeLength" },
    NodeInfo { id: 0x4282, name: "DocType" },
    NodeInfo { id: 0x4287, name: "DocTypeVersion" },
    NodeInfo { id: 0x4285, name: "DocTypeReadVersion" },
    NodeInfo { id: 0xbf, name: "CRC-32" },
    NodeInfo { id: 0xec, name: "Void" },
    NodeInfo { id: 0x1b538667, name: "SignatureSlot" },
    NodeInfo { id: 0x7e8a, name: "SignatureAlgo" },
    NodeInfo { id: 0x7e9a, name: "SignatureHash" },
    NodeInfo { id: 0x7ea5, name: "SignaturePublicKey" },
    NodeInfo { id: 0x7eb5, name: "Signature" },
    NodeInfo { id: 0x7e5b, name: "SignatureElements" },
    NodeInfo { id: 0x7e7b, name: "SignatureElementList" },
    NodeInfo { id: 0x6532, name: "SignedElement" },

    // everything else
    NodeInfo { id: 0x53ab, name: "SeekID" },
    NodeInfo { id: 0x53ac, name: "SeekPosition" },
    NodeInfo { id: 0x2ad7b1, name: "TimestampScale" },
    NodeInfo { id: 0x4489, name: "Duration" },
    NodeInfo { id: 0x4461, name: "DateUTC" },
    NodeInfo { id: 0x4d80, name: "MuxingApp" },
    NodeInfo { id: 0x5741, name: "WritingApp" },
    NodeInfo { id: 0xe7, name: "Timestamp" },
    NodeInfo { id: 0xab, name: "PrevSize" },
    NodeInfo { id: 0xa3, name: "SimpleBlock" },
    NodeInfo { id: 0xa1, name: "Block" },
    NodeInfo { id: 0x9b, name: "BlockDuration" },
    NodeInfo { id: 0xfb, name: "ReferenceBlock" },
    NodeInfo { id: 0x75a2, name: "DiscardPadding" },
    NodeInfo { id: 0xcc, name: "LaceNumber" },
    NodeInfo { id: 0xd7, name: "TrackNumber" },
    NodeInfo { id: 0x73c5, name: "TrackUID" },
    NodeInfo { id: 0x83, name: "TrackType" },
    NodeInfo { id: 0xb9, name: "FlagEnabled" },
    NodeInfo { id: 0x88, name: "FlagDefault" },
    NodeInfo { id: 0x55aa, name: "FlagForced" },
    NodeInfo { id: 0x9c, name: "FlagLacing" },
    NodeInfo { id: 0x23e383, name: "DefaultDuration" },
    NodeInfo { id: 0x536e, name: "Name" },
    NodeInfo { id: 0x22b59c, name: "Language" },
    NodeInfo { id: 0x86, name: "CodecID" },
    NodeInfo { id: 0x63a2, name: "CodecPrivate" },
    NodeInfo { id: 0x258688, name: "CodecName" },
    NodeInfo { id: 0x56aa, name: "CodecDelay" },
    NodeInfo { id: 0x56bb, name: "SeekPreRoll" },
    NodeInfo { id: 0x9a, name: "FlagInterlaced" },
    NodeInfo { id: 0x53b8, name: "StereoMode" },
    NodeInfo { id: 0x53c0, name: "AlphaMode" },
    NodeInfo { id: 0xb0, name: "PixelWidth" },
    NodeInfo { id: 0xba, name: "PixelHeight" },
    NodeInfo { id: 0x54aa, name: "PixelCropBottom" },
    NodeInfo { id: 0x54bb, name: "PixelCropTop" },
    NodeInfo { id: 0x54cc, name: "PixelCropLeft" },
    NodeInfo { id: 0x54dd, name: "PixelCropRight" },
    NodeInfo { id: 0x54b0, name: "DisplayWidth" },
    NodeInfo { id: 0x54ba, name: "DisplayHeight" },
    NodeInfo { id: 0x54b2, name: "DisplayUnit" },
    NodeInfo { id: 0x54b3, name: "AspectRatioType" },
    NodeInfo { id: 0x7671, name: "ProjectionType" },
    NodeInfo { id: 0x7672, name: "ProjectionPrivate" },
    NodeInfo { id: 0x7673, name: "ProjectionPoseYaw" },
    NodeInfo { id: 0x7674, name: "ProjectionPosePitch" },
    NodeInfo { id: 0x7675, name: "ProjectionPoseRoll" },
    NodeInfo { id: 0xb5, name: "SamplingFrequency" },
    NodeInfo { id: 0x78b5, name: "OutputSamplingFrequency" },
    NodeInfo { id: 0x9f, name: "Channels" },
    NodeInfo { id: 0x6264, name: "BitDepth" },
    NodeInfo { id: 0x5031, name: "ContentEncodingOrder" },
    NodeInfo { id: 0x5032, name: "ContentEncodingScope" },
    NodeInfo { id: 0x5033, name: "ContentEncodingType" },
    NodeInfo { id: 0x47e1, name: "ContentEncAlgo" },
    NodeInfo { id: 0x47e2, name: "ContentEncKeyID" },
    NodeInfo { id: 0x47e8, name: "AESSettingsCipherMode" },
    NodeInfo { id: 0xb3, name: "CueTime" },
    NodeInfo { id: 0xf7, name: "CueTrack" },
    NodeInfo { id: 0xf1, name: "CueClusterPosition" },
    NodeInfo { id: 0x5378, name: "CueBlockNumber" },
    NodeInfo { id: 0x73c4, name: "ChapterUID" },
    NodeInfo { id: 0x5654, name: "ChapterStringUID" },
    NodeInfo { id: 0x91, name: "ChapterTimeStart" },
    NodeInfo { id: 0x85, name: "ChapString" },
    NodeInfo { id: 0x437c, name: "ChapLanguage" },
    NodeInfo { id: 0x68ca, name: "TargetTypeValue" },
    NodeInfo { id: 0x63ca, name: "TargetType" },
    NodeInfo { id: 0x63c5, name: "TagTrackUID" },
    NodeInfo { id: 0x45a3, name: "TagName" },
    NodeInfo { id: 0x447a, name: "TagLanguage" },
    NodeInfo { id: 0x4484, name: "TagDefault" },
    NodeInfo { id: 0x4487, name: "TagString" },
    NodeInfo { id: 0x4485, name: "TagBinary" },  
    NodeInfo { id: 0x23314f, name: "TrackTimestampScale" },
    NodeInfo { id: 0xa7, name: "Position" },
    NodeInfo { id: 0x73a4, name: "SegmentUID" },
];

fn get_node_info<'a>(id: u64) -> Option<&'a NodeInfo<'static>> {
    NODE_INFOS.iter().find(|&info| info.id == id)
}

#[derive(Clone)]
pub struct Node {
    element: Element,
    children: Vec<Node>,
}

impl Node {
    #[allow(dead_code)]
    fn get_element(&self) -> Element {
        self.element.clone()
    }

    #[allow(dead_code)]
    fn get_children(&self) -> Vec<Node> {
        self.children.clone()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        let name = match get_node_info(self.element.id) {
            Some(info) => info.name,
            None => "Node",
        };

        let mut dbg = f.debug_struct(name);
        dbg.field("element", &self.element);

        // ignore children if empty
        if self.children.len() > 0 {
            dbg.field("children", &self.children);
        }

        dbg.finish()
    }
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
        let header = EBMLHeaderNode(self.build_node_tree());
        // parse segments
        let root = SegmentNode(self.build_node_tree());
        Ok(WebmFile {
            header: header,
            root: root,
        })
    }

    fn build_node_tree(&mut self) -> Node {
        // parse next element
        let elem = self.parse_element();
        let mut children: Vec<Node> = Vec::new();
        
        // if elem is a master, build child node tree
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
        // get the ID size
        let id_size = count_leading_zeros(read_bytes(&mut self.reader, 1)[0]) + 1;
        // seek back one byte
        self.reader.seek(SeekFrom::Current(-1)).unwrap();

        // read ID
        let id = bytes_to_uint(&read_bytes(&mut self.reader, id_size as usize));
        // read next vint
        let size = read_vint(&mut self.reader);

        // Match all IDs to a given element type
        let kind = match id {
            0xe7 | 0xab | 0xcc |
            0xd7 | 0x83 | 0xb9 |
            0x88 | 0x9c | 0x9a |
            0xb0 | 0xba | 0x9f |
            0xb3 | 0xf1 | 0xf7 |
            0xa7 |
            0x4286 | 0x42f7 | 0x42f2 |
            0x42f3 | 0x4287 | 0x4285 |
            0x53ac | 0x73c5 | 0x55aa |
            0x56aa | 0x56bb | 0x53b8 |
            0x53c0 | 0x5378 |
            0x2ad7b1 | 0x23e383         => ElementKind::UInt,

            0xfb |
            0x75a2                      => ElementKind::SInt,

            0xb5 |
            0x4489 |
            0x23314f                    => ElementKind::Float,

            0x4461                      => ElementKind::Date,

            0x86 |
            0x4282 |
            0x22b59c                    => ElementKind::String,

            0x9b |
            0x4d80 | 0x5741 | 0x536e |
            0x258688                    => ElementKind::UTF8,

            0xa3 | 0xa1 |
            0xec | 0xbf |
            0x53ab | 0x63a2 | 0x73a4    => ElementKind::Binary,

            0xa0 | 0x8e | 0xe8 |
            0xae | 0xe0 | 0xe1 |
            0xbb | 0xb7 |
            0x4dbb |
            0x1a45dfa3 | 0x18538067 |
            0x114d9b74 | 0x1549a966 |
            0x1f43b675 | 0x1654ae6b |
            0x1c53bb6b                  => ElementKind::Master,

            // Failsafe, we can check for these in testing
            _                           => ElementKind::Unknown,
        };

        // assign the element data
        // if master, ignore data
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

impl EBMLHeaderNode {
    pub fn get_version(&self) -> u64 {
        find_node_data!(self.get_children(), 0x4286).unwrap().into()
    }

    pub fn get_read_version(&self) -> u64 {
        find_node_data!(self.get_children(), 0x42f7).unwrap().into()
    }

    pub fn get_max_id_length(&self) -> u64 {
        find_node_data!(self.get_children(), 0x42f2).unwrap().into()
    }

    pub fn get_max_size_length(&self) -> u64 {
        find_node_data!(self.get_children(), 0x42f3).unwrap().into()
    }

    pub fn get_doc_type(&self) -> String {
        find_node_data!(self.get_children(), 0x4282).unwrap().into()
    }

    pub fn get_doc_type_version(&self) -> u64 {
        find_node_data!(self.get_children(), 0x4287).unwrap().into()
    }

    pub fn get_doc_type_read_version(&self) -> u64 {
        find_node_data!(self.get_children(), 0x4285).unwrap().into()
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
        find_node_data!(self.get_children(), 0x53ab).unwrap().into()
    }

    pub fn get_seek_position(&self) -> u64 {
        find_node_data!(self.get_children(), 0x53ac).unwrap().into()
    }
}

impl InfoNode {
    pub fn get_timestamp_scale(&self) -> u64 {
        find_node_data!(self.get_children(), 0x2ad7b1).unwrap().into()
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
        find_node_data!(self.get_children(), 0x4d80).unwrap().into()
    }

    pub fn get_writing_app(&self) -> String {
        find_node_data!(self.get_children(), 0x5741).unwrap().into()
    }
}

impl ClusterNode {
    pub fn get_timestamp(&self) -> u64 {
        find_node_data!(self.get_children(), 0xe7).unwrap().into()
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

impl TracksNode {
    pub fn get_track_entries(&self) -> Vec<TrackEntryNode> {
        filter_nodes!(self.get_children(), TrackEntryNode, 0xae)
    }
}

impl TrackEntryNode {
    pub fn get_track_number(&self) -> u64 {
        find_node_data!(self.get_children(), 0xd7).unwrap().into()
    }

    pub fn get_track_uid(&self) -> u64 {
        find_node_data!(self.get_children(), 0x73c5).unwrap().into()
    }

    pub fn get_track_type(&self) -> u64 {
        find_node_data!(self.get_children(), 0x83).unwrap().into()
    }

    pub fn is_enabled(&self) -> bool {
        find_node_data!(self.get_children(), 0xb9).unwrap().into()
    }

    pub fn is_default(&self) -> bool {
        find_node_data!(self.get_children(), 0x88).unwrap().into()
    }

    pub fn is_forced(&self) -> bool {
        find_node_data!(self.get_children(), 0x55aa).unwrap().into()
    }

    pub fn is_laced(&self) -> bool {
        find_node_data!(self.get_children(), 0x9c).unwrap().into()
    }

    pub fn get_default_duration(&self) -> Option<u64> {
        match find_node_data!(self.get_children(), 0x23e383) {
            Some(d) => Some(d.into()),
            None => None,
        }
    }

    pub fn get_name(&self) -> Option<String> {
        match find_node_data!(self.get_children(), 0x536e) {
            Some(d) => Some(d.into()),
            None => None,
        }
    }

    pub fn get_language(&self) -> Option<String> {
        match find_node_data!(self.get_children(), 0x22b59c) {
            Some(d) => Some(d.into()),
            None => None,
        }
    }

    pub fn get_codec_id(&self) -> String {
        find_node_data!(self.get_children(), 0x86).unwrap().into()
    }

    pub fn get_codec_private(&self) -> Option<Vec<u8>> {
        match find_node_data!(self.get_children(), 0x63a2) {
            Some(d) => Some(d.into()),
            None => None,
        }
    }

    pub fn get_codec_name(&self) -> Option<String> {
        match find_node_data!(self.get_children(), 0x258688) {
            Some(d) => Some(d.into()),
            None => None,
        }
    }

    pub fn get_codec_delay(&self) -> Option<u64> {
        match find_node_data!(self.get_children(), 0x56aa) {
            Some(d) => Some(d.into()),
            None => None,
        }
    }

    pub fn get_seek_preroll(&self) -> u64 {
        find_node_data!(self.get_children(), 0x56bb).unwrap().into()
    }

    pub fn get_video_settings(&self) -> Option<VideoNode> {
        find_node!(self.get_children(), VideoNode, 0xe0)
    }

    pub fn get_audio_settings(&self) -> Option<AudioNode> {
        find_node!(self.get_children(), AudioNode, 0xe1)
    }

    pub fn get_encoding_settings(&self) -> Option<ContentEncodingsNode> {
        find_node!(self.get_children(), ContentEncodingsNode, 0x6d80)
    }
}

impl VideoNode {
    pub fn get_interlacing_flag(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x9a)
    }

    pub fn get_stereo_mode(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x53b8)
    }

    pub fn get_alpha_mode(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x53c0)
    }

    pub fn get_pixel_width(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0xb0)
    }

    pub fn get_pixel_height(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0xba)
    }

    pub fn get_pixel_crop_bottom(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x54aa)
    }

    pub fn get_pixel_crop_top(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x54bb)
    }

    pub fn get_pixel_crop_left(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x54cc)
    }

    pub fn get_pixel_crop_right(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x54dd)
    }

    pub fn get_display_width(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x54b0)
    }

    pub fn get_display_height(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x54ba)
    }

    pub fn get_display_unit(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x54b2)
    }

    pub fn get_aspect_ratio_type(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x54b3)
    }
}

impl ProjectionNode {
    pub fn get_type(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x7671)
    }

    pub fn get_private(&self) -> Option<Vec<u8>> {
        find_node_data_opt!(self.get_children(), 0x7672)
    }

    pub fn get_pose_yaw(&self) -> f64 {
        find_node_data_mand!(self.get_children(), 0x7673)
    }

    pub fn get_pose_pitch(&self) -> f64 {
        find_node_data_mand!(self.get_children(), 0x7674)
    }

    pub fn get_pose_roll(&self) -> f64 {
        find_node_data_mand!(self.get_children(), 0x7675)
    }
}

impl AudioNode {
    pub fn get_sampling_frequency(&self) -> f64 {
        find_node_data_mand!(self.get_children(), 0xb5)
    }

    pub fn get_output_sampling_frequency(&self) -> Option<f64> {
        find_node_data_opt!(self.get_children(), 0x78b5)
    }

    pub fn get_num_channels(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x9f)
    }

    pub fn get_bit_depth(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x6264)
    }
}

impl ContentEncodingsNode {
    pub fn get_encodings(&self) -> Vec<ContentEncodingNode> {
        filter_nodes!(self.get_children(), ContentEncodingNode, 0x6240)
    }
}

impl ContentEncodingNode {
    pub fn get_order(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x5031)
    }

    pub fn get_scope(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x5032)
    }

    pub fn get_type(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x5033)
    }

    pub fn get_encryption_node(&self) -> ContentEncryptionNode {
        find_node!(self.get_children(), ContentEncryptionNode, 0x5035).unwrap()
    }
}

impl ContentEncryptionNode {
    pub fn get_algorithm_type(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x47e1)
    }

    pub fn get_key_id(&self) -> Option<Vec<u8>> {
        find_node_data_opt!(self.get_children(), 0x47e2)
    }

    pub fn get_aes_settings(&self) -> Option<ContentEncAESSettingsNode> {
        find_node!(self.get_children(), ContentEncAESSettingsNode, 0x47e7)
    }
}

impl ContentEncAESSettingsNode {
    pub fn get_mode(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x47e8)
    }
}

impl CuesNode {
    pub fn get_cue_points(&self) -> Vec<CuePointNode> {
        filter_nodes!(self.get_children(), CuePointNode, 0xbb)
    }
}

impl CuePointNode {
    pub fn get_time(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0xb3)
    }

    pub fn get_positions(&self) -> Vec<CueTrackPositionsNode> {
        filter_nodes!(self.get_children(), CueTrackPositionsNode, 0xb7)
    }
}

impl CueTrackPositionsNode {
    pub fn get_track(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0xf7)
    }

    pub fn get_cluster_position(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0xf1)
    }

    pub fn get_block_number(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x5378)
    }
}

impl ChaptersNode {
    pub fn get_edition_entries(&self) -> Vec<EditionEntryNode> {
        filter_nodes!(self.get_children(), EditionEntryNode, 0x45b9)
    }
}

impl EditionEntryNode {
    pub fn get_chapter_atoms(&self) -> Vec<ChapterAtomNode> {
        filter_nodes!(self.get_children(), ChapterAtomNode, 0xb6)
    }
}

impl ChapterAtomNode {
    pub fn get_uid(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x73c4)
    }

    pub fn get_string_uid(&self) -> Option<String> {
        find_node_data_opt!(self.get_children(), 0x5654)
    }

    pub fn get_start_time(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x91)
    }

    pub fn get_displays(&self) -> Vec<ChapterDisplayNode> {
        filter_nodes!(self.get_children(), ChapterDisplayNode, 0x80)
    }
}

impl ChapterDisplayNode {
    pub fn get_string(&self) -> String {
        find_node_data_mand!(self.get_children(), 0x85)
    }

    pub fn get_languages(&self) -> Vec<String> {
        filter_nodes_raw!(self.get_children(), 0x437c)
            .map(|node| node.element.data.into_string())
            .collect()
    }
}

impl TagsNode {
    pub fn get_tags(&self) -> Vec<TagNode> {
        filter_nodes!(self.get_children(), TagNode, 0x7373)
    }
}

impl TagNode {
    pub fn get_targets(&self) -> TargetsNode {
        find_node!(self.get_children(), TargetsNode, 0x63c0).unwrap()
    }
}

impl TargetsNode {
    pub fn get_type_value(&self) -> Option<u64> {
        find_node_data_opt!(self.get_children(), 0x68ca)
    }

    pub fn get_type(&self) -> Option<String> {
        find_node_data_opt!(self.get_children(), 0x63ca)
    }

    pub fn get_track_uid(&self) -> Vec<u64> {
        filter_nodes_raw!(self.get_children(), 0x63c5)
            .map(|node| node.element.data.into_uint())
            .collect()
    }
}

impl SimpleTagNode {
    pub fn get_name(&self) -> String {
        find_node_data_mand!(self.get_children(), 0x45a3)
    }

    pub fn get_language(&self) -> String {
        find_node_data_mand!(self.get_children(), 0x447a)
    }

    pub fn get_default(&self) -> u64 {
        find_node_data_mand!(self.get_children(), 0x4484)
    }

    pub fn get_string(&self) -> Option<String> {
        find_node_data_opt!(self.get_children(), 0x4487)
    }

    pub fn get_binary(&self) -> Option<Vec<u8>> {
        find_node_data_opt!(self.get_children(), 0x4485)
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
            ElementKind::Float  => self.data.into_float().to_string(),
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

impl Into<String> for ElementData {
    fn into(self) -> String {
        self.into_string()
    }
}

impl Into<u64> for ElementData {
    fn into(self) -> u64 {
        self.into_uint()
    }
}

impl Into<i64> for ElementData {
    fn into(self) -> i64 {
        self.into_int()
    }
}

impl Into<f64> for ElementData {
    fn into(self) -> f64 {
        self.into_float()
    }
}

impl Into<Vec<u8>> for ElementData {
    fn into(self) -> Vec<u8> {
        self.into_vec()
    }
}

impl Into<bool> for ElementData {
    fn into(self) -> bool {
        self.into_int() == 1
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
