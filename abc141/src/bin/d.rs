use std::collections::BinaryHeap;

fn main() {
    proconio::input! {
        n: u32,
        mut m: u32,
        a: [u64; n]
    }
    let mut bh = BinaryHeap::from(a);

    while m != 0 {
        let mm = bh.pop().unwrap() / 2;
        bh.push(mm);
        m -= 1;
    }

    println!("{}", bh.iter().fold(0, |sum, a| sum + a))
}
