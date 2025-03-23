use euler::check_solution;
use std::collections::HashSet;

const TARGET: i64 = 600851475143;

fn find_max_prime_factor() -> i64 {
    let target = TARGET;
    if target < 2 {
        panic!("Target  must be 2 or greater")
    }
    let mut result = None;
    let mut primes = HashSet::new();
    for num in 2..=((target as f64).sqrt().floor() as i64) {
        if target % num == 0 {
            let mut is_prime = true;
            for prime in &primes {
                if num % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.insert(num);
                result = Some(num);
            }
        }
    }
    if let None = result {
        panic!("Could not find prime factors of number {}", target);
    } else {
        result.unwrap()
    }
}

fn main() {
    check_solution(3, "common", &find_max_prime_factor);
}
