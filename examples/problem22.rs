use euler;
use std::fs::File;
use std::io::Read;

const NAMES_FILE: &str = "data/22_names.txt";
const A_CODE: i32 = 'A' as i32;

fn load_names() -> Vec<String> {
    let mut file = File::open(NAMES_FILE).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
        .split(",")
        .map(|x| x[1..x.len() - 1].to_string())
        .collect::<Vec<_>>()
}

fn get_score(pos: i32, name: &String) -> i32 {
    let mut result = 0;
    for chr in name.chars() {
        result += (chr as i32 - A_CODE) + 1;
    }
    pos * result
}

fn solution_common() -> i64 {
    let mut names = load_names();
    let mut score_sum = 0;
    names.sort();
    for (pos, name) in names.iter().enumerate() {
        score_sum += get_score(pos as i32 + 1, name)
    }
    score_sum.into()
}

fn main() {
    euler::run_solution(22, "common", &solution_common);
}
