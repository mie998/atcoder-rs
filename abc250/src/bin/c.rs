use proconio::input;

fn main() {
    input! { n: usize, q: usize, xs: [usize; q] };
    let mut v: Vec<usize> = (1..n + 1).into_iter().collect();
    let mut pos: Vec<usize> = (0..n + 1)
        .into_iter()
        .map(|x| if x >= 1 { x - 1 } else { 0 })
        .collect();

    for x in xs {
        let idx = pos[x];
        if idx >= v.len() {
            panic!("Segmentation fault");
        }

        if idx == n - 1 {
            let item1 = x;
            let item2 = v[pos[v[idx - 1]]];
            pos.swap(item1, item2);
            v.swap(idx - 1, idx);
        } else {
            let item1 = x;
            let item2 = v[pos[v[idx + 1]]];
            pos.swap(item1, item2);
            v.swap(idx, idx + 1);
        }
    }

    println!(
        "{}",
        v.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
