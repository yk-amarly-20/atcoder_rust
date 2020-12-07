#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(a: i64, b: i64, c: i64, d: i64) -> usize {
    if a == c && b == d {
        0
    } else if a + b == c + d
        ||  a - b == c - d
        || (a - c).abs() + (b - d).abs() <= 3
    {
        1
    } else if (a + b + c + d) % 2 == 0
        || (a - c).abs() + (b - d).abs() <= 6
        || (a + b - c - d).abs() <= 3
        || (a - b - c + d).abs() <= 3
    {
        2
    } else {
        3
    }

}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        r1: (i64, i64), r2: (i64, i64),
    }

    println!("{}", solve(r1.0, r1.1, r2.0, r2.1));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    }
}
