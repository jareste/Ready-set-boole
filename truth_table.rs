use crate::evaluation::eval_formula;

pub fn print_truth_table(formula: &str) {
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
