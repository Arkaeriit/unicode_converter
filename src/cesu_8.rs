/// The CESU-8 module handle the __Compatibility Encoding Scheme for
/// UTF-16: 8-Bit__.

use crate::unicode_encoding::UnicodeEncodingError::*;
use crate::unicode_encoding::UnicodeEncodingError;
use crate::unicode_encoding::UnicodeEncoding;
use crate::endian_aware_byte_streamer;
use crate::utf_32::Utf32;
use crate::utf_16::Utf16;
use crate::utf_8::Utf8;

/// A very basic wrapper for CESU-8 encoded bytes
pub struct Cesu8 {
    /// The list of CESU-8 encoded bytes.
    data: Vec<u8>
}

/* ---------------------------- Helper functions ---------------------------- */

const SMALL_DATA_LIMIT: u32 = 0xFFFF;

fn utf_32_glyph_to_cesu_8(glyph: u32) -> Vec<u8> {
    let glyph_in_vec = Utf32{data: vec![glyph]};
    if glyph <= SMALL_DATA_LIMIT {
        return glyph_in_vec.convert_to::<Utf8>().to_bytes(false);
    } else {
        let utf16 = glyph_in_vec.convert_to::<Utf16>();
        let mut ret: Vec<u8> = Vec::new();
        for surrogate in utf16.data {
            let surrogate_in_vec = Utf32{data: vec![surrogate as u32]};
            for byte in surrogate_in_vec.convert_to::<Utf8>().to_bytes(false) {
                ret.push(byte);
            }
        }
        return ret;
    }
}

/* --------------------------------- Testing -------------------------------- */

#[test]
fn test_utf_32_glyph_to_cesu_8() {
    let g1: u32 = 0x0045;
    let v1: Vec<u8> = vec![0x45];
    assert_eq!(v1, utf_32_glyph_to_cesu_8(g1));

    let g2: u32 = 0x0205;
    let v2: Vec<u8> = vec![0xC8, 0x85];
    assert_eq!(v2, utf_32_glyph_to_cesu_8(g2));

    let g3: u32 = 0x10400;
    let v3: Vec<u8> = vec![0xED, 0xA0, 0x81, 0xED, 0xB0, 0x80];
    assert_eq!(v3, utf_32_glyph_to_cesu_8(g3));
}

