use num::abs_sub;

fn main() {
    proconio::input! {
        (x1,y1): (i32,i32),
        (x2,y2): (i32,i32)
    }

    println!("{}", (x1 - x2).abs() + (y1 - y2).abs() + 1)
}
