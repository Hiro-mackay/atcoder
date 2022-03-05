fn main() {
    proconio::input! {
        n:usize,
        s: proconio::marker::Chars
    }

    let mut ans = 0;

    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                let arr = vec![i, j, k];
                let mut ind = 0;

                for si in 0..n {
                    if s[si] as i32 - 48 == arr[ind] {
                        ind += 1
                    }

                    if ind == 3 {
                        ans += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", ans)
}
