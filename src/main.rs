extern crate unicode_converter;

use unicode_converter::unicode_encoding::UnicodeEncoding;

fn main() {
    let bytes: [u8; 4] = [0x61, 0, 0, 0];
    unicode_converter::utf_32::Utf32::from_bytes(&bytes.as_slice(), true).unwrap();
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use unicode_converter::utf_32::Utf32;
    use unicode_converter::utf_16::Utf16;
    use unicode_converter::utf_8::Utf8;
    use unicode_converter::unicode_encoding::UnicodeEncoding;


    #[test]
    fn basic_utf_32_test() {
        let mut random_bytes: Vec<u8> = Vec::new();
        for i in 0..128 {
            if i % 4 < 2 { // Check to ensure that the _unicode codepoints_ are somewhat valid
                random_bytes.push(i);
            } else {
                random_bytes.push(0);
            }
        }
        let utf32_glyphs = Utf32::from_bytes(random_bytes.as_slice(), false).unwrap();
        let converted_bytes =utf32_glyphs.to_bytes(false);
        assert_eq!(converted_bytes, random_bytes);
    }

    /// Tests all types conversion.
    fn string_conv(reference: &str) {
        let utf32 = Utf32::from_string(reference).unwrap();
        let conv_1 = utf32.to_string();
        assert_eq!(reference, conv_1);
        let utf8 = Utf8::from_string(reference).unwrap();
        let conv_2 = utf8.to_string();
        assert_eq!(reference, conv_2);
        let utf16 = Utf16::from_string(reference).unwrap();
        let conv_3 = utf16.to_string();
        assert_eq!(reference, conv_3);
    }

    #[test]
    fn testing_string_conv() {
        string_conv("Avcde");
        string_conv("aeé¤ㅢㅟ😎🐤");
    }

    #[test]
    fn test_null() {
        let bytes: [u8; 2] = ['a' as u8, 0];
        let utf8 = match Utf8::from_bytes(&bytes, false) {
            Ok(x) => x,
            Err(_) => {panic!("Error in utf8 from bytes.\n");}
        };
        let s = utf8.to_string();
        assert_eq!(s, "a\0");
    }

    #[test]
    fn fancy_codepoints() {
        let utf8_str = "aeé¤ㅢㅟ😎🐤";
        let unicode_codepoints = Utf32{data: vec!['a' as u32, 'e' as u32, 0x00E9, 0x00A4, 0x3162, 0x315F, 0x1F60E, 0x1F424]};
        let conv = Utf32::from_string(utf8_str).unwrap();
        assert!(conv == unicode_codepoints);
    }

}

