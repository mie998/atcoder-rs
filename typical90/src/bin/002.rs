fn int_to_parence(bit: usize) -> String {
    let mut s = String::new();
    let mut bit_cp = bit;
    while bit_cp > 0 {
        if bit_cp & 1 == 1 {
            s.push('(');
        } else {
            s.push(')');
        }
        bit_cp >>= 1;
    }

    s.chars().rev().collect::<String>()
}

fn main() {
    proconio::input! {
        n: usize,
    }

    if n % 2 == 1 {
        return;
    }

    let is_valid = |s: &str| -> bool {
        s.chars().fold(0, |acc, c| {
            if acc < 0 {
                return 10000;
            }

            if c == '(' {
                return acc + 1;
            } else {
                return acc - 1;
            }
        }) == 0
    };

    let mut ans: Vec<String> = vec![];
    for mut bit in 0..(1 << (n - 1)) {
        bit += 1 << (n - 1);
        let s = int_to_parence(bit);
        if s.len() == n && is_valid(&s.clone()) {
            ans.push(s);
        }
    }

    ans.sort();
    for s in ans {
        println!("{}", s);
    }
}
