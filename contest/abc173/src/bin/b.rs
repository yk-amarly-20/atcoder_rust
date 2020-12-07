#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(n: usize) -> () {
    let (mut ac, mut wa, mut tle, mut re) = (0, 0, 0, 0);

    for _ in 0..n {
        input! {
            s: String
        }

        match &s[..] {
            "AC" => {
                ac += 1;
            }
            "RE" => {
                re += 1;
            }
            "TLE" => {
                tle += 1;
            }
            "WA" => {
                wa += 1;
            }
            _ => {
                panic!()
            }
        }
    }

    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
    }

    solve(n);

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
