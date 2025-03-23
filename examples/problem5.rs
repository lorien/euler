use euler::check_solution;

const TARGET: i64 = 20;

fn solution_common() -> i64 {
    let mut factors: Vec<i64> = vec![];
    for mut num in 2..=TARGET {
        for factor in &factors {
            if num % factor == 0 {
                num /= factor;
            }
        }
        if num > 1 {
            factors.push(num);
        }
    }
    let mut result = 1;
    for factor in &factors {
        result *= factor;
    }
    result
}

fn main() {
    check_solution(5, "common", &solution_common);
}
