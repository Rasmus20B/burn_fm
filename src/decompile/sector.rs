use super::{chunk, encoding_util::get_int};
use crate::decompile::encoding_util;

#[derive(Clone, Default)]
pub struct Sector<'a> {
    pub id: usize,
    pub deleted: bool,
    pub level: u32,
    pub previous: u32,
    pub next: usize,
    pub payload: &'a [u8],
    pub chunks: Vec::<chunk::Chunk<'a>>
}

impl<'a> Sector<'a> {
    fn new(id: usize,
           deleted: bool,
           level: u32,
           previous: u32,
           next: usize,
           payload: &'a [u8],
           chunks: Vec::<chunk::Chunk<'a>>) -> Self {
        Self {
            id,
            deleted,
            level,
            previous,
            next,
            payload,
            chunks
        }
    }
}

pub fn get_sector(sector: &[u8], id: usize) -> Sector {
    Sector::new(
        id,
        sector[0] != 0,
        sector[1] as u32 & 0x00FFFFFF,
        get_int(&sector[4..8]) as u32,
        get_int(&sector[8..12]),
        &sector[20..],
        Vec::<chunk::Chunk>::new()
        )
}
