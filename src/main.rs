extern crate unicode_converter;
use clap::Parser;

use unicode_converter::unicode_encoding::UnicodeEncodingError;
use unicode_converter::unicode_encoding::UnicodeEncoding;
use unicode_converter::utf_8::Utf8;
use unicode_converter::utf_16::Utf16;
use unicode_converter::utf_32::Utf32;

fn main() {
    let arg = Args::parse();
    let input_file = match &arg.input_file as &str {
        "-" => "/dev/stdin",
        x => x,
    };
    let decoded_message: Utf32 = match try_to_read_with_encoding(input_file, &arg.decoding_input) {
        Some(x) => match x {
            Ok(y) => match y {
                Ok(z) => z,
                Err(z) => {
                    eprintln!("Error, invalid {} file.", arg.input_file);
                    eprintln!("The error is: {:?}", z);
                    std::process::exit(3);
                },
            },
            Err(y) => {
                eprintln!("Error, unable to read input file: {}.", y);
                std::process::exit(2);
            },
        },
        None => {
            eprintln!("Error, unknown input encoding.");
            std::process::exit(1);
        },
    };
    let encoded_stream = match try_to_encode_data(&decoded_message, &arg.encoding_output) {
        Some(x) => x,
        None => {
            eprintln!("Error, unknown output encoding.");
            std::process::exit(1);
        }
    };
    match std::fs::write(&arg.output_file, &encoded_stream) {
        Ok(_) => {},
        Err(x) => {
            eprintln!("Error, unable to write to output_file file: {}.", x);
            std::process::exit(2);
        }
    }
}

/// Try to read a file with the encoding given as a string. If it works,
/// returns it converted to UTF-32. The results are encapsulated the same was
/// as the from_file function but in an option block where Node is returned if
/// the type is not known
fn try_to_read_with_encoding(filename: &str, encoding: &str)  -> Option<Result<Result<Utf32, UnicodeEncodingError>, std::io::Error>> {
    macro_rules! ttrwe_case {
        ($type: ty, $big_engian: expr) => {
            {
                let encoded = <$type>::from_file(filename, $big_engian);
                match encoded {
                    Ok(x) => match x {
                        Ok(y) => Some(Ok(Ok(y.to_utf_32()))),
                        Err(y) => Some(Ok(Err(y))),
                    }
                    Err(x) => Some(Err(x)),
                }
            }
        }
    }
    match encoding {
        "UTF-8" => ttrwe_case!(Utf8, false),
        "UTF-16" => ttrwe_case!(Utf16, false),
        "UTF-32" => ttrwe_case!(Utf32, false),
        "UTF-16_be" => ttrwe_case!(Utf16, true),
        "UTF-32_be" => ttrwe_case!(Utf32, true),
        _ => None,
    }
}

/// Try to convert an UTF-32 in the encoding given as an argument string.
/// If the encoding is not valid, None is returned. Then, the data is converted
/// to a string of bytes.
fn try_to_encode_data(utf32: &Utf32, encoding: &str) -> Option<Vec<u8>> {
    macro_rules! tted_case {
        ($type: ty, $big_engian: expr) => {
            Some(<$type>::from_utf_32(utf32).to_bytes($big_engian))
        }
    }
    match encoding {
        "UTF-8" => tted_case!(Utf8, false),
        "UTF-16" => tted_case!(Utf16, false),
        "UTF-32" => tted_case!(Utf32, false),
        "UTF-16_be" => tted_case!(Utf16, true),
        "UTF-32_be" => tted_case!(Utf32, true),
        _ => None,
    }
}

/// A tool to convert Unicode text files between multiple Unicode encodings.
/// The available encodings are:
/// * UTF-8
/// * UTF-16
/// * UTF-32
///
/// By default, the data is assumed to be little-endian, but for encodings with
/// multi-byte words such as UTF-16 or UTF-32, you can add the `_be` suffix to
/// indicate that you want to work with big-endian data.
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    /// Input file used as input. You can use `-` if you mean `/dev/stdin`
    #[clap(short, long)]
    input_file: String,

    /// Input file encoding
    #[clap(short, long)]
    decoding_input: String,

    /// Output file
    #[clap(short, long, default_value = "/dev/stdout")]
    output_file: String,

    /// Output file encoding
    #[clap(short, long)]
    encoding_output: String,
}

