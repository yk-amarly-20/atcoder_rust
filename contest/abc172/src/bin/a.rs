#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(a: usize) -> usize {
    a + a.pow(2) + a.pow(3)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        a: usize,
    }

    println!("{}", solve(a));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
