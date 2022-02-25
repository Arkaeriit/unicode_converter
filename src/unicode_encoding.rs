/// The `unicode_encoding` trait contains the basic function shared with all
/// the other encodings in this module. This is converting the data from and to
/// UTF-32 and writing it to or reading it from a file. Furthermore, it is also
/// possible to convert from and to a slice of bytes.

use crate::utf_32::Utf32;

pub trait UnicodeEncoding {
    fn from_utf_32(data_utf_32: &Utf32) -> Self;
    fn to_utf_32(&self) -> Utf32;
    fn from_bytes(bytes: &[u8], big_endian: bool) -> Self;
    fn to_bytes(&self, big_endian: bool) -> Vec<u8>;
    //fn from_file(filename: &str) -> Self;
    //fn to_file(data: &Self, filename: &str);
}

