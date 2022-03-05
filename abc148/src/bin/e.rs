fn main() {
    proconio::input! {
        n:u64
    }
    let mut ans = 0;
    if n % 2 == 0 {
        ans = n / 10;
        let m = n / 10;

        let mut l = 5;

        while l <= m {
            ans += m / l;
            l *= 5;
        }
    }
    println!("{}", ans)
}
