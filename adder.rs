fn adder(a: u32, b: u32) -> u32
{
    let mut a: u32 = a;
    let mut b: u32 = b;
    
    while b != 0
    {
        let carry = a & b;
        a = a ^ b;
        b = carry << 1;
    }
    
    a
}

fn main() {
    println!("######### ADDER #########");
    println!("{}", adder(91, 5));
    println!("{}", adder(4_294_967_294, 1));
    println!("{}", adder(4_294_967_295, 1));
    println!("######### ADDER #########");
    println!("");
}
