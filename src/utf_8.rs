/// The UTF-8 module is quite important as it can be used to convert Rust's
/// strings into the other unicode encoding types.

use crate::unicode_encoding::UnicodeEncodingError::*;
use crate::unicode_encoding::UnicodeEncodingError;
use crate::unicode_encoding::UnicodeEncoding;
use crate::utf_32::Utf32;

/// A very basic wrapper for UTF-8 encoded data.
pub struct Utf8 {
    pub data: Vec<u8>
}

impl Utf8 {
    /// Converts a Rust String into `Utf8` struct.
    pub fn from_string(s: &str) -> Result<Self, UnicodeEncodingError> {
        let utf = Utf8{data: s.to_string().as_bytes().to_vec()};
        match utf.check_sanity() {
            NoError => return Ok(utf),
            x => return Err(x),
        }
    }

    /// Converts a `Utf8` struct to a Rust string
    pub fn to_string(&self) -> String {
        let bytes = self.data.clone();
        let ret = std::str::from_utf8(&bytes).unwrap().to_string(); // As we are manipulating sanitized UTF-8 data it _should_ be OK to use unwrap here.
        return ret;
    }

    /// Goes through a whole UTF-8 data to ensure that it is valid.
    fn check_sanity_utf8(&self) -> UnicodeEncodingError {
        let mut index: usize = 0;
        while index < self.data.len() {
            let (_glyph, len) = match utf_8_glyph_to_utf_32(&self.data, index) {
                Err(x) => {return x;},
                Ok(x) => x,
            };
            index += len;
            check_index_ok(index, &self.data);
        }
        return NoError;
    }


}

impl UnicodeEncoding for Utf8 {
    /// Convert UTF-32 data to UTF-8.
    fn from_utf_32(data_utf_32: &Utf32) -> Self {
        let mut data: Vec<u8> = Vec::new();
        for glyph in &data_utf_32.data {
            for new_byte in utf_32_glyph_to_utf_8(*glyph) {
                data.push(new_byte);
            }
        }
        let utf = Utf8{data: data};
        return utf;
    }

    /// Convert UFT-8 data to UTF-32.
    fn to_utf_32(&self) -> Utf32 {
        let mut index: usize = 0;
        let mut data: Vec<u32> = Vec::new();
        while index < self.data.len() {
            let (glyph, len) = match utf_8_glyph_to_utf_32(&self.data, index) {
                Err(_) => {panic!("[UNICODE ENCODING ERROR] Invalid UTF-8 glyph. This should not have happen if the source was safely generated with from_string or from_bytes. This could happen if from_string_no_check was used. This need to be corrected from the library's user side.");},
                Ok(x) => x,
            };
            data.push(glyph);
            index += len;
            check_index_ok(index, &self.data);
        }
        return Utf32{data: data};
    }

    /// Convert the instance of `Utf8` type to a vector of byte.
    /// No transformation is needed.
    fn to_bytes(&self, _big_endian: bool) -> Vec<u8> {
        let ret = self.data.clone();
        return ret;
    }

    /// Consider a stream of UTF-8 encoded byte and turn it into a `Utf8` type.
    /// It only copies the bytes.
    fn from_bytes_no_check(bytes: &[u8], _big_endian: bool) -> Result<Self, UnicodeEncodingError> {
        let ret = Utf8{data: bytes.to_vec()};
        match ret.check_sanity_utf8() {
            NoError => {return Ok(ret);},
            x => {return Err(x)},
        }
    }

}

/* ---------------------------- Helper functions ---------------------------- */

// Constants used to easy the manipulation of UTF-8 encoded values.
const   ONE_BYTE_GLYPH_MASK: u8 = 0b1000_0000;
const   ONE_BYTE_GLYPH_CODE: u8 = 0b0000_0000;
const   TWO_BYTE_GLYPH_MASK: u8 = 0b1110_0000;
const   TWO_BYTE_GLYPH_CODE: u8 = 0b1100_0000;
const THREE_BYTE_GLYPH_MASK: u8 = 0b1111_0000;
const THREE_BYTE_GLYPH_CODE: u8 = 0b1110_0000;
const  FOUR_BYTE_GLYPH_MASK: u8 = 0b1111_1000;
const  FOUR_BYTE_GLYPH_CODE: u8 = 0b1111_0000;
const   CNT_BYTE_GLYPH_MASK: u8 = 0b1100_0000;
const   CNT_BYTE_GLYPH_CODE: u8 = 0b1000_0000;

