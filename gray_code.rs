fn gray_code(n: u32) -> u32
{
    n ^ (n >> 1)
}

fn main()
{
    println!("######### GRAY #########");
    println!("{} || 0", gray_code(0));
    println!("{} || 1", gray_code(1));
    println!("{} || 3", gray_code(2));
    println!("{} || 2", gray_code(3));
    println!("{} || 6", gray_code(4));
    println!("{} || 7", gray_code(5));
    println!("{} || 5", gray_code(6));
    println!("{} || 4", gray_code(7));
    println!("{} || 12", gray_code(8));
    println!("######### GRAY #########");
    println!("");
}

