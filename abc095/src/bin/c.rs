fn main() {
    proconio::input! {
        a:u32,
        b:u32,
        c:u32,
        x:u32,
        y:u32
    }

    let mut ac = 0;
    let mut bc = 0;
    let mut cc = 0;

    if a + b > c * 2 {
        cc = std::cmp::min(x, y) * 2;
    } else {
        let min = std::cmp::min(x, y);
        ac = min;
        bc = min;
    }

    if ac < x && cc / 2 < x {
        if a < c * 2 {
            ac += x - (ac + cc / 2)
        } else {
            cc += (x - (ac + cc / 2)) * 2
        }
    }

    if bc < y && cc / 2 < y {
        if b < c * 2 {
            bc += y - (bc + cc / 2)
        } else {
            cc += (y - (bc + cc / 2)) * 2
        }
    }

    println!("{}", ac * a + bc * b + cc * c)
}
