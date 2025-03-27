use euler;
//use std::collections::HashMap;

//fn follow_chain(mut num: i64, cache: &mut HashMap<i64, i64>) -> i64 {
//    if let Some(len) = cache.get(&num) {
//        return *len;
//    }
//    if num == 1 {
//        return 1;
//    }
//    let result = if num % 2 == 0 {
//        1 + follow_chain(num / 2, cache)
//    } else {
//        1 + follow_chain(3 * num + 1, cache)
//    };
//    cache.insert(num, result);
//    result
//}

fn follow_chain(mut num: i64) -> i64 {
    let mut len = 1;
    while num != 1 {
        len += 1;
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
    }
    len
}

fn solution_common() -> i64 {
    let mut result_num = 0;
    let mut longest_chain = 0;
    //let mut cache: HashMap<i64, i64> = HashMap::new();
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
