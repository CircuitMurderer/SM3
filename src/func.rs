#[inline]
pub fn t_j(j: usize) -> u32 {
    if j >= 16 && j <= 63 { 0x7a879d8a }
    else if j <= 15 { 0x79cc4519 }
    else { 0x0u32 }
}

#[inline]
pub fn ff_j(j: usize, x: u32, y: u32, z: u32) -> u32 {
    if j >= 16 && j <= 63 { (x & y) | (x & z) | (y & z) }
    else if j <= 15 { x ^ y ^ z }
    else { 0x0u32 }
}

#[inline]
pub fn gg_j(j: usize, x: u32, y: u32, z: u32) -> u32 {
    if j >= 16 && j <= 63 { (x & y) | (!x & z) }
    else if j <= 15 { x ^ y ^ z }
    else { 0x0u32 }
}

#[inline]
pub fn p_0(x: u32) -> u32 {
    x ^ x.rotate_left(9) ^ x.rotate_left(17)
}

#[inline]
pub fn p_1(x: u32) -> u32 {
    x ^ x.rotate_left(15) ^ x.rotate_left(23)
}