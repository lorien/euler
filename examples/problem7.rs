use euler::check_solution;

const TARGET: i64 = 10001;

fn solution_common() -> i64 {
    let target_prime_idx = TARGET;
    assert!(target_prime_idx > 0);
    let mut prime_idx = 1;
    let mut num = 2;
    let mut target_prime = num;
    let mut primes: Vec<i64> = vec![];
    while prime_idx < target_prime_idx {
        num += if num == 2 { 1 } else { 2 };
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
            primes.push(num);
            target_prime = num;
            prime_idx += 1;
        }
    }
    target_prime
}

fn main() {
    check_solution(7, "common", &solution_common);
}
