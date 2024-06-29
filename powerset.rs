pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let n = set.len();
    for i in 0..(1 << n) {
        let mut subset = Vec::new();
        for j in 0..n {
            if i & (1 << j) != 0 {
                subset.push(set[j]);
            }
        }
        result.push(subset);
    }
    result
}
