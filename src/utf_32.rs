/// The UTF-32 module is the Rosetta Stone of this module, all other modules
/// only convert from and to UTF-32. Thus, this module only need to take care
/// of writing and reading encoded values to and from a file.

use crate::unicode_encoding::UnicodeEncoding;

pub struct Utf32 {
    data: Vec<u32>
}

impl Utf32 {
    fn clone(&self) -> Utf32 {
        let new_data = self.data.clone();
        let new_struct = Utf32{data: new_data};
        return new_struct;
    }
}

impl UnicodeEncoding for Utf32 {
    fn from_utf_32(data_utf_32: &Utf32) -> Utf32 {
        return data_utf_32.clone();
    }

    fn to_utf_32(data: &Utf32) -> Utf32 {
        return data.clone();
    }

    fn from_bytes(bytes: &[u8]) -> Utf32 {
        if bytes.len() % 4 != 0 {
            panic!("Ho no!");
        }
        let mut data: Vec<u32> = Vec::new();
        for i in 0..bytes.len()/4 {
            let mut new_glyph: u32 = bytes[i*4].into();
            new_glyph |= (bytes[i*4 + 1] as u32) << 8;
            new_glyph |= (bytes[i*4 + 2] as u32) << 16;
            new_glyph |= (bytes[i*4 + 3] as u32) << 24;
            data.push(new_glyph);
        }
        return Utf32{data: data};
    }

}
