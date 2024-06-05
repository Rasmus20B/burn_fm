
#[derive(Debug, Clone, PartialEq)]
pub enum ChunkType {
    DataSimple = 0,
    RefSimple = 1,
    RefLong = 2,
    DataSegment = 3,
    PathPush = 4,
    PathPop = 5,
    Noop = 6,
}

#[derive(Clone)]
pub struct Chunk<'a> {
    pub ctype: ChunkType,
    pub data: Option<&'a [u8]>,
    pub ref_data: Option<&'a [u8]>,
    pub path: Vec::<usize>,
    pub segment_idx: Option<u8>,
    pub ref_simple: Option<u16>,
}

impl<'a> Chunk<'a> {
    pub fn new(ctype: ChunkType,
           data: Option<&'a [u8]>,
           ref_data: Option<&'a [u8]>,
           path: Vec::<usize>,
           segment_idx: Option<u8>,
           ref_simple: Option<u16>,
        ) -> Self {
        Self {
            ctype,
            data,
            ref_data,
            path,
            segment_idx,
            ref_simple,
        }
    }
}
