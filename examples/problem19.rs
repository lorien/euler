fn get_month_days(year: i32, month: i32) -> i32 {
    if month == 2 {
        if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
            return 29;
        } else {
            return 28;
        }
    }
    if month == 4 || month == 6 || month == 9 || month == 11 {
        return 30;
    }
    return 31;
}

fn solution_common() -> i64 {
    let mut count = 0;
    let mut year = 1900;
    let mut month = 1;
    let mut day = 1;
    let mut week_day = 1;
    while year < 2001 {
        let month_days = get_month_days(year, month);
        if month < 12 {
            month += 1;
        } else {
            year += 1;
            month = 1;
        }
        week_day = (week_day + month_days) % 7;
        if year > 1900 && week_day == 0 {
            count += 1;
        }
        //println!(
        //    "+{} -> {}.{}.{} -> {}",
        //    month_days, day, month, year, week_day
        //);
    }
    count
}

fn main() {
    euler::run_solution(19, "common", &solution_common);
}
