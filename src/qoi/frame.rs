use crate::qoi::chunk::*;
use crate::utils::{in_range, split_type};

fn index_position(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32 * 3 + g as u32 * 5 + b as u32 * 7 + a as u32 * 11) % 64
}
pub fn encode_bytes(raw_bytes: &[u8], w: u32, h: u32, cc: u8, cs: u8) -> Vec<u8> {
    let mut buffer = Vec::new();
    let header_chunk = QoiMetadata {
        magic_bytes: "qoif",
        width: w,
        height: h,
        channels: cc,
        color_space: cs,
    };
    let mut cache = [Pixel::default(); 64];
    unsafe {
        buffer.extend(split_type(&header_chunk).to_vec());
    }
    let mut prev_pixel = Pixel {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    let mut curr_streak: u8 = 0;
    let total_bytes =
        (header_chunk.width * header_chunk.height * header_chunk.channels as u32) as usize;
    for i in (0..total_bytes as usize).step_by(header_chunk.channels as usize) {
        let curr_pixel = Pixel {
            r: raw_bytes[i + R_OFFSET],
            g: raw_bytes[i + G_OFFSET],
            b: raw_bytes[i + B_OFFSET],
            a: raw_bytes[i + A_OFFSET],
        };
        if curr_pixel == prev_pixel {
            curr_streak += 1;
            if curr_streak == 62 || i == (total_bytes - header_chunk.channels as usize) {
                buffer.push(OP_RUN | (curr_streak - 1));
                curr_streak = 0;
            }
        } else {
            let cache_idx =
                index_position(curr_pixel.r, curr_pixel.g, curr_pixel.b, curr_pixel.a) as usize;
            assert!(cache_idx >= 0 && cache_idx < 64);
            if curr_streak > 0 {
                buffer.push(OP_RUN | (curr_streak - 1));
                curr_streak = 0;
            } else if cache[cache_idx] == curr_pixel {
                buffer.push(OP_INDEX | cache_idx as u8);
            } else {
                cache[cache_idx] = curr_pixel;
                if prev_pixel.a == curr_pixel.a {
                    let r_diff = (curr_pixel.r - prev_pixel.r) as i8;
                    let g_diff = (curr_pixel.g - prev_pixel.g) as i8;
                    let b_diff = (curr_pixel.b - prev_pixel.b) as i8;

                    let rg_diff = r_diff - g_diff;
                    let bg_diff = b_diff - g_diff;

                    if in_range(r_diff, -3, 2) && in_range(g_diff, -3, 2) && in_range(b_diff, -3, 2)
                    {
                        buffer.push(
                            OP_DIFF
                                | ((r_diff + 2) << 4) as u8
                                | ((g_diff + 2) << 4) as u8
                                | ((b_diff + 2) << 4) as u8,
                        );
                    } else if in_range(rg_diff, -9, 8)
                        && in_range(g_diff, -33, 32)
                        && in_range(bg_diff, -9, 8)
                    {
                        buffer.push(OP_LUMA | (g_diff + 32) as u8);
                        buffer.push(((rg_diff + 8) << 4) as u8 | (bg_diff + 8) as u8);
                    } else {
                        buffer.push(OP_RGB);
                        unsafe { buffer.extend(split_type(&curr_pixel).iter()) };
                        buffer.pop();
                    }
                } else {
                    buffer.push(OP_RGBA);
                    unsafe {
                        buffer.extend(split_type(&curr_pixel).iter());
                    }
                }
            }
        }
        prev_pixel = curr_pixel;
    }
    buffer
}
