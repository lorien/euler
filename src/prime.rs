fn is_prime(num: i32) -> bool {
    let num_sqrt = num.isqrt();
    let mut div = 4;
    while div <= num_sqrt {
        if num % div == 0 {
            return false;
        }
        div += 1;
    }
    return true;
}

fn collect_primes(target_idx: i32) -> Vec<i32> {
    let mut primes: Vec<i32> = vec![];
    primes.push(0);
    primes.push(2);
    primes.push(3);
    let mut prime_idx = 2;
    let mut num = 5;
    while prime_idx < target_idx {
        for delta in (0..3).step_by(2) {
            if is_prime(num + delta) {
                primes.push(num + delta);
                prime_idx += 1;
                if prime_idx >= target_idx {
                    break;
                }
            }
        }
        num += 6;
    }
    primes
}
