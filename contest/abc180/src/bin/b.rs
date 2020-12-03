#[allow(unused_imports)]
use proconio::{
    input, fastout
};
use std::cmp::max;

#[allow(non_snake_case)]
fn ManhattanDistance(n: usize, x: Vec<i64>) -> i64 {

    let mut dist: i64 = 0;
    for i in 0..n {
        dist += x[i].abs();
    }

    dist
}

#[allow(non_snake_case)]
fn EuclidDistance(n: usize, x: Vec<i64>) -> f64 {

    let mut dist: i64 = 0;
    for i in 0..n {
        dist += x[i] * x[i];
    }

    (dist as f64).sqrt()
}

#[allow(non_snake_case)]
fn ChebyshevDistance(n: usize, x: Vec<i64>) -> i64 {

    let mut dist: i64 = 0;
    for i in 0..n {
        dist = max(dist, x[i].abs());
    }

    dist
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    input! {
        n: usize,
        x: [i64; n],
    }

    println!("{}", ManhattanDistance(n, x.clone()));
    println!("{}", EuclidDistance(n, x.clone()));
    println!("{}", ChebyshevDistance(n, x.clone()));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
