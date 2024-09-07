use std::char::decode_utf16;
use std::fs::{File, write};
use std::io::Read;
use std::path::Path;
use std::collections::{BTreeMap, HashMap};

use crate::component::RelationComparison;
use crate::fm_script_engine::fm_script_engine_instructions::{ScriptStep, INSTRUCTIONMAP, Instruction};
use crate::{chunk, component, dbcharconv, decompile, metadata_constants};
use crate::file::FmpFile;
use crate::decompile::sector;

use crate::chunk::{get_chunk_from_code, ChunkType};
use crate::encoding_util::{fm_string_decrypt, get_int, get_path_int};

const SECTOR_SIZE : usize = 4096;

fn decompile_calculation(bytecode: &[u8]) -> String {
    let mut it = bytecode.iter().peekable();
    let mut result = String::new();
    let mut in_get = false;

    while let Some(c) = it.next() {
        match c {
            0x4 => {
                result.push('(');
            }
            0x5 => {
                result.push(')');
            }
            0x2d => {
                result.push_str("Abs");
            }
            0x9b => {
                result.push_str("Get");
            }
            0x9c => {
                match it.next().unwrap() {
                    0x1d => {
                        result.push_str("CurrentTime");
                    }
                    0x20 => {
                        result.push_str("AccountName");
                    }
                    0x49 => {
                        result.push_str("DocumentsPath");
                    }
                    0x5d => {
                        result.push_str("DocumentsPathListing");
                    }
                    _ => {}
                }
            }
            0x9d => {
                result.push_str("Acos");
            }
            0xfb => {
                match it.next().unwrap() {
                    0x3 => { result.push_str("Char")}
                    _ => eprintln!("unrecognized intrinsic.")
                }
            }
            0x10 => {
                /* decode number */
                for i in 0..19 {
                    let cur = it.next();
                    if i == 8 {
                        result.push_str(&cur.unwrap().to_string());
                    }
                }
            },
            0x13 => {
                /* Processing String */
                let n = it.next();
                let mut s = String::new();
                for i in 1..=*n.unwrap() as usize {
                    s.push(*it.next().unwrap() as char);
                }
                let mut text = String::new();
                text.push('"');
                text.push_str(&fm_string_decrypt(s.as_bytes()));
                text.push('"');

                result.push_str(&text);
            }
            0x1a => {
                /* decode variable */
                let n = it.next();
                let mut name_arr = String::new();
                for i in 1..=*n.unwrap() as usize {
                    name_arr.push(*it.next().unwrap() as char);
                }
                let name = fm_string_decrypt(name_arr.as_bytes());
                result.push_str(&name);
            },
            0x25 => {
                result.push('+');
            }
            0x26 => {
                result.push('-');
            }
            0x27 => {
                result.push('*');
            }
            0x28 => {
                result.push('/');
            },
            0x41 => {
                result.push('<');
            }
            0x43 => {
                result.push_str("<=");
            }
            0x44 => {
                result.push_str("==");
            }
            0x46 => {
                result.push_str("!=");
            }
            0x47 => {
                result.push_str(">=");
            }
            0x49 => {
                result.push('>');
            }
            0x50 => {
                result.push('&');
            }
            0xC => {
                result.push(' ');
            }
            _ => {

            }
        }

    }
    return result;
}

fn print_chunk(chunk: &chunk::Chunk, path: &Vec<String>) {
    match chunk.ctype {
        ChunkType::DataSegment => {
            println!("Path:{:?}::segment:{:?}::data:{:?}::size:{:?}::ins:{:x}", 
                 &path.clone(),
                 chunk.segment_idx.unwrap(),
                 chunk.data.unwrap_or(&[]),
                 chunk.data.unwrap().len(),
                 chunk.code);
        },
        ChunkType::RefSimple => {
            match chunk.ref_simple.unwrap() {
                216 => chunk.ref_simple,
                _ => chunk.ref_simple
            };
            println!("Path:{:?}::reference:{:?}::ref_data:{:?}::size:{}::ins:{:x}", 
                 &path.clone(),
                 chunk.ref_simple.unwrap(),
                 chunk.data.unwrap_or(&[]),
                 chunk.data.unwrap().len(),
                 chunk.code);
        }
        ChunkType::DataSimple => {
            if chunk.data.is_some() && !chunk.data.unwrap().is_empty() {
                println!("Path:{:?}::reference:na::ref_data:{:?}::size:{}::ins:{:x}", 
                     &path.clone(),
                     chunk.data.unwrap(),
                     chunk.data.unwrap().len(),
                     chunk.code);
            }
        }
        ChunkType::RefLong => {
            let decoded : String = chunk.ref_data.unwrap()
                .chunks(2)
                .map(|x|    
                    if x.len() < 2  {
                        '\0' 
                    } else {
                        dbcharconv::decode_char(x[0], x[1])
                    }
                )
                .collect();
            println!("Path:{:?}::reference:{:?}::ref_data:{:?}::size:{}::ins:{:x}", 
                 &path.clone(),
                 decoded,
                 chunk.data.unwrap(),
                 chunk.data.unwrap().len(),
                 chunk.code);
        }
        ChunkType::PathPush => {
            println!("Path:{:?}::reference:PUSH::ref_data:{:?}::size:{}::ins:{:x}", 
                 &path.clone(),
                 chunk.data.unwrap_or(&[]),
                 chunk.data.unwrap().len(),
                 chunk.code);
        }
        ChunkType::PathPop => {
            println!("Path:{:?}::reference:POP::ref_data:{:?}::size:{}::ins:{:x}", 
                 &path.clone(),
                 chunk.data.unwrap_or(&[]),
                 0,
                 chunk.code);
        }
        ChunkType::Noop => {
            println!("Path:{:?}::reference:NOOP::ref_data:{:?}::size:{}::ins:{:x}", 
                 &path.clone(),
                 chunk.data.unwrap_or(&[]),
                 0,
                 chunk.code);
        }
    }
}

