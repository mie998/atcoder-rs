#[derive(Debug, Clone, Copy)]
struct Node {
    index: usize,
    dist: usize,
}

fn main() {
    proconio::input! {
        n: usize,
        bs: [(usize, usize); n - 1],
    }

    let mut paths: Vec<Vec<usize>> = vec![vec![]; n];
    for b in bs {
        paths[b.0 - 1].push(b.1 - 1);
        paths[b.1 - 1].push(b.0 - 1);
    }

    let mut seen: Vec<bool> = vec![false; n];
    let node = dfs(&paths, &mut seen, 0, 0);
    seen = vec![false; n];
    let ans_node = dfs(&paths, &mut seen, node.index, 0);
    println!("{}", ans_node.dist + 1);
}

fn dfs(paths: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize, dist: usize) -> Node {
    if seen[v] {
        return Node {
            index: v,
            dist: dist,
        };
    }
    let mut nodes: Vec<Node> = vec![];
    seen[v] = true;
    for u in paths[v].iter() {
        let node = dfs(paths, seen, *u, dist + 1);
        nodes.push(node);
    }

    nodes.sort_by(|a, b| b.dist.cmp(&a.dist));
    nodes[0]
}
