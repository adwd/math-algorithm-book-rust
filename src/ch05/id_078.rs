use std::collections::VecDeque;

use proconio::derive_readable;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [Edge; m],
    }

    solve(n, m, &e).iter().for_each(|v| println!("{}", v));
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
        arr.iter().map(|&(x, y)| Self::new(x, y)).collect()
    }
}

fn solve(n: usize, _m: usize, e: &[Edge]) -> Vec<i64> {
    // 隣接リストの作成
    let mut adj = vec![vec![]; n + 1];
    for edge in e {
        adj[edge.a].push(edge.b);
        adj[edge.b].push(edge.a);
    }

    let mut dist = vec![-1; n + 1];
    dist[1] = 0;

    let mut q = VecDeque::new();
    q.push_back(1);

    while let Some(pos) = q.pop_front() {
        for nex in adj[pos].iter() {
            if dist[*nex] == -1 {
                dist[*nex] = dist[pos] + 1;
                q.push_back(*nex);
            }
        }
    }

    dist.into_iter()
        .skip(1)
        .map(|v| if v == -1 { 120 } else { v.min(120) })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edges = Edge::from_array(&[(1, 2), (1, 3), (2, 4), (2, 5), (3, 4), (4, 6), (5, 6)]);
        assert_eq!(solve(6, 7, &edges), vec![0, 1, 1, 2, 2, 3]);
    }
}
