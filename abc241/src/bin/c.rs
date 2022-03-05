use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n:usize,
        s:[Chars; n]
    }

    for i in 0..n {
        for j in 0..n {
            let mut y = 0;
            let mut t = 0;
            let mut nn = 0;
            let mut nnn = 0;
            
            for k in 0..6 {
                if j < n - 5 {
                    if s[i][j + k] == '#' {
                        y += 1
                    }
                }
                if i < n - 5 {
                    if s[i + k][j] == '#' {
                        t += 1
                    }
                }
                if j < n - 5 && i < n - 5 {
                    if s[i + k][j + k] == '#' {
                        nn += 1
                    }
                    if s[i + 5 - k][j + k] == '#' {
                        nnn += 1
                    }
                }
            }
            if y >= 4 || t >= 4 || nn >= 4 || nnn >= 4 {
                return println!("Yes");
            }
        }
    }

    println!("No")
}
