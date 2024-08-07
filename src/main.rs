use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use four_cc::FourCC;
use libheif_rs::{HeifContext, ItemId};
use exif::{Error, Exif, In, Reader, Value};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the HEIC file
    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Convert PathBuf to &str
    let file_path = args.file.to_str().ok_or("Invalid file path")?;

    let ctx = HeifContext::read_from_file(file_path)?;

    // Get primary image handle
    let handle = ctx.primary_image_handle()?;

    // Extract basic metadata
    println!("HEIC Metadata:");
    println!("Width: {}", handle.width());
    println!("Height: {}", handle.height());
    println!("Has alpha channel: {}", handle.has_alpha_channel());
    println!("Luma bits per pixel: {}", handle.luma_bits_per_pixel());
    println!("Chroma bits per pixel: {}", handle.chroma_bits_per_pixel());
    let exif_four_cc: FourCC = FourCC(*b"Exif");
    let meta_data = handle.all_metadata();
    let mut result;
    for i in meta_data {
        let temp: FourCC = i.item_type;
        if i.item_type == exif_four_cc
        {
            result = parse_exif_data(i.raw_data).unwrap();
        }
    }
    Ok(())
}

/// Parses raw exif data and converts it a hashmap of exif tags and values.
/// Data must be given in raw format (i.e. WITH the first 4 bytes).
pub fn parse_exif_data(data: Vec<u8>) -> Option<HashMap<exif::Tag, Value>> {
    // Parse Exif data
    // Skip first 4 bytes
    let exif = match exif::Reader::new().read_raw(Vec::from(&data[4..])) {
        Ok(a) => {
            println!("Valid EXIF data found!");
            a
        }
        Err(_error) => { return None; }
    };
    let mut to_return: HashMap<exif::Tag, Value> = HashMap::new();
    for f in exif.fields() {
        to_return.insert(f.tag, f.value.clone());
    };
    return Some(to_return);
}