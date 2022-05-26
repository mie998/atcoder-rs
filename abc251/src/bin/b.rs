use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, w: usize, mut a: [usize; n]
    }

    let mut dp = vec![false; w + 5];
    for i in 1..4 {
        for vars in a.iter().combinations(i) {
            let sum = vars.iter().fold(0, |s, x| s + *x);
            if sum <= w {
                dp[sum] = true;
            }
        }
    }

    println!(
        "{}",
        dp.iter().fold(0, |sum, b| sum + if *b { 1 } else { 0 })
    )
}
