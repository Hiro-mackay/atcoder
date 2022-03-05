fn main() {
    proconio::input! {
        K:usize
    }

    let mut vec: Vec<usize> = Vec::new();

    let mut p = K;
    while p != 0 {
        vec.push((p % 2) * 2);
        p = p / 2;
    }

    if vec[vec.len() - 1] == 0 {
        vec.remove(vec.len() - 1);
    }

    let ans = vec
        .iter()
        .rev()
        .map(|i| (i).to_string())
        .collect::<String>();

    println!("{}", ans);
}
