pub fn adder(a: u32, b: u32) -> u32
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
    let mut a: u32 = 91;
    let mut b: u32 = 5;
    assert_eq!(adder(a, b), a + b, "the operation: {} + {} = {} is not equal.",a,b, a + b);
    println!("{} || {}", adder(a, b), a + b);

    /**/
    a = 4_294_967_294;
    b = 1;
    assert_eq!(adder(a, b), a + b, "the operation: {} + {} = {} is not equal.",a,b, a + b);
    println!("{} || {}", adder(a, b), a + b);

    /**/
    a = 4_000_000_000;
    b = 294_967_295;
    assert_eq!(adder(a, b), a + b, "the operation: {} + {} = {} is not equal.",a,b, a + b);
    println!("{} || {}", adder(a, b), a + b);
    println!("######### ADDER #########");
    println!("");
}
