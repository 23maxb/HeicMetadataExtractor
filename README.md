# HEIC Metadata Extractor

This Rust program extracts metadata from HEIC (High Efficiency Image Format) files. It can display various types of metadata including EXIF, TIFF, GPS, and Interoperability information.
If you want me to add write functionality to this please put that in a issue and I'll likely come back to this.

## Features

- Extract and display metadata from HEIC files
- Support for different types of metadata:
  - EXIF
  - TIFF
  - GPS
  - Interoperability

## Usage

```
cli [OPTIONS] --file <FILE>
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

## Sample Usage and Output for a Photo Taken on an iPhone

./cli --file test.HEIC
Exif Data:
Exposure program: normal program
Manufacturer notes: 0x4170706c6520694f530000014d4d002900010009000000010000000e0002000700000200000002000003000700000068000004000004000900000001000000010005000900000001000000b50006000900000001000000bc0007000900000001000000010008000a0000000300000468000c000a0000000200000480000d00090000000100000017000e0009000000010000000400100009000000010000000100110002000000250000049000140009000000010000000a0017001000000001000004b5001900090000000100000002001a000200000006000004bd001f000900000001000000000020000200000025000004c30021000a00000001000004e80023000900000002000004f00025001000000001000004f80026000900000001000000030027000a0000000100000500002800090000000100000002002b00020000002500000508002d00090000000100000bc5002e00090000000100000001002f000900000001000000400033000900000001000010000034000900000001000000040035000900000001000000030036000900000001000006ac003700090000000100000004003a00090000000100000004003b00090000000100000000003c00090000000100000004004100090000000100000000004a00090000000100000002004d000a000000010000052d004e00070000007900000535000000008e008700830069006e004900460049003e00370039003400230022001f002500be00a9000001da00a8004e00d200a700660051006a005b001d001f0022003f00c100ad0004012a01f2005c00e4004801e80072007600410027003e0059006a00c500b3000a01350119019b00c500ff0057017d00710032002f003b0065007300c900b40013013a0133019a0177016c01ab00600079003b00340041006e007700cf00bc0018013c014f010d0317029f01ff007300b30050003200570077008000d600bd0027013f01ed00d4001e018e012602bb00590157004500790090008e00e100c9001801b8002501a400a000fd002a02a0019d015000520091006c005700f500ec00af002d015f01d100e0009500b1002501af003a0058008a007f0083000b01b1001a014e011e01d70046016100410030003200540060007b004d003e00e901270138014f011c010301c50097007700e000a9000e01b5008b006f004e007901370138014f010f01fb00cf00e3000501e100c800da00d40083005c005a003d013c0143014a01e000eb00f600d900b600840066006600c70057005900670063017c0152014901d500ca00ad0065008900780051002c00310040003b0042004002e40163013d01cc00e700cb00fd00de00a700ab00270024003d003b003a00c003250261012901cf00bc00ad00f100010167003c0029002200140038002e0062706c6973743030d4010203040506070855666c6167735576616c75655974696d657363616c655565706f636810011300004e74c0d76885123b9aca0010000811171d272d2f383d000000000000010100000000000000090000000000000000000000000000003ffffff7fb0004eab3fffee7b7000117deffffd5930001d451000000e700000100000003030000010044323036384532392d424243362d343043452d384346452d45343031454141324434324500000000002250200071383235730045433445344637302d393330322d344439312d423438442d44333346384438303431354200000078a800007393000000721000003d000000000000008e0001fcb500000bf839364143303739442d444545362d343833302d393936362d37453037333833374542313300000000220000000162706c6973743030d201020304513151321001a2050ad20607080953322e3153322e322340490b9b000000002340ec37a000000000d206070b0c230000000000000000234041800000000000080d0f1113161b1f232c353a430000000000000101000000000000000d0000000000000000000000000000004c
Lens make: "Apple"
Lens model: "iPhone 11 Pro back triple camera 4.25mm f/1.8"
Offset data of DateTimeOriginal: "-07:00"
Date and time of original data generation: 2024-08-09 22:08:07
Valid image height: 3024
Flash: not fired, no return light detection function, suppressed
Metering mode: pattern
White balance: auto white balance
Photographic sensitivity: 64
Date and time of digital data generation: 2024-08-09 22:08:07
Offset data of DateTime: "-07:00"
Composite image: composite (general)
Exposure mode: auto exposure
Scene type: directly photographed image
Color space information: uncalibrated
Brightness: 4.668240888512523
F number: 1.8
Exif version: 2.32
Aperture: 1.6959938128383605
Valid image width: 4032
Exposure time: 1/100
Offset data of DateTimeDigitized: "-07:00"
DateTimeDigitized subseconds: "086"
Focal length in 35 mm film: 26
Lens focal length: 4.25
Lens specification: 1.5399999618512084-6 mm, f/1.8-2.4
Subject area: rectangle (x=1999, y=1501, w=2209, h=1388)
Sensing method: one-chip color area sensor
Exposure bias: 0
DateTimeOriginal subseconds: "086"
Shutter speed: 6.645010808965752
Tiff Data:
Manufacturer of image input equipment: "Apple"
File change date and time: 2024-08-09 22:08:07
Software used: "17.0"
Image resolution in width direction: 72
Image resolution in height direction: 72
Orientation of image: row 0 at right and column 0 at top
Unit of X and Y resolution: inch
Model of image input equipment: "iPhone 11 Pro"
GPS Data:
Bearing of destination: 310.1130676552363
Speed unit: km/h
Altitude reference: below sea level
Reference for direction of image: true direction
North or south latitude: N
GPS date: 2024-08-09
GPS time (atomic clock): 07:08:05.54
Direction of image: 310.1130676552363
Altitude: 0.8707609177789427
Longitude: 4 deg 46 min 0.82 sec
Latitude: 52 deg 18 min 43.41 sec
East or West Longitude: E
Horizontal positioning error: 76.45153272576637
Speed of GPS receiver: 0
Reference for bearing of destination: true direction
No Interop Data Found.
