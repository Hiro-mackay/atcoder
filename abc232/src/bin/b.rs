fn main() {
    proconio::input! {
        s:proconio::marker::Chars,
        t:proconio::marker::Chars
    }

    let mut diff = t[0] as i32 - s[0] as i32;

    if diff < 0 {
        diff += 26
    }

    for i in 1..s.len() {
        let mut ans = t[i] as i32 - s[i] as i32;
        if ans < 0 {
            ans += 26
        }
        if ans != diff {
            return println!("No");
        }
    }
    println!("Yes")
}
