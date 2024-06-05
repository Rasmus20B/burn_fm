
pub fn get_path_int(bytes : &[u8]) -> usize {
    match bytes.len() {
        1 => bytes[0] as usize,
        2 => 0x80 + ((bytes[0] as usize) & 0x7f << 8) + bytes[1] as usize,
        _ => 0
    }
}

pub fn get_int(bytes: &[u8]) -> usize {
    match bytes.len() {
        1 => bytes[0] as usize,
        2 => ((bytes[0] as usize) << 8) + (bytes[1] as usize),
        4 => (get_int(&bytes[0..2]) << 16) + get_int(&bytes[2..4]),
        _ => 0
    }
}

pub fn fm_string_decrypt(bytes: &[u8]) -> String {
    match String::from_utf8(bytes
                                 .into_iter()
                                 .map(|c| c ^ 0x5A)
                                 .collect::<Vec<u8>>()) {
        Ok(v) => v.to_string(),
        Err(e) => "value not utf-8.".to_string()
    }
}
