use itertools::Itertools;

fn main() {
    proconio::input! {
        h: usize, w: usize, xs: [String; h]
    }
    for s in xs {
        let mut vs = s.trim().chars().collect_vec();
        for j in 1..vs.len() {
            if vs[j - 1] == 'T' && vs[j] == 'T' {
                vs[j - 1] = 'P';
                vs[j] = 'C';
            }
        }
        println!("{}", vs.iter().join(""));
    }
}
