/* Double Byte Encoding/Decoding for scheme found in FileMaker File.
 * For now the encoding is a hardcoded lookup. */ 
// 12, f,
// 12, 25,
// 12, 3d,
// 12, 50,
// 12, 6b,
// 12, a3,
// 12, b0,
// 12, d3,
// 12, ec,
// 13, 5,
// 13, 1e,
// 13, 30,
// 13, 5f,
// 13, 6d,
// 13, 8e,
// 13, b3,
// 13, c8,
// 13, da,
// 14, 10,
// 14, 33,
// 14, 53,
// 14, 7b,
// 14, 8d,
// 14, 97,
// 14, 9c,
// 14, ad

pub const ENCODING_MAPPING: [(u8, u8, char); 26] = [
    (0x12, 0xf, 'a'), (0x12, 0x25, 'b'), (0x12, 0x3d, 'c'),
    (0x12, 0x50, 'd'), (0x12, 0x6b, 'e'), (0x12, 0xa3, 'f'),
    (0x12, 0xb0, 'g'), (0x12, 0xd3, 'h'), (0x12, 0xec, 'i'),
    (0x13, 0x5, 'j'), (0x13, 0x1e, 'k'), (0x13, 0x30, 'l'),
    (0x13, 0x5f, 'm'), (0x13, 0x6d, 'n'), (0x13, 0x8e, 'o'),
    (0x13, 0xb3, 'p'), (0x13, 0xc8, 'q'), (0x13, 0xda, 'r'),
    (0x14, 0x10, 's'), (0x14, 0x33, 't'), (0x14, 0x53, 'u'),
    (0x14, 0x7b, 'v'), (0x14, 0x8d, 'w'), (0x14, 0x97, 'x'),
    (0x14, 0x9c, 'y'), (0x14, 0xad, 'z')
];

pub fn decode_char(high: u8, low: u8) -> char {
    ENCODING_MAPPING.iter()
        .find(|&&(h, l, _)| h == high && l == low)
        .map(|&(_, _, ch) | ch)
        .unwrap_or('?')
}

pub fn encode_char(ch: char) -> (u8, u8) {
    ENCODING_MAPPING.iter()
        .find(|&&(_, _, c)| c == ch.to_ascii_lowercase())
        .map(|&(h, l, _) | (h, l))
        .unwrap_or((0, 0))
}

#[cfg(test)]
mod tests {
    use crate::dbcharconv::decode_char;

    use super::encode_char;

    #[test]
    fn encode_test() {
        let text = "hello";
        let encoded : Vec<(u8, u8)> = text.chars()
            .map(|c| encode_char(c))
            .collect();

        assert_eq!(
            vec![(0x12, 0xd3), (0x12, 0x6b), (0x13, 0x30), (0x13, 0x30), (0x13, 0x8e)],
            encoded
        );

        let decoded :String = encoded.iter()
            .map(|a| decode_char(a.0, a.1))
            .collect();

        assert_eq!(decoded, "hello")
    }
}



