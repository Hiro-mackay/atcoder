fn main() {
    proconio::input! {
        n: usize
    }
    let arr: Vec<_> = n.to_string().chars().collect();

    let a: i32 = format!("{}{}{}", arr[0], arr[1], arr[2]).parse().unwrap();
    let b: i32 = format!("{}{}{}", arr[1], arr[2], arr[0]).parse().unwrap();
    let c: i32 = format!("{}{}{}", arr[2], arr[0], arr[1]).parse().unwrap();

    print!("{}", a + b + c)
}
