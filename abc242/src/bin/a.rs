fn main() {
    proconio::input! {
        (a,b,c,x): (f64,f64,f64,f64)
    }

    if x <= a {
        return println!("1.000000000000");
    }

    if b < x {
        return println!("0.000000000000");
    }

    return println!("{:.12}", c / (b - a));
}
