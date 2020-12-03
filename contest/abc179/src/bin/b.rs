#[allow(unused_imports)]
use proconio::{
    input, fastout
};

fn solve(n: usize, dice: Vec<Vec<i32>>) -> String {

    for i in 0..(n - 2) {
        if (dice[i][0] == dice[i][1]) &&
        (dice[i + 1][0] == dice[i + 1][1]) && (dice[i + 2][0] == dice[i + 2][1]) {
            return "Yes".to_string();
        }
    }

    return "No".to_string();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        n: usize,
        dice: [[i32; 2]; n],
    }

    println!("{}", solve(n, dice));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
