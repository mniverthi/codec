const CHUNK_LENGTH_OFFSET: usize = 4;
const CHUNK_TYPE_OFFSET: usize = 4;
const CRC_OFFSET: usize = 4;
const PNG_HEADER_OFFSET: usize = 8;
#[derive(Debug, Default)]
pub struct Chunk {
    pub length: u32,
    pub data_type: String,
    pub data: Vec<u8>,
    pub cyclic_redundancy_check: u32,
}
pub fn compare_vecs<T: std::cmp::PartialEq>(v1: &[T], v2: &[T]) -> bool {
    let matching = v1
        .iter()
        .zip(v2.iter())
        .filter(|&(v1, v2)| v1 == v2)
        .count();
    matching == v1.len() && matching == v2.len()
}
pub fn extract_chunks(image_bytes: &[u8]) -> Result<Vec<Chunk>, Box<dyn std::error::Error>> {
    if !compare_vecs(
        &image_bytes[0..PNG_HEADER_OFFSET],
        &vec![137, 80, 78, 71, 13, 10, 26, 10],
    ) {
        Err("Not a PNG file")?;
    }
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut idx = PNG_HEADER_OFFSET;
    let size = image_bytes.len() as usize;
    while idx < size {
        let chunk_length = combine_bytes(&image_bytes[idx..(idx + CHUNK_LENGTH_OFFSET)]);
        idx += CHUNK_LENGTH_OFFSET;

        let chunk_type = std::str::from_utf8(&image_bytes[idx..(idx + CHUNK_TYPE_OFFSET)]).unwrap();
        idx += CHUNK_TYPE_OFFSET;

        let mut chunk_data_buffer = vec![0 as u8; chunk_length as usize];
        let chunk_data_slice = &image_bytes[idx..(idx + (chunk_length as usize))];
        chunk_data_buffer.clone_from_slice(&chunk_data_slice);
        idx += chunk_length as usize;

        let crc = combine_bytes(&image_bytes[idx..(idx + CRC_OFFSET)]);
        idx += CRC_OFFSET;

        // println!("{}", chunk_length);
        // println!("{}", chunk_type);
        // println!("{:?}", chunk_data_buffer);
        // println!("{}", crc);
        let curr_chunk = Chunk {
            length: chunk_length,
            data_type: chunk_type.to_string(),
            data: chunk_data_buffer,
            cyclic_redundancy_check: crc,
        };
        chunks.push(curr_chunk);
    }
    Ok(chunks)
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
        if c.data_type.eq(&"IDAT") {
            buffer.extend(c.data.iter());
        }
    }
    buffer
}
