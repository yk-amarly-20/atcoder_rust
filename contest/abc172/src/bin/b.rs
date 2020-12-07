#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(s: String, t: String) -> usize {

    let mut ans: usize = 0;
    let s_vec = s.as_str().chars();
    let t_vec: Vec<char> = t.as_str().chars().collect();

    for (i, c) in s_vec.enumerate() {
        if c != t_vec[i] {
            ans += 1
        }
    }

    ans
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        s: String, t: String,
    }

    println!("{}", solve(s, t));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
