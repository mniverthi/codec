use crate::png::frame;
pub const BYTES_PER_PIXEL: usize = 4;

#[derive(Debug)]
enum PngFilter {
    Zero(u8),
    Sub(u8),
    Up(u8),
    Avg(u8),
    Paeth(u8),
}

fn match_filter(filter_byte: u8) -> Result<PngFilter, Box<dyn std::error::Error>> {
    match filter_byte {
        0 => Ok(PngFilter::Zero(0)),
        1 => Ok(PngFilter::Sub(1)),
        2 => Ok(PngFilter::Up(2)),
        3 => Ok(PngFilter::Avg(3)),
        4 => Ok(PngFilter::Paeth(4)),
        _ => Err("Invalid filter type")?,
    }
}
fn reverse_zero_filter(curr_scan_line: &[u8]) -> Vec<u8> {
    curr_scan_line.to_vec()
}

fn reverse_sub_filter(curr_scan_line: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for i in 0..curr_scan_line.len() {
        if i as i32 - BYTES_PER_PIXEL as i32 >= 0 {
            result.push(curr_scan_line[i].wrapping_add(result[i - BYTES_PER_PIXEL]));
        } else {
            result.push(curr_scan_line[i]);
        }
    }
    result
}
fn reverse_up_filter(curr_scan_line: &[u8], prev_reconstructed_line: &[u8]) -> Vec<u8> {
    curr_scan_line
        .iter()
        .zip(prev_reconstructed_line.iter())
        .map(|(&a, &b)| a.wrapping_add(b))
        .collect()
}
fn reverse_avg_filter(curr_scan_line: &[u8], prev_reconstructed_line: &[u8]) -> Vec<u8> {
    let up = reverse_up_filter(curr_scan_line, prev_reconstructed_line);
    let sub = reverse_sub_filter(curr_scan_line);
    up.iter()
        .zip(sub.iter())
        .zip(curr_scan_line.iter())
        .map(|((&a, &b), &c)| (((a as i16 + b as i16) / 2) + c as i16) as u8)
        .collect()
}
fn reverse_paeth_filter(curr_scan_line: &[u8], prev_reconstructed_line: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for i in 0..curr_scan_line.len() {
        let left = if i >= BYTES_PER_PIXEL {
            out[i - BYTES_PER_PIXEL] as i32
        } else {
            0
        };
        let up = prev_reconstructed_line[i] as i32;
        let diag = if i >= BYTES_PER_PIXEL {
            prev_reconstructed_line[i - BYTES_PER_PIXEL] as i32
        } else {
            0
        };
        let p = left + up - diag;
        let pleft = (p - left).abs();
        let pup = (p - up).abs();
        let pdiag = (p - diag).abs();
        let mut decoded_byte = diag as u8;
        if pleft <= pup && pleft <= pdiag {
            decoded_byte = left as u8;
        } else if pup <= pdiag {
            decoded_byte = up as u8;
        }
        decoded_byte = decoded_byte.wrapping_add(curr_scan_line[i]);
        out.push(decoded_byte);
    }
    out
}
pub fn reconstruct_image(
    metadata: &frame::PngImageMetadata,
    data: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut reconstruction = Vec::new();
    let width = metadata.width as usize;
    let height = metadata.height as usize;
    let stride: usize = 1 + width * BYTES_PER_PIXEL;
    let default_buffer = vec![0 as u8; stride - 1];
    assert_eq!(
        data.len() as u32,
        metadata.height * stride as u32,
        "Invalid image data, dimension mismatch"
    );
    for idx in (0..height * stride).step_by(stride) {
        let curr_scan_line = &data[idx + 1..idx + stride];
        let prev_reconstructed_line = if reconstruction.len() != 0 {
            &reconstruction[reconstruction.len() - (stride - 1)..reconstruction.len()]
        } else {
            &default_buffer
        };
        let filter_type = match_filter(data[idx])?;
        let mut reconstructed_row = match filter_type {
            PngFilter::Zero(0) => reverse_zero_filter(curr_scan_line),
            PngFilter::Sub(1) => reverse_sub_filter(curr_scan_line),
            PngFilter::Up(2) => reverse_up_filter(curr_scan_line, prev_reconstructed_line),
            PngFilter::Avg(3) => reverse_avg_filter(curr_scan_line, prev_reconstructed_line),
            PngFilter::Paeth(4) => reverse_paeth_filter(curr_scan_line, prev_reconstructed_line),
            _ => Err("Invalid filter type")?,
        };
        reconstruction.append(&mut reconstructed_row);
    }
    assert_eq!(reconstruction.len(), width * height * BYTES_PER_PIXEL);
    Ok(reconstruction)
}
