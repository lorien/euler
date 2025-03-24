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

fn solution_another() -> i64 {
    let target_prime_idx = TARGET;
    assert!(target_prime_idx > 0);
    let mut prime_idx = 1;
    let mut num = 2;
    let mut target_prime = num;
    while prime_idx < target_prime_idx {
        num += if num == 2 { 1 } else { 2 };
        let mut is_prime = true;
        if num != 3 && num % 3 == 0 {
            continue;
        }
        let num_sqrt = (num as f64).sqrt().floor() as i64;
        let mut factor = 5;
        while factor <= num_sqrt {
            if num % factor == 0 || num % (factor + 2) == 0 {
                is_prime = false;
                break;
            }
            factor += 6;
        }
        if is_prime {
            target_prime = num;
            prime_idx += 1;
        }
    }
    target_prime
}

fn main() {
    check_solution(7, "common", &solution_common);
    check_solution(7, "another", &solution_another);
}
