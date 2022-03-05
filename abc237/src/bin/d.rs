fn main() {
    proconio::input! {
        n: usize,
        a: proconio::marker::Chars
    }

    let mut ans = vec![];
    let mut d = false;

    for (i, v) in a.iter().enumerate() {
        if v == &'L' {
            ans.push(i)
        } else {
            if d {
                print!(" {}", i)
            } else {
                d = true;
                print!("{}", i)
            }
        }
    }

    if d {
        print!(" {}", n);
    } else {
        print!("{}", n);
    }

    for v in ans.iter().rev() {
        print!(" {}", v)
    }
}
