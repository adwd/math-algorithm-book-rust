use std::collections::VecDeque;

use proconio::derive_readable;
use proconio::input;

/// 044 - Shortest Path Problem
/// この問題は、書籍中のコード 4.5.3 に対応します。
/// N 頂点 M 辺の無向グラフが与えられます。各頂点には 1 から N までの番号が付けられており、
/// i 番目の辺は頂点 Ai と頂点 Bi を結んでいます。
/// 1 以上 N 以下の全ての整数 k について、以下の問いに答えてください。
/// 頂点 1 から頂点 k まで、辺を何本かたどって移動することを考えるとき、
/// たどるべき辺の本数の最小値を出力せよ。ただし、移動不可能な場合は −1 を出力せよ。
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
}

fn solve(n: usize, _m: usize, e: &[Edge]) -> Vec<i32> {
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

    dist.into_iter().skip(1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(3, 2, &[Edge::new(1, 3), Edge::new(2, 3)]),
            vec![0, 2, 1]
        );
    }
}
