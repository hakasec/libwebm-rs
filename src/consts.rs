use crate::ebml::NodeInfo;

// Magic number for webm files
#[allow(dead_code)]
pub const MAGIC_NUMBER: [u8; 4] = [
    0x1a,
    0x45,
    0xdf,
    0xa3
];

pub const ID_EBMLHEADERNODE: u64 = 0x1a45dfa3;
pub const ID_SEGMENTNODE: u64 = 0x18538067;
pub const ID_SEEKHEADNODE: u64 = 0x114d9b74;
pub const ID_SEEKNODE: u64 = 0x4dbb;
pub const ID_INFONODE: u64 = 0x1549a966;
pub const ID_CLUSTERNODE: u64 = 0x1f43b675;
pub const ID_BLOCKGROUPNODE: u64 = 0xa0;
pub const ID_SLICESNODE: u64 = 0x8e;
pub const ID_TRACKSNODE: u64 = 0x1654ae6b;
pub const ID_TRACKENTRYNODE: u64 = 0xae;
pub const ID_VIDEONODE: u64 = 0xe0;
pub const ID_AUDIONODE: u64 = 0xe1;
pub const ID_CONTENTENCODINGSNODE: u64 = 0x6d80;
pub const ID_CONTENTENCODINGNODE: u64 = 0x6240;
pub const ID_CONTENTENCRYPTIONNODE: u64 = 0x5035;
pub const ID_CONTENTENCAESSETTINGSNODE: u64 = 0x47e7;
pub const ID_CUESNODE: u64 = 0x1c53bb6b;
pub const ID_CUEPOINTNODE: u64 = 0xbb;
pub const ID_CUETRACKPOSITIONSNODE: u64 = 0xb7;
pub const ID_CHAPTERSNODE: u64 = 0x1043a770;
pub const ID_EDITIONENTRYNODE: u64 = 0x45b9;
pub const ID_CHAPTERATOMNODE: u64 = 0xb6;
pub const ID_CHAPTERDISPLAYNODE: u64 = 0x80;
pub const ID_TAGSNODE: u64 = 0x1254c367;
pub const ID_TAGNODE: u64 = 0x7373;
pub const ID_TARGETSNODE: u64 = 0x63c0;
pub const ID_SIMPLETAGNODE: u64 = 0x67c8;
pub const ID_EBMLVERSION: u64 = 0x4286;
pub const ID_EBMLREADVERSION: u64 = 0x42f7;
pub const ID_EBMLMAXIDLENGTH: u64 = 0x42f2;
pub const ID_EBMLMAXSIZELENGTH: u64 = 0x42f3;
pub const ID_DOCTYPE: u64 = 0x4282;
pub const ID_DOCTYPEVERSION: u64 = 0x4287;
pub const ID_DOCTYPEREADVERSION: u64 = 0x4285;
pub const ID_CRC32: u64 = 0xbf;
pub const ID_VOID: u64 = 0xec;
pub const ID_SIGNATURESLOT: u64 = 0x1b538667;
pub const ID_SIGNATUREALGO: u64 = 0x7e8a;
pub const ID_SIGNATUREHASH: u64 = 0x7e9a;
pub const ID_SIGNATUREPUBLICKEY: u64 = 0x7ea5;
pub const ID_SIGNATURE: u64 = 0x7eb5;
pub const ID_SIGNATUREELEMENTS: u64 = 0x7e5b;
pub const ID_SIGNATUREELEMENTLIST: u64 = 0x7e7b;
pub const ID_SIGNEDELEMENT: u64 = 0x6532;
pub const ID_SEEKID: u64 = 0x53ab;
pub const ID_SEEKPOSITION: u64 = 0x53ac;
pub const ID_TIMESTAMPSCALE: u64 = 0x2ad7b1;
pub const ID_DURATION: u64 = 0x4489;
pub const ID_DATEUTC: u64 = 0x4461;
pub const ID_MUXINGAPP: u64 = 0x4d80;
pub const ID_WRITINGAPP: u64 = 0x5741;
pub const ID_TIMESTAMP: u64 = 0xe7;
pub const ID_PREVSIZE: u64 = 0xab;
pub const ID_SIMPLEBLOCK: u64 = 0xa3;
pub const ID_BLOCK: u64 = 0xa1;
pub const ID_BLOCKDURATION: u64 = 0x9b;
pub const ID_REFERENCEBLOCK: u64 = 0xfb;
pub const ID_DISCARDPADDING: u64 = 0x75a2;
pub const ID_LACENUMBER: u64 = 0xcc;
pub const ID_TRACKNUMBER: u64 = 0xd7;
pub const ID_TRACKUID: u64 = 0x73c5;
pub const ID_TRACKTYPE: u64 = 0x83;
pub const ID_FLAGENABLED: u64 = 0xb9;
pub const ID_FLAGDEFAULT: u64 = 0x88;
pub const ID_FLAGFORCED: u64 = 0x55aa;
pub const ID_FLAGLACING: u64 = 0x9c;
pub const ID_DEFAULTDURATION: u64 = 0x23e383;
pub const ID_NAME: u64 = 0x536e;
pub const ID_LANGUAGE: u64 = 0x22b59c;
pub const ID_CODECID: u64 = 0x86;
pub const ID_CODECPRIVATE: u64 = 0x63a2;
pub const ID_CODECNAME: u64 = 0x258688;
pub const ID_CODECDELAY: u64 = 0x56aa;
pub const ID_SEEKPREROLL: u64 = 0x56bb;
pub const ID_FLAGINTERLACED: u64 = 0x9a;
pub const ID_STEREOMODE: u64 = 0x53b8;
pub const ID_ALPHAMODE: u64 = 0x53c0;
pub const ID_PIXELWIDTH: u64 = 0xb0;
pub const ID_PIXELHEIGHT: u64 = 0xba;
pub const ID_PIXELCROPBOTTOM: u64 = 0x54aa;
pub const ID_PIXELCROPTOP: u64 = 0x54bb;
pub const ID_PIXELCROPLEFT: u64 = 0x54cc;
pub const ID_PIXELCROPRIGHT: u64 = 0x54dd;
pub const ID_DISPLAYWIDTH: u64 = 0x54b0;
pub const ID_DISPLAYHEIGHT: u64 = 0x54ba;
pub const ID_DISPLAYUNIT: u64 = 0x54b2;
pub const ID_ASPECTRATIOTYPE: u64 = 0x54b3;
pub const ID_PROJECTIONTYPE: u64 = 0x7671;
pub const ID_PROJECTIONPRIVATE: u64 = 0x7672;
pub const ID_PROJECTIONPOSEYAW: u64 = 0x7673;
pub const ID_PROJECTIONPOSEPITCH: u64 = 0x7674;
pub const ID_PROJECTIONPOSEROLL: u64 = 0x7675;
pub const ID_SAMPLINGFREQUENCY: u64 = 0xb5;
pub const ID_OUTPUTSAMPLINGFREQUENCY: u64 = 0x78b5;
pub const ID_CHANNELS: u64 = 0x9f;
pub const ID_BITDEPTH: u64 = 0x6264;
pub const ID_CONTENTENCODINGORDER: u64 = 0x5031;
pub const ID_CONTENTENCODINGSCOPE: u64 = 0x5032;
pub const ID_CONTENTENCODINGTYPE: u64 = 0x5033;
pub const ID_CONTENTENCALGO: u64 = 0x47e1;
pub const ID_CONTENTENCKEYID: u64 = 0x47e2;
pub const ID_AESSETTINGSCIPHERMODE: u64 = 0x47e8;
pub const ID_CUETIME: u64 = 0xb3;
pub const ID_CUETRACK: u64 = 0xf7;
pub const ID_CUECLUSTERPOSITION: u64 = 0xf1;
pub const ID_CUEBLOCKNUMBER: u64 = 0x5378;
pub const ID_CHAPTERUID: u64 = 0x73c4;
pub const ID_CHAPTERSTRINGUID: u64 = 0x5654;
pub const ID_CHAPTERTIMESTART: u64 = 0x91;
pub const ID_CHAPSTRING: u64 = 0x85;
pub const ID_CHAPLANGUAGE: u64 = 0x437c;
pub const ID_TARGETTYPEVALUE: u64 = 0x68ca;
pub const ID_TARGETTYPE: u64 = 0x63ca;
pub const ID_TAGTRACKUID: u64 = 0x63c5;
pub const ID_TAGNAME: u64 = 0x45a3;
pub const ID_TAGLANGUAGE: u64 = 0x447a;
pub const ID_TAGDEFAULT: u64 = 0x4484;
pub const ID_TAGSTRING: u64 = 0x4487;
pub const ID_TAGBINARY: u64 = 0x4485;
pub const ID_TRACKTIMESTAMPSCALE: u64 = 0x23314f;
pub const ID_POSITION: u64 = 0xa7;
pub const ID_SEGMENTUID: u64 = 0x73a4;

