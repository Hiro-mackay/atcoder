fn main() {
    proconio::input! {
        n:usize,
        a: [i32; n],
    }

    let mut o = 0;
    let mut oi = 0;
    let mut e = 0;
    let mut ei = 0;

    for (i, v) in a.iter().enumerate() {
        if i % 2 == 0 {
            oi += 1;
            o += *v
        } else {
            ei += 1;
            e += *v
        }
    }

    println!("{} , {} , {} , {}", o, oi, e, ei);
    println!("{:?}", std::cmp::max(o / oi, e / ei));

    let ss = n / 2;

    println!("{}", std::cmp::max(a[ss - 1], a[ss]));
}
