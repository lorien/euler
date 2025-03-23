use euler::check_solution;

const TARGET: i32 = 4_000_000;

fn solution_iter() -> i32 {
    let mut first = 1;
    let mut second = 1;
    let mut sum_of_even = 0;
    while second <= TARGET {
        (first, second) = (second, first + second);
        if second <= TARGET {
            if second % 2 == 0 {
                sum_of_even += second;
            }
        }
    }
    sum_of_even
}

fn solution_each_third() -> i32 {
    let mut first = 1;
    let mut second = 1;
    let mut third = first + second;
    let mut sum_of_even = third;
    // shift window of size 3
    while third <= TARGET {
        first = second + third;
        second = third + first;
        third = first + second;
        if third <= TARGET {
            sum_of_even += third;
        }
    }
    sum_of_even
}

fn solution_formula() -> i32 {
    let mut first = 2;
    let mut second = 8;
    let mut sum_of_even = first;
    while second <= TARGET {
        sum_of_even += second;
        (first, second) = (second, 4 * second + first);
    }
    sum_of_even
}

fn main() {
    check_solution(2, "iter", &solution_iter);
    check_solution(2, "each-3rd", &solution_each_third);
    check_solution(2, "formula", &solution_formula);
}
