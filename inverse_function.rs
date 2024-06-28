fn reverse_map(n: f64) -> (u16, u16) {
    let r = (n * 0xffff_ffffu32 as f64) as u32;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    
    for i in 0..16 {
        y |= (((r >> (i * 2)) & 1) as u16) << i;
        x |= (((r >> (i * 2 + 1)) & 1) as u16) << i;
    }
    
    (x, y)
}

fn main() {
    // Test cases
    let test_values = vec![0.0, 0.5, 0.75, 0.1, 0.9, 1.0];

    println!("######### INVERSE FUNCTION #########");
    for &value in &test_values {
        let (x, y) = reverse_map(value);
        println!("reverse_map({}) = ({}, {})", value, x, y);
    }
    println!("######### INVERSE FUNCTION #########");
}
