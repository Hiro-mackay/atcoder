fn calc() {}

fn main() {
    proconio::input! {
        n:usize
    }

    let mut arr = Vec::new();

    for i in (0..(2 * n) - 1).rev() {
        proconio::input! {
            a:[f64; i + 1]
        }
        arr.push(a);
    }

    let mut p = vec![0; 2 * n];
    let mut result = 0f64;

    for (i, _arr) in arr.iter().enumerate() {
        let mut _max = 0f64;
        let mut index = 0;
        if p[i] == 0 {
            p[i] = 1;

            for (j, v) in _arr.iter().enumerate() {
                if _max < *v {
                    index = j + i + 1;
                    _max = *v;
                }
            }
            p[index] = 1;
            result += _max
        }
    }

    println!("{}", result)
}
