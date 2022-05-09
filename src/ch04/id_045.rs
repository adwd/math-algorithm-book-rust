use proconio::derive_readable;
use proconio::input;

/// 045 - Easy Graph Problem
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

fn solve(n: usize, _m: usize, e: &[Edge]) -> usize {
    // 隣接リストの作成
    let mut adj = vec![vec![]; n + 1];
    for edge in e {
        adj[edge.a].push(edge.b);
        adj[edge.b].push(edge.a);
    }

    let mut count = 0;
    for i in 1..=n {
        let mut lesser = 0;
        for e in adj[i].iter() {
            if e < &i {
                lesser += 1;
            }
        }
        if lesser == 1 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                5,
                5,
                &[
                    Edge::new(1, 2),
                    Edge::new(1, 3),
                    Edge::new(3, 2),
                    Edge::new(5, 2),
                    Edge::new(4, 2)
                ]
            ),
            3
        );
        assert_eq!(solve(2, 1, &[Edge::new(1, 2)]), 1);
    }
}
