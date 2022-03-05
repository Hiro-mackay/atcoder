fn main() {
    proconio::input! {
        (n,m): (usize,usize),
        k: [[usize];m],
        p: [usize;m]
    }

    let mut ans = 0;

    for bit in 0..=1 << n {
        let mut r = true;
        for i in 0..m {
            let mut count = 0;
            for j in &k[i] {
                if (bit & (1 << j)) != 0 {
                    count += 1
                }
            }

            println!("{},{} ,{:?}", bit, count, k[i]);
            if count % 2 != p[i] {
                r = false;
                break;
            }
        }

        if r == true {
            ans += 1
        }
    }

    println!("{}", ans)
}
