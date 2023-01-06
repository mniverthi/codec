use std::error::Error;
use std::io::Read;

use crate::png::chunk;
use crate::utils::combine_bytes;

const DIMENSION_OFFSET: usize = 4;
#[derive(Debug, Default)]
pub struct PngImageMetadata {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}
#[derive(Debug)]
pub struct PngImage {
    pub metadata: PngImageMetadata,
    pub data: Vec<u8>,
}
impl PngImageMetadata {
    pub fn new(header_chunk: &chunk::Chunk) -> Self {
        assert_eq!(
            header_chunk.data_type, "IHDR",
            "Chunk is not a header chunk, instead is {}",
            &header_chunk.data_type
        );
        let png_image_meta = PngImageMetadata {
            width: combine_bytes(&header_chunk.data[0..DIMENSION_OFFSET]),
            height: combine_bytes(&header_chunk.data[DIMENSION_OFFSET..2 * DIMENSION_OFFSET]),
            bit_depth: header_chunk.data[2 * DIMENSION_OFFSET],
            color_type: header_chunk.data[2 * DIMENSION_OFFSET + 1],
            compression_method: header_chunk.data[2 * DIMENSION_OFFSET + 2],
            filter_method: header_chunk.data[2 * DIMENSION_OFFSET + 3],
            interlace_method: header_chunk.data[2 * DIMENSION_OFFSET + 4],
        };
        png_image_meta
    }
}
impl PngImage {
    pub fn new(meta: PngImageMetadata, data: &[u8]) -> Self {
        let png_image = PngImage {
            metadata: meta,
            data: data.to_vec(),
        };
        png_image
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
