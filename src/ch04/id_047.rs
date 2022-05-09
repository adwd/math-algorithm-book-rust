use proconio::derive_readable;
use proconio::input;

/// 047 - Bipartite Graph
fn main() {
    input! {
        n: usize,
        m: usize,
        e: [Edge; m],
    }

    println!("{}", solve(n, m, &e));
}

#[derive_readable]
#[derive(Debug)]
struct Edge {
    a: usize,
    b: usize,
}

impl Edge {
    fn new(a: usize, b: usize) -> Self {
        Self { a, b }
    }

    fn from_array(arr: &[(usize, usize)]) -> Vec<Self> {
        arr.iter().map(|&(a, b)| Self::new(a, b)).collect()
    }
}

fn solve(n: usize, _m: usize, e: &[Edge]) -> &str {
    // 隣接リストの作成
    let mut adj = vec![vec![]; n + 1];
    for edge in e {
        adj[edge.a].push(edge.b);
        adj[edge.b].push(edge.a);
    }

    // 0: white, 1: red, 2: black
    let mut color = vec![0; n + 1];
    for i in 1..=n {
        if color[i] == 0 {
            color[i] = 1;
            dfs(i, &mut color, &adj);
        }
    }

    for edge in e {
        if color[edge.a] == color[edge.b] {
            return "No";
        }
    }

    "Yes"
}

fn dfs(pos: usize, color: &mut Vec<u8>, adj: &Vec<Vec<usize>>) {
    for i in adj[pos].iter() {
        if color[*i] == 0 {
            color[*i] = 3 - color[pos];
            dfs(*i, color, adj);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edges = Edge::from_array(&[(1, 5), (1, 6), (2, 7), (3, 7), (4, 6), (5, 8), (6, 8)]);
        assert_eq!(solve(8, 7, &edges), "Yes");
    }
    #[test]
    fn it_works2() {
        let edges = Edge::from_array(&[(1, 6), (2, 6), (3, 6), (2, 4), (3, 5), (1, 3), (1, 4)]);
        assert_eq!(solve(6, 7, &edges), "No");
    }
}
