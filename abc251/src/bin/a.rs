use proconio::input;

fn main() {
    input! {s: String};
    match s.len() {
        1 => println!("{}", repeat_string(s, 6)),
        2 => println!("{}", repeat_string(s, 3)),
        3 => println!("{}", repeat_string(s, 2)),
        _ => println!("bug"),
    }
}

fn repeat_string(s: String, n: usize) -> String {
    return std::iter::repeat(s).take(n).collect();
}
