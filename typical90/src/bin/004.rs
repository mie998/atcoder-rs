fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }
    let mut rows = vec![0; h];
    let mut columns = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            rows[i] += a[i][j];
            columns[j] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", rows[i] + columns[j] - a[i][j]);
        }
        println!();
    }
}
