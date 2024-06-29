use crate::adder;

pub fn multiplier(a: u32, b: u32) -> u32 
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
