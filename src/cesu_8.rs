/// The CESU-8 module handle the __Compatibility Encoding Scheme for
/// UTF-16: 8-Bit__.

use crate::unicode_encoding::UnicodeEncodingError::*;
use crate::unicode_encoding::UnicodeEncodingError;
use crate::unicode_encoding::UnicodeEncoding;
use crate::utf_32::Utf32;
use crate::utf_16::*;
use crate::utf_8::Utf8;

/// A wrapper for CESU-8 encoded bytes
pub struct Cesu8 {
    /// As CESU-8 is made of UTF-8 data, it makes sense to reuse the UTF-8 type
    /// here.
    pub data: Utf8,
}

impl UnicodeEncoding for Cesu8 {
    /// Convert UTF-32 data to CESU-8.
    fn from_utf_32(data_utf32: &Utf32)-> Self {
        let mut data: Vec<u8> = Vec::new();
        for glyph in &data_utf32.data {
            let new_bytes = utf_32_glyph_to_cesu_8(*glyph);
            for byte in new_bytes {
                data.push(byte);
            }
        }
        return Cesu8{data: Utf8::from_bytes_no_check(&data, false).unwrap()};
    }

    /// Convert CESU-8 to UTF-32.
    fn to_utf_32(&self) -> Utf32 {
        let tmp_utf32 = self.data.to_utf_32();
        let mut ret: Vec<u32> = Vec::new();
        let mut loop_pointer = 0;
        while loop_pointer < tmp_utf32.data.len() {
            if loop_pointer < tmp_utf32.data.len() - 1 {
                match compatible_codepoints(tmp_utf32.data[loop_pointer], tmp_utf32.data[loop_pointer+1]) {
                    NoError => {
                        ret.push(tmp_utf32.data[loop_pointer]);
                        loop_pointer = loop_pointer + 1;
                    },
                    AmbiguousUnpairedSurrogates => {
                        let surrogate_pair = vec![tmp_utf32.data[loop_pointer] as u16, tmp_utf32.data[loop_pointer+1] as u16];
                        let utf16_bit = Utf16{data: surrogate_pair};
                        ret.push(utf16_bit.to_utf_32().data[0]);
                        loop_pointer = loop_pointer + 2;
                    },
                    x => {
                        eprintln!("[UNICODE ENCODING ERROR] {:?}.", x);
                        panic!("This should not have happen if the source was safely generated with from_string or from_bytes. This could happen if from_string_no_check was used. This need to be corrected from the library's user side.");
                    }
                }
            } else {
                ret.push(tmp_utf32.data[loop_pointer]);
                loop_pointer = loop_pointer + 1;
            }
        }
        return Utf32{data: ret};
    }

    /// Convert the instance of `Cesu8` to a vector of bytes, all the heavy
    /// lifting in made in the UTF-8 module.
    fn to_bytes(&self, big_endian: bool) -> Vec<u8> {
        return self.data.to_bytes(big_endian);
    }

    /// Convert a stream of bytes encoded as CESU-8 into an instance of the
    /// `Cesu8` type. All the heavy lifting in made in the UTF-8 module.
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

#[test]
fn test_utf32_to_cesu_8_and_back() {
    fn conv_two_ways(glyph: u32) {
        let v = utf_32_glyph_to_cesu_8(glyph);
        let cesu = Cesu8::from_bytes_no_check(&v, false).unwrap();
        let utf32 = cesu.to_utf_32();
        let glyph_back = utf32.data[0];
        assert_eq!(glyph_back, glyph);
    }

    conv_two_ways(0x0045);
    conv_two_ways(0x0205);
    conv_two_ways(0x10400);
}

