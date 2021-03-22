use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32
    }

    if (y % 2) == 0 && (y - 2 * x) >= 0 && (4 * x - y) >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
