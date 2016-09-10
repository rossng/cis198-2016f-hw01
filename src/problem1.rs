pub fn sum(slice: &[i32]) -> i32 {
    slice.iter().fold(0, |acc, &x| acc + x)
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for v in vs {
        if !result.contains(v) {
            result.push(*v);
        }
    }
    result
}

pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut result = vec![];
    for v in vs {
        if pred(*v) {
            result.push(*v);
        }
    }
    result
}
