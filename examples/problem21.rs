use euler;
use std::collections::HashSet;

fn get_div_sum(num: i32) -> i32 {
    let num_sqrt = (num as f32).sqrt().floor() as i32;
    let mut div_sum = if num > 1 { 1 } else { 0 };
    let mut div = 2;
    while div <= num_sqrt {
        if num % div == 0 {
            let div_res = num / div;
            div_sum += div;
            if div_res != div {
                div_sum += div_res;
            }
        }
        div += 1;
    }
    div_sum
}

fn solution_common() -> i64 {
    let target = 10000;
    let mut numbers = HashSet::new();
    for num in 1..target {
        let div_sum = get_div_sum(num);
        if num != div_sum {
            let div_sum2 = get_div_sum(div_sum);
            if num == div_sum2 {
                println!("{}, {}", num, div_sum);
                numbers.insert(num);
                numbers.insert(div_sum);
            }
        }
    }
    println!("{:?}", numbers);
    numbers.iter().sum::<i32>().into()
}

fn main() {
    euler::run_solution(21, "common", &solution_common);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_div_sum() {
        assert_eq!(284, get_div_sum(220));
        assert_eq!(220, get_div_sum(284));
    }
}
