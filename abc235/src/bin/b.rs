fn main() {
    proconio::input! {
        n:usize,
        h:[u32; n]
    }
    for i in 1..n {
        if h[i] <= h[i - 1] {
            return print!("{}", h[i - 1]);
        }
    }
    print!("{}", h[n - 1])
}
