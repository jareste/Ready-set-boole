pub fn reverse_map(n: f64) -> (u16, u16) {
    let r = (n * 0xffff_ffffu32 as f64) as u32;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    
    for i in 0..16 {
        y |= (((r >> (i * 2)) & 1) as u16) << i;
        x |= (((r >> (i * 2 + 1)) & 1) as u16) << i;
    }
    
    (x, y)
}
