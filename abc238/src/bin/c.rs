fn main() {
    proconio::input! {
        n:u64
    }

    let mut ten = 10u64;

    let mut ans = 0;

    let mut ama = n % ten;
    let mut mas = (n / ten) * ten;

    for i in 1..=ama {
        ans += i
    }

    let calc = 55;

    loop {
        if mas > ten {
            ten *= 10;
        } else {
            break;
        }
        ama = mas % ten;
        mas = (n / ten) * ten;

        ans += (ama - 1) * ten * calc + 1
    }

    println!("{}", ans)
}
