use std::collections::HashMap;

fn main() {
    proconio::input! {
       mut s:proconio::marker::Chars
    }
    s.sort();
    println!("{}", s.iter().collect::<String>())
}
