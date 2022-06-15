use std::collections::HashMap;

fn main() {
    proconio::input! {
        n:usize,
        xy: [(usize, usize); n],
        s: proconio::marker::Chars
    }
    let mut map: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();

    for i in 0..n {
        let v = if s[i] == 'R' { 1 } else { -1 };
        let hash = map.entry(xy[i].1).or_insert([].to_vec());

        let t = (xy[i].0, v);
        hash.push(t);
    }

    for (_, v) in map.iter().clone() {
        if v.len() < 2 {
            continue;
        }

        let mut a = v.clone();

        a.sort_by_key(|k| k.0);

        let mut r = false;

        for (_, b) in a {
            if b == 1 {
                r = true
            } else {
                if r == true {
                    return println!("Yes");
                }
            }
        }
    }

    return println!("No");
}
