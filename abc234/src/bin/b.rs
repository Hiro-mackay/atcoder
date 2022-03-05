fn main() {
    proconio::input! {
        n: usize,
        mat: [[i64; 2]; n]
    }

    let mut ans = 0;
    for i in 0..n {
        let lx = mat[i][0];
        let ly = mat[i][1];
        for j in 0..n {
            let rx = mat[j][0];
            let ry = mat[j][1];
            let x = lx - rx;
            let y = ly - ry;
            ans = std::cmp::max(ans, x * x + y * y);
        }
    }

    println!("{}", (ans as f64).sqrt())
}
