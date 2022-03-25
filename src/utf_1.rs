/// This module handle the now deprecated UTF-1 encoding.

//use crate::unicode_encoding::UnicodeEncodingError::*;
use crate::unicode_encoding::UnicodeEncodingError;
use crate::unicode_encoding::UnicodeEncoding;
use crate::utf_32::Utf32;

/// A very basic wrapper for UTF-1 encoded data.
pub struct Utf1 {
    pub data: Vec<u8>
}

impl UnicodeEncoding for Utf1 {
    /// Convert UTF-32 data to UTF-1.
    fn from_utf_32(data_utf_32: &Utf32) -> Self {
        let mut data: Vec<u8> = Vec::new();
        for glyph in &data_utf_32.data {
            for new_byte in utf_32_glyph_to_utf_1(*glyph) {
                data.push(new_byte);
            }
        }
        let utf = Utf1{data: data};
        return utf;
    }

    /// Convert UFT-1 data to UTF-32.
    fn to_utf_32(&self) -> Utf32 {
        let mut index: usize = 0;
        let mut data: Vec<u32> = Vec::new();
        while index < self.data.len() {
            let (glyph, len) = utf_1_glyph_to_utf_32(&self.data, index);
            data.push(glyph);
            index += len;
        }
        return Utf32{data: data};
    }

    /// Convert the instance of `Utf1` type to a vector of byte.
    /// No transformation is needed.
    fn to_bytes(&self, _big_endian: bool) -> Vec<u8> {
        let ret = self.data.clone();
        return ret;
    }

    /// Consider a stream of UTF-1 encoded byte and turn it into a `Utf8` type.
    /// It only copies the bytes
    fn from_bytes_no_check(bytes: &[u8], _big_endian: bool) -> Result<Self, UnicodeEncodingError> {
        let ret = Utf1{data: bytes.to_vec()};
        return Ok(ret);
    }
    
}

/* ---------------------------- Helper functions ---------------------------- */

// The source used to make this code is here: https://web.archive.org/web/20150318032101/http://kikaku.itscj.ipsj.or.jp/ISO-IR/178.pdf

// Constants used to easily manipulate UTF-1 encoded values.
const     FIRST_CATEGORY_GLYPH_LIMIT: u32 = 0xA0;
const    SECOND_CATEGORY_GLYPH_LIMIT: u32 = 0x100;
const     THIRD_CATEGORY_GLYPH_LIMIT: u32 = 0x4016;
const    FOURTH_CATEGORY_GLYPH_LIMIT: u32 = 0x38E2E;

const                   UTF_1_MODULO: u32 = 0xBE;

const      THIRD_CATEGORY_GLYPH_TERM: u32 = 0xA1;
const     FOURTH_CATEGORY_GLYPH_TERM: u32 = 0xF6;
const      FIFTH_CATEGORY_GLYPH_TERM: u32 = 0xFC;

const  FIRST_CATEGORY_SEQUENCE_LIMIT: u32 = FIRST_CATEGORY_GLYPH_LIMIT;
const SECOND_CATEGORY_SEQUENCE_LIMIT: u32 = 0xF6;
const  THIRD_CATEGORY_SEQUENCE_LIMIT: u32 = 0xFC;

/// Transform function with a lot of magic numbers I don't fully understand. ^^'
fn t(z: u8) -> u8 {
    if z < 0x5E {
        z + 0x21
    } else if z < 0xBE {
        z + 0x42
    } else if z < 0xDF {
        z - 0xBE
    } else {
        z - 0x60
    }
}

/// Convert an UTF-32 glyph into the equivalents UTF-1 bytes.
fn utf_32_glyph_to_utf_1(glyph: u32) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    if glyph < FIRST_CATEGORY_GLYPH_LIMIT {
        ret.push(glyph as u8);
    } else if glyph < SECOND_CATEGORY_GLYPH_LIMIT {
        ret.push(FIRST_CATEGORY_GLYPH_LIMIT as u8);
        ret.push(glyph as u8);
    } else if glyph < THIRD_CATEGORY_GLYPH_LIMIT {
        let y = glyph - SECOND_CATEGORY_GLYPH_LIMIT;
        ret.push((THIRD_CATEGORY_GLYPH_TERM + y / UTF_1_MODULO) as u8);
        ret.push(t((y % UTF_1_MODULO) as u8));
    } else if glyph < FOURTH_CATEGORY_GLYPH_LIMIT {
        let y = glyph - THIRD_CATEGORY_GLYPH_LIMIT;
        ret.push((FOURTH_CATEGORY_GLYPH_TERM + y / UTF_1_MODULO / UTF_1_MODULO) as u8);
        ret.push(t((y / UTF_1_MODULO % UTF_1_MODULO) as u8));
        ret.push(t((y % UTF_1_MODULO) as u8));
    } else {
        let y = glyph - FOURTH_CATEGORY_GLYPH_LIMIT;
        ret.push((FIFTH_CATEGORY_GLYPH_TERM + y / UTF_1_MODULO / UTF_1_MODULO / UTF_1_MODULO / UTF_1_MODULO) as u8);
        ret.push(t((y / UTF_1_MODULO / UTF_1_MODULO / UTF_1_MODULO % UTF_1_MODULO) as u8));
        ret.push(t((y / UTF_1_MODULO / UTF_1_MODULO % UTF_1_MODULO) as u8));
        ret.push(t((y / UTF_1_MODULO % UTF_1_MODULO) as u8));
        ret.push(t((y % UTF_1_MODULO) as u8));
    }
    return ret;
}

