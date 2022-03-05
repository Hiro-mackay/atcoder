use num_integer::Roots;

fn main() {
    proconio::input! {
        k:u64
    }

    let l = k.to_string().len();
    let mut ans = l + 1;

    for i in 1..k.sqrt() + 1 {
        if k % i == 0 {
            ans = ans.min(std::cmp::max(
                i.to_string().len(),
                (k / i).to_string().len(),
            ))
        }
    }
    println!("{}", ans);
}
