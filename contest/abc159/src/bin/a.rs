use proconio::*;

fn main() {
    input! {
        n: usize, 
        m: usize, 
    };
    
    
    let ans = combination(n) + combination(m);
    println!("{}", ans);
}

fn combination(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else {
        return n * (n - 1)  / 2;
    }
}
