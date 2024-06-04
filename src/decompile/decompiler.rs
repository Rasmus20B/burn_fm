use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::exit;

use crate::component;
use crate::decompile::sector;
use crate::decompile::chunk;
use crate::decompile::encoding_util::{self, get_int, get_path_int};

const SECTOR_SIZE : usize = 4096;

pub fn decompile_fmp12_file(path: &Path) {
    
    let mut file = File::open(path).expect("unable to open file.");
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Unable to read file.");

    println!("Found file. Size: {}", buffer.len());

    let mut offset = SECTOR_SIZE;
    let mut next_id = 0;
    let mut sectors = Vec::<sector::Sector>::new();
    while offset > 0 {
        let sector = sector::get_sector(&buffer[offset..offset+SECTOR_SIZE], next_id);
        next_id = sector.next;
        offset = SECTOR_SIZE * sector.next as usize;
        sectors.push(sector);
    }

    println!("Sectors found: {}", sectors.len());
    let mut data = Vec::<component::VecWrapper>::new();
    data.push(component::VecWrapper::Tables(Vec::<component::FMComponentTable>::new()));
    data.push(component::VecWrapper::Fields(Vec::<component::FMComponentField>::new()));

    for sector in sectors {
        offset = 0;
        while offset < sector.payload.len() {
            let mut chunk_type = sector.payload[offset];


            if (chunk_type & 0xC0) == 0xC0 {
                println!("went from {:x}", chunk_type);
                chunk_type &= 0x3F;
            }
            println!("chunktype {:x}", chunk_type);
            match chunk_type {
                0x00 => {
                    offset += 1;
                    if sector.payload[1] == 0x00 { continue; }
                    offset += 1;
                },
                0x01 | 0x02 | 0x03 | 0x04 | 0x05 => {
                    offset += 1;
                    offset += 1;
                    offset += (chunk_type == 0x01) as usize + (2 * (chunk_type - 0x01) as usize);
                }
                0x06 => {
                    offset += 1;
                    offset += 1;
                    offset += sector.payload[offset] as usize;
                    offset += 1;
                },
                0x07 => {
                    offset += 1;
                    offset += 1;
                    let len = encoding_util::get_int(&sector.payload[offset..offset+2]);
                    offset += 2;
                    offset += len;
                },
                0x08 => {
                    offset += 1;
                    offset += 2;
                },
                0x09 | 0x0A | 0x0B | 0x0C | 0x0D => {
                    offset += 1;
                    offset += 2;
                    offset += (chunk_type == 0x09) as usize + (2 *(chunk_type - 0x09) as usize);
                },
                0x0E => {
                    if sector.payload[offset + 1] != 0xFF {
                        offset += 1;
                        offset += 2;
                        offset += sector.payload[offset] as usize;
                        offset += 1;
                    } else {
                        offset += 1;
                        offset += 6;
                    }
                },
                0x0F => {
                    if sector.payload[offset+1] == 0x80 {
                        offset += 2;
                        offset += 1;
                        offset += get_int(&sector.payload[offset..=offset+2]);
                        offset += 2;
                    }
                    offset += 1;
                },
                0x10 => {
                    offset += 1;
                    offset += 3;
                },
                0x11 | 0x12 | 0x13 | 0x14 | 0x15 => {
                    offset += 1;
                    offset += 3 + (chunk_type == 0x11) as usize + (2 * (chunk_type as usize - 0x11));
                },
                0x16 => {
                    offset += 1;
                    offset += 3;
                    offset += sector.payload[offset] as usize;
                    offset += 1;
                }
                0x17 => {
                    offset += 1;
                    offset += 3;
                    let len = get_path_int(&sector.payload[offset..offset+2]);
                    offset += 2;
                    offset += len;
                },
                0x1B => {
                    if sector.payload[offset + 1] == 0x00 {
                        offset += 2;
                        offset += 1;
                        offset += 4;
                    } else {
                        offset += 1;
                        let len = sector.payload[offset] as usize;
                        offset += 1;
                        offset += len + (chunk_type == 0x19) as usize + (2 * (chunk_type as usize - 0x19));
                    }
                },
                0x19 | 0x1A | 0x1C | 0x1D => {
                    offset += 1;
                    let len = sector.payload[offset] as usize;
                    offset += 1;
                    offset += len + (chunk_type == 0x19) as usize + (2 * (chunk_type as usize - 0x19));
                },
                0x1E => {
                    offset += 1;
                    let ref_len = sector.payload[offset] as usize;
                    offset += 1;
                    offset += ref_len;
                    let len = sector.payload[offset] as usize;
                    offset += 1;
                    offset += len;
                },
                0x1F => {
                    offset += 1;
                    let ref_len = sector.payload[offset];
                    offset += 1;
                    let len = get_path_int(&sector.payload[offset..offset+2]);
                    offset += 2;
                    offset += len;
                },
                0x20 => {
                    offset += 1;
                    let mut len = 1;
                    if sector.payload[offset] == 0xFE {
                        offset += 1;
                        len = 8;
                    }
                    let idx = encoding_util::get_path_int(&sector.payload[offset..offset+1]);
                    offset += len;
                    println!("Pushing {} to path from special", idx);
                },
                0x28 => {
                    offset += 1;
                    let idx = encoding_util::get_path_int(&sector.payload[offset..=offset+2]);
                    offset += 2;
                },
                0x30 => {
                    offset += 1;
                    offset += 3;
                },
                0x38 => {
                    offset += 1;
                    let len = sector.payload[offset] as usize;
                    offset += 1;
                    offset += len;
                },
                0x3D | 0x40 => {
                    offset += 1;
                },
                0x80 => {
                    offset += 1;
                }
                _ => {
                    println!("Nah. {:x}", chunk_type);
                    exit(0);
                }
            };
        }

    }

}
