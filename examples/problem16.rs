use euler;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

fn solution_bigint() -> i64 {
    let mut res = BigUint::from(2_u32).pow(1000);
    //println!("{}", res);
    let mut sum = BigUint::from(0_u32);
    while res > BigUint::from(0_u32) {
        sum += &res % 10_u32;
        res /= 10_u32;
    }
    sum.to_i64().unwrap()
}

fn solution_column() -> i64 {
    let mut digits = vec![2];
    for power in 2..=1000 {
        let mut buffer = 0;
        for idx in (0..digits.len()).rev() {
            let sum = digits[idx] * 2 + buffer;
            digits[idx] = sum % 10;
            buffer = sum / 10;
        }
        while buffer > 0 {
            digits.insert(0, buffer % 10);
            buffer /= 10;
        }
    }
    digits.iter().sum()
}

fn main() {
    euler::run_solution(16, "bigint", &solution_bigint);
    euler::run_solution(16, "column", &solution_column);
}
