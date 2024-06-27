#[derive(Debug)]
struct Params
{
    left: String,
    right: String,
}

fn get_params(formula: &str, pos: usize) -> Params
{
    let mut stack = Vec::new();
    for c in formula[..pos].chars()
    {
        stack.push(c.to_string());
    }
    let right = stack.pop().unwrap();
    let left = stack.pop().unwrap();
    Params { left, right }
}

fn rm_double_negations(s: &str) -> String
{
    let mut r = String::new();
    let mut should_pass = false;
    for i in 0..s.len()
    {
        if should_pass
        {
            should_pass = false;
            continue;
        }
        if s.chars().nth(i).unwrap() != '!'
        {
            r.push(s.chars().nth(i).unwrap());
            continue;
        }
        if i + 1 == s.len() || s.chars().nth(i + 1).unwrap() != '!'
        {
            r.push(s.chars().nth(i).unwrap());
            continue;
        }
        should_pass = true;
    }
    r
}

fn rm_material_conditions(s: &str) -> String
{
    let mut r = String::new();
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() != '>'
        {
            r.push(s.chars().nth(i).unwrap());
            continue;
        }
        let ps = get_params(&(r.clone() + ">"), r.len());
        let a = ps.left;
        let b = ps.right;
        r = r[..r.len() - b.len()].to_string();
        r.push_str(&format!("!{}|", b));
    }
    r
}

fn rm_equivalence(s: &str) -> String
{
    let mut r = String::new();
    for i in 0..s.len()
    {
        if s.chars().nth(i).unwrap() != '='
        {
            r.push(s.chars().nth(i).unwrap());
            continue;
        }
        let ps = get_params(&(r.clone() + "="), r.len());
        let a = ps.left;
        let b = ps.right;
        r.push_str(&format!(">{}{}>&", b, a));
    }
    r
}

fn rm_xor(s: &str) -> String {
    let mut r = String::new();
    for i in 0..s.len()
    {
        if s.chars().nth(i).unwrap() != '^'
        {
            r.push(s.chars().nth(i).unwrap());
            continue;
        }
        let ps = get_params(&(r.clone() + "^"), r.len());
        let a = ps.left;
        let b = ps.right;
        r.push_str(&format!("!&{}{}!&|", b, a));
    }
    r
}

fn apply_morgan(s: &str) -> String
{
    let mut r = String::new();
    let mut should_skip = false;
    for i in 0..s.len()
    {
        if should_skip
        {
            should_skip = false;
            continue;
        }
        if i == s.len() - 1 || !((s.chars().nth(i + 1).unwrap() == '!') && (s.chars().nth(i).unwrap() == '&' || s.chars().nth(i).unwrap() == '|'))
        {
            r.push(s.chars().nth(i).unwrap());
            continue;
        }
        let ps = get_params(&(r.clone() + "&"), r.len());
        let a = ps.left;
        let b = ps.right;
        r = r[..r.len() - b.len()].to_string();
        r.push_str(&format!("!{}!{}", b, if s.chars().nth(i).unwrap() == '|' { "&" } else { "|" }));
        should_skip = true;
    }
    r
}

fn is_valid(s: &str) -> bool
{
    !(s.contains("!!") || s.contains("&!") || s.contains("|!") || s.contains("=") || s.contains("^") || s.contains(">"))
}

fn negation_normal_form(formula: &str) -> String
{
    let mut s = formula.to_string();
    loop
    {
        s = rm_xor(&s);
        s = rm_equivalence(&s);
        s = rm_material_conditions(&s);
        s = apply_morgan(&s);
        s = rm_double_negations(&s);
        if is_valid(&s)
        {
            break;
        }
    }
    s
}

fn main()
{
    println!("######### NEGATION NORMAL FORM #########");
    println!("{} || A!B!|", negation_normal_form("AB&!"));
    println!("{} || A!B!&", negation_normal_form("AB|!"));
    println!("{} || A!B|", negation_normal_form("AB>"));
    println!("{} || AB&A!B!&|", negation_normal_form("AB="));
    println!("{} || A!B!&C!|", negation_normal_form("AB|C&!")); 
    println!("######### NEGATION NORMAL FORM #########");
    println!("");
}
