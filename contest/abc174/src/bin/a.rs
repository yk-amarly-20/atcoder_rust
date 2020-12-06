#[allow(unused_imports)]
use proconio::{
    input, fastout
};

fn solve(x: i32) -> String {

    if x >= 30 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        x: i32,
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
