use proconio::input;

const S_LENGTH: usize = 10;

fn main() {
    input! { n: usize, ss: [String; n] }
    let mut ans = 9999;
    for i in 0..S_LENGTH {
        let mut sum = 0;
        for j in 0..S_LENGTH {
            let mut cnt = 0;
            for s in ss.iter() {
                if s.chars().nth(j).unwrap() == std::char::from_digit(i as u32, 10).unwrap() {
                    sum = std::cmp::max(j + 10 * cnt, sum);
                    cnt += 1;
                }
            }
        }
        ans = std::cmp::min(ans, sum);
    }
    println!("{}", ans);
}
