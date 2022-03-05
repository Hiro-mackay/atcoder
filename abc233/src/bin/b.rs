fn main() {
    proconio::input! {
        l:usize,
        r:usize,
        s: String
    }

    if l != 1 {
        print!("{}", &s[..l - 1])
    }

    print!("{}", &s[l - 1..r].chars().rev().collect::<String>());

    if r != s.len() + 1 {
        print!("{}", &s[r..s.len()])
    }
}
