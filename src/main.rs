extern crate unicode_converter;
use clap::Parser;

use unicode_converter::unicode_encoding::UnicodeEncodingError;
use unicode_converter::unicode_encoding::UnicodeEncoding;
use unicode_converter::utf_8::Utf8;
use unicode_converter::utf_16::Utf16;
use unicode_converter::utf_32::Utf32;

fn main() {
    let arg = Args::parse();
    let decoded_message: Utf32 = match try_to_read_with_encoding(&arg.input_file, &arg.decoding_input, false) {
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
    let encoded_stream = match try_to_encode_data(&decoded_message, &arg.encoding_output, false) {
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
fn try_to_read_with_encoding(filename: &str, encoding: &str, big_engian: bool)  -> Option<Result<Result<Utf32, UnicodeEncodingError>, std::io::Error>> {
    macro_rules! ttrwe_case {
        ($type: ty) => {
            {
                let encoded = <$type>::from_file(filename, big_engian);
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
        "UTF-8" => ttrwe_case!(Utf8),
        "UTF-16" => ttrwe_case!(Utf16),
        "UTF-32" => ttrwe_case!(Utf32),
        _ => None,
    }
}

/// Try to convert an UTF-32 in the encoding given as an argument string.
/// If the encoding is not valid, None is returned. Then, the data is converted
/// to a string of bytes.
fn try_to_encode_data(utf32: &Utf32, encoding: &str, big_engian: bool) -> Option<Vec<u8>> {
    macro_rules! tted_case {
        ($type: ty) => {
            Some(<$type>::from_utf_32(utf32).to_bytes(big_engian))
        }
    }
    match encoding {
        "UTF-8" => tted_case!(Utf8),
        "UTF-16" => tted_case!(Utf16),
        "UTF-32" => tted_case!(Utf32),
        _ => None,
    }
}

/// A tool to convert Unicode encodings
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    /// Input file used as input. You can use `-` if you mean `/dev/stdin`.
    #[clap(short, long)]
    input_file: String,

    /// Input file encoding. Could be `UTF-8`, `UTF-16` or `UTF-32`.
    #[clap(short, long)]
    decoding_input: String,

    /// Output file, default to `/dev/stdout`.
    #[clap(short, long, default_value = "/dev/stdout")]
    output_file: String,

    /// Input file encoding. Could be `UTF-8`, `UTF-16` or `UTF-32`.
    #[clap(short, long)]
    encoding_output: String,
}

