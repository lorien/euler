use euler;
use std::cmp::max;

fn make_number(mut inp: i32) -> Vec<i32> {
    let mut result = vec![];
    while inp > 0 {
        result.push(inp % 10);
        inp /= 10;
    }
    result
}

fn add(num1: &Vec<i32>, num2: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut buffer = 0;
    for pos in 0..max(num1.len(), num2.len()) {
        buffer += num1.get(pos).unwrap_or(&0) + num2.get(pos).unwrap_or(&0);
        result.push(buffer % 10);
        buffer /= 10;
    }
    if buffer > 0 {
        result.push(buffer);
    }
    result
}

fn mult_scalar(num: &Vec<i32>, factor: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut buffer = 0;
    for digit in num.iter() {
        buffer += *digit * factor;
        result.push(buffer % 10);
        buffer /= 10;
    }
    while buffer > 0 {
        result.push(buffer % 10);
        buffer /= 10;
    }
    result
}

fn mult(num1: &Vec<i32>, num2: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![0];
    for (pos, dig2) in num2.iter().enumerate() {
        result = add(
            &result,
            &mult_scalar(&num1, dig2 * 10_i32.pow(pos.try_into().unwrap())),
        );
    }
    result
}

fn solution_common() -> i64 {
    let mut num = make_number(100);
    for factor in 2..=99 {
        num = mult_scalar(&num, factor);
    }
    num.iter().sum::<i32>().into()
}

fn main() {
    euler::run_solution(20, "common", &solution_common);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_number() {
        assert_eq!(make_number(1), vec![1]);
        assert_eq!(make_number(100), vec![0, 0, 1]);
        assert_eq!(make_number(990), vec![0, 9, 9]);
    }

    #[test]
    fn test_add() {
        assert_eq!(vec![1], add(&vec![0], &vec![1]));
        assert_eq!(vec![8, 9, 0, 1], add(&vec![9, 9], &vec![9, 9, 9]));
        assert_eq!(vec![9, 9, 0, 1], add(&vec![0, 0, 1], &vec![9, 9, 9]));
    }

    #[test]
    fn test_mult_scalar() {
        assert_eq!(vec![1], mult_scalar(&vec![1], 1));
        assert_eq!(vec![6, 7, 1], mult_scalar(&vec![2], 88));
        assert_eq!(vec![3, 2, 6, 7], mult_scalar(&vec![9, 9], 77));
    }

    #[test]
    fn test_mult() {
        assert_eq!(vec![1], mult(&vec![1], &vec![1]));
        assert_eq!(vec![5, 2], mult(&vec![5], &vec![5]));
        assert_eq!(vec![0, 0, 9, 0, 9], mult(&vec![100], &vec![909]));
    }
}
