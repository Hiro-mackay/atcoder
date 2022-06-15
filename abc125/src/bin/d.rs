fn main() {
    proconio::input! {
        n:usize,
        a:[i64; n]
    }

    let ans = a
        .iter()
        .map(|&x| if x < 0 { x * -1 } else { x })
        .fold(0, |acc, x| {
            println!("{}", x);
            acc + x
        });

    println!("{}", ans)
}
