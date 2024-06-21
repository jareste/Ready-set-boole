mod adder;

fn multiplier(a: u32, b: u32) -> u32 
{
    let mut a = a;
    let mut b = b;
    let mut result = 0;
    while b != 0
    {
        if b & 1 != 0 
        {
            result = adder::adder(result, a);
        }
        a <<= 1;
        b >>= 1;
    }
    result
}

fn main() {
    println!("######### MULTIPLIER #########");
    let mut a: u32 = 3;
    let mut b: u32 = 5;
    assert_eq!(multiplier(a, b), a * b, "the operation: {} * {} = {} is not equal.",a,b, a * b);
    println!("{} || {}", multiplier(a, b), a * b);
    
    /**/
    a = 300;
    b = 5;
    assert_eq!(multiplier(a, b), a * b, "the operation: {} * {} = {} is not equal.",a,b, a * b);
    println!("{} || {}", multiplier(a, b), a * b);
    
    /**/
    a = 3001;
    b = 50;
    assert_eq!(multiplier(a, b), a * b, "the operation: {} * {} = {} is not equal.",a,b, a * b);
    println!("{} || {}", multiplier(a, b), a * b);
    println!("######### MULTIPLIER #########");
    println!("");
}
