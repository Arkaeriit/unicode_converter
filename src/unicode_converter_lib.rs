/// The UTF-32 module is the Rosetta Stone of this module, all other modules
/// only convert from and to UTF-32. Thus, this module only need to take care
/// of writing and reading encoded values to and from a stream of bytes.
pub mod utf_32;

/// The `unicode_encoding` module contains the `UnicodeEncoding` trait that
/// contains the common capabilities for all Unicode encodings.
pub mod unicode_encoding;