// Constants packed into an array for easy access.
const GLYPH_MASK_ARR: [u8; 4] = [ONE_BYTE_GLYPH_MASK,
                                 TWO_BYTE_GLYPH_MASK,
                                 THREE_BYTE_GLYPH_MASK,
                                 FOUR_BYTE_GLYPH_MASK];
const GLYPH_CODE_ARR: [u8; 4] = [ONE_BYTE_GLYPH_CODE,
                                 TWO_BYTE_GLYPH_CODE,
                                 THREE_BYTE_GLYPH_CODE,
                                 FOUR_BYTE_GLYPH_CODE];

/// Count the number of used bits in a number assuming all left-padding zeros
/// are unused.
fn number_non_nul_bits(n: u32) -> usize {
    for i in 0..32 {
        if n >> i == 0 {
            return i;
        }
    }
    panic!("[IMPOSSIBLE ERROR] There is not enough bits in n for this to happen.");
}

/// Convert an UTF-32 glyph into the equivalents UTF-8 bytes.
fn utf_32_glyph_to_utf_8(glyph: u32) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    let nnnb = number_non_nul_bits(glyph);
    if nnnb <= 7 {
        ret.push((glyph & 0xFF) as u8);
    } else if nnnb <= 11 {
        ret.push(TWO_BYTE_GLYPH_CODE   | ((glyph >>  6) as u8) & !TWO_BYTE_GLYPH_CODE);
        ret.push(CNT_BYTE_GLYPH_CODE   | ((glyph >>  0) as u8) & !CNT_BYTE_GLYPH_MASK);
    } else if nnnb <= 16 {
        ret.push(THREE_BYTE_GLYPH_CODE | ((glyph >> 12) as u8) & !THREE_BYTE_GLYPH_MASK);
        ret.push(CNT_BYTE_GLYPH_CODE   | ((glyph >>  6) as u8) & !CNT_BYTE_GLYPH_MASK);
        ret.push(CNT_BYTE_GLYPH_CODE   | ((glyph >>  0) as u8) & !CNT_BYTE_GLYPH_MASK);
    } else if nnnb <= 21 {
        ret.push(FOUR_BYTE_GLYPH_CODE  | ((glyph >> 18) as u8) & !FOUR_BYTE_GLYPH_MASK);
        ret.push(CNT_BYTE_GLYPH_CODE   | ((glyph >> 12) as u8) & !CNT_BYTE_GLYPH_MASK);
        ret.push(CNT_BYTE_GLYPH_CODE   | ((glyph >>  6) as u8) & !CNT_BYTE_GLYPH_MASK);
        ret.push(CNT_BYTE_GLYPH_CODE   | ((glyph >>  0) as u8) & !CNT_BYTE_GLYPH_MASK);
    } else {
        panic!("[UNICODE ENCODING ERROR] Invalid UTF-32 glyph. This should not have happen if the source was safely generated with from_string or from_bytes. This could happen if from_string_no_check was used. This need to be corrected from the library's user side.");
    }
    return ret;
}

/// Convert an UTF-8 glyph into UTF-32 and tells how many bytes are making this
/// glyph.
/// The inputs are a stream of UTF-8 encoded data and the index of the
/// beginning of the new glyph. The return value are the number of char used to
/// encode the glyph.
/// If the glyph does not makes sense, an error will be raised.
fn utf_8_glyph_to_utf_32(utf8_data: &Vec<u8>, start: usize) -> Result<(u32, usize), UnicodeEncodingError> {
    let mut glyph_len = 0;
    let mut glyph: u32 = 0;
    for i in 0..4 {
        if (utf8_data[start] & GLYPH_MASK_ARR[i]) == GLYPH_CODE_ARR[i] {
            glyph_len = i + 1;
            glyph = (utf8_data[start] & !GLYPH_MASK_ARR[i]) as u32;
            break;
        }
    }
    if glyph_len == 0 {
        return Err(InvalidUtf8Prefix);
    }
    if glyph_len + start > utf8_data.len() {
        return Err(MissingEncodedBytes);
    }
    for i in 1..glyph_len {
        if utf8_data[start+i] & CNT_BYTE_GLYPH_MASK != CNT_BYTE_GLYPH_CODE {
            return Err(IncoherentUtf8Codepoint);
        }
        glyph <<= 6;
        glyph |= (utf8_data[start+i] & !CNT_BYTE_GLYPH_MASK) as u32;
    }
    return Ok((glyph, glyph_len));
}

