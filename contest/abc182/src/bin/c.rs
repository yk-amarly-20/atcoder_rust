#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(n: i64) -> isize {
    let n_str = &n.to_string();
    let mut table = vec![0; 3];
    let mut sum: usize = 0;
    let mut keta = 0;

    for c in n_str.chars() {
        let alpha = c as usize - 48;
        sum += alpha;
        table[(alpha % 3) as usize] += 1;
        keta += 1;
    }

    let r = sum % 3;
    let mut ans = if r == 0 {
        0
    } else if r == 1 {
        if table[1] >= 1 {
            1
        } else if table[2] >= 2 {
            2
        } else {
            -1
        }
    } else {
        if table[2] >= 1 {
            1
        } else if table[1] >= 2 {
            2
        } else {
            -1
        }
    };

    if ans >= keta {
        ans = -1;
    }

    ans
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        n: i64,
    }

    println!("{}", solve(n));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
