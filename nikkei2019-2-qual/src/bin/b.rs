use num::pow;

fn main() {
    proconio::input! {
        n:usize,
        d: [usize; n]
    }
    let md = 998244353;

    let mut g = vec![0; d.iter().max().unwrap() + 1];

    if d[0] != 0 {
        return println!("0");
    }

    for i in d {
        g[i] += 1
    }

    if g[1] == 0 {
        return println!("0");
    }

    let mut ans = 1;
    for i in 1..g.len() {
        ans *= pow(g[i - 1], g[i]);
        ans %= md;
    }

    println!("{}", ans)
}
