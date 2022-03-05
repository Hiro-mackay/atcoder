fn main() {
    proconio::input! {
        (n,m): (usize,usize),
        h:[i32;n],
        uv: [(usize, usize); m]
    }

    let nega = -999999999;

    let mut t = vec![nega; n];
    let mut max = 0;
    t[0] = 0;

    for (u, v) in uv {
        let hu = h[u - 1];
        let hv = h[v - 1];
        let mut prev = t[u - 1];
        if prev == nega {
            prev = 0
        }
        if hu > hv {
            t[v - 1] = prev + (hu - hv)
        } else if hu < hv {
            if t[v - 1] == nega {
                t[v - 1] = prev + (-2 * (hv - hu))
            } else {
                t[v - 1] = t[v - 1].max(prev + (-2 * (hv - hu)))
            }
        } else {
            t[v - 1] = prev
        }

        max = max.max(t[v - 1]);
    }

    println!("{:?}", max)
}
