mod chunk;
mod image;
use std::{env, fs};
use std::path::Path;

const PNG_HEADER_OFFSET: u32 = 8;
pub fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(Path::new(path))?;
    Ok(data)
}

pub fn compare_vecs<T: std::cmp::PartialEq>(v1: &[T], v2: &[T]) -> bool {
    let matching = v1.iter().zip(v2.iter()).filter(|&(v1, v2)| v1 == v2).count();
    matching == v1.len() && matching == v2.len()
}   
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        Err("Need to specify file path")?;
    }
    let image_path = &args[1];
    let image_bytes = read_file(image_path).unwrap();
    let size = image_bytes.len() as usize;
    if !compare_vecs(&image_bytes[0..PNG_HEADER_OFFSET as usize], &vec![137, 80, 78, 71, 13, 10, 26, 10]) {
        Err("Not a PNG file")?;
    }
    let mut idx = PNG_HEADER_OFFSET as usize;

    let mut chunks: Vec<chunk::Chunk> = Vec::new();
    while idx < size {
        let chunk_length = chunk::combine_bytes(&image_bytes[idx..(idx + chunk::CHUNK_LENGTH_OFFSET as usize)]);
        idx += chunk::CHUNK_LENGTH_OFFSET as usize;

        let chunk_type = std::str::from_utf8(&image_bytes[idx..(idx + chunk::CHUNK_TYPE_OFFSET as usize)]).unwrap();
        idx += chunk::CHUNK_TYPE_OFFSET as usize;

        let mut chunk_data_buffer = vec![0 as u8; chunk_length as usize];
        let chunk_data_slice = &image_bytes[idx..(idx + (chunk_length as usize))];
        chunk_data_buffer.clone_from_slice(&chunk_data_slice);
        idx += chunk_length as usize;

        let crc = chunk::combine_bytes(&image_bytes[idx..(idx + chunk::CRC_OFFSET as usize)]);
        idx += chunk::CRC_OFFSET as usize;

        // println!("{}", chunk_length);
        // println!("{}", chunk_type);
        // println!("{:?}", chunk_data_buffer);
        // println!("{}", crc);

        let current_chunk = chunk::Chunk {
            length: chunk_length,
            datatype: chunk_type.to_string(),
            data: chunk_data_buffer, 
            cyclic_redundancy_check: crc
        };
        chunks.push(current_chunk);
    }

    let compressed_data = chunk::combine_chunk_data(&chunks);
    let uncompressed_data = image::inflate_bytes(&compressed_data).unwrap_or(vec![0]);
    println!("{:?}", uncompressed_data);
    println!("{}", uncompressed_data.len());
    Ok(())
}
