#[allow(unused_imports)]
use proconio::{
    input,
    fastout
};

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;

    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }

        ans += (a + b) * (b - a + 1) / 2;
    }

    println!("{}", ans);
}
