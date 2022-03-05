fn main() {
    proconio::input! {
        a:u64
    }
    for i in 1..=a {
        if a == i * i * i {
            return println!("YES");
        }
    }
    println!("NO")
}
