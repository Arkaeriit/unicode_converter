/// The UTF-8 module is quite important as it can be used to convert Rust's
/// strings into the other unicode encoding types.

use crate::unicode_encoding::UnicodeEncoding;

/// A very basic wrapper for UTF-8 encoded data.
pub struct Utf8 {
    /// The list of UTF-8 encoded bytes.
    data: Vec<u8>
}

impl Utf8 {
    /// Converts a Rust String into `Utf8` struct.
    fn from_string(s: &str) -> Self {
        return Utf8{data: s.to_string().as_bytes().to_vec()};
    }

    /// Converts a `Utf8` struct to a Rust string
    fn to_string(&self) -> String {
        return std::str::from_utf8(&self.data).unwrap().to_string();
    }
}

//impl UnicodeEncoding for Utf8 {


//}

// Constants used to easy the manipulation of UTF-8 encoded values.
const   one_byte_glyph_mask: u8 = 0b1000_0000;
const   one_byte_glyph_code: u8 = 0b0000_0000;
const   two_byte_glyph_mask: u8 = 0b1110_0000;
const   two_byte_glyph_code: u8 = 0b1100_0000;
const three_byte_glyph_mask: u8 = 0b1111_0000;
const three_byte_glyph_code: u8 = 0b1110_0000;
const  four_byte_glyph_mask: u8 = 0b1111_1000;
const  four_byte_glyph_code: u8 = 0b1111_0000;
const   cnt_byte_glyph_mask: u8 = 0b1100_0000;
const   cnt_byte_glyph_code: u8 = 0b1000_0000;

// Constants packed into an array for easy access.
const glyph_mask_arr: [u8; 4] = [one_byte_glyph_mask,
                                 two_byte_glyph_mask,
                                 three_byte_glyph_mask,
                                 four_byte_glyph_mask];
const glyph_code_arr: [u8; 4] = [one_byte_glyph_code,
                                 two_byte_glyph_code,
                                 three_byte_glyph_code,
                                 four_byte_glyph_code];


/// Count the number of used bits in a number assuming all left-padding zeros
/// are unused.
fn number_non_nul_bits(n: u32) -> usize {
    for i in 0..32 {
        if n >> i == 0 {
            return i;
        }
    }
    panic!("Error in number_non_nul_bits. This should not have happen.");
}

/// Convert an UTF-32 glyph into the equivalents UTF-8 bytes.
fn utf_32_glyph_to_utf_8(glyph: u32) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    let nnnb = number_non_nul_bits(glyph);
    if nnnb <= 7 {
        ret.push((glyph & 0xFF) as u8);
    } else if nnnb <= 11 {
        ret.push(two_byte_glyph_code   | ((glyph >>  6) as u8) & !two_byte_glyph_code);
        ret.push(cnt_byte_glyph_code   | ((glyph >>  0) as u8) & !cnt_byte_glyph_mask);
    } else if nnnb <= 16 {
        ret.push(three_byte_glyph_code | ((glyph >> 12) as u8) & !three_byte_glyph_mask);
        ret.push(cnt_byte_glyph_code   | ((glyph >>  6) as u8) & !cnt_byte_glyph_mask);
        ret.push(cnt_byte_glyph_code   | ((glyph >>  0) as u8) & !cnt_byte_glyph_mask);
    } else if nnnb <= 21 {
        ret.push(four_byte_glyph_code  | ((glyph >> 18) as u8) & !four_byte_glyph_mask);
        ret.push(cnt_byte_glyph_code   | ((glyph >> 12) as u8) & !cnt_byte_glyph_mask);
        ret.push(cnt_byte_glyph_code   | ((glyph >>  6) as u8) & !cnt_byte_glyph_mask);
        ret.push(cnt_byte_glyph_code   | ((glyph >>  0) as u8) & !cnt_byte_glyph_mask);
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
fn utf_8_glyph_to_utf_32(utf8_data: Vec<u8>, start: usize) -> (u32, usize) {
    let mut glyph_len = 0;
    let mut glyph: u32 = 0;
    for i in 0..4 {
        if (utf8_data[start] & glyph_mask_arr[i]) == glyph_code_arr[i] {
            glyph_len = i + 1;
            glyph = (utf8_data[start] & !glyph_mask_arr[i]) as u32;
            break;
        }
    }
    if glyph_len == 0 {
        panic!("Invalid prefix");
    }
    for i in 1..glyph_len {
        glyph <<= 6;
        glyph |= (utf8_data[start+i] & !cnt_byte_glyph_mask) as u32;
    }
    return (glyph, glyph_len);
}

/* --------------------------------- Testing -------------------------------- */

fn double_conv(glyph: u32) {
    let conv = utf_32_glyph_to_utf_8(glyph);
    let (conv_back, len) = utf_8_glyph_to_utf_32(conv, 0);
    println!("len of glyph {}: {}", conv_back, len);
    assert_eq!(glyph, conv_back);
}

#[test]
fn test_u32_to_u8_and_back() {
    const code_to_test: [u32; 8] = ['a' as u32, 'e' as u32, 0x00E9, 0x00A4, 0x3162, 0x315F, 0x1F60E, 0x1F424];
    for glyph in code_to_test {
        double_conv(glyph);
    }
}
