use im_rc::HashMap;

fn main() {
    proconio::input! {
        (n,m): (usize,usize),
        s: [String; n],
        t: [String;m]
    }

    let mut map = HashMap::new();

    for _t in t {
        map.insert(_t, true);
    }

    for _s in s {
        let hash = map.get(&_s);
        match hash {
            Some(_h) => {
                println!("Yes")
            }
            None => {
                println!("No")
            }
        }
    }
}


