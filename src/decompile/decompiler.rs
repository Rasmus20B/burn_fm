use std::fs::File;
use std::io::{self, Read};
use std::ops::Deref;
use std::path::Path;
use std::process::exit;
use std::collections::HashMap;

use crate::{component, metadata_constants};
use crate::file::{self, FmpFile};
use crate::decompile::sector;
use crate::decompile::chunk;

use super::chunk::{get_chunk_from_code, Chunk, ChunkType};
use super::encoding_util::fm_string_decrypt;

const SECTOR_SIZE : usize = 4096;

pub fn decompile_fmp12_file(path: &Path) -> FmpFile {
    
    let mut file = File::open(path).expect("unable to open file.");
    let mut fmp_file = FmpFile { name: "name".to_string(), tables: HashMap::new() };
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Unable to read file.");

    let mut offset = SECTOR_SIZE;
    let mut next_id = 0_usize;
    let mut sectors = Vec::<sector::Sector>::new();
    while offset > 0 {
        let sector = sector::get_sector(&buffer[offset..offset+SECTOR_SIZE], next_id);
        println!("last sector: {}", sector.next);
        next_id = sector.next;
        offset = SECTOR_SIZE * sector.next as usize;
        sectors.push(sector);
    }

    println!("Found: {} sectors", sectors.len());


    for sector in &mut sectors {
        offset = 0;
        let mut path = Vec::<usize>::new();
        let mut chunks =  Vec::<chunk::Chunk>::new();
        while offset < sector.payload.len() {
            let chunk = get_chunk_from_code(&sector.payload, &mut offset, &mut path)
                .expect("Unable to decompile chunk: ");

            match &path.iter().as_slice() {
                [4, 1, 7, x, ..] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    // println!("data: {}", s);
                },
                [x, 3, 5, y] => {
                    if x >= &128 {
                        if chunk.ctype == ChunkType::PathPush {
                            // println!("table: {}, column: {}", x, y);
                            if !fmp_file.tables.contains_key(&(*x - 128)) {
                                fmp_file.tables.insert(*x - 128,
                            component::FMComponentTable { 
                            table_name: "".to_string(),
                            created_by_account: "".to_string(),
                            create_by_user: "".to_string(),
                            fields: HashMap::new() });
                            }
                            fmp_file.tables.get_mut(&(*x - 128)).
                                unwrap().fields
                                .insert(*y as u16, component::FMComponentField { 
                                    data_type: "".to_string(),
                                    field_description: "".to_string(),
                                    field_name: "".to_string(),
                                    field_type: "".to_string(),
                                    created_by_account: "".to_string(),
                                    created_by_user: "".to_string() });
                        } else {
                            let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                            // println!("instr: {:x}. ref: {:?}, data: {}", chunk.code, chunk.ref_simple, s);
                            let tidx = x - 128;
                            match chunk.ref_simple.unwrap_or(0) {
                                metadata_constants::FIELD_TYPE => {
                                    fmp_file.tables.get_mut(&tidx)
                                        .unwrap().fields
                                        .get_mut(&(*y as u16))
                                        .unwrap()
                                        .field_type = s
                                },
                                metadata_constants::COMPONENT_DESC => {
                                    fmp_file.tables.get_mut(&tidx)
                                        .unwrap().fields
                                        .get_mut(&(*y as u16))
                                        .unwrap()
                                        .field_description = s
                                },
                                metadata_constants::COMPONENT_NAME => {
                                    fmp_file.tables.get_mut(&tidx).unwrap()
                                        .fields
                                        .get_mut(&(*y as u16))
                                        .unwrap()
                                        .field_name = s
                                },
                                metadata_constants::CREATOR_ACCOUNT_NAME => { 
                                    fmp_file.tables.get_mut(&tidx).unwrap()
                                        .fields
                                        .get_mut(&(*y as u16))
                                        .unwrap()
                                        .created_by_account = s 
                                },
                                metadata_constants::CREATOR_USER_NAME => {
                                    fmp_file.tables.get_mut(&tidx).unwrap()
                                        .fields
                                        .get_mut(&(*y as u16))
                                        .unwrap()
                                        .created_by_user = s 
                                },
                                _ => {},
                            };
                        }
                    }

                },
                [3, 16, 5, x] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                             &path.clone(),
                             chunk.ref_simple,
                             s);
                    if chunk.ctype == ChunkType::PathPush {
                        if !fmp_file.tables.contains_key(x) {
                            fmp_file.tables.insert(*x - 128, component::FMComponentTable { 
                                table_name: "".to_string(),
                                created_by_account: "".to_string(),
                                create_by_user: "".to_string(),
                                fields: HashMap::new() });
                        } 
                    } else {
                        match chunk.ref_simple.unwrap_or(0) {
                            metadata_constants::COMPONENT_NAME => { 
                                fmp_file.tables.get_mut(&(x - 128)).unwrap().table_name = s },
                            _ => {}
                        }
                    }
                },
                _ => { 
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    // println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                    //          &path.clone(),
                    //          chunk.ref_simple,
                    //          s);
                }
            }
            chunks.push(chunk);
        }
        sector.chunks = chunks;
    }
    return fmp_file;
}
