use text_io::read;
fn main() {
    let n: usize = read!();
    // [lb, ub) の領域を二分探索する
    let mut lb = 0;
    let mut ub = n;
    for _ in 0..20 {
        let idx = ((ub + lb) as f64 / 2.0).floor() as usize;
        println!("? {}", idx);
        let v: usize = read!();
        if v == 0 {
            lb = idx;
        } else {
            ub = idx;
        }

        if ub - lb == 1 {
            println!("! {}", lb);
            return;
        }
    }
}
