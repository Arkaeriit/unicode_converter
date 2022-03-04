/// The UTF-16 module manipulates UTF-16 data.

use crate::unicode_encoding::UnicodeEncodingError::*;
use crate::unicode_encoding::UnicodeEncodingError;
use crate::unicode_encoding::UnicodeEncoding;
use crate::endian_aware_byte_streamer;
use crate::utf_32::Utf32;

/// A very basic wrapper for UTF-16 encoded data.
pub struct Utf16 {
    /// The list of UTF-16 encoded bytes.
    data: Vec<u16>
}

/// Push the content of vec2 into vec1
fn transvase(vec1: &mut Vec<u16>, vec2: &Vec<u16>) {
    for u in vec2 {
        vec1.push(*u);
    }
}

impl UnicodeEncoding for Utf16 {
    /// Convert UTF-32 data to UTF-16.
    fn from_utf_32(data_utf_32: &Utf32) -> Self {
        let mut data: Vec<u16> = Vec::new();
        for glyph in &data_utf_32.data {
            match utf_32_glyph_to_utf_16(*glyph) {
                Ok(new_glyph) => {transvase(&mut data, &new_glyph);},
                Err(UnpairedSurrogateNotification) => {data.push(*glyph as u16)}, // If the Unicode data have been sanitized, we are right to assume that there is no possible ambiguity with unpaired surrogates.
                Err(_) => {panic!("[UNICODE ENCODING ERROR] Invalid UTF-16 glyph. This should not have happen if the source was safely generated with from_string or from_bytes. This could happen if from_string_no_check was used. This need to be corrected from the library's user side.");},
            }
        }
        return Utf16{data: data};
    }

    /// Convert UTF-16 data to UTF-32.
    fn to_utf_32(&self) -> Utf32 {
        let mut index: usize = 0;
        let mut data: Vec<u32> = Vec::new();
        while index < self.data.len() {
            let (glyph, len) = utf_16_glyph_to_utf_32(&self.data, index);
            data.push(glyph);
            index += len;
        }
        return Utf32{data: data};
    }

    /// Converts a stream of byte that _should_ be encoded in UTF-32 into the
    /// `Utf32` type.
    fn from_bytes_no_check(bytes: &[u8], big_endian: bool) -> Result<Self, UnicodeEncodingError> {
        let ret = Utf16{data: endian_aware_byte_streamer::from_bytes::<u16>(bytes, big_endian)?};
        return Ok(ret);
    }

    /// Converts an instance of the `Utf32` type into a vector of bytes that is
    /// the UTF-32 encoded content.
    fn to_bytes(&self, big_endian: bool) -> Vec<u8> {
        return endian_aware_byte_streamer::to_bytes::<u16>(&self.data, big_endian);
    }

}

/* ---------------------------- Helper functions ---------------------------- */

// Constants used in UTF-32 / UTF-16 conversion.

const       BASIC_PLANE_1_START: u32 = 0x0000_0000;
const         BASIC_PLANE_1_END: u32 = 0x0000_D7FF;
const       BASIC_PLANE_2_START: u32 = 0x0000_E000;
const         BASIC_PLANE_2_END: u32 = 0x0000_FFFF;
const SUPPLEMENTARY_PLANE_START: u32 = 0x0001_0000;
const   SUPPLEMENTARY_PLANE_END: u32 = 0x0010_FFFF;
const  HIGH_SURROGATE: u16 = 0xD800;
const   LOW_SURROGATE: u16 = 0xDC00;
const  SURROGATE_MASK: u16 = 0b11111100_00000000;

/// Turns an UTF-32 glyph into a one or two 16 bit numbers as an UTF-16 value.
fn utf_32_glyph_to_utf_16(glyph: u32) -> Result<Vec<u16>, UnicodeEncodingError>  {
    let mut ret: Vec<u16> = Vec::new();
    if     (glyph >= BASIC_PLANE_1_START && glyph <= BASIC_PLANE_1_END)
        || (glyph >= BASIC_PLANE_2_START && glyph <= BASIC_PLANE_2_END) {
        ret.push(glyph as u16);
    } else if glyph >= SUPPLEMENTARY_PLANE_START && glyph <= SUPPLEMENTARY_PLANE_END {
        let (high_surrogate, low_surrogate) = glyph_into_surrogates(glyph);
        ret.push(high_surrogate);
        ret.push(low_surrogate);
    } else if glyph > BASIC_PLANE_1_END && glyph < BASIC_PLANE_2_START {
        return Err(UnpairedSurrogateNotification);
    } else {
        return Err(InvalidCodepointTooManyBits);
    }
    return Ok(ret);
}

