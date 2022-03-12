extern crate unicode_converter;
use clap::Parser;

use unicode_converter::unicode_encoding::UnicodeEncoding;

fn main() {
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

