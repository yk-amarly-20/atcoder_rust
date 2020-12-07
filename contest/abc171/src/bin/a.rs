#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(c: char) -> String {
    if c.is_uppercase() {
        "A".to_string()
    } else {
        "a".to_string()
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        c: char,
    }

    println!("{}", solve(c));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}",err),
        _ => (),
    };
}
