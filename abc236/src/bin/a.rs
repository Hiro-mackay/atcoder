fn main() {
    proconio::input! {
       mut s: proconio::marker::Chars,
        n: usize,
        m: usize,
    }
    let t = s[n - 1];
    s[n - 1] = s[m - 1];
    s[m - 1] = t;
    print!("{}", s.iter().collect::<String>());
}
