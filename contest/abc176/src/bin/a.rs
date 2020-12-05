#[allow(unused_imports)]
use proconio::{
    input, fastout
};

fn solve(n: i32, x: i32, t: i32) -> i32 {

    let ans: i32;
    if n % x == 0 {
        ans = t * (n / x);
    } else {
        ans = t * (n / x + 1);
    }

    ans
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: i32, x: i32, t: i32
    }

    println!("{}", solve(n, x, t));

    Ok(())
}


fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
