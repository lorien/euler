use euler::check_solution;

const TARGET_HIGH: i64 = 999;
const TARGET_LOW: i64 = 100;

fn get_digits(num: i64) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];
    let mut num = num;
    while num > 10 {
        digits.push((num % 10).try_into().unwrap());
        num /= 10;
    }
    digits.push(num.try_into().unwrap());
    digits
}

fn is_palindrom(digits: Vec<u8>) -> bool {
    let len = digits.len();
    for idx in 0..=len / 2 {
        if digits[idx] != digits[len - 1 - idx] {
            return false;
        }
    }
    true
}

fn solution_common() -> i64 {
    let mut result = 0;
    for factor1 in (TARGET_LOW..=TARGET_HIGH).rev() {
        for factor2 in (TARGET_LOW..=factor1).rev() {
            let product = factor1 * factor2;
            let digits = get_digits(product);
            if is_palindrom(digits) {
                if product > result {
                    result = product;
                }
            }
        }
    }
    result
}

fn solution_speedup() -> i64 {
    let mut result = 0;
    for factor1 in (TARGET_LOW..=TARGET_HIGH).rev() {
        for factor2 in (TARGET_LOW..=factor1).rev() {
            let product = factor1 * factor2;
            if product < result {
                break;
            }
            let digits = get_digits(product);
            if is_palindrom(digits) {
                if product > result {
                    result = product;
                }
            }
        }
    }
    result
}

fn solution_speedup_eleven() -> i64 {
    let mut result = 0;
    let mut factor1 = TARGET_HIGH;
    while factor1 >= TARGET_LOW {
        let mut factor2 = factor1;
        let mut factor2_delta = 1;
        if factor1 % 11 != 0 {
            factor2 = factor1 - factor1 % 11;
            factor2_delta = 11;
        }
        while factor2 >= TARGET_LOW {
            let product = factor1 * factor2;
            if product < result {
                break;
            }
            let digits = get_digits(product);
            if is_palindrom(digits) {
                if product > result {
                    result = product;
                }
            }
            factor2 -= factor2_delta;
        }
        factor1 -= 1;
    }
    result
}

fn main() {
    check_solution(4, "common", &solution_common);
    check_solution(4, "speedup", &solution_speedup);
    check_solution(4, "boost", &solution_speedup_eleven);
}
