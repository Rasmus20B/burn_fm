use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::{BTreeMap, HashMap};

use crate::fm_script_engine::fm_script_engine_instructions::{ScriptStep, INSTRUCTIONMAP};
use crate::{component, metadata_constants};
use crate::file::FmpFile;
use crate::decompile::sector;

use crate::chunk::{get_chunk_from_code, ChunkType};
use crate::encoding_util::fm_string_decrypt;

const SECTOR_SIZE : usize = 4096;

pub fn decompile_fmp12_file(path: &Path) -> FmpFile {
    
    let mut file = File::open(path).expect("unable to open file.");
    let mut fmp_file = FmpFile::new();
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
    let mut script_segments: HashMap<usize, BTreeMap<usize, Vec<u8>>> = HashMap::new();

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
                        let mut tmp = component::FMComponentRelationship::new();
                        tmp.table1 = fmp_file.table_occurrences.len() as u16;
                        tmp.table2 = chunk.data.unwrap()[2] as u16;
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
                                table_actual: fmp_file.tables.get(&(chunk.data.unwrap()[6] as usize)).unwrap().table_name.clone(),
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
                },
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
                            table_name: String::new(),
                            created_by_account: String::new(),
                            create_by_user: String::new(),
                            fields: HashMap::new() });
                            }
                            fmp_file.tables.get_mut(&(x.parse::<usize>().unwrap() - 128))
                                .unwrap().fields
                                    .insert(y.parse::<usize>().unwrap() as u16, component::FMComponentField { 
                                        data_type: String::new(),
                                        field_description: String::new(),
                                        field_name: String::new(),
                                        field_type: String::new(),
                                        created_by_account: String::new(),
                                        created_by_user: String::new() });
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
                                table_name: String::new(),
                                created_by_account: String::new(),
                                create_by_user: String::new(),
                                fields: HashMap::new() 
                            });
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
                ["17", "5", x, "4"] => {
                    println!("TOP LEVEL: Path: {:?} :: ", path); 
                    if chunk.ctype == ChunkType::PathPush {
                        let script = script_segments.get(&x.parse().unwrap());
                        if script.is_none() {
                            script_segments.insert(x.parse().unwrap(), BTreeMap::new());
                        }
                        continue;
                    } else if chunk.ctype == ChunkType::DataSegment {
                        let n = chunk.segment_idx.unwrap() as usize;
                        script_segments.get_mut(&x.parse().unwrap())
                            .unwrap()
                            .insert(n, chunk.data.unwrap().to_vec());
                    }
                },
                ["17", "5", x, ..] => {
                    if chunk.ctype == ChunkType::PathPop 
                        || chunk.ctype == ChunkType::PathPush {
                        continue;
                    }
                    if chunk.segment_idx == Some(4) {
                            let instrs = chunk.data.unwrap().chunks(28);
                            for (i, ins) in instrs.enumerate() {
                                if ins.len() >= 21 {
                                let oc = &INSTRUCTIONMAP[ins[21] as usize];
                                if oc.is_some() {
                                    let tmp = ScriptStep {
                                        opcode: oc.clone().unwrap(),
                                        index: 0,
                                        switches: Vec::new(),
                                    };
                                    fmp_file.scripts
                                        .get_mut(&x.parse().unwrap()).unwrap().instructions.push(tmp);
                                }
                                }
                            }
                    } else {
                        let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                        match &chunk.ref_simple {
                            // Some(2) => {
                            // }
                            Some(4) => {
                                let instrs = chunk.data.unwrap().chunks(28);
                                for ins in instrs {
                                    if ins.len() >= 21 {
                                        // println!("{}, ref_data: {}", 
                                        //         i + 1,
                                        //      ins[21]);
                                    let oc = &INSTRUCTIONMAP[ins[21] as usize];
                                    if oc.is_some() {
                                        let tmp = ScriptStep {
                                            opcode: oc.clone().unwrap(),
                                            index: 0,
                                            switches: Vec::new(),
                                        };
                                        fmp_file.scripts
                                            .get_mut(&x.parse().unwrap()).unwrap().instructions.push(tmp);
                                    }
                                    }
                                }
                            },
                            None => {
                                if chunk.ctype == ChunkType::DataSegment {
                                    // println!("Data: {:?}. Segment: {:?}. Data: {:?}", chunk.path, chunk.segment_idx, chunk.data)
                                } else {
                                    // println!("Instruction: {:?}. Data: {:?}", chunk.ctype, chunk.data)
                                }
                            },
                            _ => {
                                // println!("Path: {:?}. reference: {:?}, ref_data: {:?}", 
                                //      &path.clone(),
                                //      chunk.ref_simple,
                                //      chunk.data);
                            },
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
                                        arguments: Vec::new(),
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
                }
            }
        }


        idx = sectors[idx].next;
    }
    /* Assemble scripts */
    for (script, segments) in &mut script_segments {
        let mut instructions = Vec::<u8>::new();
        for s in segments {
            instructions.append(s.1);
        }

        for instr in instructions.chunks(28) {
            if instr.len() < 28 {
                continue;
            }
            let op = &INSTRUCTIONMAP[instr[21] as usize];
            if op.is_some() {
                let tmp = ScriptStep {
                    opcode: op.clone().unwrap(),
                    index: instr[2] as usize,
                    switches: vec![],
                };
                fmp_file.scripts.get_mut(&script).unwrap()
                    .instructions.push(tmp);
            }
        }
    }
    return fmp_file;
}
