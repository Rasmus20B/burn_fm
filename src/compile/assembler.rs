use crate::{encoding_util::{put_int, put_path_int}, FmpFile};

use std::{fs::{write, File}, io::Read, path::Path};
use super::header::HEADER_INIT;
use crate::encoding_util;

pub struct Assembler<'a> {
    idx: usize,
    local_idx: u16,
    buffer: Vec<u8>,
    file: &'a FmpFile,
}

impl<'a> Assembler<'a> {
    pub fn new(input: &'a FmpFile) -> Self {
        Self {
            idx: 0,
            local_idx: 0,
            buffer: vec![],
            file: input,
        }
    }

    fn emit_noop(&mut self) {
        self.buffer[self.idx] = 0x80;
        self.idx += 1;
    }


    /* Only used when value is not a known datatype */
    pub fn calc_small_kv_ins(&self, val: &[u8]) -> u8 {
        let code = ((val.len() + 1).div_ceil(2)) as u8;
        if code > 0x5 {
            return 0x6;
        }

        return code;
        ((val.len() + 1).div_ceil(2)) as u8
    }

    fn emit_simple_kv(&mut self, key: u8, val: &[u8]) {
        let ins = match key {
            16 => { 0x6 }
            216 => { 0x6 }
            _ => { self.calc_small_kv_ins(val) }
        };

        self.buffer[self.idx] = ins;
        self.idx += 1;


        match ins {
            0x01 | 0x02 | 0x03 | 0x04 | 0x05 => {
                self.buffer[self.idx] = key as u8;
                self.idx += 1;
                self.buffer.splice(self.idx..self.idx+val.len(), val.to_vec());
                self.idx += val.len();
            }
            0x6 => {
                self.buffer[self.idx] = key as u8;
                println!("Ins: {} -> key {} :: {:?} :: len {}", ins, key, val, val.len() as u8);
                self.idx += 1;
                self.buffer[self.idx] = val.len() as u8;
                self.idx += 1;
                self.buffer.splice(self.idx..self.idx+val.len(), val.to_vec());
                self.idx += val.len();
            }
            _ => {}
        }
    }

    fn pop_directory(&mut self) {
        self.buffer[self.idx] = 0x40;
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
    }

    pub fn append_blank_chunk(&mut self) {
        self.buffer.resize(self.buffer.len() + 4096, 0);
        self.idx = self.buffer.len() - 4095;
        self.local_idx = 0;
    }

