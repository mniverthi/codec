use crate::image;
use std::cmp::min;
pub const BYTES_PER_PIXEL: usize = 4;
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
    curr_scan_line
        .iter()
        .scan(0 as u8, |state, &x| {
            let out = *state + x;
            *state = x;
            Some(out)
        })
        .collect()
}
fn reverse_up_filter(curr_scan_line: &[u8], prev_reconstructed_line: &[u8]) -> Vec<u8> {
    curr_scan_line
        .iter()
        .zip(prev_reconstructed_line.iter())
        .map(|(&a, &b)| a + b)
        .collect()
}
fn reverse_avg_filter(curr_scan_line: &[u8], prev_reconstructed_line: &[u8]) -> Vec<u8> {
    let up = reverse_up_filter(curr_scan_line, prev_reconstructed_line);
    let sub = reverse_sub_filter(curr_scan_line);
    up.iter()
        .zip(sub.iter())
        .zip(curr_scan_line.iter())
        .map(|((&a, &b), &c)| (((a as f32 + b as f32) / 2.0).floor() + c as f32) as u8)
        .collect()
}
fn reverse_paeth_filter(curr_scan_line: &[u8], prev_reconstructed_line: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for i in 0..curr_scan_line.len() {
        let left = if i != 0 { out[i - 1] as i32 } else { 0 };
        let up = prev_reconstructed_line[i] as i32;
        let diag = if i != 0 {
            prev_reconstructed_line[i - 1] as i32
        } else {
            0
        };
        let p = left + up - diag;
        let pleft = (p - left).abs();
        let pup = (p - up).abs();
        let pdiag = (p - diag).abs();
        out.push(curr_scan_line[i] + min(min(pleft, pup), pdiag) as u8);
    }
    out
}
pub fn reconstruct_image(
    png_image: &image::PngImage,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut reconstruction = Vec::new();
    let width = png_image.width as usize;
    let height = png_image.height as usize;
    let stride: usize = 1 + width * BYTES_PER_PIXEL;
    assert_eq!(
        png_image.data.len() as u32,
        png_image.height * stride as u32,
        "Invalid image data, dimension mismatch"
    );
    for idx in (0..height * stride).step_by(stride) {
        let curr_scan_line = &png_image.data[idx + 1..idx + stride];
        let prev_reconstructed_line =
            &reconstruction[reconstruction.len() - stride..reconstruction.len()];
        let filter_type = match_filter(curr_scan_line[0])?;
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
    Ok(reconstruction)
}
