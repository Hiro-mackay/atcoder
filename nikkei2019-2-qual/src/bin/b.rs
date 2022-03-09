use num::pow;

fn main() {
    proconio::input! {
        n:usize,
        d: [usize; n]
    }
    let md = 998244353;

    let mut g = vec![0; 100];

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
    let t = g.into_iter().filter(|&x| x != 0).collect::<Vec<usize>>();
    for i in 1..t.len() {
        ans *= pow(t[i - 1], t[i]);
        ans %= md;
    }

    println!("{}", ans)
}
