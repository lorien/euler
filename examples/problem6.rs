use euler::check_solution;

const TARGET: i64 = 100;

fn solution_common() -> i64 {
    let mut sum_of_squares = 0;
    for num in 1..=TARGET {
        sum_of_squares += num.pow(2);
    }
    let square_of_sum = ((TARGET * (TARGET + 1)) / 2).checked_pow(2).unwrap();
    square_of_sum - sum_of_squares
}

fn solution_formula() -> i64 {
    // 1^2 + ... + n^2 = (1/6) * (2n^3 + 3n^2 + n)
    let square_of_sum = ((TARGET * (TARGET + 1)) / 2).checked_pow(2).unwrap();
    let sum_of_squares = (2 * TARGET.pow(3) + 3 * TARGET.pow(2) + TARGET) / 6;
    square_of_sum - sum_of_squares
}

fn main() {
    check_solution(6, "common", &solution_common);
    check_solution(6, "formual", &solution_formula);
}
