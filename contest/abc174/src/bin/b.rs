#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(d: i64, x: Vec<Vec<i64>>) -> i32 {
    let mut ans = 0;
    for p in x.iter() {
        let dist = &p[0] * &p[0] + &p[1] * &p[1];
        if d * d >= dist {
            ans += 1;
        }
    }

    ans
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize, d: i64, x: [[i64; 2]; n],
    }

    println!("{}", solve(d, x));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
