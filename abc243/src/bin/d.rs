use libm::pow;

fn main() {
    proconio::input! {
        (a,b): (u128, u128),
        s: proconio::marker::Chars
    }

    let mut ans = b;

    for _s in s {
        if _s == 'U' {
            ans /= 2;
        } else if _s == 'L' {
            ans *= 2
        } else {
            ans = ans * 2 + 1
        }
    }

    println!("{}", ans)
}
