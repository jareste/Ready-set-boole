use crate::truth_table::substitute;
use crate::evaluation::eval_formula;

pub fn sat(formula: &str) -> bool {
    let variables = formula.chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>();
    let var_count = variables.len();
    let rows = 1 << var_count;

    for i in 0..rows {
        let mut env = std::collections::HashMap::new();
        for (j, &var) in variables.iter().enumerate() {
            env.insert(var, (i & (1 << j)) != 0);
        }

        if eval_formula(&substitute(formula, &env)) {
            return true;
        }
    }
    false
}
