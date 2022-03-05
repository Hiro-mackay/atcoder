use std::process;

fn main() {
    proconio::input! {
        s: proconio::marker::Chars
    }

    let len = s.len();
    let mut max = 0;
    let acgt = ['A', 'C', 'G', 'T'];

    for i in 0..len {
        let mut c = 0;
        for j in i..len {
            if acgt.contains(&s[j]) {
                c += 1;
            } else {
                break;
            }
        }

        max = max.max(c);
        c = 0
    }
    println!("{}", max)
}
