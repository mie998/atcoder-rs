use proconio::input;

const BLACK: char = '#';
const WHITE: char = '.';
fn main() {
    input! {n: isize, a: isize, b: isize};
    let mut original_tile = String::from("");
    for _ in 0..a {
        for _ in 0..b {
            original_tile.push(WHITE);
        }
        original_tile.push('\n')
    }
    original_tile = original_tile.trim().to_string();

    let mut ans_tile = String::from("");
    for _ in 0..n {
        for _ in 0..n {
            let tile = black2white(&original_tile);
            ans_tile = push_str_row(&ans_tile, &tile).to_string();
        }
        ans_tile.push('\n')
    }
    print!("{}", ans_tile);
}

fn black2white(s: &String) -> String {
    let mut out = String::from("");
    for c in s.chars() {
        if c == BLACK {
            out.push(WHITE);
        } else {
            out.push(c);
        }
    }
    return out;
}
fn white2black(s: &String) -> String {
    let mut out = String::from("");
    for c in s.chars() {
        if c == WHITE {
            out.push(BLACK);
        } else {
            out.push(c);
        }
    }
    return out;
}

fn push_str_row(s1: &str, s2: &str) -> String {
    let mut split1 = s1.split('\n');
    let mut split2 = s2.split('\n');
    let mut result = String::from("");
    loop {
        match (split1.next(), split2.next()) {
            (Some(sp1), Some(sp2)) => {
                result.push_str(sp1);
                result.push_str(sp2);
                result.push('\n');
            }
            (Some(_), None) => {
                println!("there is remaining chars in parameter1");
                break;
            }
            (None, Some(_)) => {
                println!("there is remaining chars in parameter2");
                break;
            }
            (None, None) => break,
        }
    }
    return result;
}
