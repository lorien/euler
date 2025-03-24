use euler::check_solution;

const TARGET: i64 = 1000;

fn solution_common() -> i64 {
    for num1 in 1..=(TARGET - 3) {
        for num2 in num1..=(TARGET - 3) {
            for num3 in num2..=(TARGET - 3) {
                if num1 + num2 + num3 == 1000 && num1.pow(2) + num2.pow(2) == num3.pow(2) {
                    return num1 * num2 * num3;
                }
            }
        }
    }
    panic!("Could not find solution");
}

fn main() {
    check_solution(9, "common", &solution_common);
}
