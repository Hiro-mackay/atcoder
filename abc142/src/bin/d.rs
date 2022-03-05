fn euclid(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    euclid(b, a % b)
}
fn main() {
    proconio::input! {
        (a,b): (usize,usize)
    }

    let gcd = euclid(a, b);

    let mut ans = 0;

    for i in 1..gcd {
        let ss = euclid(ans, i);
        if ss == 1 {
            ans += 1
        }
    }

    println!("{}", gcd)
}
