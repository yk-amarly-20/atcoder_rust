#[allow(unused_imports)]
use proconio::{
    input, fastout
};
use std::collections::HashSet;

fn solve(n: usize, l: Vec<i64>) -> i32 {
    let mut ans: i32 = 0;
    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            for k in (j + 1)..n {
                if judge_triangle(l[i], l[j], l[k]) {
                    ans += 1;
                }
            }
        }
    }

    ans
}

fn judge_triangle(a: i64, b: i64, c: i64) -> bool {

    let hash: HashSet<i64> = vec![a, b, c].into_iter().collect();
    if hash.len() == 3 {
        if c < a + b {
            return true;
        }
    }

    return false;
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n:usize, mut l: [i64; n],
    }
    l.sort();
    println!("{}", solve(n, l));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
