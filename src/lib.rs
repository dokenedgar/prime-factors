pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut divisor: u64 = 2;
    let mut quotient = n;
    if n == 1 {
        return prime_factors;
    } else if n == 2 || n == 3 {
        prime_factors.push(n);
        return prime_factors;
    }
    loop {
        if divisor == n {
            break;
        } else {
            if quotient % divisor == 0 {
                quotient = quotient / divisor;
                prime_factors.push(divisor);
            } else {
                divisor += 1;
            }
        }
    }
    prime_factors
}
