use crate::{encoding_util::{put_int, put_path_int}, FmpFile};

use std::{fs::{write, File}, io::Read, path::Path};
use super::header::HEADER_INIT;
use crate::encoding_util;

pub struct Assembler {
    idx: usize,
    local_idx: u16,
    buffer: Vec<u8>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            idx: 0,
            local_idx: 0,
            buffer: vec![],
        }
    }

    fn emit_noop(&mut self) {
        self.buffer[self.idx] = 0x80;
        self.idx += 1;
    }

    fn push_directory(&mut self, dir: usize) {
        let d = put_path_int(dir);
        let ins = match d.len() {
            1 => { 0x20 }
            2 => { 0x40 }
            _ => { 0x48 }
        };

        println!("Ins: {:x}, dir: {:?}", ins, d);

        self.buffer[self.idx] = ins;
        self.idx += 1;
        self.buffer.splice(self.idx..self.idx + d.len(), d.clone());
        self.idx += d.len();
        println!("buf: {:x?}", &self.buffer[self.idx-3..self.idx]);
    }

    pub fn append_blank_chunk(&mut self) {
        self.buffer.resize(self.buffer.len() + 4096, 0);
        self.idx = self.buffer.len() - 4095;
        println!("{} / {}", self.idx, self.buffer.len());
        self.local_idx = 0;
    }

    pub fn assemble_fmp12(&mut self, schema: &FmpFile) {

        self.buffer.extend(HEADER_INIT);
        println!("SIZE: {}", HEADER_INIT.len());

        let name_header = schema.name.clone() + "/";
        self.buffer.splice(3082..3082+name_header.len(), name_header.as_bytes().to_vec());
        // println!("{} == {:?}", name_header, &self.buffer[3082..3082+name_header.len()]);

        self.append_blank_chunk();
        self.buffer.splice((self.idx+8) as usize..(self.idx+12) as usize, 4_u32.to_be_bytes());
        self.append_blank_chunk();
        self.buffer.splice((self.idx+8) as usize..(self.idx+12) as usize, 3_u32.to_be_bytes());
        self.idx += 20;
        self.emit_noop();
        self.push_directory(2);
        self.append_blank_chunk();
        self.buffer.splice((self.idx+8) as usize..(self.idx+12) as usize, 4_u32.to_be_bytes());
        self.append_blank_chunk();
        self.buffer.splice((self.idx+8) as usize..(self.idx+12) as usize, 0_u32.to_be_bytes());
    }

    pub fn emit_assembly(&self, filename: &str) {
        write(format!("{}.fmp12", filename), &self.buffer).expect("unable to write to file.");
    }
}

