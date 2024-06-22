pub fn eval_formula(formula: &str) -> bool {
    let mut stack = Vec::new();
    for c in formula.chars()
    {
        match c
        {
            '0' => stack.push(false),
            '1' => stack.push(true),
            '!' =>
            {
                if stack.len() == 1
                {
                    panic!("Invalid formula. please introduce a valid formula.");
                }
                let a = stack.pop().unwrap();
                stack.push(!a);
            }
            '&' =>
            {
                if stack.len() == 1
                {
                    panic!("Invalid formula. please introduce a valid formula.");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a && b);
            }
            '|' =>
            {
                if stack.len() == 1
                {
                    panic!("Invalid formula. please introduce a valid formula.");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a || b);
            }
            '^' =>
            {
                if stack.len() == 1
                {
                    panic!("Invalid formula. please introduce a valid formula.");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a ^ b);
            }
            '>' =>
            {
                if stack.len() == 1
                {
                    panic!("Invalid formula. please introduce a valid formula.");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(!a || b);
            }
            '=' =>
            {
                if stack.len() == 1
                {
                    panic!("Invalid formula. please introduce a valid formula.");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a == b);
            }
            _ => panic!("Invalid character found. please introduce only valid chars: 0, 1, !, &, |, ^, >, ="),
        }
    }
    if stack.len() != 1
    {
        panic!("Invalid formula. please introduce a valid formula.");
    }
    stack.pop().unwrap()
}

fn main()
{
    println!("######### BOOLEVAL #########");
    println!("{} || true", eval_formula("1"));
    println!("{} || false", eval_formula("0"));
    println!("{} || false", eval_formula("10&"));
    println!("{} || true", eval_formula("11&"));
    println!("{} || false", eval_formula("111&^"));
    println!("{} || true", eval_formula("10|"));
    println!("######### BOOLEVAL #########");
    println!("");
}
