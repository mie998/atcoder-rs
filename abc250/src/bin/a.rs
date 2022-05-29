use proconio::input;

fn main() {
    input! { h: isize, w: isize, r: isize, c: isize, };
    let mut score = 0;
    score += judge_adjacent(r, 1, h);
    score += judge_adjacent(c, 1, w);
    println!("{}", score);
}

fn judge_adjacent(x: isize, lower_limit: isize, upper_limit: isize) -> isize {
    let mut var = 0;
    if (x - 1) >= lower_limit && (x - 1) <= upper_limit {
        var += 1;
    }
    if (x + 1) <= upper_limit {
        var += 1;
    }
    return var;
}
