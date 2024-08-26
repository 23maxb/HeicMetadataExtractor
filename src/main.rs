use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;

use clap::Parser;
use exif::{Context, Tag, Value};
use four_cc::FourCC;
use libheif_rs::HeifContext;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the HEIC file
    #[arg(short, long)]
    file: PathBuf,

    /// Operation to perform
    /// Possible Values: (if omitted defaults to all)
    /// -All (Default)
    ///     Prints out all the data avaialble
    /// - Image Dimensions
    ///     Prints out the dimensions of the image.
    /// - Date
    ///     Prints out the date the image was taken.
    /// - Camera Info
    ///     Prints out information about the camera.
    /// - GPS Info
    ///     Prints out GPS information.
    #[arg(short, long)]
    operation: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Convert PathBuf to &str
    let file_path = args.file.to_str().ok_or("Invalid file path")?;

    let ctx = HeifContext::read_from_file(file_path).expect("HEIC File does not exist.");

    // Get primary image handle
    let handle = ctx.primary_image_handle()?;

    // Extract basic metadata
    let exif_four_cc: FourCC = FourCC(*b"Exif");
    let meta_data = handle.all_metadata();
    let mut result: Result<HashMap<exif::Tag, Value>, &str> = Result::Err("No EXIF data found.");
    for i in meta_data {
        if i.item_type == exif_four_cc
        {
            result = parse_exif_data(i.raw_data);
        }
    }
    if result.is_ok() {
        let mut exif_data: HashMap<Tag, Value> = HashMap::new();
        let mut tiff_data: HashMap<Tag, Value> = HashMap::new();
        let mut gps_data: HashMap<Tag, Value> = HashMap::new();
        let mut interop_data: HashMap<Tag, Value> = HashMap::new();

        for (key, value) in result.unwrap() {
            if (key.description().is_none()) {
                continue;
            }
            match key.context()
            {
                Context::Tiff => { tiff_data.insert(key, value); }
                Context::Exif => { exif_data.insert(key, value); }
                Context::Gps => { gps_data.insert(key, value); }
                Context::Interop => { interop_data.insert(key, value); }
                _ => { panic!("Unknown Context. Please open a new issue on the github."); }
            }
        }
        if exif_data.len() > 0 {
            println!("Exif Data:");
            for (key, value) in exif_data {
                println!("{}: {}", key.description().unwrap(), value.display_as(key));
            }
        } else { println!("No Exif Data Found."); }
        if tiff_data.len() > 0 {
            println!("Tiff Data:");
            for (key, value) in tiff_data {
                println!("{}: {}", key.description().unwrap(), value.display_as(key));
            }
        } else { println!("No Tiff Data Found."); }
        if gps_data.len() > 0 {
            println!("GPS Data:");
            for (key, value) in gps_data {
                println!("{}: {}", key.description().unwrap(), value.display_as(key));
            }
        } else { println!("No Gps Data Found."); }
        if interop_data.len() > 0 {
            println!("Interop Data:");
            for (key, value) in interop_data {
                println!("{}: {}", key.description().unwrap(), value.display_as(key));
            }
        } else { println!("No Interop Data Found.") }
    } else { println!("No metadata found in {}", file_path); }
    Ok(())
}

pub fn get_dimensions(path: &str) -> (u32, u32) {
    let ctx = HeifContext::read_from_file(path).expect("HEIC File does not exist.");
    let handle = ctx.primary_image_handle().expect("No primary image handle found.");
    return (handle.width(), handle.height());
}

/// Parses raw exif data and converts it a hashmap of exif tags and values.
/// Data must be given in raw format (WITH the first 4 bytes).
pub fn parse_exif_data<'a>(data: Vec<u8>) -> Result<HashMap<exif::Tag, Value>, &'a str> {
    // Parse Exif data
    // Skip first 4 bytes
    let exif = match exif::Reader::new().read_raw(Vec::from(&data[4..])) {
        Ok(a) => {
            a
        }
        Err(_error) => { return Result::Err("Not Valid Exif Data."); }
    };
    let mut to_return: HashMap<exif::Tag, Value> = HashMap::new();
    for f in exif.fields() {
        //println!("{:?}", f.tag);

        to_return.insert(f.tag, f.value.clone());
    };
    return Result::Ok(to_return);
}