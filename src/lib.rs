pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut divisor: u64 = 2;

    loop {
        if n > 1 {
            loop {
                if n % divisor == 0 {
                    prime_factors.push(divisor);
                    n /= divisor;
                } else {
                    break;
                }
            }
            divisor += 1;
        } else {
            break;
        }
    }

    prime_factors
}
