fn main() {
    proconio::input! {
        n: usize,
        l: usize,
        k: usize,
        mut ax: [usize; n],
    };

    ax.push(l);
    let check = |score: &usize| -> bool {
        let mut sep = 0;
        let mut seps: Vec<usize> = vec![];
        for a in ax.iter() {
            if (a - sep) >= *score {
                sep = *a;
                seps.push(*a);
            }
        }
        // l を ax に含んでいるため、k 分割の k+1 要素が全て条件を満たすためには
        // ax の最終要素についても seps に入っていないといけない。
        // そのため、seps.len() は k を超える必要がある。
        seps.len() > k
    };

    let mut lb = 0;
    let mut ub = l;
    loop {
        if ub - lb <= 1 {
            break;
        }
        let mid = (lb + ub) / 2;
        if check(&mid) {
            lb = mid;
        } else {
            ub = mid;
        }
    }

    println!("{}", lb);
}
