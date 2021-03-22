use proconio::*;

fn main() {
    input! {
        x: f64, 
    }

    let n = (1.0_f64 + ((x.log10() - 2.0_f64) / 1.01_f64.log10())).floor() as usize;

    println!("{}", n - 1);
}
