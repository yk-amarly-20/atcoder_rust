#[allow(unused_imports)]
use proconio::{
    input, fastout,
};

fn solve(n: usize, p: Vec<Vec<f64>>) -> String {

    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            for k in (j + 1)..n {
                if check_straight(&p[i], &p[j], &p[k]) {
                    return "Yes".to_string();
                }
            }
        }
    }

    "No".to_string()
}

fn check_straight(p1: &Vec<f64>, p2: &Vec<f64>, p3: &Vec<f64>) -> bool {

    let vec_x: Vec<f64> = vec![p1[0] - p2[0], p1[1] - p2[1]];
    let vec_y: Vec<f64> = vec![p2[0] - p3[0], p2[1] - p3[1]];

    if vec_x[0] / vec_y[0] == vec_x[1] / vec_y[1] {
        return true;
    } else {
        return false;
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize, p: [[f64; 2]; n],
    }

    println!("{}", solve(n, p));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
