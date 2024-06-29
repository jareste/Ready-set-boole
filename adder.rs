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
