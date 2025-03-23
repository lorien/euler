use euler::check_solution;

const TARGET: i64 = 999;

fn sum_of_seq(end: i64) -> i64 {
    // start is 1
    (end * (end + 1)) / 2
}

fn sum_divisible_by(div: i64, limit: i64) -> i64 {
    div * sum_of_seq((limit - (limit % div)) / div)
}

fn solution_formula() -> i64 {
    sum_divisible_by(3, TARGET) + sum_divisible_by(5, TARGET) - sum_divisible_by(15, TARGET)
}

fn solution_iter() -> i64 {
    let mut sum: i64 = 0;
    for num in 1..=TARGET {
        if num % 3 == 0 || num % 5 == 0 {
            sum += num;
        }
    }
    sum
}

fn main() {
    check_solution(1, "iter", &solution_iter);
    check_solution(1, "formula", &solution_formula);
}

#[test]
fn test_sum_of_seq() {
    assert_eq!(sum_of_seq(1), 1);
    assert_eq!(sum_of_seq(3), 6);
}

#[test]
fn test_sum_divisble_by() {
    assert_eq!(sum_divisible_by(2, 5), 6);
    assert_eq!(sum_divisible_by(3, 10), 18);
    assert_eq!(sum_divisible_by(5, 10), 5);
    assert_eq!(sum_divisible_by(15, 10), 0);
}
