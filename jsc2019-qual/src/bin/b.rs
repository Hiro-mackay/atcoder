fn calc(n: u64) -> u64 {
    return (n * (n + 1)) / 2;
}

fn main() {
    proconio::input! {
        n:usize,
        k:u64,
        a: [usize; n]
    }
    let len = a.len();
    let mut r: u64 = 0;
    let mut l: u64 = 0;
    for i in 0..len {
        for j in i..len {
            if a[i] > a[j] {
                r += calc(k) % 1_000_000_007
            }
            if a[i] < a[j] {
                l += calc(k - 1) % 1_000_000_007
            }
        }
    }
    println!("{}", (r + l) % 1_000_000_007);
}
