use euler;

const RAW_DATA: &str = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";

fn load_lines() -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    for line in RAW_DATA.lines() {
        if line.is_empty() {
            continue;
        }
        res.push(
            line.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }
    res
}

fn follow(y: i32, x: i32, mx: &Vec<Vec<i32>>, mut cur_path: i32, max_path: &mut i32) {
    cur_path += mx[y as usize][x as usize];
    if y == mx.len() as i32 - 1 {
        if cur_path > *max_path {
            *max_path = cur_path;
        }
    } else {
        follow(y + 1, x, mx, cur_path, max_path);
        follow(y + 1, x + 1, mx, cur_path, max_path);
    }
}

fn solution_common() -> i64 {
    let mut max_path = 0;
    follow(0, 0, &load_lines(), 0, &mut max_path);
    max_path.into()
}

fn main() {
    euler::run_solution(18, "common", &solution_common);
}
