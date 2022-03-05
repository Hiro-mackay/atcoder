fn main() {
    proconio::input! {
        n:usize
    }

    for i in 1..10 {
        if n % i == 0 && n / i <= 9 {
            return println!("Yes");
        }
    }

    println!("No")
}
