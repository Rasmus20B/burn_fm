
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
        4 => (get_int(&bytes[0..2]) << 16) + get_int(&bytes[2..=3]),
        _ => 0
    }
}
