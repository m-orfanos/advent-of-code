pub fn evaluate_intermediary_states(history: Vec<i64>) -> Vec<Vec<i64>> {
    let mut pyramid: Vec<Vec<i64>> = vec![];
    pyramid.push(history);
    loop {
        let next = get_next(&pyramid[pyramid.len() - 1]);
        let cnt = next.iter().filter(|n| n.eq(&&0)).count();
        if cnt == next.len() {
            break;
        }
        pyramid.push(next);
    }
    pyramid
}

fn get_next(digits: &Vec<i64>) -> Vec<i64> {
    let mut next = vec![];
    let mut j = 1;
    for i in 0..digits.len() - 1 {
        let a = digits[i];
        let b = digits[j];

        let c = b - a;
        next.push(c);

        j += 1;
    }
    next
}
