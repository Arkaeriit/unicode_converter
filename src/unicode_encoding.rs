/// The `unicode_encoding` module contains the `UnicodeEncoding` trait that
/// contains the common capabilities for all Unicode encodings.

use crate::utf_32::Utf32;
use crate::utf_8::Utf8;

/// The `UnicodeEncoding` trait contains the basic function shared with all
/// the other encodings in this crate. This is converting the data from and to
/// UTF-32 and writing it to or reading it from a file. Furthermore, it is also
/// possible to convert from and to a slice of bytes.
pub trait UnicodeEncoding {
    // Function that need to be implemented by the various types.
    /// The function `from_utf_32` takes a Utf32 struct and convert its content
    /// to the desired encoding. This function should always be implemented by
    /// the encoding's type.
    fn from_utf_32(data_utf_32: &Utf32) -> Self;

    /// The function `to_utf_32` converts data from the desired encoding to
    /// UTF-32. It should always be implemented by the encoding's type.
    fn to_utf_32(&self) -> Utf32;

    /// The function `from_bytes_no_check` takes a stream of bytes and
    /// interpret it as it was in the desired encoding. It should always be
    /// implemented by the encoding's type. This does not uses the check_sanity
    /// function.
    fn from_bytes_no_check(bytes: &[u8], big_endian: bool) -> Result<Self, UnicodeEncodingError> where Self: Sized;

    /// The function `to_bytes` takes the raw-data of encoded content
    /// and convert it to a vector of bytes. It should always be implemented by
    /// the encoding's type.
    fn to_bytes(&self, big_endian: bool) -> Vec<u8>;

    // Functions implemented in this trait
    /// Converts an Unicode encoded content and converts it to Rust's string.
    fn to_string(&self) -> String {
        let utf8 = self.convert_to::<Utf8>();
        return utf8.to_string();
    }

    /// Takes a Rust string and converts it into Unicode encoded content.
    fn from_string(s: &str) -> Result<Self, UnicodeEncodingError> where Self: Sized {
        let utf8 = Utf8::from_string(s)?;
        return Ok(utf8.convert_to::<Self>());
    }

    /// Converts from one Unicode encoding to an other.
    fn convert_to<T: UnicodeEncoding> (&self) -> T {
        let utf32 = self.to_utf_32();
        let ret: T = T::from_utf_32(&utf32);
        return ret;
    }

    /// Tell if the encoded content is equal to an other encoded content,
    /// regardless of the chosen encoding.
    fn content_eq<T: UnicodeEncoding>(&self, other: &T) -> bool {
        return self.to_utf_32() == other.to_utf_32();
    }

    /// Checks that the unicode data is valid.
    fn check_sanity(&self) -> UnicodeEncodingError {
        let utf32 = self.to_utf_32();
        return utf32.check_sanity_utf32();
    }

    /// The function `from_bytes` takes a stream of bytes and interpret it as
    /// it was in the desired encoding. It should always be implemented by the
    /// encoding's type.
    fn from_bytes(bytes: &[u8], big_endian: bool) -> Result<Self, UnicodeEncodingError> where Self: Sized {
        let ret: Self = Self::from_bytes_no_check(bytes, big_endian)?;
        match ret.check_sanity() {
            UnicodeEncodingError::NoError => Ok(ret),
            x => Err(x),
        }
    }

    //fn from_file(filename: &str) -> Self;
    //fn to_file(data: &Self, filename: &str);
}

#[derive(Debug)]
pub enum UnicodeEncodingError {
    NoError,
    InvalidCodepointTooManyBits,
}

