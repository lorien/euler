use euler::check_solution;

const RAW_DATA: &str = "
08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
";

fn read_matrix() -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = vec![];
    for raw_line in RAW_DATA.lines() {
        if raw_line.is_empty() {
            continue;
        }
        let mut line: Vec<i64> = vec![];
        for raw_num in raw_line.split(" ") {
            line.push(
                raw_num
                    .parse()
                    .expect(&format!("Fail to parse [{}]", raw_num)),
            );
        }
        result.push(line);
    }
    result
}

fn solution_common() -> i64 {
    let mx = read_matrix();
    let window_len = 4;
    let num_rows = mx.len();
    let num_cols = mx[0].len();
    let mut max_product = 0;
    let mut product;
    for y in 0..num_rows {
        for x in 0..num_cols {
            if x + window_len < num_cols {
                product = mx[y][x] * mx[y][x + 1] * mx[y][x + 2] * mx[y][x + 3];
                if product > max_product {
                    max_product = product;
                }
            }
            if y + window_len < num_rows {
                product = mx[y][x] * mx[y + 1][x] * mx[y + 2][x] * mx[y + 3][x];
                if product > max_product {
                    max_product = product;
                }
            }
            if y + window_len < num_rows && x + window_len < num_cols {
                product = mx[y][x] * mx[y + 1][x + 1] * mx[y + 2][x + 2] * mx[y + 3][x + 3];
                if product > max_product {
                    max_product = product;
                }
            }
            if y >= window_len && x + window_len < num_cols {
                product = mx[y][x] * mx[y - 1][x + 1] * mx[y - 2][x + 2] * mx[y - 3][x + 3];
                if product > max_product {
                    max_product = product;
                }
            }
        }
    }
    max_product
}

#[derive(Copy, Clone)]
enum Direction {
    Bottom,
    Right,
    BottomRight,
    TopRight,
}

const ALL_DIRECTIONS: &[Direction] = &[
    Direction::Bottom,
    Direction::Right,
    Direction::BottomRight,
    Direction::TopRight,
];

fn get_deltas(dir: Direction) -> (i64, i64) {
    match dir {
        Direction::Bottom => (1, 0),
        Direction::Right => (0, 1),
        Direction::BottomRight => (1, 1),
        Direction::TopRight => (-1, 1),
    }
}

fn process_direction(
    mx: &Vec<Vec<i64>>,
    y: i64,
    x: i64,
    num_rows: i64,
    num_cols: i64,
    window_len: i64,
    dir: Direction,
    max_product: i64,
) -> Option<i64> {
    let mut result = None;
    let (y_delta, x_delta) = get_deltas(dir);
    let mut max_product = max_product;
    if (y_delta == 0
        || (y_delta > 0 && y + window_len < num_rows)
        || (y_delta < 0 && y >= window_len))
        && (x_delta == 0
            || (x_delta > 0 && x + window_len < num_cols)
            || (x_delta < 0 && x >= window_len))
    {
        let mut product = 1;
        for step in 0..window_len {
            product *= mx[(y + y_delta * step) as usize][(x + x_delta * step) as usize];
        }
        if product > max_product {
            max_product = product;
            result = Some(product);
        }
    }
    result
}

fn solution_general() -> i64 {
    let mx = read_matrix();
    let window_len = 4;
    let num_rows = mx.len() as i64;
    let num_cols = mx[0].len() as i64;
    let mut max_product = 0;
    for y in 0..num_rows {
        for x in 0..num_cols {
            for &dir in ALL_DIRECTIONS.iter() {
                if let Some(prod) = process_direction(
                    &mx,
                    y as i64,
                    x as i64,
                    num_rows,
                    num_cols,
                    window_len,
                    dir,
                    max_product,
                ) {
                    max_product = prod;
                }
            }
        }
    }
    max_product
}

fn main() {
    check_solution(11, "common", &solution_common);
    check_solution(11, "general", &solution_general);
}
