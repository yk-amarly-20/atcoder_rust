#[allow(unused_imports)]
use proconio::{
    input, fastout
};

fn solve(x: i32) -> i32 {
    if x == 0 {
        1
    } else {
        0
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        x:i32,
    }

    println!("{}", solve(x));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
