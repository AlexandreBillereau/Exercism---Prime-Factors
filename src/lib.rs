pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut number = n;
    let mut div = 2;
    while div <= number {
        if number % div == 0 {
            factors.push(div);
            number /= div;
        } else {
            div += 1;
        }
    }
    factors
}
