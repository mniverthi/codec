pub const QOI_PADDING: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];

pub const R_OFFSET: usize = 0;
pub const G_OFFSET: usize = 1;
pub const B_OFFSET: usize = 2;
pub const A_OFFSET: usize = 3;

pub const OP_INDEX: u8 = 0x00; /* 00xxxxxx */
pub const OP_DIFF: u8 = 0x40; /* 01xxxxxx */
pub const OP_LUMA: u8 = 0x80; /* 10xxxxxx */
pub const OP_RUN: u8 = 0xc0; /* 11xxxxxx */
pub const OP_RGB: u8 = 0xfe; /* 11111110 */
pub const OP_RGBA: u8 = 0xff; /* 11111111 */

#[derive(Debug, Default)]
pub struct QoiMetadata<'a> {
    pub magic_bytes: &'a str,
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub color_space: u8,
}

#[derive(Debug, Default, Eq, Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
