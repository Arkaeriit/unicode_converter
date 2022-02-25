extern crate unicode_converter;

use unicode_converter::unicode_encoding::UnicodeEncoding;

fn main() {
    let bytes: [u8; 4] = [0x61, 0, 0, 0];
    unicode_converter::utf_32::Utf32::from_bytes(&bytes.as_slice(), true);
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use unicode_converter::utf_32::Utf32;
    use unicode_converter::unicode_encoding::UnicodeEncoding;


    #[test]
    fn test_basic_utf_32_test() {
        let mut random_bytes: Vec<u8> = Vec::new();
        for i in 0..128 {
            random_bytes.push(i);
        }
        let utf32_glyphs = Utf32::from_bytes(random_bytes.as_slice(), false);
        let converted_bytes =utf32_glyphs.to_bytes(false);
        assert_eq!(converted_bytes, random_bytes);
    }

}

