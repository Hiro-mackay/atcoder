fn main() {
    proconio::input! {
        n:usize,
        k:i64,
        a:[i64;n]
    }
    let mut b = vec![0i64; n];
    let mut ans = 0;
    for i in 0..n {
        for j in 0..=i {
            b[j] += a[i];
            if b[j] == k {
                ans += 1
            }
        }
    }
    print!("{}", ans)
}
