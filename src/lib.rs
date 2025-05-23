use std::collections::BTreeMap;
use std::time::Instant;

const SOLUTIONS_FILE: &str = "data/solutions.yml";
const BENCHMARK_RUN_MILLIS: i32 = 1000;
const COUNT_SUFFIXES: &[&str] = &["", "K", "M", "B"];
const COUNT_STEP_FACTOR: i32 = 1000;
type ErrorMessage = String;

fn read_solution(problem_num: i64) -> Result<i64, ErrorMessage> {
    let data: BTreeMap<String, i64> = serde_yaml::from_reader(
        std::fs::File::open(SOLUTIONS_FILE)
            .map_err(|_e| format!("Could not open solutions file {}", SOLUTIONS_FILE))?,
    )
    .map_err(|_e| format!("Could not parse solutions file {}", SOLUTIONS_FILE))?;
    let key = format!("problem{}", problem_num);
    if let Some(val) = data.get(&key) {
        Ok(*val)
    } else {
        Err(format!("No solution found for problem #{}", problem_num))
    }
}

fn render_count(count: i32) -> String {
    let mut suffix_idx = 0;
    let suffixes_len = COUNT_SUFFIXES.len();
    let mut count: f64 = count as f64;
    while count > COUNT_STEP_FACTOR.into() && suffix_idx < suffixes_len - 1 {
        count /= COUNT_STEP_FACTOR as f64;
        suffix_idx += 1;
    }
    format!("{:.1}{}", count, COUNT_SUFFIXES[suffix_idx])
}

fn match_valid_solution(problem_num: i64, result: i64) {
    let valid_solution_opt = read_solution(problem_num);
    match valid_solution_opt {
        Err(err) => {
            println!("Error happened: {}", err);
            println!("Result: [ {} ]. Valid solution is not defined yet", result);
        }
        Ok(valid_solution) => {
            if result != valid_solution {
                println!(
                    "Error! Result: {}, valid solution: {}",
                    result, valid_solution
                );
            } else {
                println!("Valid!");
            }
        }
    }
}

pub fn check_solution(problem_num: i64, msg: &str, func: &dyn Fn() -> i64) {
    let now = Instant::now();
    let mut num_iterations = 0;
    let result = loop {
        let result = func();
        num_iterations += 1;
        if now.elapsed().as_millis() as i32 > BENCHMARK_RUN_MILLIS {
            break result;
        }
    };
    let elapsed = now.elapsed().as_millis();

    println!(
        "{}: {} iters in {:.3} sec",
        msg,
        render_count(num_iterations),
        elapsed as f64 / 1000.0
    );
    match_valid_solution(problem_num, result);
}

pub fn run_solution(problem_num: i64, msg: &str, func: &dyn Fn() -> i64) {
    let now = Instant::now();
    let result = func();
    println!(
        "Solution {} took {:.4} sec. Result: {}",
        msg,
        (now.elapsed().as_millis() as f64) / 1000.0,
        result
    );
    match_valid_solution(problem_num, result);
}
