use proconio::*;

fn main() {
    input! {
        k: usize, 
        a: usize, 
        b: usize, 
    };

    let ret = b / k * k;    // b以下の最大のkの倍数
    
    if a <= ret {
        println!("OK");
    } else {
        println!("NG");
    }
}
