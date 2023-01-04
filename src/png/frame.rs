use std::error::Error;
use std::io::Read;

use crate::png::chunk;

const DIMENSION_OFFSET: usize = 4;
#[derive(Debug, Default)]
pub struct PngImage {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
    pub data: Vec<u8>,
}
impl PngImage {
    pub fn new(header_chunk: &chunk::Chunk, uncompressed_data: &[u8]) -> Self {
        PngImage {
            width: chunk::combine_bytes(&header_chunk.data[0..DIMENSION_OFFSET]),
            height: chunk::combine_bytes(
                &header_chunk.data[DIMENSION_OFFSET..2 * DIMENSION_OFFSET],
            ),
            bit_depth: header_chunk.data[2 * DIMENSION_OFFSET],
            color_type: header_chunk.data[2 * DIMENSION_OFFSET + 1],
            compression_method: header_chunk.data[2 * DIMENSION_OFFSET + 2],
            filter_method: header_chunk.data[2 * DIMENSION_OFFSET + 3],
            interlace_method: header_chunk.data[2 * DIMENSION_OFFSET + 4],
            data: uncompressed_data.to_vec(),
        }
    }
}
pub fn inflate_bytes(data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut decoder = flate2::read::ZlibDecoder::new(data);
    let flag = decoder.read_to_end(&mut buffer).unwrap_or(0);
    if flag == 0 {
        Err("Could not decode file")?;
    }
    Ok(buffer)
}
