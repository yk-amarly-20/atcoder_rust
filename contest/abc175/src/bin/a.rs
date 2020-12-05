#[allow(unused_imports)]
use proconio::{
    input, fastout
};

fn solve(s: &str) -> usize {

    if s == "RRR" {
        3
    } else if s == "SRR" || s == "RRS" {
        2
    } else if s == "SSS"{
        0
    } else {
        1
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        s: String,
    }
    println!("{}", solve(&s));
    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
