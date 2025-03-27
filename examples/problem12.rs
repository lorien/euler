use euler;
use std::time::Instant;

const FIRST_PRIMES: &[i64] = &[2, 3, 5, 7];
const FIRST_PRIMES_LEN: usize = 4;

fn collect_primes(primes: Vec<i64>, target_idx: i64) -> Vec<i64> {
    let mut result = vec![];
    if primes.len() < FIRST_PRIMES_LEN {
        for &num in FIRST_PRIMES.iter() {
            result.push(num);
        }
    }
    let mut prime_idx = result.len() as i64;
    let mut num: i64 = result.last().unwrap() + 2;
    while prime_idx < target_idx {
        num += 2;
        if num % 3 == 0 {
            continue;
        }
        let mut div = 5;
        let div_limit = num.isqrt();
        while div <= div_limit {
            if num % div == 0 || num % (div + 2) == 0 {
                break;
            }
            div += 6;
        }
        if div > div_limit {
            prime_idx += 1;
            result.push(num);
        }
    }
    result
}

fn get_div_count(tri_num: i64, primes: &Vec<i64>, max_prime: i64) -> i64 {
    let mut prime_idx = 1;
    let tri_num_half = ((tri_num as f64) / 2.0).ceil() as i64;
    let mut div_count = 1;
    loop {
        let prime = primes[(prime_idx - 1) as usize];
        if max_prime < tri_num_half {
            panic!(
                "Not enough primes. Current number is {}, half is {}",
                tri_num, tri_num_half
            );
        }
        if prime > tri_num_half {
            break;
        }
        let mut div_power: i64 = 1;
        let mut num = tri_num;
        if num % prime == 0 {
            while num % prime == 0 {
                num /= prime;
                if num == prime || num % prime != 0 {
                    break;
                }
                div_power += 1;
            }
            div_count *= div_power + 1;
        }
        prime_idx += 1;
    }
    if div_count == 1 {
        //  no divs => number is prime
        div_count = 2;
    }
    div_count
}

fn solution_common() -> i64 {
    let target_div_count = 501;
    let now = Instant::now();
    const PRIMES_COUNT: i64 = 3_000_000;
    let primes = collect_primes(vec![], PRIMES_COUNT);
    let max_prime = *primes.last().unwrap();
    println!(
        "Found {} primes in {:.2} sec",
        PRIMES_COUNT,
        (now.elapsed().as_millis() as f64) / 1000.0
    );
    println!("Max prime is {}", max_prime);
    let mut idx = 0;
    let mut tri_num: i64;
    let mut max_div_count = 0;
    let mut div_count;
    loop {
        idx += 1;
        tri_num = (idx * (idx + 1)) / 2;
        div_count = get_div_count(tri_num, &primes, max_prime);
        if div_count > max_div_count {
            println!(
                "Number: {}, sqrt: {}, div_count: {}",
                tri_num,
                tri_num.isqrt(),
                div_count,
            );
            max_div_count = div_count;
        }
        if div_count >= target_div_count {
            break;
        }
    }
    tri_num
}

fn main() {
    euler::run_solution(12, "common", &solution_common);
}
