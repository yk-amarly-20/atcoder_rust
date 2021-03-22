use proconio::input;

fn main() {
    input! {
        x: u32, n: usize,
        p: [u32; n]
    }

    let mut ans = 0;
    for i in 0..100 {
        if !p.contains(&(x - i)) {
            ans = x - i;
            break;
        }
        if !p.contains(&(x + i)) {
            ans = x + i;
            break;
        }
    }

    println!("{}", ans);
}
