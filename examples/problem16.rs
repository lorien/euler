use euler;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

fn solution_bigint() -> i64 {
    let mut res = BigUint::from(2_u32).pow(1000);
    //println!("{}", res);
    let mut sum = BigUint::from(0_u32);
    let ten = BigUint::from(10_u32);
    while res >= ten {
        sum += &res % &ten;
        res /= &ten;
    }
    sum += res;
    sum.to_i64().unwrap()
}

fn main() {
    euler::run_solution(16, "bigint", &solution_bigint);
}
