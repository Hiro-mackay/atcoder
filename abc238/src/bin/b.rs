fn main() {
    proconio::input! {
        n:usize,
        a: [usize;n]
    }

    let mut q = vec![0];
    let mut rad = 0;

    for v in a {
        rad = (rad + v) % 360;
        q.push(rad);
    }

    q.push(360);

    q.sort();
    let mut max = 0;

    for i in 1..q.len() {
        max = max.max(q[i] - q[i - 1])
    }

    print!("{}", max)
}
