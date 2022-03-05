use itertools::Itertools;

fn main() {
    proconio::input! {
        (n,m): (usize,usize),
        ab: [(usize, usize); m],
        cd: [(usize, usize); m]
    }

    for perm in (1..=n).permutations(n) {
        let mut ans = false;
        for i in 0..m {
            ans = ab[i].0 == perm[cd[i].0 - 1] && ab[i].1 == perm[cd[i].1 - 1]
        }

        if ans {
            return print!("Yes");
        }
    }

    print!("No")
}
