use proconio::{fastout, input};
use std::collections::HashSet;

struct Best {
    index: usize,
    score: usize,
}

#[fastout]
fn main() {
    input! {n: usize, a: [(String,usize); n]};

    let mut used = HashSet::<String>::new();
    let mut best = Best { index: 0, score: 0 };

    for (i, (s, t)) in a.into_iter().enumerate() {
        if used.contains(&s) {
            continue;
        }

        used.insert(s);
        if best.score < t {
            best.score = t;
            best.index = i;
        }
    }

    println!("{}", best.index + 1);
}
