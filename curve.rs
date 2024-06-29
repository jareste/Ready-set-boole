pub fn map(x: u16, y: u16) -> f64 {
    let mut r: u32 = 0;

    for i in 0..16 {
        r |= (((y >> i) & 1) as u32) << (i * 2);
        r |= (((x >> i) & 1) as u32) << (i * 2 + 1);
    }

    r as f64 / 0xffff_ffffu32 as f64
}
