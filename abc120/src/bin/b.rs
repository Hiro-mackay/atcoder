fn main() {
    proconio::input! {
        a:usize,
        b:usize,
        k:usize
    }

    let min = std::cmp::min(a, b);
    let mut c = 0;
    let mut ans = 0;

    for i in (0..=min).rev() {
        if a % i == 0 && b % i == 0 {
            c += 1
        }

        if c == k {
            ans = i;
            break;
        }
    }
    println!("{}", ans)
}
