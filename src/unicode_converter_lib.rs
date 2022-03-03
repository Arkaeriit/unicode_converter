/// The UTF-32 module is the Rosetta Stone of this module, all other modules
/// only convert from and to UTF-32. Thus, this module only need to take care
/// of writing and reading encoded values to and from a stream of bytes.
pub mod utf_32;

/// The `unicode_encoding` module contains the `UnicodeEncoding` trait that
/// contains the common capabilities for all Unicode encodings.
pub mod unicode_encoding;

/// The UTF-8 module is quite important as it can be used to convert Rust's
/// strings into the other unicode encoding types.
pub mod utf_8;

/// The UTF-16 module manipulates UTF-16 data.
pub mod utf_16;

/// This module is used to convert from stream of bytes to streams of numbers
/// knowing and taking care about the endianness. It works with any number type
/// that can be bit-cased to u64. It is quite dirty and rely a bit on unsafe
/// code.
mod endian_aware_byte_streamer;

