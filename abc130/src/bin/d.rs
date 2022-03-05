fn main() {
    proconio::input! {
        (n,k):(usize,usize),
        a: [usize;n],
    }

    let mut ans = 0;
    let mut arr = vec![0; n];
    let mut l = 0;

    arr[0] = a[0];
    for i in 1..n {
        arr[i] = a[i] + arr[i - 1]
    }

    for r in 0..n {
        if arr[r] >= k {
            ans += 1;

            while l < r && arr[r] - arr[l] >= k {
                l += 1
            }
            ans += l
        }
    }

    println!("{}", ans);
}
