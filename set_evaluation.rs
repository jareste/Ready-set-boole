pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>
{
    let mut stack = Vec::new();
    for c in formula.chars()
    {
        match c {
            'A'..='Z' => stack.push(sets[c as usize - 'A' as usize].clone()),
            '!' => {
                let a = stack.pop().unwrap();
                let universe = sets.iter().flatten().copied().collect::<std::collections::HashSet<_>>();
                let a_set = a.into_iter().collect::<std::collections::HashSet<_>>();
                stack.push(universe.difference(&a_set).copied().collect());
            }
            '&' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let a_set = a.into_iter().collect::<std::collections::HashSet<_>>();
                let b_set = b.into_iter().collect::<std::collections::HashSet<_>>();
                stack.push(a_set.intersection(&b_set).copied().collect());
            }
            '|' => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let a_set = a.into_iter().collect::<std::collections::HashSet<_>>();
                let b_set = b.into_iter().collect::<std::collections::HashSet<_>>();
                stack.push(a_set.union(&b_set).copied().collect());
            }
            _ => panic!("Invalid character in formula"),
        }
    }
    stack.pop().unwrap()
}
