fn main() {
    proconio::input! {
        n:u32
    }

    if n > 1 && n < 5 {
        print!("No");
    } else {
        print!("Yes")
    }
}
