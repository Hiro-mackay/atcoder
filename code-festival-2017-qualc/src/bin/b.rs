fn calc(a: &Vec<Vec<i32>>, index: usize, ss: i32, mut sum: i32) -> i32 {
    for v in a[index] {
        if index < a.len() {
            return calc(a, index + 1, ss * v, sum);
        } else {
            if ss * v % 2 == 0 {
                sum += 1
            }
        }
    }
    return sum;
}

fn main() {
    proconio::input! {
        n:usize,
        a: [i32; n]
    }

    let mut s = vec![vec![0; 3]; n];

    for (i, v) in a.iter().enumerate() {
        s[i][0] = *v - 1;
        s[i][1] = *v;
        s[i][2] = *v + 1;
    }

    let ans = calc(&s, 0, 0, 0);

    println!("{}", ans)
}
