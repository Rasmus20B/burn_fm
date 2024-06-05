use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::exit;

use std::str;

use crate::component;
use crate::file::{self, FmpFile};
use crate::decompile::sector;
use crate::decompile::chunk;
use crate::decompile::encoding_util::{self, get_int, get_path_int};

use super::chunk::{Chunk, ChunkType};

const SECTOR_SIZE : usize = 4096;

pub fn decompile_fmp12_file(path: &Path) -> FmpFile {
    
    let mut file = File::open(path).expect("unable to open file.");
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Unable to read file.");

    let mut offset = SECTOR_SIZE;
    let mut next_id = 0;
    let mut sectors = Vec::<sector::Sector>::new();
    while offset > 0 {
        let sector = sector::get_sector(&buffer[offset..offset+SECTOR_SIZE], next_id);
        next_id = sector.next;
        offset = SECTOR_SIZE * sector.next as usize;
        sectors.push(sector);
    }


    for sector in &mut sectors {
        offset = 0;
        let mut depth = 0;
        let mut path = Vec::<usize>::new();
        let mut chunks =  Vec::<chunk::Chunk>::new();
        while offset < sector.payload.len() {
            let mut chunk_code = sector.payload[offset];
            let mut ctype = ChunkType::Noop;
            let mut data: Option<&[u8]> = None;
            let mut ref_data: Option<&[u8]> = None;
            let mut segidx: Option<u8> = None;
            let mut ref_simple: Option<u16> = None;
            let mut delayed = false;

            if (chunk_code & 0xC0) == 0xC0 {
                chunk_code &= 0x3F;
                delayed = true;
            }

            // println!("{:x}", chunk_code);
            match chunk_code {
                0x00 => {
                    offset += 1;
                    ctype = ChunkType::DataSimple;
                    data = Some(&sector.payload[offset..offset]);
                    if sector.payload[1] == 0x00 { continue; }
                    offset += 1;
                },
                0x01 | 0x02 | 0x03 | 0x04 | 0x05 => {
                    offset += 1;
                    ctype = ChunkType::RefSimple;
                    ref_simple = Some(sector.payload[offset] as u16);
                    offset += 1;
                    let len = (chunk_code == 0x01) as usize + (2 * (chunk_code - 0x01) as usize);
                    data = Some(&sector.payload[offset..offset+len]);
                    offset += len;
                }
                0x06 => {
                    offset += 1;
                    ctype = ChunkType::RefSimple;
                    ref_simple = Some(sector.payload[offset] as u16);
                    offset += 1;
                    let len = sector.payload[offset] as usize;
                    offset += 1;
                    data = Some(&sector.payload[offset..offset+len]);
                    offset += data.unwrap().len();
                },
                0x07 => {
                    offset += 1;
                    ctype = ChunkType::DataSegment;
                    segidx = Some(sector.payload[offset]);
                    offset += 1;
                    let len = encoding_util::get_int(&sector.payload[offset..offset+2]);
                    offset += 2;
                    data = Some(&sector.payload[offset..offset+len]);
                    offset += len;
                },
                0x08 => {
                    offset += 1;
                    ctype = ChunkType::DataSimple;
                    data = Some(&sector.payload[offset..offset+2]);
                    offset += 2;
                },
                0x09 | 0x0A | 0x0B | 0x0C | 0x0D => {
                    offset += 1;
                    ctype = ChunkType::RefSimple;
                    ref_simple = Some(get_path_int(&sector.payload[offset..offset+2]) as u16);
                    offset += 2;
                    let len = (chunk_code == 0x09) as usize + (2 *(chunk_code - 0x09) as usize);
                    data = Some(&sector.payload[offset..offset+len]);
                    offset += len;
                },
                0x0E => {
                    if sector.payload[offset + 1] != 0xFF {
                        offset += 1;
                        ctype = ChunkType::RefSimple;
                        ref_simple = Some(get_path_int(&sector.payload[offset..offset+2]) as u16);
                        // ref_data = Some(&sector.payload[ref_offset..ref_offset]);
                        offset += 2;
                        let len = sector.payload[offset] as usize;
                        offset += 1;
                        data = Some(&sector.payload[offset..offset+len]);
                        offset += len;
                    } else {
                        offset += 1;
                        ctype = ChunkType::DataSimple;
                        data = Some(&sector.payload[offset..offset+6]);
                        offset += 6;
                    }
                },
                0x0F => {
                    if sector.payload[offset+1] == 0x80 {
                        ctype = ChunkType::DataSegment;
                        offset += 2;
                        segidx = Some(sector.payload[offset]);
                        offset += 1;
                        let len = get_int(&sector.payload[offset..offset+2]);
                        offset += 2;
                        data = Some(&sector.payload[offset..offset+len]);
                        offset += len;
                    }
                },
                0x10 => {
                    offset += 1;
                    ctype = ChunkType::DataSimple;
                    data = Some(&sector.payload[offset..offset+3]);
                    offset += 3;
                },
                0x11 | 0x12 | 0x13 | 0x14 | 0x15 => {
                    offset += 1;
                    ctype = ChunkType::DataSimple;
                    let len = 3 + (chunk_code == 0x11) as usize + (2 * (chunk_code as usize - 0x11));
                    data = Some(&sector.payload[offset..offset+len]);
                    offset += len;
                },
                0x16 => {
                    offset += 1;
                    ctype = ChunkType::RefLong;
                    ref_data = Some(&sector.payload[offset..offset+3]);
                    offset += 3;
                    let len = sector.payload[offset] as usize;
                    data = Some(&sector.payload[offset..offset+len]);
                    offset += len;
                }
                0x17 => {
                    offset += 1;
                    ctype = ChunkType::RefLong;
                    ref_data = Some(&sector.payload[offset..offset+3]);
                    offset += 3;
                    let len = get_path_int(&sector.payload[offset..offset+2]);
                    offset += 2;
                    data = Some(&sector.payload[offset..offset+len]);
                    offset += len;
                },
                0x1B => {
                    if sector.payload[offset + 1] == 0x00 {
                        offset += 2;
                        ctype = ChunkType::RefSimple;
                        ref_simple = Some(sector.payload[offset] as u16);
                        offset += 1;
                        data = Some(&sector.payload[offset..offset+4]);
                        offset += 4;
                    } else {
                        offset += 1;
                        ctype = ChunkType::DataSimple;
                        let len = sector.payload[offset] as usize;
                        offset += 1;
                        offset += len + (chunk_code == 0x19) as usize + (2 * (chunk_code as usize - 0x19));
                    }
                },
                0x19 | 0x1A | 0x1C | 0x1D => {
                    offset += 1;
                    ctype = ChunkType::DataSimple;
                    let len = sector.payload[offset] as usize;
                    offset += 1;
                    offset += len + (chunk_code == 0x19) as usize + (2 * (chunk_code as usize - 0x19));
                },
                0x1E => {
                    offset += 1;
                    ctype = ChunkType::RefLong;
                    let ref_len = sector.payload[offset] as usize;
                    offset += 1;
                    offset += ref_len;
                    let len = sector.payload[offset] as usize;
                    offset += 1;
                    offset += len;
                },
                0x1F => {
                    offset += 1;
                    ctype = ChunkType::RefLong;
                    let ref_len = sector.payload[offset];
                    offset += 1;
                    let len = get_path_int(&sector.payload[offset..offset+2]);
                    offset += 2;
                    offset += len;
                },
                0x20 => {
                    offset += 1;
                    ctype = ChunkType::PathPush;
                    if sector.payload[offset] == 0xFE {
                        offset += 1;
                        data = Some(&sector.payload[offset..offset+8]);
                    } else {
                        data = Some(&sector.payload[offset..offset+1]);
                    }
                    let idx = encoding_util::get_path_int(&sector.payload[offset..offset+1]);
                    offset += data.unwrap().len();
                    path.push(idx);
                    depth+=1;
                },
                0x23 => {
                    offset += 1;
                    ctype = ChunkType::DataSimple;
                    let len = sector.payload[offset] as usize;
                    offset += 1;
                    data = Some(&sector.payload[offset..offset+len]);
                    offset += len;
                },
                0x28 => {
                    offset += 1;
                    ctype = ChunkType::PathPush;
                    data = Some(&sector.payload[offset..offset+2]);
                    let idx = encoding_util::get_path_int(&sector.payload[offset..offset+2]);
                    offset += 2;
                    path.push(idx);
                    depth+=1;
                },
                0x30 => {
                    offset += 1;
                    ctype = ChunkType::PathPush;
                    data = Some(&sector.payload[offset..offset+3]);
                    // let dir = 0x80 + ((sector.payload[offset + 1] as usize) << 8) + sector.payload[offset + 2] as usize;
                    let dir = get_path_int(&sector.payload[offset..offset+3]);
                    path.push(dir.into());
                    offset += 3;
                    depth+=1;
                },
                0x38 => {
                    offset += 1;
                    ctype = ChunkType::PathPush;
                    let len = sector.payload[offset] as usize;
                    offset += 1;
                    data = Some(&sector.payload[offset..offset+2]);
                    let dir = get_path_int(&sector.payload[offset..offset+2]);
                    path.push(dir);
                    offset += len;
                    depth+=1;
                },
                0x3D | 0x40 => {
                    ctype = ChunkType::PathPop;
                    offset += 1;
                    path.pop();
                },
                0x80 => {
                    ctype = ChunkType::Noop;
                    offset += 1;
                }
                _ => {
                    eprintln!("Error: Invalid chunk code. {}", chunk_code);
                    exit(-1);
                }
            };

            match &path.iter().as_slice() {
                [3, 16, 5, x] => {
                    if *x >= 128 {
                        let s = match String::from_utf8(data.unwrap_or(&[0])
                                                     .into_iter()
                                                     .map(|c| c ^ 0x5A)
                                                     .collect::<Vec<u8>>()) {
                            Ok(v) => v.to_string(),
                            Err(e) => "value not utf-8.".to_string()
                        };

                        println!("code: {:x}", chunk_code);
                        if ctype != ChunkType::RefLong &&
                            ctype != ChunkType::PathPush &&
                            ctype != ChunkType::Noop {
                            println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                                     &path.clone(),
                                     ref_simple,
                                     s);
                        }
                    } else {
                        println!("Path: {:?}", &path.clone());
                    }
                }
                _ => { }
            }

            if delayed == true {
                path.pop();
            }
            chunks.push(chunk::Chunk::new(ctype,
                                          data,
                                          ref_data,
                                          path.clone(),
                                          segidx,
                                          ref_simple));
        }
        sector.chunks = chunks;
    }
    let mut file = FmpFile { name: "name".to_string(), tables: vec![] };
    return file;
}
