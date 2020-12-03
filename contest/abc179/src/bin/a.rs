#[allow(unused_imports)]
use proconio::{
    input, fastout
};

fn solve(s: String) -> String {

    #[allow(unused_assignments)]
    let mut ans: String = "".to_string();
    if s.ends_with("s") {
        ans = s + "es";
    } else {
        ans = s + "s";
    }

    ans
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        s: String,
    }

    println!("{}", solve(s));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
