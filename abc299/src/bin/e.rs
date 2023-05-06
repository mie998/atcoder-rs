use std::collections::VecDeque;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
        k: usize,
        conds: [(usize, usize); k],
    }

    let mut edge_by_vertex = vec![vec![]; n];
    for (from, to) in edges {
        edge_by_vertex[from - 1].push(to - 1);
        edge_by_vertex[to - 1].push(from - 1);
    }

    let mut dists = vec![vec![100000; n]; n];
    for i in 0..n {
        let mut seen = vec![false; n];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        seen[i] = true;
        dists[i][i] = 0;
        for e in edge_by_vertex[i].iter() {
            queue.push_back((*e, 1));
        }
        while !queue.is_empty() {
            let (v, cost) = queue.pop_front().unwrap();
            if seen[v] {
                continue;
            }
            seen[v] = true;
            dists[i][v] = cost;
            for e in edge_by_vertex[v].iter() {
                queue.push_back((*e, cost + 1));
            }
        }
    }

    let mut ans = vec!['1'; n];
    conds.iter().for_each(|(p, d)| {
        (0..n).into_iter().for_each(|j| {
            if dists[*p - 1][j] < *d {
                ans[j] = '0';
            }
        })
    });

    let is_ok = conds.into_iter().all(|(p, d)| {
        (0..n)
            .into_iter()
            .any(|j| dists[p - 1][j] == d && ans[j] == '1')
    });

    if is_ok && ans.contains(&'1') {
        println!("Yes\n{}", ans.iter().collect::<String>());
    } else {
        println!("No");
    }
}
