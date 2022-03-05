fn calc(n: i32) -> i32 {
    return n * n + 2 * n + 3;
}

fn main() {
    proconio::input! {
        n: i32,
    }

    println!("{:?}", calc(calc(calc(n) + n) + calc(calc(n))));
}