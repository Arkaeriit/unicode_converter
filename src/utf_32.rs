/// The UTF-32 module is the Rosetta Stone of this module, all other modules
/// only convert from and to UTF-32. Thus, this module only need to take care
/// of writing and reading encoded values to and from a string of byte.

use crate::unicode_encoding::UnicodeEncoding;
use crate::endian_aware_byte_streamer;

/// A very basic wrapper for UTF-32 encoded data.
pub struct Utf32 {
    /// A list of the Unicode glyphs as codepoints as defined in UTF-32.
    pub data: Vec<u32> // TODO: Not public but use itterator instead
}

impl PartialEq for Utf32 {
    fn eq(&self, other: &Utf32) -> bool {
        return self.data == other.data;
    }
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
    fn from_bytes(bytes: &[u8], big_endian: bool) -> Self {
        return Utf32{data: endian_aware_byte_streamer::from_bytes::<u32>(bytes, big_endian)};
    }

    /// Converts an instance of the `Utf32` type into a vector of bytes that is
    /// the UTF-32 encoded content.
    fn to_bytes(&self, big_endian: bool) -> Vec<u8> {
        return endian_aware_byte_streamer::to_bytes::<u32>(&self.data, big_endian);
    }
}

#[test]
fn test_data_content() {
    let data: [u8; 4] = [0, 1, 2, 3];
    let utf_32_glyph = Utf32::from_bytes(data.as_slice(), false);
    assert_eq!(utf_32_glyph.data[0], 0x03020100);
}

