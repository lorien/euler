use std::collections::HashMap;

type Cache = HashMap<(i64, i64), i64>;

fn count_ways(x: i64, y: i64, width: i64, height: i64, cache: &mut Cache) -> i64 {
    let mut ways = 0;
    let shape = (width - x, height - y);
    if let Some(ways_cached) = cache.get(&shape) {
        return *ways_cached;
    }
    if x < width - 1 {
        ways += count_ways(x + 1, y, width, height, cache);
    };
    if y < height - 1 {
        ways += count_ways(x, y + 1, width, height, cache);
    }
    if ways == 0 {
        ways = 1;
    }
    cache.insert(shape, ways);
    ways
}

fn solution_common() -> i64 {
    let mut cache = Cache::new();
    count_ways(0, 0, 21, 21, &mut cache)
}

fn main() {
    euler::run_solution(15, "common", &solution_common);
}
