fn main() {
    let l = 0x1234567890123456u64;
    // assert_eq!(l.to_be_bytes(), [0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]);

    let mut chunk = Vec::new();
    chunk.push(((l >> 56) & 0xff) as u8);
    chunk.push(((l >> 48) & 0xff) as u8);
    chunk.push(((l >> 40) & 0xff) as u8);
    chunk.push(((l >> 32) & 0xff) as u8);
    chunk.push(((l >> 24) & 0xff) as u8);
    chunk.push(((l >> 16) & 0xff) as u8);
    chunk.push(((l >> 8) & 0xff) as u8);
    chunk.push((l & 0xff) as u8);
    assert_eq!(l.to_be_bytes(), chunk.as_slice());

    let b_i = [0x1u8, 0x2u8, 0x3u8, 0x4u8];
    let k = u32::from(b_i[0]) << 24
            | u32::from(b_i[1]) << 16
            | u32::from(b_i[2]) << 8
            | u32::from(b_i[3]);

    assert_eq!(u32::from_be_bytes(b_i), k);

    let x = [0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12] as [u32; 8];
    println!("{:#?}", &x[4..9]);
}