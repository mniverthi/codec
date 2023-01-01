pub const CHUNK_LENGTH_OFFSET: u32 = 4;
pub const CHUNK_TYPE_OFFSET: u32 = 4;
pub const CRC_OFFSET: u32 = 4;
pub struct Chunk {
    pub length: u32,
    pub datatype: String,
    pub data: Vec<u8>,
    pub cyclic_redundancy_check: u32
}
pub fn combine_bytes(bytes: &[u8]) -> u32 {
    let one = bytes[0] as u32;
    let two = bytes[1] as u32;
    let three = bytes[2] as u32;
    let four = bytes[3] as u32;
    (one << 24 | two << 16 | three << 8 | four) as u32
}
pub fn combine_chunk_data(chunks: &[Chunk]) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    for c in chunks {
        if c.datatype.eq(&"IDAT") {
            buffer.extend(c.data.iter());
        }
    }
    return buffer;
}