use itertools::Itertools;

fn main() {
    proconio::input! {
        n: i64,
        x: i64,
        mut a: [i64; n],
    }
    let sa = a.into_iter().sorted().collect::<Vec<i64>>();
    let sac = sa.clone();
    for item in sa {
        let v = x + item;
        match sac.binary_search(&v) {
            Ok(_) => {
                println!("Yes");
                return;
            }
            Err(_) => continue,
        }
    }
    println!("No");
}
