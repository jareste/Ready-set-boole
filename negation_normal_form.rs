fn negation_normal_form(formula: &str) -> String {
    let mut stack = Vec::new();
    for c in formula.chars() {
        match c {
            '0' | '1' | 'A'..='Z' => stack.push(c.to_string()),
            '!' => {
                let a = stack.pop().unwrap();
                stack.push(format!("{}!", a));
            }
            '&' | '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(format!("{}{}{}", a, b, c));
            }
            _ => panic!("Invalid character in formula"),
        }
    }
    stack.pop().unwrap()
}

fn main() {
    println!("######### NEGATION NORM FORM #########");
    println!("{}", negation_normal_form("AB|!")); // A!B!|
    println!("######### NEGATION NORM FORM #########");
    println!("");
}
