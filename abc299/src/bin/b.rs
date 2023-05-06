use itertools::enumerate;

fn main() {
    proconio::input! {
        n: usize,
        t: usize,
        cs: [usize; n],
        rs: [usize; n],
    }
    let mut ans_t: (usize, usize) = (0, 0);
    let mut ans_1: (usize, usize) = (0, 0);
    for (i, c) in enumerate(cs.iter()) {
        if *c == t {
            if ans_t.0 < rs[i] {
                ans_t = (rs[i], i + 1);
            }
        }
        if *c == cs[0] {
            if ans_1.0 < rs[i] {
                ans_1 = (rs[i], i + 1);
            }
        }
    }
    if ans_t.0 != 0 {
        println!("{}", ans_t.1)
    } else {
        println!("{}", ans_1.1)
    }
}
