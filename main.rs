mod adder;
mod multiplier;
mod gray_code;
mod evaluation;
mod truth_table;
mod negation_normal_form;
mod conjuntive_normal_form;
mod sat;
mod powerset;
mod set_evaluation;
mod curve;
mod inverse_function;

fn main() {
    println!("######### ADDER #########");
    let mut a: u32 = 91;
    let mut b: u32 = 5;
    assert_eq!(adder::adder(a, b), a + b, "the operation: {} + {} = {} is not equal.",a,b, a + b);
    println!("{} || {}", adder::adder(a, b), a + b);

    a = 4_294_967_294;
    b = 1;
    assert_eq!(adder::adder(a, b), a + b, "the operation: {} + {} = {} is not equal.",a,b, a + b);
    println!("{} || {}", adder::adder(a, b), a + b);

    a = 4_000_000_000;
    b = 294_967_295;
    assert_eq!(adder::adder(a, b), a + b, "the operation: {} + {} = {} is not equal.",a,b, a + b);
    println!("{} || {}", adder::adder(a, b), a + b);
    println!("######### ADDER #########");
    println!("");

    /**/
    println!("######### MULTIPLIER #########");
    let mut a: u32 = 3;
    let mut b: u32 = 5;
    assert_eq!(multiplier::multiplier(a, b), a * b, "the operation: {} * {} = {} is not equal.",a,b, a * b);
    println!("{} || {}", multiplier::multiplier(a, b), a * b);
    
    a = 300;
    b = 5;
    assert_eq!(multiplier::multiplier(a, b), a * b, "the operation: {} * {} = {} is not equal.",a,b, a * b);
    println!("{} || {}", multiplier::multiplier(a, b), a * b);
    
    a = 3001;
    b = 50;
    assert_eq!(multiplier::multiplier(a, b), a * b, "the operation: {} * {} = {} is not equal.",a,b, a * b);
    println!("{} || {}", multiplier::multiplier(a, b), a * b);
    println!("######### MULTIPLIER #########");
    println!("");

    /**/
    println!("######### GRAY #########");
    println!("{} || 0", gray_code::gray_code(0));
    println!("{} || 1", gray_code::gray_code(1));
    println!("{} || 3", gray_code::gray_code(2));
    println!("{} || 2", gray_code::gray_code(3));
    println!("{} || 6", gray_code::gray_code(4));
    println!("{} || 7", gray_code::gray_code(5));
    println!("{} || 5", gray_code::gray_code(6));
    println!("{} || 4", gray_code::gray_code(7));
    println!("{} || 12", gray_code::gray_code(8));
    println!("######### GRAY #########");
    println!("");

    /**/
    println!("######### BOOLEAN EVALUATION #########");
    println!("{} || true", evaluation::eval_formula("1"));
    println!("{} || false", evaluation::eval_formula("0"));
    println!("{} || false", evaluation::eval_formula("1!0&"));
    println!("{} || true", evaluation::eval_formula("11&"));
    println!("{} || false", evaluation::eval_formula("111&^"));
    println!("{} || true", evaluation::eval_formula("10|"));
    println!("######### BOOLEAN EVALUATION #########");
    println!("");

    /**/
    println!("######### TRUTH_TABLE #########");
    truth_table::print_truth_table("A!B&");
    println!("######### TRUTH_TABLE #########");
    println!("");

    /**/
    println!("######### NEGATION NORMAL FORM #########");
    let formulas = vec![
        "AB&!",
        "AB|!",
        "AB>",
        "AB=",
        "AB|C&!",
    ];

    for formula in formulas {
        println!("{} -> {}", formula, negation_normal_form::negation_normal_form(formula));
    }
    println!("######### NEGATION NORMAL FORM #########");
    println!("");

    /**/
    println!("######### CONJUNCTIVE NORMAL FORM #########");
    let formulas = vec![
        "AB&!",
        "AB|!",
        "AB|C&",
        "AB|C|D|",
        "AB&C&D&",
        "AB&!C!|",
        "AB|!C!&",
    ];

    for formula in formulas {
        println!("{} -> {}", formula, conjuntive_normal_form::conjunctive_normal_form(formula));
    }
    println!("######### CONJUNCTIVE NORMAL FORM #########");
    println!("");


    /**/
    println!("######### SAT #########");
    println!("{} || true", sat::sat("AB|"));
    println!("{} || true", sat::sat("AB&"));
    println!("{} || false", sat::sat("AA!&"));
    println!("{} || false", sat::sat("AA^"));
    println!("######### SAT #########");
    println!("");

    /**/
    println!("######### POWERSET #########");
    let set = vec![1, 2, 3];
    let pset = powerset::powerset(set);
    for subset in pset {
        println!("{:?}", subset);
    }
    println!("######### POWERSET #########");
    println!();


    println!("######### SET EVALUATION #########");
    let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
    let result = set_evaluation::eval_set("AB&", sets.clone());
    println!("{:?}", result);

    let result = set_evaluation::eval_set("AB|", sets);
    println!("{:?}", result);

    let sets = vec![vec![0, 1, 2]];
    let result = set_evaluation::eval_set("A!", sets.clone());
    println!("{:?}", result);

    println!("######### SET EVALUATION #########");
    println!("");

    /**/
    println!("######### CURVE #########");
    let test_pairs = vec![
        (0, 0),
        (32767, 65424),
        (65424, 32767),
        (10922, 21845),
        (54612, 43691),
        (65535, 65535),
    ];

    for &(x, y) in &test_pairs {
        let result = curve::map(x, y);
        println!("map({}, {}) = {}", x, y, result);
    }
    println!("######### CURVE #########");
    println!();

    /**/
    println!("######### INVERSE FUNCTION #########");
    let test_values = vec![0.0, 0.5, 0.75, 0.1, 0.9, 1.0];

    for &value in &test_values {
        let (x, y) = inverse_function::reverse_map(value);
        println!("reverse_map({}) = ({}, {})", value, x, y);
    }
    println!("######### INVERSE FUNCTION #########");


}