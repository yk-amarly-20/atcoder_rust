#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(k: usize, mut p: Vec<usize>) -> usize {

    let mut ans: usize = 0;
    p.sort();

    for i in 0..k {
        ans += p[i];
    }

    ans
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        n: usize, k: usize, p: [usize; n],
    }

    println!("{}", solve(k, p));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