/// Inverse transform of t
fn u(z: u8) -> u8 {
    if z < 0x21 {
        z + 0xBE
    } else if z < 0x7F {
        z - 0x21
    } else if z < 0xA0 {
        z + 0x60
    } else {
        z - 0x42
    }
}

/// U as u32
fn uu(z: u8) -> u32 {
    return u(z) as u32;
}

/// Convert an UTF-1 glyph into UTF-32 and tells how many bytes are making this
/// glyph.
/// The inputs are a stream of UTF-1 encoded data and the index of the
/// beginning of the new glyph. The return value are the number of char used to
/// encode the glyph.
/// If the glyph does not makes sense, an error will be raised.
fn utf_1_glyph_to_utf_32(utf1_data: &Vec<u8>, start: usize) -> (u32, usize) {
    let first_byte = utf1_data[start] as u8;
    if first_byte < FIRST_CATEGORY_SEQUENCE_LIMIT as u8 {
        return (first_byte as u32, 1);
    } else if first_byte == FIRST_CATEGORY_SEQUENCE_LIMIT as u8 {
        return (utf1_data[start+1] as u32, 2);
    } else if first_byte < SECOND_CATEGORY_SEQUENCE_LIMIT as u8 {
        let mut ret = ((first_byte as u32)- 1 - FIRST_CATEGORY_SEQUENCE_LIMIT) * UTF_1_MODULO;
        ret += uu(utf1_data[start+1]);
        ret += SECOND_CATEGORY_GLYPH_LIMIT;
        return (ret, 2);
    } else if first_byte < THIRD_CATEGORY_SEQUENCE_LIMIT as u8 {
        let mut ret = ((first_byte as u32) - SECOND_CATEGORY_SEQUENCE_LIMIT) * UTF_1_MODULO * UTF_1_MODULO;
        ret += uu(utf1_data[start+1]) * UTF_1_MODULO;
        ret += uu(utf1_data[start+2]);
        ret += THIRD_CATEGORY_GLYPH_LIMIT;
        return (ret, 3);
    } else {
        let mut ret = ((first_byte as u32) - THIRD_CATEGORY_SEQUENCE_LIMIT) * UTF_1_MODULO * UTF_1_MODULO * UTF_1_MODULO * UTF_1_MODULO;
        ret += uu(utf1_data[start+1]) * UTF_1_MODULO * UTF_1_MODULO * UTF_1_MODULO;
        ret += uu(utf1_data[start+2]) * UTF_1_MODULO * UTF_1_MODULO;
        ret += uu(utf1_data[start+3]) * UTF_1_MODULO;
        ret += uu(utf1_data[start+4]);
        ret += FOURTH_CATEGORY_GLYPH_LIMIT;
        return (ret, 5);
    }
}

/* --------------------------------- Testing -------------------------------- */

#[test]
/// Test that various code-points are converted right.
fn test_utf_32_to_utf_1_raw() {
    fn simple_conv(glyph: u32, vec: Vec<u8>) {
        let conv = utf_32_glyph_to_utf_1(glyph);
        assert_eq!(conv, vec);
    }
    simple_conv(0x000045, vec![0x45]);
    simple_conv(0x00007F, vec![0x7F]);
    simple_conv(0x000080, vec![0x80]);
    simple_conv(0x00009F, vec![0x9F]);
    simple_conv(0x0000A0, vec![0xA0, 0xA0]);
    simple_conv(0x0000BF, vec![0xA0, 0xBF]);
    simple_conv(0x0000C0, vec![0xA0, 0xC0]);
    simple_conv(0x0000FF, vec![0xA0, 0xFF]);
    simple_conv(0x000100, vec![0xA1, 0x21]);
    simple_conv(0x00015D, vec![0xA1, 0x7E]);
    simple_conv(0x0001BD, vec![0xA1, 0xFF]);
    simple_conv(0x0001BE, vec![0xA2, 0x21]);
    simple_conv(0x0007FF, vec![0xAA, 0x72]);
    simple_conv(0x000800, vec![0xAA, 0x73]);
    simple_conv(0x000FFF, vec![0xB5, 0x48]);
    simple_conv(0x001000, vec![0xB5, 0x49]);
    simple_conv(0x00D7FF, vec![0xF7, 0x2F, 0xC3]);
    simple_conv(0x00FDEF, vec![0xF7, 0x62, 0xD9]);
    simple_conv(0x038E2D, vec![0xFB, 0xFF, 0xFF]);
    simple_conv(0x038E2E, vec![0xFC, 0x21, 0x21, 0x21, 0x21]);
    simple_conv(0x10FFFF, vec![0xFC, 0x21, 0x39, 0x6E, 0x6C]);
}

#[test]
/// Tests that t and u are indeed the inverse of one another.
fn test_u_v() {
    for i in 0..0x100 {
        let i_u8 = i as u8;
        assert_eq!(t(u(i_u8)), i_u8);
    }
}

#[test]
/// Test that various code-points are converted back right.
fn test_utf_32_to_utf_1_and_back() {
    fn double_conv(glyph: u32) {
        let conv = utf_32_glyph_to_utf_1(glyph);
        let (conv_back, size) = utf_1_glyph_to_utf_32(&conv, 0);
        assert_eq!(glyph, conv_back);
        assert_eq!(size, conv.len());
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
    double_conv(0x0007FF);
    double_conv(0x000800);
    double_conv(0x000FFF);
    double_conv(0x001000);
    double_conv(0x00D7FF);
    double_conv(0x00FDEF);
    double_conv(0x038E2D);
    double_conv(0x038E2E);
    double_conv(0x10FFFF);
}

