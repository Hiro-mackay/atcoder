use num::Integer;

fn main() {
    proconio::input! {
        (a,b): (usize,usize)
    }

    let mut g = a.gcd(&b);

    let mut ans = 1;
    let mut i = 2;
    while i * i <= g {
        if g % i == 0 {
            ans += 1;
            while g % i == 0 {
                g /= i;
            }
        }
        i += 1
    }
    if g > 1 {
        ans += 1
    }
    println!("{}", ans)
}
