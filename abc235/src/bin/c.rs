use std::collections::HashMap;

fn main() {
    proconio::input! {
        n:usize,
        q:usize,
        a: [usize; n],
        qq: [[usize; 2]; q],
    }
    let mut hash = HashMap::new();
    for i in 0..n {
        hash.entry(a[i]).or_insert(vec![]).push(i + 1);
    }
    for i in 0..q {
        let x = qq[i][0];
        let k = qq[i][1];
        let v = hash.get(&x);
        match v {
            Some(vv) => {
                if vv.len() >= k {
                    println!("{}", vv[k - 1])
                } else {
                    println!("{}", -1)
                }
            }
            None => {
                println!("{}", -1)
            }
        }
    }
}
