mod chunk;
mod filters;
mod image;
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
    let chunks = chunk::extract_chunks(&image_bytes)?;
    let compressed_data = chunk::combine_chunk_data(&chunks);
    let uncompressed_data = image::inflate_bytes(&compressed_data).unwrap_or(vec![0]);
    let png_image = image::PngImage::new(&chunks[0], &uncompressed_data);
    if png_image.data.len() as u32
        != png_image.height * (1 + png_image.width * filters::BYTES_PER_PIXEL as u32)
    {
        Err("Invalid image data")?;
    }
    let result = filters::reconstruct_image(&png_image)?;
    // println!("{:?}", uncompressed_data);
    // println!("{}", uncompressed_data.len());
    Ok(())
}
