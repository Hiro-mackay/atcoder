use std::collections::HashMap;

fn main() {
    proconio::input! {
        n:usize,
        a: [usize; n],
        b: [usize; n]
    }

    let mut map = HashMap::new();
    let mut comp = 0;

    for i in 0..n {
        if a[i] == b[i] {
            comp += 1;
        } else {
            let c = map.entry(a[i]).or_insert(0);
            *c += 1;
            let c = map.entry(b[i]).or_insert(0);
            *c += 1
        }
    }
    println!("{}", comp);

    comp = 0;
    for (_, v) in map.iter() {
        if *v == 2 {
            comp += 1
        }
    }

    println!("{}", comp)
}
