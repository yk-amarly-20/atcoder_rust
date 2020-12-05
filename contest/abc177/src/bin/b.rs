#[allow(unused_imports)]
use proconio::{
    input, fastout
};
use std::cmp::min;

fn solve(s: &str, t: &str) -> usize {
    let s_len = s.len();
    let t_len = t.len();
    let mut ans = t_len;

    let s_v = s.chars().collect::<Vec<char>>();
    let t_v = t.chars().collect::<Vec<char>>();

    for i in 0..(s_len - t_len + 1) {
        let mut count = 0;
        for j in 0..t_len {
            if s_v[i + j] != t_v[j] {
                count += 1;
            }
        }

        ans = min(ans, count);
    }

    ans
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        s: String,
        t: String,
    }

    println!("{}", solve(&s, &t));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
