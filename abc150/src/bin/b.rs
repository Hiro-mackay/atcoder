fn main() {
    proconio::input! {
        n:usize,
        a:proconio::marker::Chars
    }

    let diff = ['A', 'B', 'C'];
    let mut ind = 0;
    let mut ans = 0;

    for c in a {
        if diff[ind] == c {
            ind += 1
        } else if diff[0] == c {
            ind = 1
        } else {
            ind = 0
        }

        if ind == 3 {
            ind = 0;
            ans += 1
        }
    }

    println!("{}", ans)
}
