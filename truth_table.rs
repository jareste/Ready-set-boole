// sobra
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
                if stack.len() == 0
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

fn print_truth_table(formula: &str) {
    let mut variables = Vec::new();
    for c in formula.chars() {
        if c.is_alphabetic() && !variables.contains(&c) {
            variables.push(c);
        }
    }

    let rows = 1 << variables.len();
    for i in 0..rows {
        let mut env = std::collections::HashMap::new();
        for (j, &var) in variables.iter().enumerate() {
            env.insert(var, (i & (1 << j)) != 0);
        }

        for &var in &variables {
            print!("| {} ", if *env.get(&var).unwrap() { 1 } else { 0 });
        }
        print!("| {} |\n", if eval_formula(&substitute(formula, &env)) { 1 } else { 0 });
    }
}

pub fn substitute(formula: &str, env: &std::collections::HashMap<char, bool>) -> String {
    let mut result = String::new();
    for c in formula.chars() {
        if c.is_alphabetic() {
            result.push(if *env.get(&c).unwrap() { '1' } else { '0' });
        } else {
            result.push(c);
        }
    }
    result
}

fn main() {
    println!("######### TRUTH_TABLE #########");
    print_truth_table("A!B&");
    println!("######### TRUTH_TABLE #########");
    println!("");
}
