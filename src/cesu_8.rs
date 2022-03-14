/// The CESU-8 module handle the __Compatibility Encoding Scheme for
/// UTF-16: 8-Bit__.

use crate::unicode_encoding::UnicodeEncodingError::*;
use crate::unicode_encoding::UnicodeEncodingError;
use crate::unicode_encoding::UnicodeEncoding;
use crate::utf_32::Utf32;
use crate::utf_16::Utf16;
use crate::utf_8::Utf8;

/// A wrapper for CESU-8 encoded bytes
pub struct Cesu8 {
    /// As CESU-8 is made of UTF-8 data, it makes sense to reuse the UTF-8 type
    /// here.
    data: Utf8,
}

impl UnicodeEncoding for Cesu8 {
    /// Convert UTF-32 data to UTF-16.
    fn from_utf_32(data_utf32: &Utf32)-> Self {
        let mut data: Vec<u8> = Vec::new();
        for glyph in &data_utf32.data {
            let new_bytes = utf_32_glyph_to_cesu_8(*glyph);
            for byte in new_bytes {
                data.push(byte);
            }
        }
        return Cesu8{data: Utf8{data: data}};
    }

    /// Convert CESI-8 to UTF-32.
    fn to_utf_32(&self) -> Utf32 {
        return Utf32{data: vec![]};
    }

    fn to_bytes(&self, big_endian: bool) -> Vec<u8> {
        return self.data.to_bytes(big_endian);
    }

    fn from_bytes_no_check(bytes: &[u8], big_endian: bool) -> Result<Self, UnicodeEncodingError> {
        match Utf8::from_bytes_no_check(bytes, big_endian) {
            Ok(x) => Ok(Cesu8{data: x}),
            Err(y) => Err(y),
        }
    }



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

