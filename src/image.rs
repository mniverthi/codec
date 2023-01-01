use std::io::Read;
use std::error::Error;
use flate2;
pub struct PngImage {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
    pub color_type: u32,
    pub compression_method: u32,
    pub filter_method: u32,
    pub interlace_method: u32,
    pub data: Vec<u8>
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