pub const NODE_INFOS: [NodeInfo<'static>; 122] = [
    NodeInfo { id: ID_EBMLHEADERNODE, name: "EBMLHeaderNode" },
    NodeInfo { id: ID_SEGMENTNODE, name: "SegmentNode" },
    NodeInfo { id: ID_SEEKHEADNODE, name: "SeekHeadNode" },
    NodeInfo { id: ID_SEEKNODE, name: "SeekNode" },
    NodeInfo { id: ID_INFONODE, name: "InfoNode" },
    NodeInfo { id: ID_CLUSTERNODE, name: "ClusterNode" },
    NodeInfo { id: ID_BLOCKGROUPNODE, name: "BlockGroupNode" },
    NodeInfo { id: ID_SLICESNODE, name: "SlicesNode" },
    NodeInfo { id: ID_TRACKSNODE, name: "TracksNode" },
    NodeInfo { id: ID_TRACKENTRYNODE, name: "TrackEntryNode" },
    NodeInfo { id: ID_VIDEONODE, name: "VideoNode" },
    NodeInfo { id: ID_AUDIONODE, name: "AudioNode" },
    NodeInfo { id: ID_CONTENTENCODINGSNODE, name: "ContentEncodingsNode" },
    NodeInfo { id: ID_CONTENTENCODINGNODE, name: "ContentEncodingNode" },
    NodeInfo { id: ID_CONTENTENCRYPTIONNODE, name: "ContentEncryptionNode" },
    NodeInfo { id: ID_CONTENTENCAESSETTINGSNODE, name: "ContentEncAESSettingsNode" },
    NodeInfo { id: ID_CUESNODE, name: "CuesNode" },
    NodeInfo { id: ID_CUEPOINTNODE, name: "CuePointNode" },
    NodeInfo { id: ID_CUETRACKPOSITIONSNODE, name: "CueTrackPositionsNode" },
    NodeInfo { id: ID_CHAPTERSNODE, name: "ChaptersNode" },
    NodeInfo { id: ID_EDITIONENTRYNODE, name: "EditionEntryNode" },
    NodeInfo { id: ID_CHAPTERATOMNODE, name: "ChapterAtomNode" },
    NodeInfo { id: ID_CHAPTERDISPLAYNODE, name: "ChapterDisplayNode" },
    NodeInfo { id: ID_TAGSNODE, name: "TagsNode" },
    NodeInfo { id: ID_TAGNODE, name: "TagNode" },
    NodeInfo { id: ID_TARGETSNODE, name: "TargetsNode" },
    NodeInfo { id: ID_SIMPLETAGNODE, name: "SimpleTagNode" },

    // non-master nodes
    // ebml header
    NodeInfo { id: ID_EBMLVERSION, name: "EBMLVersion" },
    NodeInfo { id: ID_EBMLREADVERSION, name: "EBMLReadVersion" },
    NodeInfo { id: ID_EBMLMAXIDLENGTH, name: "EBMLMaxIDLength" },
    NodeInfo { id: ID_EBMLMAXSIZELENGTH, name: "EBMLMaxSizeLength" },
    NodeInfo { id: ID_DOCTYPE, name: "DocType" },
    NodeInfo { id: ID_DOCTYPEVERSION, name: "DocTypeVersion" },
    NodeInfo { id: ID_DOCTYPEREADVERSION, name: "DocTypeReadVersion" },
    NodeInfo { id: ID_CRC32, name: "CRC-32" },
    NodeInfo { id: ID_VOID, name: "Void" },
    NodeInfo { id: ID_SIGNATURESLOT, name: "SignatureSlot" },
    NodeInfo { id: ID_SIGNATUREALGO, name: "SignatureAlgo" },
    NodeInfo { id: ID_SIGNATUREHASH, name: "SignatureHash" },
    NodeInfo { id: ID_SIGNATUREPUBLICKEY, name: "SignaturePublicKey" },
    NodeInfo { id: ID_SIGNATURE, name: "Signature" },
    NodeInfo { id: ID_SIGNATUREELEMENTS, name: "SignatureElements" },
    NodeInfo { id: ID_SIGNATUREELEMENTLIST, name: "SignatureElementList" },
    NodeInfo { id: ID_SIGNEDELEMENT, name: "SignedElement" },

    // everything else
    NodeInfo { id: ID_SEEKID, name: "SeekID" },
    NodeInfo { id: ID_SEEKPOSITION, name: "SeekPosition" },
    NodeInfo { id: ID_TIMESTAMPSCALE, name: "TimestampScale" },
    NodeInfo { id: ID_DURATION, name: "Duration" },
    NodeInfo { id: ID_DATEUTC, name: "DateUTC" },
    NodeInfo { id: ID_MUXINGAPP, name: "MuxingApp" },
    NodeInfo { id: ID_WRITINGAPP, name: "WritingApp" },
    NodeInfo { id: ID_TIMESTAMP, name: "Timestamp" },
    NodeInfo { id: ID_PREVSIZE, name: "PrevSize" },
    NodeInfo { id: ID_SIMPLEBLOCK, name: "SimpleBlock" },
    NodeInfo { id: ID_BLOCK, name: "Block" },
    NodeInfo { id: ID_BLOCKDURATION, name: "BlockDuration" },
    NodeInfo { id: ID_REFERENCEBLOCK, name: "ReferenceBlock" },
    NodeInfo { id: ID_DISCARDPADDING, name: "DiscardPadding" },
    NodeInfo { id: ID_LACENUMBER, name: "LaceNumber" },
    NodeInfo { id: ID_TRACKNUMBER, name: "TrackNumber" },
    NodeInfo { id: ID_TRACKUID, name: "TrackUID" },
    NodeInfo { id: ID_TRACKTYPE, name: "TrackType" },
    NodeInfo { id: ID_FLAGENABLED, name: "FlagEnabled" },
    NodeInfo { id: ID_FLAGDEFAULT, name: "FlagDefault" },
    NodeInfo { id: ID_FLAGFORCED, name: "FlagForced" },
    NodeInfo { id: ID_FLAGLACING, name: "FlagLacing" },
    NodeInfo { id: ID_DEFAULTDURATION, name: "DefaultDuration" },
    NodeInfo { id: ID_NAME, name: "Name" },
    NodeInfo { id: ID_LANGUAGE, name: "Language" },
    NodeInfo { id: ID_CODECID, name: "CodecID" },
    NodeInfo { id: ID_CODECPRIVATE, name: "CodecPrivate" },
    NodeInfo { id: ID_CODECNAME, name: "CodecName" },
    NodeInfo { id: ID_CODECDELAY, name: "CodecDelay" },
    NodeInfo { id: ID_SEEKPREROLL, name: "SeekPreRoll" },
    NodeInfo { id: ID_FLAGINTERLACED, name: "FlagInterlaced" },
    NodeInfo { id: ID_STEREOMODE, name: "StereoMode" },
    NodeInfo { id: ID_ALPHAMODE, name: "AlphaMode" },
    NodeInfo { id: ID_PIXELWIDTH, name: "PixelWidth" },
    NodeInfo { id: ID_PIXELHEIGHT, name: "PixelHeight" },
    NodeInfo { id: ID_PIXELCROPBOTTOM, name: "PixelCropBottom" },
    NodeInfo { id: ID_PIXELCROPTOP, name: "PixelCropTop" },
    NodeInfo { id: ID_PIXELCROPLEFT, name: "PixelCropLeft" },
    NodeInfo { id: ID_PIXELCROPRIGHT, name: "PixelCropRight" },
    NodeInfo { id: ID_DISPLAYWIDTH, name: "DisplayWidth" },
    NodeInfo { id: ID_DISPLAYHEIGHT, name: "DisplayHeight" },
    NodeInfo { id: ID_DISPLAYUNIT, name: "DisplayUnit" },
    NodeInfo { id: ID_ASPECTRATIOTYPE, name: "AspectRatioType" },
    NodeInfo { id: ID_PROJECTIONTYPE, name: "ProjectionType" },
    NodeInfo { id: ID_PROJECTIONPRIVATE, name: "ProjectionPrivate" },
    NodeInfo { id: ID_PROJECTIONPOSEYAW, name: "ProjectionPoseYaw" },
    NodeInfo { id: ID_PROJECTIONPOSEPITCH, name: "ProjectionPosePitch" },
    NodeInfo { id: ID_PROJECTIONPOSEROLL, name: "ProjectionPoseRoll" },
    NodeInfo { id: ID_SAMPLINGFREQUENCY, name: "SamplingFrequency" },
    NodeInfo { id: ID_OUTPUTSAMPLINGFREQUENCY, name: "OutputSamplingFrequency" },
    NodeInfo { id: ID_CHANNELS, name: "Channels" },
    NodeInfo { id: ID_BITDEPTH, name: "BitDepth" },
    NodeInfo { id: ID_CONTENTENCODINGORDER, name: "ContentEncodingOrder" },
    NodeInfo { id: ID_CONTENTENCODINGSCOPE, name: "ContentEncodingScope" },
    NodeInfo { id: ID_CONTENTENCODINGTYPE, name: "ContentEncodingType" },
    NodeInfo { id: ID_CONTENTENCALGO, name: "ContentEncAlgo" },
    NodeInfo { id: ID_CONTENTENCKEYID, name: "ContentEncKeyID" },
    NodeInfo { id: ID_AESSETTINGSCIPHERMODE, name: "AESSettingsCipherMode" },
    NodeInfo { id: ID_CUETIME, name: "CueTime" },
    NodeInfo { id: ID_CUETRACK, name: "CueTrack" },
    NodeInfo { id: ID_CUECLUSTERPOSITION, name: "CueClusterPosition" },
    NodeInfo { id: ID_CUEBLOCKNUMBER, name: "CueBlockNumber" },
    NodeInfo { id: ID_CHAPTERUID, name: "ChapterUID" },
    NodeInfo { id: ID_CHAPTERSTRINGUID, name: "ChapterStringUID" },
    NodeInfo { id: ID_CHAPTERTIMESTART, name: "ChapterTimeStart" },
    NodeInfo { id: ID_CHAPSTRING, name: "ChapString" },
    NodeInfo { id: ID_CHAPLANGUAGE, name: "ChapLanguage" },
    NodeInfo { id: ID_TARGETTYPEVALUE, name: "TargetTypeValue" },
    NodeInfo { id: ID_TARGETTYPE, name: "TargetType" },
    NodeInfo { id: ID_TAGTRACKUID, name: "TagTrackUID" },
    NodeInfo { id: ID_TAGNAME, name: "TagName" },
    NodeInfo { id: ID_TAGLANGUAGE, name: "TagLanguage" },
    NodeInfo { id: ID_TAGDEFAULT, name: "TagDefault" },
    NodeInfo { id: ID_TAGSTRING, name: "TagString" },
    NodeInfo { id: ID_TAGBINARY, name: "TagBinary" },
    NodeInfo { id: ID_TRACKTIMESTAMPSCALE, name: "TrackTimestampScale" },
    NodeInfo { id: ID_POSITION, name: "Position" },
    NodeInfo { id: ID_SEGMENTUID, name: "SegmentUID" },
];

pub fn get_node_info<'a>(id: u64) -> Option<&'a NodeInfo<'static>> {
    NODE_INFOS.iter().find(|&info| info.id == id)
}
