
#[derive(Clone, Debug, Default)]
pub struct Sector<'a> {
    pub id: u32,
    pub deleted: bool,
    pub level: u32,
    pub previous: u32,
    pub next: u32,
    pub payload: &'a [u8],
}

impl<'a> Sector<'a> {
    fn new(id: u32,
           deleted: bool,
           level: u32,
           previous: u32,
           next: u32,
           payload: &'a [u8]) -> Self {
        Self {
            id,
            deleted,
            level,
            previous,
            next,
            payload
        }
        
    }
}

pub fn get_sector(sector: &[u8], id: u32) -> Sector {
    Sector::new(
        id,
        sector[0] != 0,
        sector[1] as u32 & 0x00FFFFFF,
        ((sector[4] as u32) << 24) as u32 |
                ((sector[5] as u32) << 16) |
                ((sector[6] as u32) << 8) |
                (sector[7]) as u32,
        ((sector[8] as u32) << 24) as u32 |
                ((sector[9] as u32) << 16) |
                ((sector[10] as u32) << 8) |
                (sector[11]) as u32,
        &sector[20..]
        )
}
