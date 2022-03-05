fn main() {
    proconio::input! {
        a:i32,
        b:i32
    }

    if b - a > 0 {
        if (b - a) % 10 == 0 {
            return println!("{}", (b - a) / 10);
        } else {
            return println!("{}", ((b - a) / 10) + 1);
        }
    }

    println!("{}", 0)
}
