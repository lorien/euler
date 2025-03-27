use euler;

fn follow_chain(mut num: i64) -> i64 {
    let mut len = 0;
    while num != 1 {
        len += 1;
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
    }
    len + 1
}

fn solution_common() -> i64 {
    let mut result_num = 0;
    let mut longest_chain = 0;
    for num in 1..=1000000 {
        let chain_len = follow_chain(num);
        if chain_len > longest_chain {
            longest_chain = chain_len;
            result_num = num;
        }
    }
    println!("Solution: {} -> {}", result_num, longest_chain);
    result_num
}

fn main() {
    euler::run_solution(14, "common", &solution_common);
}
