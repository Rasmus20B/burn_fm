
#[derive(Debug, Clone)]
pub enum ChunkType {
    DataSimple = 0,
    RefSimple = 1,
    RefLong = 2,
    DataSegment = 3,
    PathPush = 4,
    PathPop = 5,
    Noop = 6,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    ctype : ChunkType,
    length : usize,
    index : usize,
}

pub fn decode_chunk() {

}
