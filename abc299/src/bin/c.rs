use std::cmp::max;

fn main() {
    proconio::input! {
        _n: usize,
        ss: String,
    }

    let ans = if ss.contains('-') {
        ss.split('-').map(|s| s.len()).fold(0, max)
    } else {
        0
    };

    if ans == 0 {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
