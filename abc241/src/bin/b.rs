use std::collections;

fn main() {
    proconio::input! {
        (n,m): (usize, usize),
        a:[i32;n],
        b:[i32;m]
    }
    let mut ss = collections::HashMap::new();

    for i in a {
        let r = ss.get(&i);
        match r {
            Some(j) => ss.insert(i, j + 1),
            None => ss.insert(i, 1),
        };
    }

    for i in b {
        let r = ss.get(&i);
        let ans = match r {
            Some(j) => {
                if *j > 0 {
                    ss.insert(i, j - 1);
                    true
                } else {
                    false
                }
            }
            None => false,
        };

        if !ans {
            return println!("No");
        }
    }
    println!("Yes")
}
