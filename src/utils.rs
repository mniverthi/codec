pub fn combine_bytes(bytes: &[u8]) -> u32 {
    let one = bytes[0] as u32;
    let two = bytes[1] as u32;
    let three = bytes[2] as u32;
    let four = bytes[3] as u32;
    (one << 24 | two << 16 | three << 8 | four) as u32
}
pub unsafe fn split_type<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}
pub fn in_range<T>(t: T, lo: T, hi: T) -> bool {
    t > lo && t < hi
}
