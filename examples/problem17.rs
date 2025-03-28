use euler;

const BASIC_NUMBERS: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: &[&str] = &[
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

fn get_ten_spell(idx: i32) -> String {
    return TENS[idx as usize].to_string();
}

fn get_hundred_spell(idx: i32) -> String {
    return format!("{} hundred", spell(idx));
}

fn spell(num: i32) -> String {
    assert!(num > 0 && num <= 1000);
    if num < 20 {
        return BASIC_NUMBERS[num as usize].to_string();
    }
    if num < 100 {
        if (num % 10 == 0) {
            return get_ten_spell(num / 10);
        } else {
            return format!("{}-{}", get_ten_spell(num / 10), spell(num % 10));
        }
    }
    if num < 1000 {
        if num % 100 == 0 {
            return get_hundred_spell(num / 100);
        } else {
            return format!("{} and {}", get_hundred_spell(num / 100), spell(num % 100));
        }
    }
    return "one thousand".to_string();
}

fn solution_common() -> i64 {
    let mut count = 0;
    for num in 1..=1000 {
        let word = spell(num);
        println!("{}, {}, {}", num, word, word.len());
        count += word
            .chars()
            .map(|x| if x == ' ' || x == '-' { 0 } else { 1 })
            .sum::<i32>();
    }
    println!("Result: {}", count);
    count.into()
}

fn main() {
    euler::run_solution(17, "common", &solution_common);
}
