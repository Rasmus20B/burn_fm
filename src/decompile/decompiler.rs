use std::fs::File;
use std::io::Read;
use std::path::{Component, Path};
use std::collections::HashMap;
use std::time::{Duration, UNIX_EPOCH};

use crate::script_engine::script_engine_instructions::Instruction;
use crate::{component, metadata_constants};
use crate::file::FmpFile;
use crate::decompile::sector;

use chrono::{DateTime, Utc};

use crate::chunk::{get_chunk_from_code, ChunkType};
use crate::encoding_util::fm_string_decrypt;

const SECTOR_SIZE : usize = 4096;

pub fn decompile_fmp12_file(path: &Path) -> FmpFile {
    
    let mut file = File::open(path).expect("unable to open file.");
    let mut fmp_file = FmpFile { 
        name: "name".to_string(),
        tables: HashMap::new(), 
        relationships: HashMap::new(),
        layouts: HashMap::new(),
        scripts: HashMap::new(),
        table_occurrences: HashMap::new(),
    };
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Unable to read file.");

    let mut offset = SECTOR_SIZE;
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
    while idx != 0 {
        let start = idx * SECTOR_SIZE;
        let bound = start + SECTOR_SIZE;
        offset = start;

        sectors[idx] = sector::get_sector(&buffer[offset..]);
        let mut path = Vec::<String>::new();
        offset += 20;
        while offset < bound {
            let chunk = get_chunk_from_code(&buffer, 
                                            &mut offset, 
                                            &mut path, 
                                            start).expect("Unable to decode chunk.");          
            match &path.iter().map(|s| s.as_str()).collect::<Vec<_>>().as_slice() {
                ["3", "17", "5", "0", "251"] => {
                    if chunk.ctype == ChunkType::DataSimple {
                        let tmp = component::FMComponentRelationship {
                            table1: fmp_file.table_occurrences.len() as u16,
                            table2: chunk.data.unwrap()[2] as u16,
                            comparison: 0,
                        };
                        fmp_file.relationships.insert(fmp_file.relationships.len(), tmp);
                        // println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                        //      &path.clone(),
                        //      chunk.ref_simple,
                        //      chunk.data);
                    }
                },
                ["3", "17", "5", "0", ..] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    match chunk.ref_simple {
                        Some(2) => {
                            // println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                            //      &path.clone(),
                            //      chunk.ref_simple,
                            //      chunk.data);
                            let tmp = component::FMComponentTableOccurence {
                                table_occurence_name: String::new(),
                                create_by_user: String::new(),
                                created_by_account: String::new(),
                                table_actual: chunk.data.unwrap()[6] as u16,
                            };
                            fmp_file.table_occurrences.insert(fmp_file.table_occurrences.len() + 1, tmp);
                        }
                        Some(16) => {
                            fmp_file.table_occurrences
                                .get_mut(&(fmp_file.table_occurrences.len())).unwrap()
                                .table_occurence_name = s;
                        },
                        Some(129) => {
                        },
                        Some(130) => {
                        },
                        Some(131) => {
                        },
                        _ => {
                            let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                            if s.is_empty() {
                                continue;
                            }
                            if chunk.ctype == ChunkType::PathPush 
                                || chunk.ctype == ChunkType::PathPop 
                                || chunk.ctype == ChunkType::Noop {

                            } else {
                                // println!("Path: {:?}. reference: {:?}, ref_data: {:?}, data: {:?}", 
                                //      &path.clone(),
                                //      chunk.ref_simple,
                                //      chunk.ref_data,
                                //      chunk.data,
                                //      );
                            }
                        }
                    }
                }
                ["4", "1", "7", x, ..] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    match chunk.ref_simple {
                        Some(16) => {
                            if fmp_file.layouts.contains_key(&x.parse().unwrap()) {
                                fmp_file.layouts.get_mut(&x.parse().unwrap()).unwrap().layout_name = s;
                            } else {
                                let tmp = component::FMComponentLayout {
                                    layout_name: s,
                                    created_by_account: String::new(),
                                    create_by_user: String::new(),
                                };
                                fmp_file.layouts.insert(x.parse().unwrap(), tmp);
                            }
                        },
                        _ => {}
                    }
                },
                [x, "3", "5", y] => {
                    if x.parse::<usize>().unwrap() >= 128 {
                        if chunk.ctype == ChunkType::PathPush {
                            if !fmp_file.tables.contains_key(&(x.parse::<usize>().unwrap() - 128)) {
                                fmp_file.tables.insert(x.parse::<usize>().unwrap() - 128,
                            component::FMComponentTable { 
                            table_name: "".to_string(),
                            created_by_account: "".to_string(),
                            create_by_user: "".to_string(),
                            fields: HashMap::new() });
                            }
                            fmp_file.tables.get_mut(&(x.parse::<usize>().unwrap() - 128)).
                                unwrap().fields
                                .insert(y.parse::<usize>().unwrap() as u16, component::FMComponentField { 
                                    data_type: "".to_string(),
                                    field_description: "".to_string(),
                                    field_name: "".to_string(),
                                    field_type: "".to_string(),
                                    created_by_account: "".to_string(),
                                    created_by_user: "".to_string() });
                        } else {
                            let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                            // match chunk.ref_simple.unwrap_or(0) {
                            //     0 => {},
                            //     2 => { println!("Data type: {:?}", match chunk.data.unwrap_or(&[0])[1] {
                            //         1 => "Text",
                            //         2 => "Number",
                            //         3 => "Date",
                            //         4 => "Time",
                            //         5 => "Timestamp",
                            //         6 => "Container",
                            //         _ => "Unknown"
                            //
                            //     }); },
                            //     3 => { println!("Description: {:?}", s); },
                            //     16 => { println!("Field Name: {}", s); }
                            //     129 => { println!("created by user: {}", s); }
                            //     130 => { println!("created by user Account: {}", s); }
                            //     _   => { println!("instr: {:x}. ref: {:?}, data: {:?}", chunk.code, chunk.ref_simple, chunk.data.unwrap()); }
                            // };
                            let tidx = x.parse::<usize>().unwrap() - 128;
                            match chunk.ref_simple.unwrap_or(0) {
                                metadata_constants::FIELD_TYPE => {
                                    fmp_file.tables.get_mut(&tidx)
                                        .unwrap().fields
                                        .get_mut(&(y.parse::<usize>().unwrap() as u16))
                                        .unwrap()
                                        .field_type = s
                                },
                                metadata_constants::COMPONENT_DESC => {
                                    fmp_file.tables.get_mut(&tidx)
                                        .unwrap().fields
                                        .get_mut(&(y.parse::<usize>().unwrap() as u16))
                                        .unwrap()
                                        .field_description = s
                                },
                                metadata_constants::COMPONENT_NAME => {
                                    fmp_file.tables.get_mut(&tidx).unwrap()
                                        .fields
                                        .get_mut(&(y.parse::<usize>().unwrap() as u16))
                                        .unwrap()
                                        .field_name = s
                                },
                                metadata_constants::CREATOR_ACCOUNT_NAME => { 
                                    fmp_file.tables.get_mut(&tidx).unwrap()
                                        .fields
                                        .get_mut(&(y.parse::<usize>().unwrap() as u16))
                                        .unwrap()
                                        .created_by_account = s 
                                },
                                metadata_constants::CREATOR_USER_NAME => {
                                    fmp_file.tables.get_mut(&tidx).unwrap()
                                        .fields
                                        .get_mut(&(y.parse::<usize>().unwrap() as u16))
                                        .unwrap()
                                        .created_by_user = s 
                                },
                                _ => {},
                            };
                        }
                    }

                },
                ["3", "16", "5", x] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    if chunk.ctype == ChunkType::PathPush {
                        if !fmp_file.tables.contains_key(&x.parse().unwrap()) {
                            fmp_file.tables.insert(x.parse::<usize>().unwrap() - 128, component::FMComponentTable { 
                                table_name: "".to_string(),
                                created_by_account: "".to_string(),
                                create_by_user: "".to_string(),
                                fields: HashMap::new() });
                        } 
                    } else {
                        match chunk.ref_simple.unwrap_or(0) {
                            metadata_constants::COMPONENT_NAME => { 
                                fmp_file.tables.get_mut(&(x.parse::<usize>().unwrap() - 128)).unwrap().table_name = s },
                            _ => {}
                        }
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
                ["17", "5", x] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    match &chunk.ref_simple {
                        // Some(2) => {
                        //     println!("NAME: {:?}. reference: {:?}, ref_data: {:?}", 
                        //          &path.clone(),
                        //          chunk.ref_simple,
                        //          chunk.data);
                        // }
                        Some(4) => {
                            println!("TOP LEVEL: Path: {:?}", path); 
                            let instrs = chunk.data.unwrap().chunks(28);
                            for (i, ins) in instrs.enumerate() {
                                if ins.len() >= 21 {
                                    println!("{}, ref_data: {}", 
                                            i + 1,
                                         ins[21]);
                                }
                                let tmp = Instruction {
                                    opcode: unsafe {std::mem::transmute(ins[21]) },
                                    switches: Vec::new()
                                };
                                fmp_file.scripts.get_mut(&x.parse().unwrap()).unwrap().instructions.push(tmp);
                            }
                        },
                        None => {
                            if chunk.ctype == ChunkType::DataSegment {
                                // println!("Data: {:?}. Segment: {:?}. Data: {:?}", chunk.path, chunk.segment_idx, chunk.data)
                            } else {
                                // println!("Instruction: {:?}. Data: {:?}", chunk.ctype, chunk.data)
                            }
                        }
                        x => {
                            println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                                 &path.clone(),
                                 chunk.ref_simple,
                                 chunk.data);
                        }
                    }
                },
                ["17", "1", x, ..] => {
                    if chunk.ctype == ChunkType::PathPush 
                        || chunk.ctype == ChunkType::PathPop {
                        continue;
                    }
                    
                    else if chunk.ctype == ChunkType::RefSimple {
                        match chunk.ref_simple {
                            Some(16) => {
                                let handle = fmp_file.scripts.get_mut(&x.parse().unwrap());
                                if handle.is_none() {
                                    let tmp = component::FMComponentScript {
                                        script_name: fm_string_decrypt(chunk.data.unwrap()),
                                        instructions: Vec::new(),
                                        create_by_user: String::new(),
                                        created_by_account: String::new(),
                                    };

                                    fmp_file.scripts.insert(path.last().unwrap().parse().unwrap(), tmp);
                                } else {
                                    handle.unwrap().script_name = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                                }
                            },
                            _ => {
                                // println!("{}", fm_string_decrypt(chunk.data.unwrap()));
                            }
                        }
                    }
                    // if chunk.ref_simple == Some(16) {
                        // println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                        //      &path.clone(),
                        //      chunk.ref_simple,
                        //      s);
                    // } else {
                        // println!("Path: {:?}. reference: {:?}, data: {:?}, ref_data: {:?}", &path.clone(),
                        //      chunk.ref_simple,
                        //      chunk.data,
                        //      chunk.ref_data);
                    // }
                },
                _ => { 
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    // if chunk.ref_simple == Some(2) {
                    //     // println!("NEW PATH FOUND");
                    // // } else if chunk.ref_simple.unwrap() == 2 {
                    if !path.is_empty() {
                        // println!("Path: {:?}. reference: {:?}, string: {:?}, ref_data: {:?}", 
                        //      &path.clone(),
                        //      chunk.ref_simple,
                        //      s, chunk.data);
                    }
                }
            }
        }
        idx = sectors[idx].next;
    }

    return fmp_file;
}
