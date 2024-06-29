#[derive(Debug, Clone)]
struct Params {
    left: Option<Box<Params>>,
    right: Option<Box<Params>>,
    value: String,
}

impl Params {
    fn new() -> Self {
        Params {
            left: None,
            right: None,
            value: String::new(),
        }
    }
}

fn get_params(s: &str, i: usize) -> Params {
    let mut r = Params::new();
    r.left = Some(Box::new(Params::new()));
    r.right = Some(Box::new(Params::new()));

    match s.chars().nth(i).unwrap() {
        '!' => {
            r.left.as_mut().unwrap().value = s.chars().nth(i - 1).unwrap().to_string();
            r.value = r.left.as_ref().unwrap().value.clone() + "!";
        }
        '&' | '|' | '>' | '=' | '^' => {
            r.right = Some(Box::new(get_params(s, i - 1)));
            r.left = Some(Box::new(get_params(s, i - 1 - r.right.as_ref().unwrap().value.len())));
            r.value = r.left.as_ref().unwrap().value.clone() + &r.right.as_ref().unwrap().value + &s.chars().nth(i).unwrap().to_string();
        }
        _ => {
            r.value = s.chars().nth(i).unwrap().to_string();
        }
    }

    r
}

fn rm_double_negations(s: &str) -> String {
    let mut r = String::new();
    let mut should_pass = false;

    for (i, c) in s.chars().enumerate() {
        if should_pass {
            should_pass = false;
            continue;
        }

        if c != '!' {
            r.push(c);
            continue;
        }

        if i + 1 == s.len() || s.chars().nth(i + 1).unwrap() != '!' {
            r.push(c);
            continue;
        }

        should_pass = true;
    }

    r
}

fn rm_material_conditions(s: &str) -> String {
    let mut r = String::new();

    for (i, c) in s.chars().enumerate() {
        if c != '>' {
            r.push(c);
            continue;
        }

        let ps = get_params(&(r.clone() + ">"), r.len());
        let a = ps.left.as_ref().unwrap().value.clone();
        let b = ps.right.as_ref().unwrap().value.clone();

        r = r[0..r.len() - b.len()].to_string();
        r += &("!".to_owned() + &a + "|" + &b);
    }

    r
}

fn rm_equivalence(s: &str) -> String {
    let mut r = String::new();

    for (i, c) in s.chars().enumerate() {
        if c != '=' {
            r.push(c);
            continue;
        }

        let ps = get_params(&(r.clone() + "="), r.len());
        let a = ps.left.as_ref().unwrap().value.clone();
        let b = ps.right.as_ref().unwrap().value.clone();

        r += &(">".to_owned() + &b + &a + ">&");
    }

    r
}

fn rm_xor(s: &str) -> String {
    let mut r = String::new();

    for (i, c) in s.chars().enumerate() {
        if c != '^' {
            r.push(c);
            continue;
        }

        let ps = get_params(&(r.clone() + "^"), r.len());
        let a = ps.left.as_ref().unwrap().value.clone();
        let b = ps.right.as_ref().unwrap().value.clone();

        r += &("!&".to_owned() + &b + &a + "!&|");
    }

    r
}

fn apply_morgan(s: &str) -> String {
    let mut r = String::new();
    let mut should_skip = false;

    for (i, c) in s.chars().enumerate() {
        if should_skip {
            should_skip = false;
            continue;
        }

        if i == s.len() - 1 || !((s.chars().nth(i + 1).unwrap() == '!') && (c == '&' || c == '|')) {
            r.push(c);
            continue;
        }

        let ps = get_params(&(r.clone() + "&"), r.len());
        let a = ps.left.as_ref().unwrap().value.clone();
        let b = ps.right.as_ref().unwrap().value.clone();

        r = r[0..r.len() - b.len()].to_string();
        r += &("!".to_owned() + &b + "!" + if c == '|' { "&" } else { "|" });
        should_skip = true;
    }

    r
}

fn is_valid(s: &str) -> bool {
    !s.contains("!!") && !s.contains("&!") && !s.contains("|!") && !s.contains("=") && !s.contains("^") && !s.contains(">")
}

fn negation_normal_form(formula: &str) -> String {
    let mut s = formula.to_string();
    loop {
        s = rm_xor(&s);
        s = rm_equivalence(&s);
        s = rm_material_conditions(&s);
        s = apply_morgan(&s);
        s = rm_double_negations(&s);
        if is_valid(&s) {
            break;
        }
    }
    s
}

fn is_operator(c: char) -> bool {
    c == '&' || c == '|'
}

fn move_and_back(s: &str) -> String {
    let mut r = String::new();

    for (i, c) in s.chars().enumerate() {
        r.push(c);
        if !is_operator(c) {
            continue;
        }

        let ps = get_params(r.as_str(), r.len() - 1);
        let le = ps.left.as_ref().unwrap();
        let ri = ps.right.as_ref().unwrap();

        if ps.value.ends_with('&') && le.value.ends_with('&') {
            let _a = le.left.as_ref().unwrap().value.clone();
            let _b = le.right.as_ref().unwrap().value.clone();
            let c = ri.value.clone();

            r = r[0..r.len() - c.len() - 2].to_string();
            r += &(c + "&&");
        } else if ps.value.ends_with('|') && ri.value.ends_with('&') {
            let a = le.value.clone();
            let b = ri.left.as_ref().unwrap().value.clone();
            let c = ri.right.as_ref().unwrap().value.clone();

            r = r[0..r.len() - c.len() - 2].to_string();
            r += &("|".to_owned() + &a + &c + "|" + &b + "&");
        } else if ps.value.ends_with('|') && le.value.ends_with('&') {
            let a = le.value.clone();
            let b = ri.left.as_ref().unwrap().value.clone();
            let c = ri.right.as_ref().unwrap().value.clone();

            r = r[0..r.len() - a.len() - b.len() - c.len() - 2].to_string();
            r += &(a.clone() + &b + "|" + &a + &c + "|" + &b + "&");
        }
    }

    r
}

fn is_valid_cnf(s: &str) -> bool {
    let mut and_zone = true;

    for c in s.chars().rev() {
        if c == '&' && !and_zone {
            return false;
        }
        if c == '&' {
            continue;
        }
        if c != '&' {
            and_zone = false;
        }
    }

    true
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let mut r = negation_normal_form(formula);

    while !is_valid_cnf(&r) {
        r = move_and_back(&r);
    }

    r
}
