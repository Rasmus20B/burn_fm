use std::ascii::AsciiExt;
use std::fs::File;
use std::io::{self, Read};
use std::ops::Deref;
use std::path::Path;
use std::process::exit;

use std::str;

use crate::component;
use crate::file::{self, FmpFile};
use crate::decompile::sector;
use crate::decompile::chunk;
use crate::decompile::encoding_util::{self, get_int, get_path_int};

use super::chunk::{get_chunk_from_code, Chunk, ChunkType};
use super::encoding_util::fm_string_decrypt;

const SECTOR_SIZE : usize = 4096;

pub fn decompile_fmp12_file(path: &Path) -> FmpFile {
    
    let mut file = File::open(path).expect("unable to open file.");
    let mut fmp_file = FmpFile { name: "name".to_string(), tables: vec![] };
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
            let chunk = get_chunk_from_code(&sector.payload, &mut offset, &mut path).expect("Unable to decompile chunk: ");
            match &path.iter().as_slice() {
                [4, 1, 7, x, ..] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    println!("data: {}", s);
                },
                [x, 3, 5, y] => {
                    if chunk.ctype == ChunkType::PathPush {
                        println!("table: {}, column: {}", x, y);
                    }
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    println!("instr: {:x}. data: {:?}, ref: {}", chunk.code, chunk.ref_data.unwrap_or(&[0]), s);

                },
                [3, 16, 5, x] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    if chunk.ctype != ChunkType::RefLong &&
                        chunk.ctype != ChunkType::PathPush &&
                        chunk.ctype != ChunkType::Noop {
                        println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                                 &path.clone(),
                                 chunk.ref_simple,
                                 s);
                    }
                }
                _ => { }
            }
            chunks.push(chunk);
        }
        sector.chunks = chunks;
    }
    return fmp_file;
}
