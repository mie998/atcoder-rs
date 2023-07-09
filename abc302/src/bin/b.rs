use std::collections::VecDeque;
fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        s: [String; h],
    }

    let target: Vec<char> = vec!['s', 'n', 'u', 'k', 'e'];
    let c = s
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dx: Vec<isize> = vec![0, 1, 1, 1, 0, -1, -1, -1];
    let dy: Vec<isize> = vec![1, 1, 0, -1, -1, -1, 0, 1];
    for y in 0..h {
        for x in 0..w {
            if c[y][x] == target[0] {
                for i in 0..8 {
                    for j in 1..5 {
                        let cx: isize = dx[i] * j + x as isize;
                        let cy: isize = dy[i] * j + y as isize;
                        let bx: usize = if cx < 0 { w } else { cx as usize };
                        let by: usize = if cy < 0 { h } else { cy as usize };
                        let mut ans_stack: VecDeque<(usize, usize)> = VecDeque::new();
                        if bx < w && by < h {
                            // why this j referred as isize?
                            if c[by][bx] == target[j as usize] {
                                ans_stack.push_back((bx, by));
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }

                        // true answer
                        if ans_stack.len() == 4 {
                            ans_stack.push_front((x, y));
                            for i in 0..ans_stack.len() {
                                println!("{} {}", ans_stack[i].1 + 1, ans_stack[i].0 + 1);
                            }
                        }
                        return;
                    }
                }
            }
        }
    }
}
