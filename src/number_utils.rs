pub fn fibonacci(index: u32) -> u64 {
    let mut seq = [0, 1];
    for _ in 0..index {
        seq = [seq[1], seq[0] + seq[1]];
    }
    seq[0]
}
