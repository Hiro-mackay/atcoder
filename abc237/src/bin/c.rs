fn main() {
    proconio::input! {
       mut s:proconio::marker::Chars
    }

    let mut ind = s.len();
    let mut tar = 0;

    for v in s.iter().rev() {
        if *v != 'a' {
            break;
        } else {
            if s[tar] == 'a' {
                tar += 1
            }
            ind -= 1
        }
    }

    if ind <= tar {
        return println!("Yes");
    }

    let str = &s[tar..ind];

    let len = ind - 1 - tar;

    for (i, v) in str.iter().enumerate() {
        if v != &str[len - i] {
            return println!("No");
        }
    }
    return println!("Yes");
}
