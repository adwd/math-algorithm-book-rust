use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(usize, usize, usize); m],
    }

    println!("{}", solve(n, m, &e));
}

fn solve(n: usize, m: usize, e: &[(usize, usize, usize)]) -> i64 {
    // 隣接リストの作成
    let mut adj = vec![vec![]; n + 1];
    for edge in e {
        adj[edge.0].push((edge.1, edge.2));
        adj[edge.1].push((edge.0, edge.2));
    }

    let dist = dijkstra(n, m, &adj);

    if dist[n] == 1 << 60 {
        -1
    } else {
        dist[n] as i64
    }
}

fn dijkstra(n: usize, m: usize, adj: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut dist = vec![1 << 60; n + 1];
    let mut used = vec![false; n + 1];
    dist[1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((0, 1));

    while let Some(v) = queue.pop_front() {
        let pos = v.1;
        if used[pos] {
            continue;
        }
        used[pos] = true;

        for i in adj[pos].iter() {
            let to = i.0;
            let mut cost = dist[pos] + i.1;
            if pos == 0 {
                cost = i.1;
            }
            if dist[to] > cost {
                dist[to] = cost;
                queue.push_back((dist[to], to));
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edges = [(1, 2, 3), (1, 3, 4), (3, 4, 1), (2, 4, 10)];
        assert_eq!(solve(4, 4, &edges), 5);

        assert_eq!(solve(2, 0, &[]), -1);
        assert_eq!(solve(3, 1, &[(1, 2, 1)]), -1);
        assert_eq!(solve(3, 1, &[(2, 3, 1)]), -1);
    }
}
