fn main() {
    proconio::input! {
        n:proconio::marker::Chars,
    }

    println!("{}", (n[0] as i32 - 48) * (n[2] as i32 - 48));
} 
