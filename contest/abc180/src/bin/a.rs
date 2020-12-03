use proconio::{
    input, fastout
};

fn solve(n: i32, a: i32, b: i32) -> i32 {

    n - a + b
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        n: i32, a: i32, b: i32,
    }

    println!("{}", solve(n, a, b));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => ()
    };
}
