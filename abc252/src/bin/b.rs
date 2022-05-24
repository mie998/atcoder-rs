use proconio::input;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n], mut b: [usize; k]};

    let mut sorted_a = a.clone();
    sorted_a.sort_by(|x, y| y.cmp(x));
    let max = sorted_a[0];

    for (idx, val) in a.iter().enumerate() {
        if b.contains(&(idx + 1)) && *val >= max {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
