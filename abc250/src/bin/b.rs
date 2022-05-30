use proconio::input;

const BLACK: char = '#';
const WHITE: char = '.';
fn main() {
    input! {n: usize, a: usize, b: usize};
    let mut board = vec![vec![WHITE; n * b]; n * a];
    for ni in 0..n {
        for nj in 0..n {
            let ch = if (ni + nj) % 2 == 1 { BLACK } else { WHITE };
            for i in (ni * b)..(ni * b + b) {
                for j in (nj * a)..(nj * a + a) {
                    board[j][i] = ch;
                }
            }
        }
    }
    let ans = board
        .iter()
        .map(|vc| vc.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", ans);
}
