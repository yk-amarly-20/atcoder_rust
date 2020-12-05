#[allow(unused_imports)]
use proconio::{
    input, fastout
};

fn solve(s: &str) -> String {
    let mut res: usize = 0;
    let s_v = s.chars();
    for c in s_v {
        res = (res + c as usize - b'0' as usize) % 9;
    }

    if res == 0 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        s: String
    }
    println!("{}", solve(&s));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => ()
    };
}
