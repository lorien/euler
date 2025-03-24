use euler::check_solution;

const TARGET: i64 = 2_000_000;

fn solution_common() -> i64 {
    let target_prime_limit = TARGET;
    assert!(target_prime_limit > 2);
    let mut num = 2;
    let mut prime_sum = num;
    let mut primes = vec![];
    loop {
        num += if num == 2 { 1 } else { 2 };
        if num >= target_prime_limit {
            break;
        }
        let mut is_prime = true;
        let num_sqrt = (num as f64).sqrt().floor() as i64;
        for &prime in &primes {
            if prime > num_sqrt {
                break;
            }
            if num % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            prime_sum += num;
            primes.push(num);
        }
    }
    prime_sum
}

fn solution_sieve() -> i64 {
    const limit: i64 = TARGET;
    assert!(limit > 2);
    // we use only 2..=limit indexes
    let mut sieve = [true; limit as usize + 1];
    let mut num = 2;
    let mut idx: usize;
    let limit_sqrt = limit.isqrt();
    while num <= limit_sqrt {
        idx = (num * num) as usize;
        while idx < limit as usize {
            sieve[idx] = false;
            idx += num as usize;
        }
        num += 1;
    }
    let mut prime_sum = 0;
    for idx in 2..limit {
        if sieve[idx as usize] {
            prime_sum += idx;
        }
    }
    prime_sum
}

fn main() {
    check_solution(10, "common", &solution_common);
    check_solution(10, "sieve", &solution_sieve);
}
