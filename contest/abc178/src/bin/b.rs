#[allow(unused_imports)]
use proconio::{
    input, fastout,
};
use std::cmp::max;

fn solve(a: i64, b: i64, c: i64, d: i64) -> i64 {
    let mut ans = a * c;
    ans = max(ans, a * d);
    ans = max(ans, b * c);
    ans = max(ans, b * d);

    return ans;
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        a: i64,  b: i64, c: i64, d: i64,
    }

    println!("{}", solve(a, b, c, d));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
