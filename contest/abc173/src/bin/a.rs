#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(n: usize) -> usize {
    let ans: usize;

    if n % 1000 == 0 {
        ans = 0;
    } else {
        ans = 1000 - (n % 1000);
    }

    ans
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
    }

    println!("{}", solve(n));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
