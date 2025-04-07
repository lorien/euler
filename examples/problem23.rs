use euler;
use std::io;
use std::io::Write;

fn print(num: i32) {
    print!("{} ", num);
    io::stdout().flush().unwrap();
}

fn get_div_sum(num: i32) -> i32 {
    let mut div_sum = 1;
    //print(1);
    let num_sqrt = num.isqrt();
    let mut div = 2;
    while div <= num_sqrt {
        if num % div == 0 {
            div_sum += div;
            //print(div);
            if div * div != num {
                //print(num / div);
                div_sum += num / div;
            }
        }
        div += 1;
    }
    div_sum
}

fn solution_common() -> i64 {
    let mut abundants = vec![];
    let mut num = 1;
    let limit = 28123;
    while num <= limit {
        if get_div_sum(num) > num {
            //println!("Abundunt: {}", num);
            abundants.push(num);
        }
        num += 1;
    }
    let mut non_sum = vec![];
    for idx in 0..limit + 1 {
        non_sum.push(0);
    }
    non_sum[0] = 0;
    for idx1 in 0..abundants.len() {
        for idx2 in idx1..abundants.len() {
            let num1 = abundants[idx1];
            let num2 = abundants[idx2];
            if num1 + num2 <= limit {
                non_sum[(num1 + num2) as usize] = 1;
            }
        }
    }
    let mut res = 0;
    for (idx, item) in non_sum.iter().enumerate() {
        if non_sum[idx] == 0 {
            //print!("{}, ", idx);
            res += idx;
        }
    }
    //println!("");
    res as i64
}

fn main() {
    euler::run_solution(23, "common", &solution_common);
}
