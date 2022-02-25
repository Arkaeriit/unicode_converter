/// The UTF-32 module is the Rosetta Stone of this module, all other modules
/// only convert from and to UTF-32. Thus, this module only need to take care
/// of writing and reading encoded values to and from a string of byte.

use crate::unicode_encoding::UnicodeEncoding;

/// A very basic wrapper for UTF-32 encoded data.
pub struct Utf32 {
    /// A list of the Unicode glyphs as codepoints as defined in UTF-32.
    data: Vec<u32>
}

impl Clone for Utf32 {
    fn clone(&self) -> Utf32 {
        let new_data = self.data.clone();
        let new_struct = Utf32{data: new_data};
        return new_struct;
    }
}

impl UnicodeEncoding for Utf32 {
    /// A quite dummy function to comply with the need of the UnicodeEncoding
    /// trait.
    fn from_utf_32(data_utf_32: &Utf32) -> Utf32 {
        return data_utf_32.clone();
    }

    /// A quite dummy function to comply with the need of the UnicodeEncoding
    /// trait.
    fn to_utf_32(&self) -> Utf32 {
        return self.clone();
    }

    /// Converts a stream of byte that _should_ be encoded in UTF-32 into the
    /// `Utf32` type.
    fn from_bytes(bytes: &[u8], big_endian: bool) -> Utf32 {
        if bytes.len() % 4 != 0 {
            panic!("Ho no!");
        }
        let mut data: Vec<u32> = Vec::new();
        let mut endian_index: [usize; 4] = [0, 1, 2, 3];
        if big_endian {
           endian_index = [3, 2, 1, 0];
        }
        for i in 0..bytes.len()/4 {
            let mut new_glyph: u32 = 0;
            new_glyph |= (bytes[i*4 + endian_index[0]] as u32) << 0;
            new_glyph |= (bytes[i*4 + endian_index[1]] as u32) << 8;
            new_glyph |= (bytes[i*4 + endian_index[2]] as u32) << 16;
            new_glyph |= (bytes[i*4 + endian_index[3]] as u32) << 24;
            data.push(new_glyph);
        }
        return Utf32{data: data};
    }

    /// Converts an instance of the `Utf32` type into a vector of bytes that is
    /// the UTF-32 encoded content.
    fn to_bytes(&self, big_endian: bool) -> Vec<u8> {
        let mut ret: Vec<u8> = Vec::new();
        let mut endian_index: [usize; 4] = [0, 1, 2, 3];
        if big_endian {
           endian_index = [3, 2, 1, 0];
        }
        for glyph in &self.data {
            let litle_endianed_glyph = cut_u32(*glyph);
            for index in endian_index {
                ret.push(litle_endianed_glyph[index]);
            }
        }
        return ret;
    }
}

/// Cut a 32 bit number into a 4 byte array by considering that the number
/// is little endian.
fn cut_u32(n: u32) -> [u8; 4] {
    let ret = [
        (n >>  0 & 0xFF) as u8,
        (n >>  8 & 0xFF) as u8,
        (n >> 16 & 0xFF) as u8,
        (n >> 24 & 0xFF) as u8,
    ];
    return ret;
}

