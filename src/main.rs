extern crate unicode_converter;

use unicode_converter::unicode_encoding::UnicodeEncoding;

fn main() {
    let bytes: [u8; 4] = [0x61, 0, 0, 0];
    unicode_converter::utf_32::Utf32::from_bytes(&bytes.as_slice());
    println!("Hello, world!");
}

