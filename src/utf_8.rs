/// The UTF-8 module is quite important as it can be used to convert Rust's
/// strings into the other unicode encoding types.

use crate::unicode_encoding::UnicodeEncodingError::*;
use crate::unicode_encoding::UnicodeEncodingError;
use crate::unicode_encoding::UnicodeEncoding;
use crate::utf_32::Utf32;

/// A very basic wrapper for UTF-8 encoded data.
pub struct Utf8 {
    /// The list of UTF-8 encoded bytes. It also contains 3 null bytes for
    /// padding to ensure we don't access unauthorized memory.
    data: Vec<u8>
}

impl Utf8 {
    /// Converts a Rust String into `Utf8` struct.
    pub fn from_string(s: &str) -> Result<Self, UnicodeEncodingError> {
        let mut utf = Utf8{data: s.to_string().as_bytes().to_vec()};
        pad_with_0(&mut utf.data);
        match utf.check_sanity() {
            NoError => return Ok(utf),
            x => return Err(x),
        }
    }

    /// Converts a `Utf8` struct to a Rust string
    pub fn to_string(&self) -> String {
        let mut bytes = self.data.clone();
        remove_pad_with_0(&mut bytes);
        let ret = std::str::from_utf8(&bytes).unwrap().to_string();
        return ret;
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
        let mut utf = Utf8{data: data};
        pad_with_0(&mut utf.data);
        return utf;
    }

    /// Convert UFT-8 data to UTF-32.
    fn to_utf_32(&self) -> Utf32 {
        let mut index: usize = 0;
        let mut data: Vec<u32> = Vec::new();
        while self.data[index] != 0 {
            let (glyph, len) = utf_8_glyph_to_utf_32(&self.data, index);
            data.push(glyph);
            index += len;
        }
        return Utf32{data: data};
    }

    /// Convert the instance of `Utf8` type to a vector of byte. The only
    /// transformation needed is to remove the 3 padding bytes.
    fn to_bytes(&self, _big_endian: bool) -> Vec<u8> {
        let mut ret = self.data.clone();
        if ret.len() < 3 {
            panic!("[UNICODE CONVERTER INTERNAL ERROR] An Utf8 instance in not properly formed. It is missing 3 padding bytes. This should not happen as the Utf8 instance should all be padded with 0.");
        }
        remove_pad_with_0(&mut ret);
        return ret;
    }

    /// Consider a stream of UTF-8 encoded byte and turn it into a `Utf8` type.
    /// It only copies the bytes and then, add the 3 null bytes for padding.
    fn from_bytes_no_check(bytes: &[u8], _big_endian: bool) -> Result<Self, UnicodeEncodingError> {
        let mut ret = Utf8{data: bytes.to_vec()};
        pad_with_0(&mut ret.data);
        return Ok(ret);
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
        panic!("Invalid UTF-32 glyph.");
    }
    return ret;
}

/// Convert an UTF-8 glyph into UTF-32 and tells how much bytes are making this
/// glyph.
/// The inputs are a stream of UTF-8 encoded data and the index of the
/// beginning of the new glyph. The return value are the number of char used to
/// encode the glyph.
fn utf_8_glyph_to_utf_32(utf8_data: &Vec<u8>, start: usize) -> (u32, usize) {
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
        panic!("Invalid prefix");
    }
    for i in 1..glyph_len {
        glyph <<= 6;
        glyph |= (utf8_data[start+i] & !CNT_BYTE_GLYPH_MASK) as u32;
    }
    return (glyph, glyph_len);
}

/// Adds 3 null bytes at the end of the UTF-8 encoded data.
fn pad_with_0(data: &mut Vec<u8>) {
    for _ in 0..3 {
        data.push(0);
    }
}

/// Remove 3 null bytes from a vector of bytes.
fn remove_pad_with_0(v: &mut Vec<u8>) {
    for _ in 0..3 {
        match v.pop() {
            None => {panic!("[UNICODE CONVERTER INTERNAL ERROR] An Utf8 instance in not properly formed. It is missing 3 padding bytes. This should not happen as the Utf8 instance should all be padded with 0.");}
            Some(0) => {}
            Some(_) => {panic!("[UNICODE CONVERTER INTERNAL ERROR] An Utf8 instance does not ends with 3 padding bytes. This should not have happened.");}
        }
    }
}

/* --------------------------------- Testing -------------------------------- */

#[test]
/// Test that the conversion of glyphs between UTF-8 and UTF-32 works.
fn test_utf_32_to_utf_8_and_back() {

    fn double_conv(glyph: u32) {
        let conv = utf_32_glyph_to_utf_8(glyph);
        let (conv_back, len) = utf_8_glyph_to_utf_32(&conv, 0);
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

