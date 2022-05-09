use std::collections::VecDeque;

use proconio::derive_readable;
use proconio::input;
use proconio::marker::Chars;

/// 046 - 幅優先探索
fn main() {
    // 行数 R(1 ≦ R ≦ 50)
    // 列数 C(1 ≦ C ≦ 50)
    // スタート地点(sx, sy)
    // ゴール地点(gx, gy)
    input! {
        r: usize,
        c: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
        m: [Chars; r],
    }

    println!("{}", solve(r, c, sy, sx, gy, gx, &m));
}

#[derive_readable]
#[derive(Debug, Clone)]
struct Edge {
    x: usize,
    y: usize,
}

impl Edge {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn solve(r: usize, c: usize, sy: usize, sx: usize, gy: usize, gx: usize, m: &[Vec<char>]) -> usize {
    let adj = adj_matrix(r, c, m);

    let mut dist = vec![vec![-1; c + 1]; r + 1];
    dist[sy][sx] = 0;

    let mut q = VecDeque::new();
    q.push_back((sy, sx));

    while let Some((y, x)) = q.pop_front() {
        for nex in adj[y][x].iter() {
            if dist[nex.y][nex.x] == -1 {
                dist[nex.y][nex.x] = dist[y][x] + 1;
                q.push_back((nex.y, nex.x));
            }
        }
    }

    // println!("{:?}", adj);
    // print_result(&dist);

    dist[gy][gx] as usize
}

fn adj_matrix(r: usize, c: usize, m: &[Vec<char>]) -> Vec<Vec<Vec<Edge>>> {
    // 地図は1始まりの値が入っている

    let mut adj = vec![vec![vec![]; c + 1]; r + 1];
    for y in 2..r {
        for x in 2..c {
            if map(x, y, m) == '#' {
                continue;
            }

            if map(x + 1, y, m) == '.' {
                adj[y][x].push(Edge::new(x + 1, y));
                adj[y][x + 1].push(Edge::new(x, y));
            }
            if map(x, y + 1, m) == '.' {
                adj[y][x].push(Edge::new(x, y + 1));
                adj[y + 1][x].push(Edge::new(x, y));
            }
        }
    }

    adj
}

fn map(x: usize, y: usize, m: &[Vec<char>]) -> char {
    m[y - 1][x - 1]
}

fn print_result(dist: &[Vec<i32>]) {
    for row in dist.iter().skip(1) {
        for d in row.iter().skip(1) {
            if *d == -1 {
                print!("#");
            } else {
                print!("{}", d);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(solve(2, 1, &[Edge::new(1, 2)]), 1);
    }
}
