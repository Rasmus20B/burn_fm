use std::fs::File;
use std::io::{self, Read};
use std::ops::Deref;
use std::path::Path;
use std::process::exit;
use std::collections::HashMap;

use crate::{component, metadata_constants};
use crate::file::{self, FmpFile};
use crate::decompile::sector;
use crate::chunk;

use crate::chunk::{get_chunk_from_code, Chunk, ChunkType};
use crate::encoding_util::{fm_string_decrypt,get_int};

const SECTOR_SIZE : usize = 4096;

pub fn decompile_fmp12_file(path: &Path) -> FmpFile {
    
    let mut file = File::open(path).expect("unable to open file.");
    let mut fmp_file = FmpFile { 
        name: "name".to_string(),
        tables: HashMap::new(), 
        relationships: HashMap::new(),
        layouts: HashMap::new(),
    };
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Unable to read file.");

    let mut offset = SECTOR_SIZE;
    let mut next_id = 0_usize;
    let mut sectors = Vec::<sector::Sector>::new();

    let first = sector::get_sector(&buffer[offset..]);
    let n_blocks = first.next;

    sectors.resize(n_blocks + 1, sector::Sector { 
        deleted: false, 
        level: 0,
        previous: 0,
        next: 0,
        payload: &[0],
        chunks: vec![] 
    });
    sectors[0] = first;

    let mut idx = 2;
    let mut chunks =  Vec::<Chunk>::new();
    while idx != 0 {
        let start = idx * SECTOR_SIZE;
        let bound = start + SECTOR_SIZE;
        offset = start;

        sectors[idx] = sector::get_sector(&buffer[offset..]);
        let mut path = Vec::<usize>::new();
        offset += 20;
        while offset < bound {
            let chunk = get_chunk_from_code(&buffer, 
                                            &mut offset, 
                                            &mut path, 
                                            start).expect("Unable to decode chunk.");          
            match &path.iter().as_slice() {
                [4, 1, 7, x, ..] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
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
                            match chunk.ref_simple.unwrap_or(0) {
                                0 => {},
                                2 => { println!("Data type: {:?}", chunk.data.unwrap_or(&[0])); },
                                16 => { println!("Field Name: {}", s); }
                                129 => { println!("created by user: {}", s); }
                                130 => { println!("created by user Account: {}", s); }
                                _   => { println!("instr: {:x}. ref: {:?}, data: {:?}", chunk.code, chunk.ref_simple, chunk.data.unwrap()); }
                            };
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
                [3, 17, 5, 0] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    if chunk.ctype == ChunkType::PathPush {
                        // println!("NEW RELATIONSHIP FOUND");
                    } else {
                        // println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                        //      &path.clone(),
                        //      chunk.ref_simple,
                        //      s);
                    }
                    
                },
                // [x, ..] if x < &128 => {
                //     let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                //     if chunk.ctype == ChunkType::PathPush {
                //         println!("NEW PATH FOUND");
                //     } else {
                //         println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                //              &path.clone(),
                //              chunk.ref_simple,
                //              s);
                //     }
                // }
                _ => { 
                    if path.len() > 0 { 
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    if chunk.ctype == ChunkType::PathPush {
                        // println!("NEW PATH FOUND");
                    } else {
                        // println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                        //      &path.clone(),
                        //      chunk.ref_simple,
                        //      s);
                    }
                    }
                }
            }
        }
        idx = sectors[idx].next;
    }

    return fmp_file;
}
