use proconio::marker::Chars;

fn main() {
    proconio::input! {
        (n,m) : (usize,usize),
        a : [Chars;n],
        b : [Chars;m]
    }

    let mut ans = false;

    for i in 0..=n - m {
        for j in 0..=n - m {
            ans = true;
            for bi in 0..m {
                for bj in 0..m {
                    if a[i + bi][j + bj] != b[bi][bj] {
                        ans = false;
                        break;
                    }
                }
                if ans == false {
                    break;
                }
            }
            if ans == true {
                return println!("Yes");
            }
        }
    }

    println!("No")
}
