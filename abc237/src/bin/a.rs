fn main() {
    proconio::input! {
        n:i64
    }

    let posi = 2i64.pow(31);
    let nega = posi * -1;

    if nega <= n && n < posi {
        return println!("Yes");
    }

    println!("No")
}
