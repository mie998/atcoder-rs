use proconio::input;

fn main() {
    input!{
        n: usize,
        ns: [(usize, usize); n],
        q: usize,
        qs: [(usize, usize); q],
    }

    
    let calc_cum_for_class = |m: usize| -> Vec<usize> {
        let mut cum = vec![0; n + 1];
        for (i, (l, r)) in ns.iter().enumerate() {
            if *l == m {
                cum[i + 1] = cum[i] + r;
            } else {
                cum[i + 1] = cum[i];
            }
        }
        cum
    };

    let cum_one = calc_cum_for_class(1);  
    let cum_two = calc_cum_for_class(2);  

    for (a, b) in qs {
        println!("{} {}", cum_one[b] - cum_one[a - 1], cum_two[b] - cum_two[a - 1]);
    }
}
