use proconio::derive_readable;
use proconio::input;

/// 043 - Is It Connected?
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
}

fn solve(n: usize, _m: usize, e: &[Edge]) -> &str {
    // 隣接リストの作成
    let mut adj = vec![vec![]; n + 1];
    for edge in e {
        adj[edge.a].push(edge.b);
        adj[edge.b].push(edge.a);
    }

    let mut visited = vec![false; n + 1];

    dfs(1, &mut visited, &adj);
    let mut answer = true;
    for i in 1..=n {
        if !visited[i] {
            answer = false;
            break;
        }
    }

    if answer {
        "The graph is connected."
    } else {
        "The graph is not connected."
    }
}

fn dfs(pos: usize, visited: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
    visited[pos] = true;
    for i in adj[pos].iter() {
        if !visited[*i] {
            dfs(*i, visited, adj);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(3, 2, &[Edge::new(1, 3), Edge::new(2, 3)]),
            "The graph is connected."
        );
    }
}