/// Ensure that an index that is going to be used is not too big. If so, a
/// panic is caused. This should not be needed as there is already checks to
/// ensure that there is no missing bytes in UTF-8 glyphs.
fn check_index_ok(index: usize, bytes: &Vec<u8>) {
    if index > bytes.len() {
        panic!("[UNICODE ENCODING ERROR] Some bytes are missing to encode of glyph. This is a library issue as a MissingEncodedBytes error should have been raised earlier in that case.\n");
    }
}

/* --------------------------------- Testing -------------------------------- */

#[test]
/// Test that the conversion of glyphs between UTF-8 and UTF-32 works.
fn test_utf_32_to_utf_8_and_back() {

    fn double_conv(glyph: u32) {
        let conv = utf_32_glyph_to_utf_8(glyph);
        let (conv_back, len) = utf_8_glyph_to_utf_32(&conv, 0).unwrap();
        println!("len of glyph {}: {}", conv_back, len);
        assert_eq!(glyph, conv_back);
    }

    const CODE_TO_TEST: [u32; 8] = ['a' as u32, 'e' as u32, 0x00E9, 0x00A4, 0x3162, 0x315F, 0x1F60E, 0x1F424];
    for glyph in CODE_TO_TEST {
        double_conv(glyph);
    }
}

#[test]
/// Test that the conversion from string to UTF-8 and back works.
fn str_to_utf_8_and_back() {
    let s = "Laé§çà→̉ỏ";
    let conv = Utf8::from_string(s).unwrap();
    let conv_back = conv.to_string();
    assert_eq!(s, conv_back);
}

#[test]
/// Test that various code-points are converted right.
fn test_utf_32_to_utf_8_raw() {
    fn simple_conv(glyph: u32, vec: Vec<u8>) {
        let conv = utf_32_glyph_to_utf_8(glyph);
        assert_eq!(conv, vec);
    }
    simple_conv(0x000045, vec![0x45]);
    simple_conv(0x00007F, vec![0x7F]);
    simple_conv(0x000080, vec![0xC2, 0x80]);
    simple_conv(0x00009F, vec![0xC2, 0x9F]);
    simple_conv(0x0000A0, vec![0xC2, 0xA0]);
    simple_conv(0x0000BF, vec![0xC2, 0xBF]);
    simple_conv(0x0000C0, vec![0xC3, 0x80]);
    simple_conv(0x0000FF, vec![0xC3, 0xBF]);
    simple_conv(0x000100, vec![0xC4, 0x80]);
    simple_conv(0x00015D, vec![0xC5, 0x9D]);
    simple_conv(0x0001BD, vec![0xC6, 0xBD]);
    simple_conv(0x0001BE, vec![0xC6, 0xBE]);
    simple_conv(0x000205, vec![0xC8, 0x85]);
    simple_conv(0x0007FF, vec![0xDF, 0xBF]);
    simple_conv(0x000800, vec![0xE0, 0xA0, 0x80]);
    simple_conv(0x000FFF, vec![0xE0, 0xBF, 0xBF]);
    simple_conv(0x001000, vec![0xE1, 0x80, 0x80]);
    simple_conv(0x010400, vec![0xF0, 0x90, 0x90, 0x80]);
    simple_conv(0x10FFFF, vec![0xF4, 0x8F, 0xBF, 0xBF]);
}

#[test]
/// Test that various code-points are converted right both ways.
fn test_utf_32_to_utf_8_and_back_plus() {
    fn double_conv(glyph: u32) {
        let conv = utf_32_glyph_to_utf_8(glyph);
        let (conv_back, size) = utf_8_glyph_to_utf_32(&conv, 0).unwrap();
        assert_eq!(conv_back, glyph);
        assert_eq!(conv.len(), size);
    }
    double_conv(0x000045);
    double_conv(0x00007F);
    double_conv(0x000080);
    double_conv(0x00009F);
    double_conv(0x0000A0);
    double_conv(0x0000BF);
    double_conv(0x0000C0);
    double_conv(0x0000FF);
    double_conv(0x000100);
    double_conv(0x00015D);
    double_conv(0x0001BD);
    double_conv(0x0001BE);
    double_conv(0x000205);
    double_conv(0x0007FF);
    double_conv(0x000800);
    double_conv(0x000FFF);
    double_conv(0x001000);
    double_conv(0x010400);
    double_conv(0x10FFFF);
}