pub fn decompile_fmp12_file_with_header(path: &Path) -> FmpFile {
    let mut file = File::open(path).expect("unable to open file.");
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Unable to read file.");

    write("header.log", buffer);
    decompile_fmp12_file(path)
}

pub fn decompile_fmp12_file(path: &Path) -> FmpFile {
    let mut file = File::open(path).expect("unable to open file.");
    let mut fmp_file = FmpFile::new();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Unable to read file.");

    let mut offset = SECTOR_SIZE;
    let mut sectors = Vec::<sector::Sector>::new();

    let first = sector::get_sector(&buffer[offset..]);
    // println!("{:?} == {}", &buffer[offset+8..offset+12], first.next);
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
        // println!("sector: {}, next: {}", idx, sectors[idx].next);
        // println!("{:?} == {}", &buffer[offset+8..offset+12], sectors[idx].next);
        let mut path = Vec::<String>::new();
        offset += 20;
        while offset < bound {
            let chunk = get_chunk_from_code(&buffer, 
                                            &mut offset, 
                                            &mut path, 
                                            start).expect("Unable to decode chunk.");          
            print_chunk(&chunk, &path);

            match &path.iter().map(|s| s.as_str()).collect::<Vec<_>>().as_slice() {
                /* Examining relatinoships of table occurences */
                ["3", "17", "5", "0", "251"] => {
                    if chunk.ctype == ChunkType::DataSimple {
                        let mut tmp = component::FMComponentRelationship::new();
                        tmp.table1 = fmp_file.table_occurrences.len() as u16;
                        tmp.table2 = chunk.data.unwrap()[2] as u16;
                        if fmp_file.relationships.iter()
                            .filter(|x| x.1.table1 == tmp.table2 && x.1.table2 == tmp.table1)
                            .collect::<Vec<_>>().len() == 0 {
                            fmp_file.relationships.insert(fmp_file.relationships.len() + 1, tmp);
                        }
                    }
                },
                /* Examining table occurences */
                ["3", "17", "5", "0", ..] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    match chunk.ref_simple {
                        Some(2) => {
                            let tmp = component::FMComponentTableOccurence {
                                table_occurence_name: String::new(),
                                create_by_user: String::new(),
                                created_by_account: String::new(),
                                table_actual: chunk.data.unwrap()[6] as u16,
                                table_actual_name: String::new(),
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
                            }
                        }
                    }
                },
                ["3", "251", "5", x, "3"] => {
                    match chunk.ref_simple {
                        Some(1) => {
                            let comp = match chunk.data.unwrap()[0] {
                                0x0 => RelationComparison::Equal,
                                0x1 => RelationComparison::NotEqual,
                                0x2 => RelationComparison::Less,
                                0x3 => RelationComparison::LessEqual,
                                0x4 => RelationComparison::Greater,
                                0x5 => RelationComparison::GreaterEqual,
                                0x6 => RelationComparison::Cartesian,
                                _ => RelationComparison::Equal
                            };
                            let field1 = get_path_int(&chunk.data.unwrap()[2..=3]) - 128;
                            let field2 = get_path_int(&chunk.data.unwrap()[5..=6]) - 128;
                            fmp_file.relationships.get_mut(&x.parse().unwrap()).unwrap().comparison
                                = comp;
                            fmp_file.relationships.get_mut(&x.parse().unwrap()).unwrap().field1 =
                                field1 as u16;
                            fmp_file.relationships.get_mut(&x.parse().unwrap()).unwrap().field2 =
                                field2 as u16;
                        }
                        _ => {}
                    }
                },
                ["4", "5", ..] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                },
                /* Examing layouts */
                ["4", "1", "7", x, ..] => {
                    if chunk.ctype == ChunkType::PathPush {
                        fmp_file.layouts.insert(
                            x.parse::<usize>().unwrap(),
                            component::FMComponentLayout::new()
                            );
                        continue;
                    } else {
                        match chunk.ref_simple {
                            Some(2) => {
                                /* Byte 2 refers to table occurrence */
                                let layout_handle = fmp_file.layouts.get_mut(&x.parse().unwrap());
                                if layout_handle.is_none() {
                                } else {
                                    let occurrence = chunk.data.unwrap()[1] as usize - 128;
                                    fmp_file.layouts.get_mut(&x.parse().unwrap())
                                        .unwrap().table_occurrence = occurrence;
                                }
                            }
                            Some(16) => {
                                let layout_handle = fmp_file.layouts.get_mut(&x.parse().unwrap());
                                if layout_handle.is_none() {
                                } else {
                                    let s = fm_string_decrypt(chunk.data.unwrap());
                                    fmp_file.layouts.get_mut(&x.parse().unwrap())
                                        .unwrap().layout_name = s;
                                }

                            },
                            _ => {

                            }
                        }
                    }
                }
                [x, "3", "5", y] => {
                    if x.parse::<usize>().unwrap() >= 128 {
                        if chunk.ctype == ChunkType::PathPush {
                            if !fmp_file.tables.contains_key(&(x.parse::<usize>().unwrap() - 128)) {
                                fmp_file.tables.insert(x.parse::<usize>().unwrap() - 128,
                            component::FMComponentTable::new());
                            }
                            fmp_file.tables.get_mut(&(x.parse::<usize>().unwrap() - 128))
                                .unwrap().fields
                                    .insert(y.parse::<usize>().unwrap() as u16, 
                                            component::FMComponentField::new());
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
                ["3", "16", "1", "1"] => {
                    if chunk.ref_data.is_some() {
                        let s = chunk.ref_data;
                    }
                }
                /* Examining metadata for table */
                ["3", "16", "5", x] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    if chunk.ctype == ChunkType::PathPush {
                        if !fmp_file.tables.contains_key(&x.parse().unwrap()) {
                            fmp_file.tables.insert(x.parse::<usize>().unwrap() - 128, component::FMComponentTable::new());
                        } 
                    } else {
                        match chunk.ref_simple.unwrap_or(0) {
                            metadata_constants::COMPONENT_NAME => { 
                                fmp_file.tables.get_mut(&(x.parse::<usize>().unwrap() - 128)).unwrap().table_name = s },
                            _ => {}
                        }
                    }
                },
                /* Examining script code */
                ["17", "5", x, "4"] => {
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
                ["17", "5", script, "5", step, "128", "5"] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    match chunk.ref_simple.unwrap_or(0).to_string().as_str() {
                        "5" => {
                            let instrs = &mut fmp_file.scripts.get_mut(&script.parse().unwrap()).unwrap().instructions;
                            let instruction_idx = instrs
                                .iter()
                                .enumerate()
                                .filter(|x| x.1.index == step.parse::<usize>().unwrap())
                                .collect::<Vec<_>>()[0].0;

                            // println!("Searching for {step}. instructions for script {script} == {}", instrs.len());

                            match instrs.get(instruction_idx).unwrap().opcode {
                                Instruction::SetVariable => {
                                    instrs.get_mut(instruction_idx).unwrap().switches.push(s);
                                },
                                Instruction::ExitScript => {
                                    let bytecode = decompile_calculation(chunk.data.unwrap());
                                    instrs.get_mut(instruction_idx).unwrap().switches.push(bytecode);
                                },
                                _ => {

                                }
                            }

                            for i in &mut *instrs {
                                // println!("{:?}", i);
                            }
                        },
                        _ => {
                        }
                    }
                },
                ["17", "5", script, "5", step, "128"] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    // println!("step type: {:?}", fmp_file.scripts.get(&script.parse().unwrap()).unwrap());
                    match chunk.ref_simple.unwrap_or(0).to_string().as_str() {
                        "1" => {
                            // println!("Found variable: {}", s);
                            // let script = &mut fmp_file.scripts.get_mut(&x.parse().unwrap()).unwrap().clone();
                            // let instrs = &mut fmp_file.scripts.get_mut(&script.parse().unwrap()).unwrap().instructions;
                            // let mut step = &mut fmp_file.scripts.get_mut(&x.parse().unwrap()).unwrap()
                            //     .instructions.get_mut(&y.parse().unwrap());

                            let instr = &mut fmp_file.scripts
                                .get_mut(&script.parse().unwrap()).unwrap()
                                .instructions.iter_mut()
                                .filter(|x| x.index == step.parse::<usize>().unwrap())
                                .collect::<Vec<_>>()[0];

                            // println!("Searching for {step}. instructions for script {script}");

                            match instr.opcode {
                                Instruction::SetVariable => {
                                    instr.switches.push(s);
                                },
                                _ => {}
                            }
                        },
                        _ => {
                        }
                    }
                },
                /* Examining script data */
                ["17", "5", script, "5", step, "129", "5"] => {
                    let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                    match chunk.ref_simple.unwrap_or(0).to_string().as_str() {
                        "5" => {
                            let calc = decompile_calculation(chunk.data.unwrap());
                            let instr = &mut fmp_file.scripts.get_mut(&script.parse().unwrap()).unwrap()
                                .instructions.iter_mut().filter(|x| x.index == step.parse::<usize>().unwrap())
                                .collect::<Vec<_>>()[0];
                                instr.switches.push(calc);
                        },
                        _ => {

                        }
                    }
                    let instrs = &mut fmp_file.scripts.get_mut(&script.parse().unwrap()).unwrap().instructions;
                    // for i in instrs {
                    //     println!("{:?}", i);
                    // }
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
                                    let mut switch: Vec<String> = vec![];
                                    match oc {
                                        Some(Instruction::PerformScript) => {
                                            switch.push(
                                                fmp_file.scripts.get(&(ins[8] as usize)).unwrap().script_name.clone()
                                            );
                                        },
                                        _ => {}
                                    }
                                    let n = crate::encoding_util::get_path_int(&ins[2..ins[0] as usize + 1]);
                                    let tmp = ScriptStep {
                                        opcode: oc.clone().unwrap(),
                                        index: n,
                                        switches: switch,
                                    };
                                    let handle = &mut fmp_file.scripts
                                        .get_mut(&x.parse().unwrap()).unwrap().instructions;
                                        handle.insert(n, tmp);
                                    }
                                }
                            }
                    } else {
                        let s = fm_string_decrypt(chunk.data.unwrap_or(&[0]));
                        match &chunk.ref_simple {
                            Some(4) => {
                                let instrs = chunk.data.unwrap().chunks(28);
                                for ins in instrs {
                                    if ins.len() >= 21 {
                                    let oc = &INSTRUCTIONMAP[ins[21] as usize];
                                    if oc.is_some() {
                                        let mut switch: Vec<String> = vec![];
                                        match oc {
                                            Some(Instruction::PerformScript) => {
                                                switch.push(
                                                    fmp_file.scripts.get(&(ins[8] as usize)).unwrap().script_name.clone()
                                                );
                                            },
                                            _ => {}
                                        }
                                        let n = crate::encoding_util::get_path_int(&ins[2..ins[0] as usize + 1]);
                                        let tmp = ScriptStep {
                                            opcode: oc.clone().unwrap(),
                                            index: n,
                                            switches: switch,
                                        };
                                        let handle = &mut fmp_file.scripts
                                            .get_mut(&x.parse().unwrap()).unwrap().instructions;
                                            handle.push(tmp);
                                        }
                                    }
                                }
                            },
                            None => {
                            },
                            _ => {
                            },
                        }
                    }

                },
                /* Examining script metadata */
                ["17", "1", x, y, ..] => {
                    if chunk.ctype == ChunkType::PathPush 
                        || chunk.ctype == ChunkType::PathPop {
                        continue;
                    }

                    if chunk.ctype == ChunkType::RefSimple {
                        match chunk.ref_simple {
                            Some(16) => {
                                let handle = fmp_file.scripts.get_mut(&y.parse().unwrap());
                                if handle.is_none() {
                                    let tmp = component::FMComponentScript {
                                        script_name: fm_string_decrypt(chunk.data.unwrap()),
                                        instructions: vec![],
                                        create_by_user: String::new(),
                                        arguments: Vec::new(),
                                        created_by_account: String::new(),
                                    };
                                    let res = fmp_file.scripts.insert(y.parse().unwrap(), tmp);
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
                    // } else {
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
                let mut switch: Vec<String> = vec![];
                match op {
                    Some(Instruction::PerformScript) => {
                        switch.push(fmp_file.scripts.get(&(instr[8] as usize)).unwrap().script_name.clone());
                    },
                    _ => {}
                }
                let tmp = ScriptStep {
                    opcode: op.clone().unwrap(),
                    index: crate::encoding_util::get_path_int(&instr[2..instr[0] as usize + 1]),
                    switches: switch,
                };
                let handle = fmp_file.scripts.get_mut(&script).unwrap();
                handle.instructions.insert(handle.instructions.len(), tmp);
            }

        }
    }
    // for script in &fmp_file.scripts {
    //     println!("{:?}", script.1.script_name);
    //     for step in &script.1.instructions {
    //         println!("{:?}", step);
    //     }
    // }
    return fmp_file;
}
