use euler::check_solution;

const TARGET: i64 = 600851475143;

fn solution_common() -> i64 {
    let target = TARGET;
    assert!(target > 2);
    let mut result = None;
    let mut primes = vec![];
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
                primes.push(num);
                result = Some(num);
            }
        }
    }
    result.unwrap()
}

fn solution_with_division() -> i64 {
    let mut target = TARGET;
    assert!(target > 2);
    let mut num = 2;
    let mut result = None;
    while target > 1 {
        if target % num == 0 {
            target /= num;
            while target % num == 0 {
                target /= num;
            }
            result = Some(num);
        }
        num += 1;
    }
    result.unwrap()
}

fn solution_with_division_skip_even() -> i64 {
    let mut target = TARGET;
    assert!(target > 2);
    let mut num = 2;
    let mut result = None;
    while target > 1 {
        if target % num == 0 {
            target /= num;
            while target % num == 0 {
                target /= num;
            }
            result = Some(num);
        }
        // 2, 3, 5, 7 ...
        num += if num == 2 { 1 } else { 2 };
    }
    result.unwrap()
}

fn solution_with_division_skip_even_cap_sqrt() -> i64 {
    let mut target = TARGET;
    assert!(target > 2);
    let mut num = 2;
    let mut result = None;
    let mut max_factor = (target as f64).sqrt().floor() as i64;
    while target > 1 && num <= max_factor {
        if target % num == 0 {
            target /= num;
            while target % num == 0 {
                target /= num;
            }
            result = Some(num);
            max_factor = (target as f64).sqrt().floor() as i64;
        }
        // 2, 3, 5, 7 ...
        num += if num == 2 { 1 } else { 2 };
    }
    if target == 1 {
        result.unwrap()
    } else {
        target
    }
}

fn main() {
    check_solution(3, "common", &solution_common);
    check_solution(3, "with-division", &solution_with_division);
    check_solution(
        3,
        "with-division-skip-even",
        &solution_with_division_skip_even,
    );
    check_solution(
        3,
        "with-division-skip-even-cap-sqrt",
        &solution_with_division_skip_even_cap_sqrt,
    );
}
