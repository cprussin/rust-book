use std::collections::BTreeMap;

pub fn fibonacci(index: u32) -> u64 {
    let mut seq = [0, 1];
    for _ in 0..index {
        seq = [seq[1], seq[0] + seq[1]];
    }
    seq[0]
}

pub fn median(values: &mut Vec<i32>) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if values.len() % 2 == 0 {
        let half = values.len() / 2;
        (values[half] + values[half - 1]) as f64 / 2.0
    } else {
        values[values.len() / 2] as f64
    }
}

pub fn mode(values: &Vec<i32>) -> i32 {
    let mut map = BTreeMap::new();
    for value in values {
        *map.entry(value).or_insert(0) += 1;
    }
    let mut vec = Vec::from_iter(map);
    vec.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    *vec[0].0
}
