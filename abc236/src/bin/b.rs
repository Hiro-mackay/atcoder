fn main() {
    proconio::input! {
        n:usize,
        a: [usize; (4*n)-1]
    }

    let mut ans = vec![0; n];

    for _a in a {
        ans[_a - 1] = ans[_a - 1] + 1
    }
    for (i, _ans) in ans.iter().enumerate() {
        if *_ans < 4 {
            return println!("{}", i + 1);
        }
    }
}
