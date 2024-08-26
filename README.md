# HEIC Metadata Extractor

This Rust program extracts metadata from HEIC (High Efficiency Image Format) files. It can display various types of metadata including EXIF, TIFF, GPS, and Interoperability information.

## Features

- Extract and display metadata from HEIC files
- Support for different types of metadata:
  - EXIF
  - TIFF
  - GPS
  - Interoperability

## Usage

```
heic_metadata_extractor [OPTIONS] --file <FILE>
```

### Options

- `-f, --file <FILE>`: Path to the HEIC file (required)
- `-o, --operation <OPERATION>`: Operation to perform (optional)

### Operations

Currently, only the "All" operation is fully supported, which is the default if no operation is specified. This prints out all available metadata.

Future versions may support more specific operations such as:
- Image Dimensions
- Date
- Camera Info
- GPS Info

## Note

The parameter functionality for specific operations is currently under development. At present, the program will always display all available metadata regardless of the operation specified.

## Dependencies

This program uses several external crates:
- `clap` for command-line argument parsing
- `exif` for EXIF data parsing
- `four_cc` for FourCC code handling
- `libheif_rs` for HEIC file reading

## Building

Ensure you have Rust and Cargo installed, then run:

```
cargo build --release
```

## Running

After building, you can run the program with:

```
./target/release/heic_metadata_extractor --file path/to/your/file.heic
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
