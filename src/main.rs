mod png;
mod qoi;
mod utils;
use image::save_buffer;
use std::path::Path;
use std::{env, fs};

pub fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(Path::new(path))?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        Err("Need to specify file path")?;
    }
    let image_path = &args[1];
    let image_bytes = read_file(&image_path)?;
    let chunks = png::chunk::extract_chunks(&image_bytes)?;
    let compressed_data = png::chunk::combine_chunk_data(&chunks);
    let uncompressed_data = png::frame::inflate_bytes(&compressed_data).unwrap_or(vec![0]);
    let png_meta = png::frame::PngImageMetadata::new(&chunks[0]);
    let reconstruction = png::filters::reconstruct_image(&png_meta, &uncompressed_data)?;
    let png_image = png::frame::PngImage::new(png_meta, &reconstruction);
    println!("{:?}", &png_image.metadata);
    save_buffer(
        &Path::new("image.png"),
        &reconstruction,
        png_image.metadata.width,
        png_image.metadata.height,
        image::ColorType::Rgba8,
    )?;
    Ok(())
}
