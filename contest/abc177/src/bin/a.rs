#[allow(unused_imports)]
use proconio::{
    input, fastout
};

fn solve(d: f32, t: f32, s: f32) -> String {
    let ans: String;

    let time = d / s;
    if time > t {
        ans = "No".to_string();
    } else {
        ans = "Yes".to_string();
    }

    ans
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        d: f32, t: f32, s: f32,
    }

    println!("{}", solve(d, t, s));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
