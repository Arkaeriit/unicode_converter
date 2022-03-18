# Unicode converter

This repository contains both a library and a CLI tool to convert data between various Unicode encodings.

The supported encodings are:

* UTF-8
* CESU-8
* UTF-16
* UTF-32

## CLI tool

The CLI tool is meant to be a demonstration of the library but it can be used on its own if needed. It is made in a single file, `str/main.rs`.

### Usage

```
A tool to convert Unicode text files between multiple Unicode encodings. The available encodings are
UTF-8, CESU-8, UTF-16, and UTF-32. By default, the data is assumed to be little-endian, but for encodings
with multi-byte words such as UTF-16 or UTF-32, you can add the `_be` suffix to indicate that you
want to work with big-endian data

USAGE:
    unicode_converter [OPTIONS] --input-file <INPUT_FILE> --decoding-input <DECODING_INPUT> --encoding-output <ENCODING_OUTPUT>

OPTIONS:
    -d, --decoding-input <DECODING_INPUT>
            Input file encoding

    -e, --encoding-output <ENCODING_OUTPUT>
            Output file encoding

    -h, --help
            Print help information

    -i, --input-file <INPUT_FILE>
            Input file used as input. You can use `-` if you mean `/dev/stdin`

    -o, --output-file <OUTPUT_FILE>
            Output file [default: /dev/stdout]
```

### Compilation

To compile it, simply run `cargo build` as it is the only executable crate in this repository.

## Library

All the code in `src/` except for `src/main.rs` makes the Unicode encoding converting library.

### Behavior

The various Unicode encodings are all made with their own type implementing the `UnicodeEncoding` trait. Running `cargo doc` will give you complete information but the intended way of using the library is the following:

* Read data from a file or a slice of bytes. For example, too read UTF-16 data from a file, do `let content = Utf16::from_file("filename.txt", false).unwrap();`. Note the `false` used to indicate that the encoding is little-endian.
* Then, convert it to an other encoding. For example, to convert to UTF-8: `let converted = content.convert_to::<Utf8>();`.
* Finally, you can write the converted data to a new file. `converted.to_file("new_file.txt", false);`. As UTF-8 is only on one byte, the boolean argument to take care of the endianess is ignored.

