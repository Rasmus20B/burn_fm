

pub fn get_path_int(bytes : &[u8]) -> usize {
    match bytes.len() {
        1 => bytes[0] as usize,
        2 => 0x80 + ((bytes[0] as usize) & 0x7f << 8) + bytes[1] as usize,
        _ => 0
    }
}

pub fn put_path_int(n: usize) -> Vec<u8> {
    let mut res = vec![];
    let len = n.div_ceil(255);
    res.resize(len, 0);

    match len {
        1 => { res[0] = n as u8 }
        2 => { res[1] = n as u8 + 0x7f + 1; res[0] = (n as u8).wrapping_sub(res[1]) }
        _ => { return [0].to_vec() }
    }
    return res;
}

pub fn get_int(bytes: &[u8]) -> usize {
    return match bytes.len() {
        1 => bytes[0] as usize,
        2 => ((bytes[0] as usize) << 8) + (bytes[1] as usize),
        4 => (get_int(&bytes[0..2]) << 16) + get_int(&bytes[2..4]),
        _ => 0
    }
}

pub fn put_int(mut n: usize) -> Vec<u8> {
    let mut res = vec![0, 0, 0, 0];
    let mut idx = 3;
    while n > 1 {
        let cur = n % 256;
        n /= 256;
        res[idx] = cur as u8;
        idx -= 1;
    }
    return res;
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

#[cfg(test)]
mod tests {
    use crate::encoding_util::*;

    #[test]
    fn path_int_testing() {
        assert_eq!(get_path_int(&[1]), 1);
        assert_eq!(&put_path_int(1), &[1]);
        assert_eq!(&put_path_int(2), &[2]);
        assert_eq!(get_path_int(&[]), 0);
        assert_eq!(&put_path_int(266), &[128, 138]);
        assert_eq!(get_path_int(&[128, 138]), 266);
        assert_eq!(&put_path_int(0), &[0]);
    }

    #[test]
    fn int_testing() {
        assert_eq!(put_int(67), &[0, 0, 0, 67]);
        assert_eq!(get_int(&[0, 0, 0, 67]), 67);
        assert_eq!(get_int(&[0, 0, 4, 0]), 1024);
        assert_eq!(put_int(1024), &[0, 0, 4, 0]);
    }

    #[test]
    fn string_testing() {
        assert_eq!(fm_string_decrypt(&[0x7e, 0x22]), "$x");
        assert_eq!(fm_string_decrypt(&[0x7e, 0x23]), "$y");
        assert_eq!(fm_string_decrypt(&[0x19, 0x3b, 0x34, 0x39, 0x3f, 0x36]), "Cancel");
        assert_eq!(fm_string_decrypt(&[0x32, 0x3f, 0x36, 0x36, 0x35]), "hello");
        assert_eq!(fm_string_decrypt(&[]), "");
    }
}

