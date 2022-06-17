use proconio::input;

fn main() {
    input! {n: i64, k: i64, bs: [i64; k], ps: [(i64, i64); n]};
    let mut have_torches = vec![];
    let mut not_have_torches = vec![];
    for (i, p) in ps.iter().enumerate() {
        if bs.contains(&((i as i64) + 1i64)) {
            have_torches.push(p);
        } else {
            not_have_torches.push(p);
        }
    }

    let mut scores = vec![];
    for nhp in not_have_torches.iter() {
        let mut score = 1000000f64;
        for hp in have_torches.iter() {
            let x2 = (hp.0 - nhp.0).pow(2);
            let y2 = (hp.1 - nhp.1).pow(2);
            let dist = ((x2 + y2) as f64).sqrt();
            if dist < score {
                score = dist;
            }
        }
        scores.push(score);
    }
    scores.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println! {"{}", scores[0]}
}
