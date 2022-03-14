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

