fn map(x: u16, y: u16) -> f64 {
    let mut r: u32 = 0;

    for i in 0..16 {
        r |= (((y >> i) & 1) as u32) << (i * 2);
        r |= (((x >> i) & 1) as u32) << (i * 2 + 1);
    }

    r as f64 / 0xffff_ffffu32 as f64
}

fn main() {
    let test_pairs = vec![
        (0, 0),
        (32767, 65424),
        (65424, 32767),
        (10922, 21845),
        (54612, 43691),
        (65535, 65535),
    ];

    println!("######### CURVE #########");
    for &(x, y) in &test_pairs {
        let result = map(x, y);
        println!("map({}, {}) = {}", x, y, result);
    }
    println!("######### CURVE #########");
    println!();
}
