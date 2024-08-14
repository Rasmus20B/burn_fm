use crate::FmpFile;

use super::header::HEADER_INIT;

pub fn append_blank_chunk(file: &mut Vec<u8>) {
    file.resize(file.len() + 4096, 0x0);
}

pub fn assemble_fmp12(schema: &FmpFile) -> Vec<u8> {

    let mut result = vec![];
    result.extend(HEADER_INIT);

    let name_header = schema.name.clone() + "/";

    result.splice(3082..3082+name_header.len(), name_header.as_bytes().to_vec());
    // println!("{} == {:?}", name_header, &result[3082..3082+name_header.len()]);

    append_blank_chunk(&mut result);
    return result;
}