    pub fn emit_init_blobs(&mut self) {
        self.push_directory(2);
        self.emit_noop();
        self.emit_simple_kv(3, &[120, 104, 106, 116, 107, 120]);
        self.emit_simple_kv(5, &[4, 184]);
        self.emit_simple_kv(6, &[1, 10, 4, 179, 4, 115, 8, 179, 0, 19, 0, 0, 3, 124, 4, 0, 18, 5, 66, 128, 2, 0, 0, 0, 0, 66, 144, 0, 0, 0, 8, 2, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        self.emit_simple_kv(8, &[78, 152, 78, 152, 78, 152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 10, 10, 40, 53, 122, 104, 106, 116, 107, 116, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 8, 107, 122, 107, 110, 116, 107, 116, 107, 0, 0, 0, 21, 6, 107, 122, 107, 110, 116, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        self.emit_simple_kv(9, &[0, 0, 0, 0, 11, 251, 83, 44, 0, 0, 0, 0, 11, 251, 83, 44, 0, 0, 0, 0, 11, 251, 83, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 9, 104, 106, 107, 106, 104, 106, 104, 106, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        self.emit_simple_kv(11, &[0, 8, 7, 60, 51, 54, 63, 55, 59, 57, 98, 65, 50, 218, 23, 59, 57, 51, 52, 46, 53, 41, 50, 122, 18, 30, 218, 27, 42, 42, 54, 51, 57, 59, 46, 51, 53, 52, 41, 218, 28, 51, 54, 63, 23, 59, 49, 63, 40, 122, 10, 40, 53, 116, 59, 42, 42, 218, 25, 53, 52, 46, 63, 52, 46, 41, 218, 8, 63, 41, 53, 47, 40, 57, 63, 41, 218, 19, 52, 41, 46, 59, 54, 54, 63, 40, 218, 31, 34, 46, 63, 52, 41, 51, 53, 52, 41, 218, 30, 51, 57, 46, 51, 53, 52, 59, 40, 51, 63, 41, 218, 12, 47, 49, 63, 52, 61, 54, 41, 50, 116, 55, 42, 40, 7, 60, 51, 54, 63, 55, 59, 57, 90, 65, 50, 218, 23, 59, 57, 51, 52, 46, 53, 41, 50, 122, 18, 30, 218, 15, 41, 63, 40, 41, 218, 55, 63, 40, 51, 55, 59, 49, 218, 22, 51, 56, 40, 59, 40, 35, 218, 27, 42, 42, 54, 51, 57, 59, 46, 51, 53, 52, 122, 9, 47, 42, 42, 53, 40, 46, 218, 28, 51, 54, 63, 23, 59, 49, 63, 40, 218, 31, 34, 46, 63, 52, 41, 51, 53, 52, 41, 218, 30, 51, 57, 46, 51, 53, 52, 59, 40, 51, 63, 41, 218, 8, 15, 41, 63, 40, 116, 47, 42, 40]);
        self.emit_simple_kv(24, &[0, 0, 4, 3, 1, 1, 0, 0, 2, 1, 2, 0, 1, 0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 1, 0, 0]);
        self.pop_directory();
    }

    pub fn emit_table_metadata(&mut self) {
        self.push_directory(3);
        self.emit_noop();
        self.push_directory(16);
        self.emit_noop();
        self.push_directory(1);
        self.emit_noop();
    }

    pub fn assemble_fmp12(&mut self, schema: &FmpFile) {

        self.buffer.extend(HEADER_INIT);

        let name_header = schema.name.clone() + "/";
        self.buffer.splice(3082..3082+name_header.len(), name_header.as_bytes().to_vec());
        // println!("{} == {:?}", name_header, &self.buffer[3082..3082+name_header.len()]);

        self.append_blank_chunk();
        self.buffer.splice((self.idx+8) as usize..(self.idx+12) as usize, 4_u32.to_be_bytes());
        self.append_blank_chunk();
        self.buffer.splice((self.idx+8) as usize..(self.idx+12) as usize, 3_u32.to_be_bytes());
        self.idx += 20;
        self.emit_noop();
        self.emit_init_blobs();
        self.append_blank_chunk();
        self.buffer.splice((self.idx+8) as usize..(self.idx+12) as usize, 4_u32.to_be_bytes());
        self.idx += 20;
        self.emit_table_metadata();
        self.append_blank_chunk();
        self.buffer.splice((self.idx+8) as usize..(self.idx+12) as usize, 0_u32.to_be_bytes());
    }

    pub fn emit_assembly(&self, filename: &str) {
        write(format!("{}.fmp12", filename), &self.buffer).expect("unable to write to file.");
    }
}

#[cfg(test)]
mod tests {
    use super::Assembler;
    use crate::FmpFile;
    #[test]
    fn kv_instruction_test() {
        let tmp = FmpFile::new();
        let assembler = Assembler::new(&tmp);
        assert_eq!(0x4, assembler.calc_small_kv_ins(&[120, 104, 106, 116, 107, 120]));
        assert_eq!(0x2, assembler.calc_small_kv_ins(&[4, 184]));
        assert_eq!(0x5, assembler.calc_small_kv_ins(&[0, 0, 0, 4, 56, 1, 64, 1]));
        assert_eq!(0x6, assembler.calc_small_kv_ins(&[78, 152, 78, 152, 78, 152, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                23, 10, 10, 40, 53, 122, 104, 106, 116, 107, 116, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 8, 107, 122, 107, 110, 116, 107, 116,
                107, 0, 0, 0, 21, 6, 107, 122, 107, 110, 116, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0]));
    }
}