/// Tells if two Unicode code-points can be next two another or if there could
/// be an ambiguity when two unpaired surrogate are next two another. If the
/// input text is proper Unicode, this should never happen.
/// Doing so ensure that the text can be safely used as UTF-16.
pub fn compatible_codepoints(glyph1: u32, glyph2: u32) -> UnicodeEncodingError {
    match utf_32_glyph_to_utf_16(glyph1) {
        Err(InvalidCodepointTooManyBits) => InvalidCodepointTooManyBits,
        Err(UnpairedSurrogateNotification) => match utf_32_glyph_to_utf_16(glyph2) {
            Err(UnpairedSurrogateNotification) => AmbiguousUnpairedSurrogates,
            Err(x) => x,
            Ok(_) => NoError,
        },
        Ok(_) => NoError,
        Err(x) => x,
    }
}

/// Turns an Unicode code-point from the supplementary plane into a high
/// surrogate and a low surrogate.
fn glyph_into_surrogates(glyph: u32) -> (u16, u16) {
    let based_glyph = glyph - SUPPLEMENTARY_PLANE_START;
    let glyph_10_msb = (based_glyph >> 10) as u16;
    println!("base: {:#x}, msb: {:#x}", based_glyph, glyph_10_msb);
    let glyph_10_lsb = (based_glyph - ((glyph_10_msb as u32) << 10)) as u16;
    let high_surrogate = glyph_10_msb | HIGH_SURROGATE;
    let  low_surrogate = glyph_10_lsb |  LOW_SURROGATE;
    return (high_surrogate, low_surrogate);
}

/// Turns high and low surrogates into a UTF-32 glyph.
fn surrogates_to_glyph(high_surrogate: u16, low_surrogate: u16) -> u32 {
    let stripped_hs = high_surrogate & !SURROGATE_MASK; 
    let stripped_ls = low_surrogate & !SURROGATE_MASK; 
    let based_glyph = ((stripped_hs as u32) << 10) | (stripped_ls as u32);
    return based_glyph + SUPPLEMENTARY_PLANE_START;
}

/// Turns the first glyph in a stream of UTF-16 encoded data into an UTF-32
/// glyph, also indicate how many 16-bit numbers form the glyph.
fn utf_16_glyph_to_utf_32(utf16_data: &Vec<u16>, start: usize) -> (u32, usize) {
    if utf16_data[start] & SURROGATE_MASK == HIGH_SURROGATE {
        if utf16_data.len() - 1 == start { // If there is no bytes afterwards, we assume it is an unpaired surrogate and we return it as-is.
            return ((utf16_data[start] as u32), 1);
        } else {
            if utf16_data[start+1] & SURROGATE_MASK == LOW_SURROGATE { // Pairs of surrogates
                return (surrogates_to_glyph(utf16_data[start], utf16_data[start+1]), 2);
            } else { // Unpaired surrogate
                return ((utf16_data[start] as u32), 1);
            }
        }
    } else { // Basic plane glyph
        return ((utf16_data[start] as u32), 1);
    }
}

/* --------------------------------- Testing -------------------------------- */

#[test]
/// Test that the conversion of glyphs between UTF-16 and UTF-32 works.
fn test_utf_16_to_utf_32_and_back() {
    fn double_conv(glyph: u32) {
        let conv = utf_32_glyph_to_utf_16(glyph).unwrap();
        let (conv_back, len) = utf_16_glyph_to_utf_32(&conv, 0);
        println!("len of glyph {}: {}", conv_back, len);
        assert_eq!(glyph, conv_back);
    }

    const CODE_TO_TEST: [u32; 3] = [0x0012, 0xEFFF, 0x1ABCD];

    for glyph in CODE_TO_TEST {
        double_conv(glyph);
    }
}

#[test]
#[should_panic]
/// Test that unpaired surrogates panic in basic conversion.
fn test_invalid_utf16_glyphs_1() {
    let glyph = BASIC_PLANE_1_END + 10;
    utf_32_glyph_to_utf_16(glyph).unwrap();
}

#[test]
#[should_panic]
/// Test that very big code-points panic.
fn test_invalid_utf16_glyphs_2() {
    let glyph = SUPPLEMENTARY_PLANE_END + 10;
    utf_32_glyph_to_utf_16(glyph).unwrap();
}

