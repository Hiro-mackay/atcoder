fn main() {
    proconio::input! {
        mut v: i32,
        fam: [i32; 3]
    }

    let ans = ['F', 'M', 'T'];
    let mut i = 0;

    loop {
        v -= fam[i];

        if v < 0 {
            break;
        }

        i += 1;

        if i == 3 {
            i = 0
        }
    }

    println!("{}", ans[i])
}
